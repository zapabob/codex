//! QC logging functionality

use chrono::DateTime;
use chrono::Local;
use std::fs::OpenOptions;
use std::fs::{self};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::qc::QcResult;
use crate::qc::WorktreeInfo;

/// QC logger for writing structured log entries
pub struct QcLogger {
    logs_dir: PathBuf,
}

impl QcLogger {
    /// Create a new QC logger with the specified logs directory
    pub fn new<P: AsRef<Path>>(logs_dir: P) -> Self {
        Self {
            logs_dir: logs_dir.as_ref().to_path_buf(),
        }
    }

    /// Log a QC result
    pub fn log(&self, worktree: &WorktreeInfo, result: &QcResult) -> Result<PathBuf, String> {
        // Ensure logs directory exists
        fs::create_dir_all(&self.logs_dir)
            .map_err(|e| format!("Failed to create logs directory: {}", e))?;

        // Get local time
        let now: DateTime<Local> = Local::now();

        // Generate log file name: YYYY-MM-DD-{worktreename}-impl.md
        let date_str = now.format("%Y-%m-%d").to_string();
        let log_filename = format!("{}-{}-impl.md", date_str, worktree.name);
        let log_path = self.logs_dir.join(&log_filename);

        // Format the log entry
        let log_entry = self.format_log_entry(now, worktree, result);

        // Append to log file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;

        writeln!(file, "{}", log_entry).map_err(|e| format!("Failed to write log entry: {}", e))?;

        Ok(log_path)
    }

    /// Format a log entry in the specified structure
    fn format_log_entry(
        &self,
        time: DateTime<Local>,
        worktree: &WorktreeInfo,
        result: &QcResult,
    ) -> String {
        let timestamp = time.format("%Y-%m-%d %H:%M:%S %z").to_string();

        let mut entry = String::new();
        entry.push_str(&format!("## {}\n\n", timestamp));
        entry.push_str(&format!("- Worktree: {}\n", worktree.name));
        entry.push_str("- 機能: [Description of feature/change]\n");
        entry.push_str(&format!("- Profile: {}\n", result.profile));
        entry.push_str(&format!(
            "- Changed lines: +{} / -{} (Total: {})\n",
            result.lines_added,
            result.lines_deleted,
            result.total_changed_lines()
        ));
        entry.push_str(&format!("- Files changed: {}\n", result.files_changed));
        entry.push_str(&format!("- Risk score: {:.2}\n", result.risk_score));
        entry.push_str(&format!("- Recommendation: {}\n", result.recommendation));
        entry.push_str("\n### Test Results\n\n");

        // Rust tests
        entry.push_str("**Rust:**\n");
        for (cmd, status) in &result.rust_test_results {
            let status_icon = if *status { "✅" } else { "❌" };
            entry.push_str(&format!("- {} `{}`\n", status_icon, cmd));
        }
        entry.push_str("\n");

        // Web tests
        if !result.web_test_results.is_empty() {
            entry.push_str("**Web/GUI:**\n");
            for (cmd, status) in &result.web_test_results {
                let status_icon = if *status { "✅" } else { "❌" };
                entry.push_str(&format!("- {} `{}`\n", status_icon, cmd));
            }
            entry.push_str("\n");
        }

        // Warnings
        if !result.warnings.is_empty() {
            entry.push_str("### Warnings\n\n");
            for warning in &result.warnings {
                entry.push_str(&format!("- ⚠️  {}\n", warning));
            }
            entry.push_str("\n");
        }

        entry.push_str("---\n\n");
        entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qc::QcRecommendation;
    use crate::qc::TestProfile;

    #[test]
    fn test_format_log_entry() {
        let logger = QcLogger::new("/tmp/logs");
        let time = Local::now();
        let worktree = WorktreeInfo {
            path: PathBuf::from("/home/user/project"),
            name: "feature-test".to_string(),
            branch: "feature/test".to_string(),
        };
        let result = QcResult {
            profile: TestProfile::Standard,
            lines_added: 50,
            lines_deleted: 20,
            files_changed: 3,
            risk_score: 0.5,
            recommendation: QcRecommendation::Approve,
            rust_test_results: vec![("cargo test --all".to_string(), true)],
            web_test_results: vec![],
            warnings: vec![],
        };

        let entry = logger.format_log_entry(time, &worktree, &result);

        assert!(entry.contains("feature-test"));
        assert!(entry.contains("Profile: standard"));
        assert!(entry.contains("✅"));
    }
}
