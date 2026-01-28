//! Integrated worktree competition runner
//!
//! Glue layer that combines:
//! - Git worktree isolation (conflict prevention)
//! - Parallel execution (resource-limited)
//! - Optional A2A coordination signals (if enabled)
//! - QC scoring + optimization bonus
//! - Winner merge + detailed logging

use crate::orchestration::conflict_prevention::predict_file_overlaps;
use crate::orchestration::conflict_prevention::broadcast_conflict_summary;
use crate::orchestration::qc_logger::QcLogger;
use crate::orchestration::qc_merger::QcMerger;
use crate::orchestration::resource_manager::ResourceManager;
use crate::orchestration::worktree_manager::WorktreeInfo;
use crate::orchestration::worktree_manager::WorktreeManager;
use crate::orchestration::qc_optimization_evaluator::evaluate_bonus;
use anyhow::Context;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tokio::task::JoinHandle;
use tracing::info;
use tracing::warn;

#[cfg(feature = "custom-features")]
use crate::a2a_communication::{A2ACommunicationManager, A2AConfig, AgentCapability, AgentIdentity, AgentRole};

/// Competition input (single prompt executed in multiple variants).
#[derive(Debug, Clone)]
pub struct CompetitionTask {
    pub prompt: String,
    pub variants: usize,
    pub base_branch: String,
    pub target_branch: String,
}

impl Default for CompetitionTask {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            variants: 3,
            base_branch: "main".to_string(),
            target_branch: "main".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompetitionOutcome {
    pub worktrees: Vec<WorktreeInfo>,
    pub winner: WorktreeInfo,
    pub qc_scores: BTreeMap<String, crate::qc::QualityScore>,
    pub qc_bonus: BTreeMap<String, f64>,
    pub overlap_summary: Option<String>,
    pub log_file: Option<PathBuf>,
}

pub struct IntegratedCompetitionRunner {
    repo_root: PathBuf,
    worktree_manager: WorktreeManager,
    resource_manager: Arc<ResourceManager>,
    qc_merger: QcMerger,
}

impl IntegratedCompetitionRunner {
    pub fn new(repo_root: impl Into<PathBuf>) -> Result<Self> {
        let repo_root = repo_root.into();
        let worktree_manager = WorktreeManager::new(&repo_root)?;
        Ok(Self {
            repo_root,
            worktree_manager,
            resource_manager: Arc::new(ResourceManager::new()),
            qc_merger: QcMerger::new(),
        })
    }

    /// Run full competition and merge the best worktree.
    pub async fn run(&self, task: CompetitionTask) -> Result<CompetitionOutcome> {
        let variants = task.variants.clamp(2, 5);
        info!("Integrated competition starting with {variants} variants");

        // 1) Create worktrees
        let mut worktrees = Vec::new();
        for idx in 0..variants {
            let task_id = format!("comp{}", idx + 1);
            let agent_name = format!("variant{}", idx + 1);
            let wt = self.worktree_manager.create_worktree(&agent_name, &task_id)?;
            worktrees.push(wt);
        }

        // 2) Execute prompt in each worktree (parallel, resource-limited)
        let mut handles: Vec<JoinHandle<(WorktreeInfo, Result<()>)>> = Vec::new();
        for wt in worktrees.clone() {
            let prompt = task.prompt.clone();
            let repo_root = self.repo_root.clone();
            let rm = Arc::clone(&self.resource_manager);
            let handle = tokio::spawn(async move {
                // Best-effort: if we cannot acquire a slot, treat as variant failure.
                match rm.acquire_slot().await {
                    Ok(_guard) => {
                        let res = run_codex_exec_in_worktree(&repo_root, &wt, &prompt).await;
                        (wt, res)
                    }
                    Err(e) => (wt, Err(e)),
                }
            });
            handles.push(handle);
        }

        for h in handles {
            match h.await {
                Ok((_wt, Ok(()))) => {}
                Ok((wt, Err(e))) => {
                    warn!("Variant {} execution failed: {}", wt.name, e);
                }
                Err(e) => {
                    warn!("Variant task panicked: {}", e);
                }
            }
        }

        // 3) Predict overlaps (informational; isolation prevents actual conflicts)
        let overlap = predict_file_overlaps(&self.repo_root, &task.base_branch, &worktrees).ok();
        let overlap_summary = overlap.as_ref().map(|s| {
            if s.overlaps.is_empty() {
                "No overlapping modified files detected.".to_string()
            } else {
                format!("Overlaps detected: {} pair(s).", s.overlaps.len())
            }
        });

        // 3.5) Broadcast overlap summary via A2A (best-effort)
        #[cfg(feature = "custom-features")]
        if let Some(ref summary) = overlap {
            let identity = AgentIdentity {
                id: "integrated_competition_orchestrator".to_string(),
                name: "integrated_competition".to_string(),
                role: AgentRole::Orchestrator,
                capabilities: vec![AgentCapability::Coordination, AgentCapability::Communication],
                trust_score: 1.0,
                last_seen: chrono::Utc::now(),
            };
            let config = A2AConfig {
                enable_encryption: false,
                enable_authentication: false,
                enable_authorization: false,
                enable_trust_management: false,
                max_message_size: 256 * 1024,
                message_ttl_seconds: 60,
                retry_attempts: 2,
                heartbeat_interval_seconds: 30,
                coordination_timeout_seconds: 30,
            };
            let manager = A2ACommunicationManager::new(config, identity.clone());
            let _ = broadcast_conflict_summary(&manager, &identity, summary, None).await;
        }

        // 4) QC score each worktree and select best (baseline)
        let (_winner_baseline, qc_scores) = self.qc_merger.select_best_worktree(worktrees.clone()).await?;

        // 5) Compute QC optimization bonus per worktree (from report metrics)
        // We re-run QC analyze to obtain QcReport per worktree for bonus + logging.
        let mut qc_bonus: BTreeMap<String, f64> = BTreeMap::new();
        let mut bonus_rationales: Vec<String> = Vec::new();
        let qc_agent = crate::qc::QcAgent::with_config(crate::qc::QcConfig::default());
        for wt in &worktrees {
            let source_code = read_worktree_code(&wt.path)?;
            match qc_agent.analyze(&source_code, &wt.name).await {
                Ok(report) => {
                    let bonus = evaluate_bonus(&report);
                    qc_bonus.insert(wt.name.clone(), bonus.bonus);
                    bonus_rationales.push(format!("{}: {}", wt.name, bonus.rationale));
                }
                Err(_) => {
                    qc_bonus.insert(wt.name.clone(), 0.0);
                }
            }
        }

        // 6) Select winner using QC overall + QC optimization bonus
        let mut best_name: Option<String> = None;
        let mut best_score: f64 = f64::MIN;
        for (name, score) in &qc_scores {
            let bonus = qc_bonus.get(name).copied().unwrap_or(0.0);
            let total = score.overall + bonus;
            if total > best_score {
                best_score = total;
                best_name = Some(name.clone());
            }
        }
        let winner = worktrees
            .iter()
            .find(|w| Some(w.name.clone()) == best_name)
            .cloned()
            .unwrap_or_else(|| worktrees[0].clone());

        // 7) Log merge decision + bonus rationale
        let log_file = if let Ok(logger) = QcLogger::new(self.repo_root.join("_docs")) {
            let selected = winner.name.clone();
            let mut extra = String::new();
            if let Some(summary) = &overlap_summary {
                extra.push_str(&format!("\n\n## ConflictOverlapSummary\n{summary}\n"));
            }
            if !bonus_rationales.is_empty() {
                extra.push_str("\n\n## QcOptimizationBonus\n");
                for line in bonus_rationales {
                    extra.push_str(&format!("- {line}\n"));
                }
            }
            logger
                .log_merge_decision_with_notes(&selected, &qc_scores, &extra)
                .await
                .ok()
        } else {
            None
        };

        // 8) Merge winner
        self.worktree_manager
            .merge_worktree(&winner, &task.target_branch)?;

        Ok(CompetitionOutcome {
            worktrees,
            winner,
            qc_scores: qc_scores
                .into_iter()
                .collect::<BTreeMap<String, crate::qc::QualityScore>>(),
            qc_bonus,
            overlap_summary,
            log_file,
        })
    }
}

async fn run_codex_exec_in_worktree(repo_root: &std::path::Path, wt: &WorktreeInfo, prompt: &str) -> Result<()> {
    // Use codex CLI execution in the isolated worktree.
    let mut cmd = Command::new("codex");
    cmd.arg("exec")
        .arg(prompt)
        .current_dir(&wt.path);

    let output = cmd.output().await.context("Failed to run codex exec")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("codex exec failed: {stderr}");
    }

    // Ensure worktree has any changes staged? We do not auto-commit; winner merge is branch-based.
    // If codex modifies files but does not commit, merge_worktree still merges branch HEAD.
    // To keep it robust, we create an auto-commit if there are changes.
    auto_commit_if_dirty(repo_root, &wt.path, &wt.branch).await?;

    Ok(())
}

async fn auto_commit_if_dirty(_repo_root: &std::path::Path, workdir: &std::path::Path, branch: &str) -> Result<()> {
    // git status --porcelain
    let status = std::process::Command::new("git")
        .current_dir(workdir)
        .args(["status", "--porcelain"])
        .output()
        .context("git status failed")?;
    if !status.status.success() {
        return Ok(());
    }
    let stdout = String::from_utf8_lossy(&status.stdout);
    if stdout.trim().is_empty() {
        return Ok(());
    }

    // add + commit
    let _ = std::process::Command::new("git")
        .current_dir(workdir)
        .args(["add", "-A"])
        .output();

    let msg = format!("codex: competition variant update ({branch})");
    let _ = std::process::Command::new("git")
        .current_dir(workdir)
        .args(["commit", "-m", &msg])
        .output();

    Ok(())
}

fn read_worktree_code(worktree_path: &PathBuf) -> Result<String> {
    use std::fs;
    use walkdir::WalkDir;

    let mut source_content = String::new();
    for entry in WalkDir::new(worktree_path)
        .max_depth(5)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension()
            && (ext == "rs" || ext == "ts" || ext == "tsx" || ext == "js" || ext == "jsx")
            && let Ok(content) = fs::read_to_string(path)
        {
            let display = path.display();
            source_content.push_str(&format!("\n// File: {display}\n"));
            source_content.push_str(&content);
            source_content.push('\n');
        }
    }

    if source_content.is_empty() {
        source_content = "// No source files found".to_string();
    }

    Ok(source_content)
}

