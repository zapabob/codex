//! Plan-aware orchestrator.
//!
//! Enhances AutoOrchestrator to accept PlanBlock, emit telemetry, and trigger webhooks.

use crate::agents::runtime::AgentRuntime;
use crate::orchestration::AutoOrchestrator;
use crate::orchestration::CollaborationStore;
use crate::orchestration::OrchestratedResult;
use crate::orchestration::ParallelOrchestrator;
use crate::orchestration::TaskAnalyzer;
use crate::orchestration::parallel_execution::AgentTask;
use crate::orchestration::parallel_execution::AgentType;
use crate::orchestration::qc_logger::QcLogger;
use crate::orchestration::qc_merger::QcMerger;
use crate::plan::ExecutionMode;
use crate::plan::PlanBlock;
// Telemetry and webhooks modules not available
// use crate::telemetry::EventType;
// use crate::telemetry::TelemetryEvent;
// use crate::webhooks::WebhookConfig;
// use crate::webhooks::WebhookPayload;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Plan-aware orchestrator
pub struct PlanOrchestrator {
    /// Underlying auto-orchestrator
    auto_orchestrator: AutoOrchestrator,
    /// Agent runtime for single-agent execution
    runtime: Arc<AgentRuntime>,
    /// Parallel orchestrator for worktree competition
    parallel_orchestrator: ParallelOrchestrator,
    /// Task analyzer for agent selection
    task_analyzer: TaskAnalyzer,

    /// Webhook configurations (optional) - disabled, module not available
    _webhook_configs: Vec<()>, // Vec<WebhookConfig>,
}

impl PlanOrchestrator {
    /// Create a new Plan orchestrator
    pub fn new(
        runtime: Arc<AgentRuntime>,
        collaboration_store: Arc<CollaborationStore>,
        workspace_dir: std::path::PathBuf,
        _webhook_configs: Vec<()>, // Vec<WebhookConfig>,
    ) -> Self {
        let auto_orchestrator =
            AutoOrchestrator::new(Arc::clone(&runtime), collaboration_store, workspace_dir.clone());
        let parallel_orchestrator = ParallelOrchestrator::with_repo_path(workspace_dir.clone());
        let task_analyzer = TaskAnalyzer::new(0.7);

        Self {
            auto_orchestrator,
            runtime,
            parallel_orchestrator,
            task_analyzer,
            _webhook_configs,
        }
    }

    /// Execute a plan with telemetry and webhooks.
    pub async fn execute_plan(&self, plan: &PlanBlock) -> Result<OrchestratedResult> {
        let analysis = self.task_analyzer.analyze(&plan.goal);
        match plan.mode {
            ExecutionMode::Single => self.execute_single_agent(plan, analysis).await,
            ExecutionMode::Orchestrated => {
                self.auto_orchestrator
                    .orchestrate(analysis, plan.goal.clone())
                    .await
            }
            ExecutionMode::Competition => self.execute_competition(plan, analysis).await,
        }
    }

    /// Emit telemetry event - disabled, module not available
    #[allow(dead_code)]
    async fn emit_telemetry_event(
        &self,
        _event_type: (),
        _plan: &(),
        _result: Option<&OrchestratedResult>,
    ) {
        // Telemetry module not available, skip emission
    }

    /// Trigger webhook - disabled, module not available
    #[allow(dead_code)]
    async fn trigger_webhook(
        &self,
        _event_name: &str,
        _plan: &(),
        _result: Option<&OrchestratedResult>,
    ) {
        // Webhook module not available, skip
        /*
        if self.webhook_configs.is_empty() {
            return;
        }

        let summary = if let Some(r) = result {
            format!(
                "Execution completed. Orchestrated: {}. Agents: {}. Time: {:.1}s",
                r.was_orchestrated,
                r.agents_used.join(", "),
                r.total_execution_time_secs
            )
        } else {
            format!("Execution started for Plan: {}", plan.title)
        };

        let mut payload = WebhookPayload::new(
            plan.id.clone(),
            plan.title.clone(),
            plan.state.clone(),
            summary,
        );

        payload = payload.with_mode(plan.mode.to_string());
        payload = payload.with_artifacts(plan.artifacts.clone());

        // Send webhooks asynchronously
        for config in &self.webhook_configs {
            let config = config.clone();
            let payload = payload.clone();

            tokio::spawn(async move {
                if let Err(e) = crate::webhooks::send(&config, &payload).await {
                    debug!("Failed to send webhook: {}", e);
                }
            });
        }
        */
    }

    async fn execute_single_agent(
        &self,
        plan: &PlanBlock,
        mut analysis: crate::orchestration::TaskAnalysis,
    ) -> Result<OrchestratedResult> {
        let start = Instant::now();
        let agent = analysis
            .recommended_agents
            .first()
            .cloned()
            .unwrap_or_else(|| "code-reviewer".to_string());

        analysis.recommended_agents = vec![agent.clone()];

        let mut inputs = HashMap::new();
        inputs.insert("goal".to_string(), plan.goal.clone());
        inputs.insert("plan_id".to_string(), plan.id.clone());
        inputs.insert("plan_mode".to_string(), plan.mode.to_string());
        inputs.insert(
            "workspace".to_string(),
            std::env::current_dir()?.display().to_string(),
        );
        if !plan.work_items.is_empty()
            && let Ok(work_items) = serde_json::to_string(&plan.work_items)
        {
            inputs.insert("work_items".to_string(), work_items);
        }

        let budget = plan.budget.max_step.map(|value| value as usize);
        let deadline = plan.budget.cap_min;
        let result = self
            .runtime
            .delegate(&agent, &plan.goal, inputs, budget, deadline)
            .await?;

        let status = result.status;
        let duration_secs = result.duration_secs;
        let summary = format!(
            "Single-agent execution finished with status {status:?} in {duration_secs:.2}s."
        );

        Ok(OrchestratedResult {
            was_orchestrated: false,
            agents_used: vec![agent],
            execution_summary: summary,
            agent_results: vec![result.clone()],
            total_execution_time_secs: start.elapsed().as_secs_f64(),
            task_analysis: analysis,
        })
    }

    async fn execute_competition(
        &self,
        plan: &PlanBlock,
        analysis: crate::orchestration::TaskAnalysis,
    ) -> Result<OrchestratedResult> {
        let start = Instant::now();
        let prompt = build_competition_prompt(plan);

        let tasks = vec![
            AgentTask {
                agent: AgentType::Codex,
                prompt: prompt.clone(),
                worktree_path: None,
                timeout_seconds: None,
                reasoning_effort: None,
            },
            AgentTask {
                agent: AgentType::GeminiCLI,
                prompt: prompt.clone(),
                worktree_path: None,
                timeout_seconds: None,
                reasoning_effort: None,
            },
            AgentTask {
                agent: AgentType::Claudecode,
                prompt,
                worktree_path: None,
                timeout_seconds: None,
                reasoning_effort: None,
            },
        ];

        let results = self.parallel_orchestrator.execute_parallel(tasks).await?;
        let qc_merger = QcMerger::new();
        let (best_result, scores) = qc_merger.select_best_central(results.clone()).await?;
        let summary = build_competition_summary(&results, &best_result, &scores);
        let agent_results = results.iter().map(map_parallel_result).collect();

        if let Ok(cwd) = std::env::current_dir() {
            let log_dir = cwd.join("_docs");
            if let Ok(logger) = QcLogger::new(&log_dir) {
                let agent = best_result.agent;
                let best_key = format!("{agent:?}");
                let _ = logger.log_merge_decision(&best_key, &scores).await;
            }
        }

        Ok(OrchestratedResult {
            was_orchestrated: true,
            agents_used: results
                .iter()
                .map(|result| {
                    let agent = result.agent;
                    format!("{agent:?}").to_lowercase()
                })
                .collect(),
            execution_summary: summary,
            agent_results,
            total_execution_time_secs: start.elapsed().as_secs_f64(),
            task_analysis: analysis,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Coverage exercised through plan executor integration tests.
}

fn build_competition_prompt(plan: &PlanBlock) -> String {
    let mut prompt = String::new();
    prompt.push_str("Plan execution goal:\n");
    prompt.push_str(&plan.goal);
    prompt.push_str("\n\nWork items:\n");
    for item in &plan.work_items {
        let item_name = &item.name;
        prompt.push_str(&format!("- {item_name}\n"));
        if !item.files_touched.is_empty() {
            let files = item.files_touched.join(", ");
            prompt.push_str(&format!("  Files: {files}\n"));
        }
        if !item.tests.is_empty() {
            let tests = item.tests.join(", ");
            prompt.push_str(&format!("  Tests: {tests}\n"));
        }
    }

    if !plan.assumptions.is_empty() {
        let assumptions = plan.assumptions.join("; ");
        prompt.push_str(&format!("\nAssumptions: {assumptions}\n"));
    }

    if !plan.risks.is_empty() {
        prompt.push_str("\nRisks:\n");
        for risk in &plan.risks {
            let item = &risk.item;
            let mitigation = &risk.mitigation;
            prompt.push_str(&format!("- {item} (mitigation: {mitigation})\n"));
        }
    }

    prompt.push_str("\nDeliver a high-quality implementation plan and key changes.");
    prompt
}

fn build_competition_summary(
    results: &[crate::orchestration::parallel_execution::AgentResult],
    best_result: &crate::orchestration::parallel_execution::AgentResult,
    scores: &std::collections::HashMap<String, crate::qc::QualityScore>,
) -> String {
    let comparison = crate::orchestration::parallel_execution::compare_results(results);
    let total_agents = comparison.total_agents;
    let successful = comparison.successful;
    let failed = comparison.failed;
    let mut summary =
        format!("Competition executed {total_agents} agents: {successful} succeeded, {failed} failed.");

    if let Some(agent) = comparison.fastest_agent {
        if let Some(time) = comparison.fastest_time {
            summary.push_str(&format!(
                " Fastest successful agent: {agent:?} ({time:.2}s)."
            ));
        }
    }

    let agent = best_result.agent;
    let best_key = format!("{agent:?}");
    let best_name = best_key.to_lowercase();
    if let Some(score) = scores.get(&best_key) {
        let overall = score.overall;
        summary.push_str(&format!(
            " QC selected {best_name} with overall score {overall:.3}."
        ));
    } else {
        summary.push_str(&format!(" QC selected {best_name}."));
    }

    summary
}

fn map_parallel_result(
    result: &crate::orchestration::parallel_execution::AgentResult,
) -> crate::agents::types::AgentResult {
    let status = if result.success {
        crate::agents::types::AgentStatus::Completed
    } else {
        crate::agents::types::AgentStatus::Failed
    };

    crate::agents::types::AgentResult {
        agent_name: {
            let agent = result.agent;
            format!("{agent:?}").to_lowercase()
        },
        status,
        artifacts: Vec::new(),
        tokens_used: 0,
        duration_secs: result.elapsed_seconds,
        error: result.error.clone(),
    }
}
