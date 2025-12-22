//! Plan-aware orchestrator
//!
//! Enhances AutoOrchestrator to accept PlanBlock, emit telemetry, and trigger webhooks.

use crate::agents::runtime::AgentRuntime;
use crate::orchestration::AutoOrchestrator;
use crate::orchestration::CollaborationStore;
use crate::orchestration::OrchestratedResult;
use crate::orchestration::TaskAnalyzer;
// PlanBlock module not available - this orchestrator is disabled
// use crate::plan::PlanBlock;
// Telemetry and webhooks modules not available
// use crate::telemetry::EventType;
// use crate::telemetry::TelemetryEvent;
// use crate::webhooks::WebhookConfig;
// use crate::webhooks::WebhookPayload;
use anyhow::Context;
use anyhow::Result;
use std::sync::Arc;
use tracing::debug;
use tracing::info;

/// Plan-aware orchestrator
pub struct PlanOrchestrator {
    /// Underlying auto-orchestrator
    auto_orchestrator: AutoOrchestrator,

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
        let auto_orchestrator = AutoOrchestrator::new(runtime, collaboration_store, workspace_dir);

        Self {
            auto_orchestrator,
            _webhook_configs: _webhook_configs,
        }
    }

    /// Execute a plan with telemetry and webhooks - disabled, PlanBlock not available
    #[allow(dead_code)]
    pub async fn execute_plan(&self, _plan: &()) -> Result<OrchestratedResult> {
        // PlanBlock module not available - return error
        anyhow::bail!("PlanOrchestrator is disabled: PlanBlock module not available");
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
}

#[cfg(test)]
mod tests {
    use super::*;
    // Tests disabled - PlanBlock module not available
}
