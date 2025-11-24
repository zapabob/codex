//! Quality Control based merger
//!
//! Selects the highest quality code based on QC analysis results
//! for both central orchestration and worktree parallel development modes.

use crate::orchestration::parallel_execution::AgentResult;
use crate::orchestration::worktree_manager::WorktreeInfo;
use crate::qc::{QcAgent, QcConfig, QcReport, QualityScore};
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Quality Control based merger
pub struct QcMerger {
    qc_agent: Arc<QcAgent>,
}

impl QcMerger {
    /// Create a new QC merger with default configuration
    pub fn new() -> Self {
        Self::with_config(QcConfig::default())
    }

    /// Create a new QC merger with custom configuration
    pub fn with_config(config: QcConfig) -> Self {
        Self {
            qc_agent: Arc::new(QcAgent::with_config(config)),
        }
    }

    /// Select the best result from multiple agent results (Central mode)
    ///
    /// Analyzes each agent's output and selects the one with the highest quality score.
    pub async fn select_best_central(
        &self,
        results: Vec<AgentResult>,
    ) -> Result<(AgentResult, HashMap<String, QualityScore>)> {
        if results.is_empty() {
            anyhow::bail!("No results to select from");
        }

        info!("Selecting best result from {} agent results", results.len());

        let mut scores = HashMap::new();
        let mut qc_reports = Vec::new();

        // Analyze each successful result
        for (idx, result) in results.iter().enumerate() {
            if !result.success {
                debug!("Skipping failed result from {:?}", result.agent);
                continue;
            }

            let agent_name = format!("{:?}", result.agent);
            info!("Analyzing result from {}", agent_name);

            // Run QC analysis on the output
            match self
                .qc_agent
                .analyze(&result.output, &format!("agent_{}", idx))
                .await
            {
                Ok(report) => {
                    let score = report.scores.clone();
                    scores.insert(agent_name.clone(), score.clone());
                    qc_reports.push((idx, score.overall, result.clone()));
                    info!(
                        "QC analysis for {}: overall score = {:.3}",
                        agent_name, score.overall
                    );
                }
                Err(e) => {
                    warn!("QC analysis failed for {}: {}", agent_name, e);
                    // Use default score of 0.5 if analysis fails
                    let default_score = QualityScore {
                        readability: 0.5,
                        maintainability: 0.5,
                        performance: 0.5,
                        security: 0.5,
                        overall: 0.5,
                    };
                    scores.insert(agent_name.clone(), default_score.clone());
                    qc_reports.push((idx, 0.5, result.clone()));
                }
            }
        }

        if qc_reports.is_empty() {
            warn!("No successful QC analyses, selecting first result");
            return Ok((results[0].clone(), scores));
        }

        // Select the result with the highest overall score
        qc_reports.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let (best_idx, best_score, best_result) = &qc_reports[0];

        info!(
            "Selected best result from agent {:?} with quality score {:.3}",
            best_result.agent, best_score
        );

        Ok((best_result.clone(), scores))
    }

    /// Select the best worktree based on QC analysis (Worktree mode)
    ///
    /// Analyzes code in each worktree and selects the one with the highest quality score.
    pub async fn select_best_worktree(
        &self,
        worktrees: Vec<WorktreeInfo>,
    ) -> Result<(WorktreeInfo, HashMap<String, QualityScore>)> {
        if worktrees.is_empty() {
            anyhow::bail!("No worktrees to select from");
        }

        info!("Selecting best worktree from {} worktrees", worktrees.len());

        let mut scores = HashMap::new();
        let mut qc_reports = Vec::new();

        // Analyze code in each worktree
        for worktree in &worktrees {
            info!("Analyzing worktree: {}", worktree.name);

            // Read source code from worktree
            let source_code = Self::read_worktree_code(&worktree.path)?;

            // Run QC analysis
            match self
                .qc_agent
                .analyze(&source_code, &worktree.name)
                .await
            {
                Ok(report) => {
                    let score = report.scores.clone();
                    scores.insert(worktree.name.clone(), score.clone());
                    qc_reports.push((worktree.clone(), score.overall));
                    info!(
                        "QC analysis for {}: overall score = {:.3}",
                        worktree.name, score.overall
                    );
                }
                Err(e) => {
                    warn!("QC analysis failed for {}: {}", worktree.name, e);
                    // Use default score of 0.5 if analysis fails
                    let default_score = QualityScore {
                        readability: 0.5,
                        maintainability: 0.5,
                        performance: 0.5,
                        security: 0.5,
                        overall: 0.5,
                    };
                    scores.insert(worktree.name.clone(), default_score.clone());
                    qc_reports.push((worktree.clone(), 0.5));
                }
            }
        }

        if qc_reports.is_empty() {
            warn!("No successful QC analyses, selecting first worktree");
            return Ok((worktrees[0].clone(), scores));
        }

        // Select the worktree with the highest overall score
        qc_reports.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let (best_worktree, best_score) = &qc_reports[0];

        info!(
            "Selected best worktree {} with quality score {:.3}",
            best_worktree.name, best_score
        );

        Ok((best_worktree.clone(), scores))
    }

    /// Read source code from a worktree directory
    fn read_worktree_code(worktree_path: &PathBuf) -> Result<String> {
        use std::fs;
        use walkdir::WalkDir;

        let mut source_content = String::new();

        // Walk through directory and read source files
        for entry in WalkDir::new(worktree_path)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" || ext == "ts" || ext == "tsx" || ext == "js" || ext == "jsx" {
                        if let Ok(content) = fs::read_to_string(path) {
                            source_content.push_str(&format!("\n// File: {}\n", path.display()));
                            source_content.push_str(&content);
                            source_content.push_str("\n");
                        }
                    }
                }
            }
        }

        if source_content.is_empty() {
            source_content = "// No source files found".to_string();
        }

        Ok(source_content)
    }
}

impl Default for QcMerger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::parallel_execution::AgentType;

    #[tokio::test]
    async fn test_select_best_central_empty() {
        let merger = QcMerger::new();
        let result = merger.select_best_central(vec![]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_select_best_worktree_empty() {
        let merger = QcMerger::new();
        let result = merger.select_best_worktree(vec![]).await;
        assert!(result.is_err());
    }
}

