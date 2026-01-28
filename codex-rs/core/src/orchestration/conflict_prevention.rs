//! Conflict prevention utilities for worktree competition
//!
//! In worktree-based parallel development, merge conflicts are prevented by isolation.
//! This module adds *prediction + coordination* to avoid overlapping edits early and
//! to produce logs / A2A signals for transparency.

use crate::orchestration::worktree_manager::WorktreeInfo;
use anyhow::Context;
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;

#[cfg(feature = "custom-features")]
use crate::a2a_communication::{
    A2AMessage, A2ACommunicationManager, AgentIdentity, MessagePayload, MessagePriority, MessageType,
};
#[cfg(feature = "custom-features")]
use crate::security::SecurityContext;

/// Predicted overlap between worktrees (file-level).
#[derive(Debug, Clone)]
pub struct ConflictPredictionSummary {
    /// worktree_name -> modified files
    pub modified_files: BTreeMap<String, BTreeSet<String>>,
    /// Pairwise overlaps: "A<->B" -> files
    pub overlaps: BTreeMap<String, Vec<String>>,
}

/// Predict potential conflicts by comparing modified file sets between worktrees.
///
/// This uses `git diff --name-only <base>..<branch>` on the repository root.
pub fn predict_file_overlaps(
    repo_root: &std::path::Path,
    base_branch: &str,
    worktrees: &[WorktreeInfo],
) -> Result<ConflictPredictionSummary> {
    let mut modified_files: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for wt in worktrees {
        let files = git_changed_files(repo_root, base_branch, &wt.branch)
            .with_context(|| format!("Failed to compute changed files for {}", wt.name))?;
        modified_files.insert(wt.name.clone(), files);
    }

    let mut overlaps: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let names: Vec<String> = modified_files.keys().cloned().collect();
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            let a = &names[i];
            let b = &names[j];
            let set_a = modified_files.get(a).unwrap();
            let set_b = modified_files.get(b).unwrap();
            let common: Vec<String> = set_a
                .intersection(set_b)
                .cloned()
                .collect();
            if !common.is_empty() {
                overlaps.insert(format!("{a}<->{b}"), common);
            }
        }
    }

    Ok(ConflictPredictionSummary {
        modified_files,
        overlaps,
    })
}

impl ConflictPredictionSummary {
    pub fn to_json(&self) -> serde_json::Value {
        let modified_files: BTreeMap<String, Vec<String>> = self
            .modified_files
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().cloned().collect()))
            .collect();
        serde_json::json!({
            "modified_files": modified_files,
            "overlaps": self.overlaps,
        })
    }
}

/// Broadcast conflict/overlap summary to other agents via A2A (best-effort).
#[cfg(feature = "custom-features")]
pub async fn broadcast_conflict_summary(
    manager: &A2ACommunicationManager,
    sender: &AgentIdentity,
    summary: &ConflictPredictionSummary,
    correlation_id: Option<String>,
) -> Result<()> {
    let payload = serde_json::json!({
        "kind": "worktree_conflict_prediction",
        "summary": summary.to_json(),
    });

    let message = A2AMessage {
        id: uuid::Uuid::new_v4().to_string(),
        sender: sender.clone(),
        receiver: None,
        message_type: MessageType::CoordinationSignal,
        payload: MessagePayload::Custom(payload),
        priority: MessagePriority::Normal,
        ttl: std::time::Duration::from_secs(60),
        timestamp: chrono::Utc::now(),
        correlation_id,
        security_context: SecurityContext::default(),
    };

    manager
        .send_message(message)
        .await
        .map(|_| ())
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}

fn git_changed_files(
    repo_root: &std::path::Path,
    base_branch: &str,
    branch: &str,
) -> Result<BTreeSet<String>> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["diff", "--name-only", &format!("{base_branch}..{branch}")])
        .output()
        .context("Failed to run git diff --name-only")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = BTreeSet::new();
    for line in stdout.lines() {
        let p = line.trim();
        if !p.is_empty() {
            files.insert(p.to_string());
        }
    }
    Ok(files)
}

