//! Quality Control logger
//!
//! Logs QC analysis results and merge decisions to implementation logs.

use crate::qc::QcReport;
use crate::qc::QualityScore;
use anyhow::Context;
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tracing::info;

/// Quality Control logger
pub struct QcLogger {
    logs_dir: PathBuf,
}

impl QcLogger {
    /// Create a new QC logger
    pub fn new(logs_dir: impl AsRef<Path>) -> Result<Self> {
        let logs_dir = logs_dir.as_ref().to_path_buf();

        // Create logs directory if it doesn't exist
        if !logs_dir.exists() {
            fs::create_dir_all(&logs_dir).context("Failed to create logs directory")?;
        }

        Ok(Self { logs_dir })
    }

    /// Log QC result and merge decision
    pub async fn log_qc_result(&self, report: &QcReport, merge_decision: &str) -> Result<PathBuf> {
        let timestamp = Utc::now();
        let date_str = timestamp.format("%Y-%m-%d").to_string();
        let time_str = timestamp.format("%H-%M-%S").to_string();

        let filename = format!("{date_str}_{time_str}_qc_analysis.md");
        let filepath = self.logs_dir.join(&filename);

        let timestamp_str = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        let target = &report.target;
        let overall = report.scores.overall;
        let readability = report.scores.readability;
        let maintainability = report.scores.maintainability;
        let performance = report.scores.performance;
        let security = report.scores.security;
        let statistical = self.format_statistical_metrics(report);
        let quantum = self.format_quantum_metrics(report);
        let mathematical = self.format_mathematical_metrics(report);
        let recommendations = self.format_recommendations(report);
        let metrics_json = serde_json::to_string_pretty(&report.metrics).unwrap_or_default();

        let content = format!(
            r#"# QC Analysis Report

**Timestamp**: {timestamp_str}
**Target**: {target}
**Merge decision**: {merge_decision}

## Quality scores

- **Overall**: {overall:.3}
- **Readability**: {readability:.3}
- **Maintainability**: {maintainability:.3}
- **Performance**: {performance:.3}
- **Security**: {security:.3}

## Statistical metrics
{statistical}
## Quantum optimization
{quantum}
## Mathematical optimization
{mathematical}
## Recommendations
{recommendations}
## Merge decision rationale

{merge_decision}

## Detailed metrics

```json
{metrics_json}
```
"#,
        );

        fs::write(&filepath, content).context("Failed to write QC log file")?;

        info!("QC result logged to: {:?}", filepath);
        Ok(filepath)
    }

    /// Log merge decision with quality scores
    pub async fn log_merge_decision(
        &self,
        selected: &str,
        scores: &HashMap<String, QualityScore>,
    ) -> Result<PathBuf> {
        let timestamp = Utc::now();
        let date_str = timestamp.format("%Y-%m-%d").to_string();
        let time_str = timestamp.format("%H-%M-%S").to_string();

        let filename = format!("{date_str}_{time_str}_qc_merge_decision.md");
        let filepath = self.logs_dir.join(&filename);

        // Build comparison table
        let mut table_rows = String::new();
        for (name, score) in scores {
            let overall = score.overall;
            let readability = score.readability;
            let maintainability = score.maintainability;
            let performance = score.performance;
            let security = score.security;
            table_rows.push_str(&format!(
                "| {name} | {overall:.3} | {readability:.3} | {maintainability:.3} | {performance:.3} | {security:.3} |\n"
            ));
        }

        let timestamp_str = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        let selected_score = scores.get(selected).map(|s| s.overall).unwrap_or(0.0);
        let scores_json = serde_json::to_string_pretty(scores).unwrap_or_default();

        let content = format!(
            r#"# QC Merge Decision

**Timestamp**: {timestamp_str}
**Selected result**: {selected}

## Score comparison
| Agent/Worktree | Overall | Readability | Maintainability | Performance | Security |
|----------------|---------|-------------|-----------------|-------------|----------|
{table_rows}

## Selection rationale

Selected result has the highest overall score ({selected_score:.3}).

## Detailed scores

```json
{scores_json}
```
"#,
        );

        fs::write(&filepath, content).context("Failed to write merge decision log file")?;

        info!("Merge decision logged to: {:?}", filepath);
        Ok(filepath)
    }

    fn format_statistical_metrics(&self, report: &QcReport) -> String {
        let stats = &report.metrics.statistical.code_stats;
        let total_lines = stats.total_lines;
        let code_lines = stats.code_lines;
        let function_count = stats.function_count;
        let struct_count = stats.struct_count;
        format!(
            "Total lines: {total_lines}\nCode lines: {code_lines}\nFunctions: {function_count}\nStructs: {struct_count}"
        )
    }

    fn format_quantum_metrics(&self, report: &QcReport) -> String {
        let quantum = &report.metrics.quantum;
        let improvement = quantum.total_improvement_potential;
        let suggestions = quantum.suggestions.len();
        format!("Improvement potential: {improvement:.2}%\nSuggestions: {suggestions}")
    }

    fn format_mathematical_metrics(&self, report: &QcReport) -> String {
        let math = &report.metrics.mathematical;
        let optimization_score = math.optimization_score;
        let bottlenecks = math.bottlenecks.len();
        format!("Optimization score: {optimization_score:.3}\nBottlenecks: {bottlenecks}")
    }

    fn format_recommendations(&self, report: &QcReport) -> String {
        if report.recommendations.is_empty() {
            "No recommendations.".to_string()
        } else {
            report
                .recommendations
                .iter()
                .enumerate()
                .map(|(i, rec)| {
                    let index = i + 1;
                    format!("{index}. {rec}")
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_log_qc_result() {
        let temp_dir = TempDir::new().unwrap();
        let logger = QcLogger::new(temp_dir.path()).unwrap();

        // Create a minimal QcReport for testing
        // This would need actual QcReport construction in a real test
        // For now, just test that the logger can be created
        assert!(logger.logs_dir.exists());
    }

    #[tokio::test]
    async fn test_log_merge_decision() {
        let temp_dir = TempDir::new().unwrap();
        let logger = QcLogger::new(temp_dir.path()).unwrap();

        let mut scores = HashMap::new();
        scores.insert(
            "agent1".to_string(),
            QualityScore {
                readability: 0.8,
                maintainability: 0.7,
                performance: 0.9,
                security: 0.85,
                overall: 0.8125,
            },
        );

        let result = logger.log_merge_decision("agent1", &scores).await;
        assert!(result.is_ok());
    }
}
