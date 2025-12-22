//! Development mode selection and history analysis
//!
//! Provides functionality to select between central orchestration and git worktree
//! parallel development modes, with support for analyzing past implementation logs
//! and commit history.

use crate::orchestration::conflict_resolver::MergeStrategy;
use crate::plan::ExecutionMode;
use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Development mode configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DevelopmentMode {
    /// Central orchestration with conflict resolution
    /// Uses ConflictResolver for deadlock and conflict prevention
    Central {
        /// Merge strategy for conflict resolution
        merge_strategy: MergeStrategy,
        /// Enable dynamic conflict detection
        dynamic_conflict_detection: bool,
        /// Enable QC agent for quality analysis
        enable_qc: bool,
    },
    /// Git worktree parallel development
    /// Uses WorktreeManager for isolated parallel development
    Worktree {
        /// Number of parallel worktrees
        num_worktrees: usize,
        /// Auto-merge winner after competition
        auto_merge_winner: bool,
        /// Enable QC agent for quality analysis
        enable_qc: bool,
    },
}

impl Default for DevelopmentMode {
    fn default() -> Self {
        Self::Central {
            merge_strategy: MergeStrategy::ThreeWayMerge,
            dynamic_conflict_detection: true,
            enable_qc: true,
        }
    }
}

impl DevelopmentMode {
    /// Convert to plan execution mode.
    pub fn to_execution_mode(&self) -> ExecutionMode {
        match self {
            Self::Central { .. } => ExecutionMode::Orchestrated,
            Self::Worktree { .. } => ExecutionMode::Competition,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Central { .. } => {
                "Central orchestration with dynamic conflict resolution and deadlock prevention"
            }
            Self::Worktree { .. } => "Git worktree parallel development with isolated branches",
        }
    }
}

/// Implementation log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationLog {
    /// Log file path
    pub path: PathBuf,
    /// Date from filename
    pub date: String,
    /// Feature name
    pub feature: String,
    /// Content summary
    pub summary: String,
    /// Implementation details
    pub details: HashMap<String, String>,
}

/// Commit history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitEntry {
    /// Commit SHA
    pub sha: String,
    /// Commit message
    pub message: String,
    /// Author
    pub author: String,
    /// Date
    pub date: DateTime<Utc>,
    /// Files changed
    pub files: Vec<String>,
}

/// Development mode selector with history analysis
pub struct DevelopmentModeSelector {
    /// Repository root
    repo_root: PathBuf,
    /// Implementation logs directory
    logs_dir: PathBuf,
}

impl DevelopmentModeSelector {
    /// Create a new selector
    pub fn new(repo_root: impl AsRef<Path>) -> Result<Self> {
        let repo_root = repo_root.as_ref().to_path_buf();
        let logs_dir = repo_root.join("_docs");

        Ok(Self {
            repo_root,
            logs_dir,
        })
    }

    /// Analyze past implementation logs
    pub fn analyze_implementation_logs(&self) -> Result<Vec<ImplementationLog>> {
        let mut logs = Vec::new();

        if !self.logs_dir.exists() {
            warn!(
                "Implementation logs directory not found: {:?}",
                self.logs_dir
            );
            return Ok(logs);
        }

        let entries = std::fs::read_dir(&self.logs_dir)
            .context("Failed to read implementation logs directory")?;

        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_file()
                && path.extension().and_then(|s| s.to_str()) == Some("md")
                && let Some(log) = self.parse_log_file(&path)?
            {
                logs.push(log);
            }
        }

        // Sort by date (newest first)
        logs.sort_by(|a, b| b.date.cmp(&a.date));

        info!("Found {} implementation logs", logs.len());
        Ok(logs)
    }

    /// Parse a log file
    fn parse_log_file(&self, path: &Path) -> Result<Option<ImplementationLog>> {
        let content =
            std::fs::read_to_string(path).context(format!("Failed to read log file: {path:?}"))?;

        // Extract date and feature from filename (format: yyyy-mm-dd_feature.md)
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid filename")?;

        let parts: Vec<&str> = filename.splitn(2, '_').collect();
        let date = parts.first().copied().unwrap_or("").to_string();
        let feature = parts.get(1).copied().unwrap_or(filename).to_string();

        // Extract summary from content (first paragraph or heading)
        let summary = self.extract_summary(&content);

        // Extract details
        let details = self.extract_details(&content);

        Ok(Some(ImplementationLog {
            path: path.to_path_buf(),
            date,
            feature,
            summary,
            details,
        }))
    }

    /// Extract summary from markdown content
    fn extract_summary(&self, content: &str) -> String {
        // Try to find first heading or first paragraph
        for line in content.lines() {
            if line.starts_with("# ") {
                return line.trim_start_matches("# ").to_string();
            }
            if line.starts_with("## ") {
                return line.trim_start_matches("## ").to_string();
            }
            if !line.trim().is_empty() && !line.starts_with('#') {
                return line.trim().to_string();
            }
        }
        "No summary available".to_string()
    }

    /// Extract details from markdown content
    fn extract_details(&self, content: &str) -> HashMap<String, String> {
        let mut details = HashMap::new();

        // Look for key-value pairs in markdown
        for line in content.lines() {
            if line.contains(':') && !line.starts_with('#') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();
                    if !key.is_empty() && !value.is_empty() {
                        details.insert(key, value);
                    }
                }
            }
        }

        details
    }

    /// Analyze git commit history
    pub fn analyze_commit_history(
        &self,
        keywords: &[&str],
        limit: usize,
    ) -> Result<Vec<CommitEntry>> {
        let mut commits = Vec::new();

        // Build git log command
        let mut cmd = Command::new("git");
        cmd.args([
            "log",
            "--all",
            "--format=%H|%s|%an|%ai",
            "--date=iso",
            "-n",
            &limit.to_string(),
        ]);

        // Add keyword filters if provided
        for keyword in keywords {
            cmd.arg("--grep").arg(keyword);
        }

        let output = cmd
            .current_dir(&self.repo_root)
            .output()
            .context("Failed to execute git log")?;

        if !output.status.success() {
            warn!("Git log command failed: {:?}", output.status);
            return Ok(commits);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                let sha = parts[0].to_string();
                let message = parts[1].to_string();
                let author = parts[2].to_string();
                let date_str = parts[3];

                // Parse date
                let date = DateTime::parse_from_rfc3339(date_str)
                    .or_else(|_| {
                        DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z")
                            .or_else(|_| DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S"))
                    })
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                // Get files changed
                let files = self.get_commit_files(&sha)?;

                commits.push(CommitEntry {
                    sha,
                    message,
                    author,
                    date,
                    files,
                });
            }
        }

        info!("Found {} relevant commits", commits.len());
        Ok(commits)
    }

    /// Get files changed in a commit
    fn get_commit_files(&self, sha: &str) -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["show", "--name-only", "--format=", sha])
            .current_dir(&self.repo_root)
            .output()
            .context("Failed to execute git show")?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<String> = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();

        Ok(files)
    }

    /// Recommend development mode based on history
    pub fn recommend_mode(&self) -> Result<DevelopmentMode> {
        // Analyze implementation logs
        let logs = self.analyze_implementation_logs()?;

        // Analyze commit history for orchestration/worktree keywords
        let orchestration_commits = self.analyze_commit_history(
            &["orchestration", "orchestrated", "central", "conflict"],
            20,
        )?;
        let worktree_commits =
            self.analyze_commit_history(&["worktree", "parallel", "competition", "variant"], 20)?;

        // Count references in logs
        let mut central_refs = 0;
        let mut worktree_refs = 0;

        for log in &logs {
            let content_lower = log.summary.to_lowercase();
            if content_lower.contains("orchestrat")
                || content_lower.contains("central")
                || content_lower.contains("conflict")
            {
                central_refs += 1;
            }
            if content_lower.contains("worktree")
                || content_lower.contains("parallel")
                || content_lower.contains("competition")
            {
                worktree_refs += 1;
            }
        }

        debug!(
            "History analysis: central_refs={}, worktree_refs={}, orchestration_commits={}, worktree_commits={}",
            central_refs,
            worktree_refs,
            orchestration_commits.len(),
            worktree_commits.len()
        );

        // Recommend based on history
        let total_refs = central_refs + worktree_refs;
        if total_refs == 0 {
            // Default to central orchestration
            return Ok(DevelopmentMode::default());
        }

        if worktree_refs > central_refs || worktree_commits.len() > orchestration_commits.len() {
            info!("Recommending Worktree mode based on history");
            Ok(DevelopmentMode::Worktree {
                num_worktrees: 3,
                auto_merge_winner: true,
                enable_qc: true,
            })
        } else {
            info!("Recommending Central orchestration mode based on history");
            Ok(DevelopmentMode::default())
        }
    }

    /// Get mode selection summary
    pub fn get_mode_summary(&self, mode: &DevelopmentMode) -> String {
        match mode {
            DevelopmentMode::Central {
                merge_strategy,
                dynamic_conflict_detection,
                ..
            } => {
                format!(
                    "Central Orchestration Mode\n\
                    - Merge Strategy: {merge_strategy:?}\n\
                    - Dynamic Conflict Detection: {dynamic_conflict_detection}\n\
                    - Uses ConflictResolver for deadlock prevention\n\
                    - Suitable for: Multi-agent coordination in same repository"
                )
            }
            DevelopmentMode::Worktree {
                num_worktrees,
                auto_merge_winner,
                ..
            } => {
                format!(
                    "Git Worktree Parallel Development Mode\n\
                    - Number of Worktrees: {num_worktrees}\n\
                    - Auto-merge Winner: {auto_merge_winner}\n\
                    - Uses WorktreeManager for isolated branches\n\
                    - Suitable for: Parallel variant development"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_development_mode_default() {
        let mode = DevelopmentMode::default();
        assert!(matches!(mode, DevelopmentMode::Central { .. }));
    }

    #[test]
    fn test_development_mode_to_execution_mode() {
        let central = DevelopmentMode::Central {
            merge_strategy: MergeStrategy::ThreeWayMerge,
            dynamic_conflict_detection: true,
            enable_qc: true,
        };
        assert_eq!(central.to_execution_mode(), ExecutionMode::Orchestrated);

        let worktree = DevelopmentMode::Worktree {
            num_worktrees: 3,
            auto_merge_winner: true,
            enable_qc: true,
        };
        assert_eq!(worktree.to_execution_mode(), ExecutionMode::Competition);
    }
}
