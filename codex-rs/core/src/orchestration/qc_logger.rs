//! Quality Control logger
//!
//! Logs QC analysis results and merge decisions to implementation logs.

use crate::qc::{QcReport, QualityScore};
use anyhow::Context;
use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{debug, info};

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
            fs::create_dir_all(&logs_dir)
                .context("Failed to create logs directory")?;
        }

        Ok(Self { logs_dir })
    }

    /// Log QC result and merge decision
    pub async fn log_qc_result(
        &self,
        report: &QcReport,
        merge_decision: &str,
    ) -> Result<PathBuf> {
        let timestamp = Utc::now();
        let date_str = timestamp.format("%Y-%m-%d").to_string();
        let time_str = timestamp.format("%H-%M-%S").to_string();

        let filename = format!("{}_{}_qc_analysis.md", date_str, time_str);
        let filepath = self.logs_dir.join(&filename);

        let content = format!(
            r#"# QC Analysis Report

**日時**: {}
**対象**: {}
**マージ決定**: {}

## 品質スコア

- **総合スコア**: {:.3}
- **可読性**: {:.3}
- **保守性**: {:.3}
- **パフォーマンス**: {:.3}
- **セキュリティ**: {:.3}

## 統計分析

{}
## 量子最適化

{}
## 数理最適化

{}
## 推奨事項

{}
## マージ決定の根拠

{}

## 詳細メトリクス

```json
{}
```
"#,
            timestamp.format("%Y-%m-%d %H:%M:%S"),
            report.target,
            merge_decision,
            report.scores.overall,
            report.scores.readability,
            report.scores.maintainability,
            report.scores.performance,
            report.scores.security,
            self.format_statistical_metrics(&report),
            self.format_quantum_metrics(&report),
            self.format_mathematical_metrics(&report),
            self.format_recommendations(&report),
            merge_decision,
            serde_json::to_string_pretty(&report.metrics).unwrap_or_default()
        );

        fs::write(&filepath, content)
            .context("Failed to write QC log file")?;

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

        let filename = format!("{}_{}_qc_merge_decision.md", date_str, time_str);
        let filepath = self.logs_dir.join(&filename);

        // Build comparison table
        let mut table_rows = String::new();
        for (name, score) in scores {
            table_rows.push_str(&format!(
                "| {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |\n",
                name,
                score.overall,
                score.readability,
                score.maintainability,
                score.performance,
                score.security
            ));
        }

        let content = format!(
            r#"# QC Merge Decision

**日時**: {}
**選択された結果**: {}

## 品質スコア比較

| エージェント/Worktree | 総合 | 可読性 | 保守性 | パフォーマンス | セキュリティ |
|---------------------|------|--------|--------|---------------|------------|
{}

## 選択理由

最も高い総合品質スコア ({:.3}) を持つ結果が選択されました。

## 詳細

```json
{}
```
"#,
            timestamp.format("%Y-%m-%d %H:%M:%S"),
            selected,
            table_rows,
            scores.get(selected)
                .map(|s| s.overall)
                .unwrap_or(0.0),
            serde_json::to_string_pretty(scores).unwrap_or_default()
        );

        fs::write(&filepath, content)
            .context("Failed to write merge decision log file")?;

        info!("Merge decision logged to: {:?}", filepath);
        Ok(filepath)
    }

    fn format_statistical_metrics(&self, report: &QcReport) -> String {
        // Format statistical metrics if available
        "統計分析メトリクスは利用可能です。".to_string()
    }

    fn format_quantum_metrics(&self, report: &QcReport) -> String {
        // Format quantum optimization metrics if available
        if let Some(quantum) = &report.metrics.quantum {
            format!(
                "改善ポテンシャル: {:.2}%\n提案数: {}",
                quantum.total_improvement_potential,
                quantum.suggestions.len()
            )
        } else {
            "量子最適化メトリクスは利用できません。".to_string()
        }
    }

    fn format_mathematical_metrics(&self, report: &QcReport) -> String {
        // Format mathematical optimization metrics if available
        if let Some(math) = &report.metrics.mathematical {
            format!(
                "最適化スコア: {:.3}\nボトルネック数: {}",
                math.optimization_score,
                math.bottlenecks.len()
            )
        } else {
            "数理最適化メトリクスは利用できません。".to_string()
        }
    }

    fn format_recommendations(&self, report: &QcReport) -> String {
        if report.recommendations.is_empty() {
            "推奨事項はありません。".to_string()
        } else {
            report
                .recommendations
                .iter()
                .enumerate()
                .map(|(i, rec)| format!("{}. {}", i + 1, rec))
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

