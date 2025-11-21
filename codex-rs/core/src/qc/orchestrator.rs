//! QC Orchestrator main logic

use std::path::Path;
use std::process::Command;
use std::fs;

use crate::qc::TestProfile;

use serde::Deserialize;

/// QC configuration options
#[derive(Debug, Clone, Deserialize)]
pub struct QcConfig {
    pub max_lines_without_pr: Option<usize>,
use std::process::Command;
use std::fs;

use crate::qc::TestProfile;

use serde::Deserialize;

/// QC configuration options
#[derive(Debug, Clone, Deserialize)]
pub struct QcConfig {
    pub max_lines_without_pr: Option<usize>,
    pub base_branch: Option<String>,
}

/// QC orchestrator for running quality checks
pub struct QcOrchestrator {
    profile: TestProfile,
    repo_root: std::path::PathBuf,
    config: QcConfig,
}

impl QcOrchestrator {
    /// Create a new QcOrchestrator with config
    pub fn new(profile: TestProfile, repo_root: std::path::PathBuf, config: QcConfig) -> Self {
        QcOrchestrator {
            profile,
            repo_root,
            config,
        }
    }

    /// Load QcConfig from a TOML file
    pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<QcConfig, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: QcConfig = toml::from_str(&contents)?;
        Ok(config)
    }
}
impl QcOrchestrator {
    /// Create a new QcOrchestrator with config
    pub fn new(profile: TestProfile, repo_root: std::path::PathBuf, config: QcConfig) -> Self {
        QcOrchestrator {
            profile,
            repo_root,
            config,
        }
    }

    /// Load QcConfig from a TOML file
    pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<QcConfig, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: QcConfig = toml::from_str(&contents)?;
        Ok(config)
    }
}
/// Result of a QC run
#[derive(Debug, Clone)]
pub struct QcResult {
    /// Test profile used
    pub profile: TestProfile,
    /// Number of lines added
    pub lines_added: usize,
    /// Number of lines deleted
    pub lines_deleted: usize,
    /// Number of files changed
    pub files_changed: usize,
    /// Risk score (0.0 to 1.0)
    pub risk_score: f64,
    /// Merge recommendation
    pub recommendation: QcRecommendation,
    /// Rust test results (command, success)
    pub rust_test_results: Vec<(String, bool)>,
    /// Web test results (command, success)
    pub web_test_results: Vec<(String, bool)>,
    /// Warnings
    pub warnings: Vec<String>,
}

impl QcResult {
    /// Get total changed lines
    pub fn total_changed_lines(&self) -> usize {
        self.lines_added + self.lines_deleted
    }
}

/// Merge recommendation based on QC results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QcRecommendation {
    /// Changes look good, can merge directly
    Approve,
    /// Changes should be reviewed in a PR (200+ line rule or other concerns)
    RequestPr,
    /// Tests failed, should not merge
    Reject,
}

impl std::fmt::Display for QcRecommendation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QcRecommendation::Approve => write!(f, "✅ Approve (safe to merge)"),
            QcRecommendation::RequestPr => write!(f, "🔍 Request PR (review recommended)"),
            QcRecommendation::Reject => write!(f, "❌ Reject (tests failed)"),
        }
    }
}

impl QcOrchestrator {
    /// Create a new QC orchestrator
    pub fn new<P: AsRef<Path>>(repo_root: P, profile: TestProfile) -> Self {
        Self {
            profile,
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }

    /// Run QC checks
    pub fn run(&self) -> Result<QcResult, String> {
        // Analyze git diff
        let (lines_added, lines_deleted, files_changed) = self.analyze_diff()?;

        // Run Rust tests
        let rust_test_results = self.run_rust_tests();

        // Run Web tests
        let web_test_results = self.run_web_tests();

        // Calculate risk score
        let risk_score = self.calculate_risk_score(lines_added, lines_deleted, files_changed);

        // Determine recommendation
        let total_lines = lines_added + lines_deleted;
        let all_tests_passed = rust_test_results.iter().all(|(_, success)| *success)
            && web_test_results.iter().all(|(_, success)| *success);

        let mut warnings = Vec::new();

        let recommendation = if !all_tests_passed {
            warnings.push("Some tests failed".to_string());
            QcRecommendation::Reject
        } else if total_lines > 200 {
            warnings.push(format!(
                "Total changed lines ({}) exceeds 200-line threshold",
                total_lines
            ));
            QcRecommendation::RequestPr
        } else if risk_score > 0.7 {
            warnings.push(format!("High risk score: {:.2}", risk_score));
            QcRecommendation::RequestPr
        } else {
            QcRecommendation::Approve
        };

        Ok(QcResult {
            profile: self.profile,
            lines_added,
            lines_deleted,
            files_changed,
            risk_score,
            recommendation,
            rust_test_results,
            web_test_results,
            warnings,
        })
    }

    /// Analyze git diff to count changed lines and files
    fn analyze_diff(&self) -> Result<(usize, usize, usize), String> {
        let output = Command::new("git")
            .args(["diff", "--numstat", "origin/main...HEAD"])
            .current_dir(&self.repo_root)
            .output()
            .map_err(|e| format!("Failed to run git diff: {}", e))?;

        if !output.status.success() {
            return Err("git diff command failed".to_string());
        }

        let diff_output = String::from_utf8_lossy(&output.stdout);
        let mut total_added = 0;
        let mut total_deleted = 0;
        let mut file_count = 0;

        for line in diff_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                file_count += 1;
                // Handle binary files: git diff --numstat outputs "-" for added/deleted lines
                if parts[0] != "-" {
                    if let Ok(added) = parts[0].parse::<usize>() {
                        total_added += added;
                    }
                }
                if parts[1] != "-" {
                    if let Ok(deleted) = parts[1].parse::<usize>() {
                        total_deleted += deleted;
                    }
                }
                // If both parts[0] and parts[1] are "-", this is a binary file.
                // We count it in file_count, but not in lines added/deleted.
            }
        }

        Ok((total_added, total_deleted, file_count))
    }

    /// Run Rust tests based on the profile
    fn run_rust_tests(&self) -> Vec<(String, bool)> {
        let commands = self.profile.rust_commands();
        let mut results = Vec::new();

        for cmd in commands {
            let success = self.run_command(&cmd);
            results.push((cmd, success));
        }

        results
    }

    /// Run Web tests based on the profile
    fn run_web_tests(&self) -> Vec<(String, bool)> {
        let commands = self.profile.web_commands();
        let mut results = Vec::new();

        for cmd in commands {
            let success = self.run_command(&cmd);
            results.push((cmd, success));
        }

        results
    }

    /// Run a shell command and return success status
    fn run_command(&self, cmd: &str) -> bool {
        println!("Running: {}", cmd);

        // Determine working directory - use codex-rs subdirectory if it exists for cargo commands
        let work_dir = if cmd.trim().starts_with("cargo") {
            let codex_rs_path = self.repo_root.join("codex-rs");
            if codex_rs_path.exists() {
                codex_rs_path
            } else {
                self.repo_root.clone()
            }
        } else {
            self.repo_root.clone()
        };

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(&work_dir)
            .status();

        match status {
            Ok(s) => {
                let success = s.success();
                if success {
                    println!("✅ Command succeeded");
                } else {
                    println!("❌ Command failed with status: {}", s);
                }
                success
            }
            Err(e) => {
                println!("❌ Failed to execute command: {}", e);
                false
            }
        }
    }

    /// Calculate a risk score based on changes
    fn calculate_risk_score(
        &self,
        lines_added: usize,
        lines_deleted: usize,
        files_changed: usize,
    ) -> f64 {
        // Simple risk scoring:
        // - More changed lines = higher risk
        // - More files changed = higher risk
        // - Normalized to 0.0 - 1.0 range

        let total_lines = lines_added + lines_deleted;
        let line_score = (total_lines as f64 / 500.0).min(1.0); // Max at 500 lines
        let file_score = (files_changed as f64 / 20.0).min(1.0); // Max at 20 files

        // Weight: 70% lines, 30% files
        (line_score * 0.7 + file_score * 0.3).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_risk_score() {
        let orchestrator = QcOrchestrator::new("/tmp", TestProfile::Standard);

        // Low risk: few lines, few files
        let score = orchestrator.calculate_risk_score(10, 5, 2);
        assert!(score < 0.3, "Expected low risk score, got {}", score);

        // Medium risk
        let score = orchestrator.calculate_risk_score(100, 50, 5);
        assert!(
            score > 0.2 && score < 0.5,
            "Expected medium risk score, got {}",
            score
        );

        // High risk: many lines and files
        let score = orchestrator.calculate_risk_score(300, 200, 15);
        assert!(score > 0.6, "Expected high risk score, got {}", score);
    }

    #[test]
    fn test_qc_result_total_changed_lines() {
        let result = QcResult {
            profile: TestProfile::Standard,
            lines_added: 100,
            lines_deleted: 50,
            files_changed: 5,
            risk_score: 0.5,
            recommendation: QcRecommendation::Approve,
            rust_test_results: vec![],
            web_test_results: vec![],
            warnings: vec![],
        };

        assert_eq!(result.total_changed_lines(), 150);
    }
}
