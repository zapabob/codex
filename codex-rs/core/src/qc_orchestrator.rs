//! QC (Quality Control) Orchestrator
//!
//! Production-ready QC orchestrator for running tests, computing diffs, and generating recommendations.

use anyhow::{Context, Result};
use chrono::Local;
use git2::{DiffLineType, Repository};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

/// Test profile levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestProfile {
    /// Minimal testing (cargo test -p codex-cli)
    Minimal,
    /// Standard testing (cargo test --all, cargo clippy, pnpm test)
    Standard,
    /// Full testing (Standard + tarpaulin coverage, pnpm lint)
    Full,
}

impl TestProfile {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TestProfile::Minimal => "minimal",
            TestProfile::Standard => "standard",
            TestProfile::Full => "full",
        }
    }
}

impl FromStr for TestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(TestProfile::Minimal),
            "standard" => Ok(TestProfile::Standard),
            "full" => Ok(TestProfile::Full),
            _ => anyhow::bail!("Invalid test profile: {s}. Valid values: minimal, standard, full"),
        }
    }
}

/// QC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcConfig {
    /// Default test profile
    pub default_profile: TestProfile,
    /// Maximum lines changed without requiring a PR
    pub max_lines_without_pr: usize,
    /// Base reference for diff comparison (e.g., "main", "HEAD~1")
    pub base_ref: String,
}

impl Default for QcConfig {
    fn default() -> Self {
        Self {
            default_profile: TestProfile::Standard,
            max_lines_without_pr: 200,
            base_ref: "main".to_string(),
        }
    }
}

/// Input parameters for QC run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcInput {
    /// Feature description
    pub feature: String,
    /// Agent name (e.g., "codex-cli-agent")
    pub agent_name: String,
    /// AI model name (e.g., "claude-code", "gpt-4.1")
    pub ai_name: String,
    /// Test profile to use
    pub profile: TestProfile,
}

/// Diff statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffStats {
    /// Number of changed lines (additions + deletions)
    pub changed_lines: usize,
    /// Number of changed files
    pub changed_files: usize,
}

/// Status of a command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandStatus {
    /// Command was not run
    NotRun { reason: String },
    /// Command passed
    Passed,
    /// Command failed
    Failed { summary: String },
}

/// Result of a single test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test label (e.g., "Rust Tests", "Clippy")
    pub label: String,
    /// Command that was run
    pub command: String,
    /// Test status
    pub status: CommandStatus,
    /// Warnings collected
    pub warnings: Vec<String>,
}

/// Final recommendation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recommendation {
    /// Safe to merge
    MergeOk,
    /// Needs fixes before merge
    NeedsFix,
    /// Create PR for review
    CreatePrForReview,
}

impl Recommendation {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Recommendation::MergeOk => "MergeOk",
            Recommendation::NeedsFix => "NeedsFix",
            Recommendation::CreatePrForReview => "CreatePrForReview",
        }
    }
}

/// Complete QC result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcResult {
    /// Timestamp with timezone
    pub timestamp: String,
    /// Worktree name
    pub worktree: String,
    /// Diff statistics
    pub diff: DiffStats,
    /// Test results
    pub tests: Vec<TestResult>,
    /// Risk score (0.0-1.0)
    pub risk_score: f32,
    /// Final recommendation
    pub recommendation: Recommendation,
    /// Reasons for recommendation
    pub reasons: Vec<String>,
    /// Issues found
    pub issues: Vec<String>,
    /// Path to log file
    pub log_path: PathBuf,
}

/// Run QC orchestrator
pub fn run_qc(repo_root: &Path, input: QcInput, config: QcConfig) -> Result<QcResult> {
    // Get current timestamp with timezone
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S %z").to_string();

    // Open repository
    let repo = Repository::open(repo_root)
        .context("Failed to open git repository. Make sure you're in a git repository.")?;

    // Get worktree name from HEAD branch or worktree path
    let worktree = get_worktree_name(&repo)?;

    // Compute diff stats
    let diff = compute_diff_stats(&repo, &config.base_ref)?;

    // Run tests based on profile
    let tests = run_tests(repo_root, input.profile)?;

    // Compute risk score
    let risk_score = compute_risk_score(&diff, &tests);

    // Build recommendation and reasons
    let (recommendation, reasons, issues) =
        build_recommendation(&diff, &tests, &config, risk_score);

    // Write log
    let log_path = write_log(repo_root, &LogData {
        timestamp: &timestamp,
        worktree: &worktree,
        input: &input,
        diff: &diff,
        tests: &tests,
        risk_score,
        recommendation: &recommendation,
        reasons: &reasons,
        issues: &issues,
    })?;

    Ok(QcResult {
        timestamp,
        worktree,
        diff,
        tests,
        risk_score,
        recommendation,
        reasons,
        issues,
        log_path,
    })
}

/// Get worktree name from repository
fn get_worktree_name(repo: &Repository) -> Result<String> {
    // Try to get current branch name
    if let Ok(head) = repo.head()
        && let Some(branch_name) = head.shorthand()
    {
        // Sanitize branch name by replacing slashes with dashes
        return Ok(branch_name.replace('/', "-"));
    }

    // Fall back to worktree path or "detached"
    if let Some(workdir) = repo.workdir()
        && let Some(name) = workdir.file_name()
    {
        return Ok(name.to_string_lossy().to_string());
    }

    Ok("detached".to_string())
}

/// Compute diff statistics between base_ref and HEAD
fn compute_diff_stats(repo: &Repository, base_ref: &str) -> Result<DiffStats> {
    // Try to resolve the configured base reference, or fall back to common alternatives
    let base_refs_to_try = [
        base_ref,
        "origin/main",
        "origin/master",
        "HEAD~1",
    ];

    let mut last_error = None;
    let base_object = base_refs_to_try
        .iter()
        .find_map(|ref_name| match repo.revparse_single(ref_name) {
            Ok(obj) => Some(obj),
            Err(e) => {
                last_error = Some((*ref_name, e));
                None
            }
        })
        .ok_or_else(|| {
            if let Some((last_ref, err)) = last_error {
                anyhow::anyhow!(
                    "Failed to resolve any base reference. Last attempted: {last_ref}, error: {err}"
                )
            } else {
                anyhow::anyhow!("Failed to resolve any base reference")
            }
        })?;

    let base_tree = base_object
        .peel_to_tree()
        .context("Failed to peel base reference to tree")?;

    // Get HEAD tree
    let head_object = repo
        .revparse_single("HEAD")
        .context("Failed to resolve HEAD")?;
    let head_tree = head_object
        .peel_to_tree()
        .context("Failed to peel HEAD to tree")?;

    // Compute diff
    let diff = repo
        .diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)
        .context("Failed to compute diff")?;

    let mut changed_lines = 0;

    // Count changed files
    let changed_files = diff.deltas().len();

    // Count changed lines
    diff.foreach(
        &mut |_, _| true,
        None,
        None,
        Some(&mut |_, _, line| {
            match line.origin_value() {
                DiffLineType::Addition | DiffLineType::Deletion => {
                    changed_lines += 1;
                }
                _ => {}
            }
            true
        }),
    )
    .context("Failed to process diff lines")?;

    Ok(DiffStats {
        changed_lines,
        changed_files,
    })
}

/// Run tests based on profile
fn run_tests(repo_root: &Path, profile: TestProfile) -> Result<Vec<TestResult>> {
    let mut results = Vec::new();

    // Determine cargo directory (check if codex-rs exists)
    let cargo_dir = if repo_root.join("codex-rs").exists() {
        repo_root.join("codex-rs")
    } else {
        repo_root.to_path_buf()
    };

    match profile {
        TestProfile::Minimal => {
            // Rust: cargo test -p codex-cli
            results.push(run_command(
                &cargo_dir,
                "Rust CLI Tests",
                "cargo",
                &["test", "-p", "codex-cli"],
            ));
        }
        TestProfile::Standard => {
            // Rust: cargo test --all
            results.push(run_command(
                &cargo_dir,
                "Rust Tests",
                "cargo",
                &["test", "--all"],
            ));

            // Rust Lint: cargo clippy --all --all-targets -- -D warnings
            results.push(run_command(
                &cargo_dir,
                "Rust Clippy",
                "cargo",
                &["clippy", "--all", "--all-targets", "--", "-D", "warnings"],
            ));

            // Web/GUI: pnpm test or npm test
            let web_result = if command_exists("pnpm") {
                run_command(repo_root, "Web Tests", "pnpm", &["test"])
            } else if command_exists("npm") {
                run_command(repo_root, "Web Tests", "npm", &["test"])
            } else {
                TestResult {
                    label: "Web Tests".to_string(),
                    command: "pnpm test / npm test".to_string(),
                    status: CommandStatus::NotRun {
                        reason: "Neither pnpm nor npm found".to_string(),
                    },
                    warnings: vec![],
                }
            };
            results.push(web_result);
        }
        TestProfile::Full => {
            // Everything in Standard
            results.push(run_command(
                &cargo_dir,
                "Rust Tests",
                "cargo",
                &["test", "--all"],
            ));
            results.push(run_command(
                &cargo_dir,
                "Rust Clippy",
                "cargo",
                &["clippy", "--all", "--all-targets", "--", "-D", "warnings"],
            ));

            let web_result = if command_exists("pnpm") {
                run_command(repo_root, "Web Tests", "pnpm", &["test"])
            } else if command_exists("npm") {
                run_command(repo_root, "Web Tests", "npm", &["test"])
            } else {
                TestResult {
                    label: "Web Tests".to_string(),
                    command: "pnpm test / npm test".to_string(),
                    status: CommandStatus::NotRun {
                        reason: "Neither pnpm nor npm found".to_string(),
                    },
                    warnings: vec![],
                }
            };
            results.push(web_result);

            // Coverage: cargo tarpaulin
            if command_exists("cargo-tarpaulin") {
                results.push(run_command(
                    &cargo_dir,
                    "Rust Coverage",
                    "cargo",
                    &["tarpaulin", "--workspace"],
                ));
            } else {
                results.push(TestResult {
                    label: "Rust Coverage".to_string(),
                    command: "cargo tarpaulin --workspace".to_string(),
                    status: CommandStatus::NotRun {
                        reason: "cargo-tarpaulin not installed".to_string(),
                    },
                    warnings: vec![],
                });
            }

            // Web lint
            let web_lint_result = if command_exists("pnpm") {
                run_command(repo_root, "Web Lint", "pnpm", &["lint"])
            } else if command_exists("npm") {
                run_command(repo_root, "Web Lint", "npm", &["run", "lint"])
            } else {
                TestResult {
                    label: "Web Lint".to_string(),
                    command: "pnpm lint / npm run lint".to_string(),
                    status: CommandStatus::NotRun {
                        reason: "Neither pnpm nor npm found".to_string(),
                    },
                    warnings: vec![],
                }
            };
            results.push(web_lint_result);
        }
    }

    Ok(results)
}

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}

/// Run a command and return test result
fn run_command(repo_root: &Path, label: &str, cmd: &str, args: &[&str]) -> TestResult {
    let command_str = format!("{cmd} {}", args.join(" "));

    let output = match Command::new(cmd).args(args).current_dir(repo_root).output() {
        Ok(output) => output,
        Err(e) => {
            return TestResult {
                label: label.to_string(),
                command: command_str,
                status: CommandStatus::Failed {
                    summary: format!("Failed to execute command: {e}"),
                },
                warnings: vec![],
            };
        }
    };

    let status = if output.status.success() {
        CommandStatus::Passed
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let summary = stderr.lines().take(5).collect::<Vec<_>>().join("\n");
        CommandStatus::Failed { summary }
    };

    // Extract warnings from stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    let warnings: Vec<String> = stderr
        .lines()
        .filter(|line| line.contains("warning:"))
        .take(10)
        .map(|s| s.to_string())
        .collect();

    TestResult {
        label: label.to_string(),
        command: command_str,
        status,
        warnings,
    }
}

/// Compute risk score (0.0-1.0)
fn compute_risk_score(diff: &DiffStats, tests: &[TestResult]) -> f32 {
    let mut score = 0.0;

    // Add weight for failed tests (0.3 per failure, max 0.6)
    let failed_count = tests
        .iter()
        .filter(|t| matches!(t.status, CommandStatus::Failed { .. }))
        .count();
    score += (failed_count as f32 * 0.3).min(0.6);

    // Add weight for large diffs (0.2 for 200+ lines, 0.4 for 500+ lines)
    if diff.changed_lines >= 500 {
        score += 0.4;
    } else if diff.changed_lines >= 200 {
        score += 0.2;
    }

    // Clamp to 0.0-1.0
    score.min(1.0)
}

/// Build recommendation, reasons, and issues
fn build_recommendation(
    diff: &DiffStats,
    tests: &[TestResult],
    config: &QcConfig,
    risk_score: f32,
) -> (Recommendation, Vec<String>, Vec<String>) {
    let mut reasons = Vec::new();
    let mut issues = Vec::new();

    // Check for test failures
    let failed_tests: Vec<_> = tests
        .iter()
        .filter(|t| matches!(t.status, CommandStatus::Failed { .. }))
        .collect();

    let has_failures = !failed_tests.is_empty();

    // Check 200-line rule
    let exceeds_line_limit = diff.changed_lines > config.max_lines_without_pr;

    // Determine recommendation
    let recommendation = if has_failures {
        for test in &failed_tests {
            if let CommandStatus::Failed { summary } = &test.status {
                issues.push(format!("{}: {summary}", test.label));
            }
        }
        reasons.push(format!("{} test(s) failed", failed_tests.len()));
        Recommendation::NeedsFix
    } else if exceeds_line_limit {
        reasons.push(format!(
            "変更行数が{}行を超えています (200行ルール)",
            diff.changed_lines
        ));
        reasons.push("PR作成を推奨します".to_string());
        Recommendation::CreatePrForReview
    } else if risk_score > 0.7 {
        reasons.push(format!("リスクスコアが高い: {risk_score:.2}"));
        Recommendation::NeedsFix
    } else {
        reasons.push("全てのテストが成功しました".to_string());
        reasons.push(format!("変更行数: {} 行", diff.changed_lines));
        Recommendation::MergeOk
    };

    (recommendation, reasons, issues)
}

/// Data for writing log
struct LogData<'a> {
    timestamp: &'a str,
    worktree: &'a str,
    input: &'a QcInput,
    diff: &'a DiffStats,
    tests: &'a [TestResult],
    risk_score: f32,
    recommendation: &'a Recommendation,
    reasons: &'a [String],
    issues: &'a [String],
}

/// Write log to _docs/logs
fn write_log(repo_root: &Path, data: &LogData) -> Result<PathBuf> {
    // Create logs directory
    let logs_dir = repo_root.join("_docs/logs");
    fs::create_dir_all(&logs_dir).context("Failed to create _docs/logs directory")?;

    // Generate log filename: YYYY-MM-DD-{worktree}-impl.md
    let date = Local::now().format("%Y-%m-%d").to_string();
    let log_filename = format!("{date}-{}-impl.md", data.worktree);
    let log_path = logs_dir.join(&log_filename);

    // Build log content
    let mut content = String::new();

    // Header section
    content.push_str(&format!("## {}\n\n", data.timestamp));
    content.push_str(&format!("- Worktree: {}\n", data.worktree));
    content.push_str(&format!("- 機能: {}\n", data.input.feature));
    content.push_str(&format!("- エージェント名: {}\n", data.input.agent_name));
    content.push_str(&format!("- AI名: {}\n", data.input.ai_name));
    content.push_str(&format!("- プロファイル: {}\n\n", data.input.profile.as_str()));

    // Diff stats
    content.push_str("### 変更統計\n\n");
    content.push_str(&format!("- 変更ファイル数: {}\n", data.diff.changed_files));
    content.push_str(&format!("- 変更行数: {}\n\n", data.diff.changed_lines));

    // Test results
    content.push_str("### テスト結果\n\n");
    for test in data.tests {
        let status_str = match &test.status {
            CommandStatus::NotRun { reason } => format!("⊘ SKIPPED ({reason})"),
            CommandStatus::Passed => "✓ PASSED".to_string(),
            CommandStatus::Failed { .. } => "✗ FAILED".to_string(),
        };
        content.push_str(&format!("- **{}**: {status_str}\n", test.label));
        content.push_str(&format!("  - Command: `{}`\n", test.command));
        if !test.warnings.is_empty() {
            content.push_str(&format!("  - Warnings: {}\n", test.warnings.len()));
        }
    }
    content.push('\n');

    // Risk assessment
    content.push_str("### リスク評価\n\n");
    content.push_str(&format!("- リスクスコア: {:.2}\n", data.risk_score));
    content.push_str(&format!(
        "- 推奨アクション: **{}**\n\n",
        data.recommendation.as_str()
    ));

    // Reasons
    if !data.reasons.is_empty() {
        content.push_str("### 理由\n\n");
        for reason in data.reasons {
            content.push_str(&format!("- {reason}\n"));
        }
        content.push('\n');
    }

    // Issues
    if !data.issues.is_empty() {
        content.push_str("### 発見された問題\n\n");
        for issue in data.issues {
            content.push_str(&format!("- {issue}\n"));
        }
        content.push('\n');
    }

    content.push_str("---\n\n");

    // Append to file (or create if it doesn't exist)
    if log_path.exists() {
        let existing = fs::read_to_string(&log_path)
            .with_context(|| format!("Failed to read log file: {}", log_path.display()))?;
        let new_content = format!("{existing}{content}");
        fs::write(&log_path, new_content)
            .with_context(|| format!("Failed to append to log file: {}", log_path.display()))?;
    } else {
        fs::write(&log_path, content)
            .with_context(|| format!("Failed to create log file: {}", log_path.display()))?;
    }

    Ok(log_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_from_str() {
        assert_eq!(
            TestProfile::from_str("minimal").unwrap(),
            TestProfile::Minimal
        );
        assert_eq!(
            TestProfile::from_str("standard").unwrap(),
            TestProfile::Standard
        );
        assert_eq!(TestProfile::from_str("full").unwrap(), TestProfile::Full);
        assert_eq!(TestProfile::from_str("FULL").unwrap(), TestProfile::Full);
        assert!(TestProfile::from_str("invalid").is_err());
    }

    #[test]
    fn test_profile_as_str() {
        assert_eq!(TestProfile::Minimal.as_str(), "minimal");
        assert_eq!(TestProfile::Standard.as_str(), "standard");
        assert_eq!(TestProfile::Full.as_str(), "full");
    }

    #[test]
    fn test_recommendation_as_str() {
        assert_eq!(Recommendation::MergeOk.as_str(), "MergeOk");
        assert_eq!(Recommendation::NeedsFix.as_str(), "NeedsFix");
        assert_eq!(
            Recommendation::CreatePrForReview.as_str(),
            "CreatePrForReview"
        );
    }

    #[test]
    fn test_compute_risk_score() {
        let diff = DiffStats {
            changed_lines: 50,
            changed_files: 5,
        };
        let tests = vec![TestResult {
            label: "Test".to_string(),
            command: "test".to_string(),
            status: CommandStatus::Passed,
            warnings: vec![],
        }];
        let score = compute_risk_score(&diff, &tests);
        assert!(score < 0.3);

        // Test with failures
        let tests_failed = vec![TestResult {
            label: "Test".to_string(),
            command: "test".to_string(),
            status: CommandStatus::Failed {
                summary: "error".to_string(),
            },
            warnings: vec![],
        }];
        let score = compute_risk_score(&diff, &tests_failed);
        assert!(score >= 0.3);

        // Test with large diff
        let large_diff = DiffStats {
            changed_lines: 250,
            changed_files: 20,
        };
        let score = compute_risk_score(&large_diff, &tests);
        assert!(score >= 0.2);
    }

    #[test]
    fn test_build_recommendation() {
        let config = QcConfig::default();

        // Test passing case
        let diff = DiffStats {
            changed_lines: 50,
            changed_files: 5,
        };
        let tests = vec![TestResult {
            label: "Test".to_string(),
            command: "test".to_string(),
            status: CommandStatus::Passed,
            warnings: vec![],
        }];
        let (rec, reasons, issues) = build_recommendation(&diff, &tests, &config, 0.1);
        assert_eq!(rec, Recommendation::MergeOk);
        assert!(!reasons.is_empty());
        assert!(issues.is_empty());

        // Test failure case
        let tests_failed = vec![TestResult {
            label: "Test".to_string(),
            command: "test".to_string(),
            status: CommandStatus::Failed {
                summary: "error".to_string(),
            },
            warnings: vec![],
        }];
        let (rec, _, issues) = build_recommendation(&diff, &tests_failed, &config, 0.5);
        assert_eq!(rec, Recommendation::NeedsFix);
        assert!(!issues.is_empty());

        // Test 200-line rule
        let large_diff = DiffStats {
            changed_lines: 250,
            changed_files: 20,
        };
        let (rec, reasons, _) = build_recommendation(&large_diff, &tests, &config, 0.2);
        assert_eq!(rec, Recommendation::CreatePrForReview);
        assert!(reasons.iter().any(|r| r.contains("200行ルール")));
    }
}
