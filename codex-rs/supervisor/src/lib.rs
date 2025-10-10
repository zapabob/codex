mod aggregator;
mod assigner;
mod async_subagent;
mod autonomous_dispatcher;
mod executor;
pub mod integrated;
mod planner;
pub mod subagent;
pub mod thinking_process;
mod token_tracker;
pub mod types;

use anyhow::Result;
use types::AggregatedResult;
use types::Assignment;
use types::Plan;
use types::SupervisorConfig;
use types::SupervisorResult;
use types::TaskResult;

pub use async_subagent::{AsyncSubAgent, AsyncSubAgentManager, AsyncSubAgentNotification, Inbox, NotificationType};
pub use autonomous_dispatcher::{AutonomousDispatcher, AutoCallTrigger, TaskClassification};
pub use subagent::AgentType;
pub use subagent::SubAgent;
pub use subagent::SubAgentManager;
pub use thinking_process::{ThinkingProcess, ThinkingProcessManager, ThinkingStep, ThinkingStepBuilder, ThinkingStepType};
pub use token_tracker::{TokenAllocationStrategy, TokenLimit, TokenTracker, TokenUsage};
pub use types::CoordinationStrategy;
pub use types::ManagementStyle;
pub use types::MergeStrategy;

/// Main supervisor for coordinating multiple agents
pub struct Supervisor {
    config: SupervisorConfig,
}

impl Supervisor {
    /// Create a new supervisor with the given configuration
    pub fn new(config: SupervisorConfig) -> Self {
        Self { config }
    }

    /// Coordinate execution of a goal using multiple agents
    pub async fn coordinate_goal(
        &self,
        goal: &str,
        agents_hint: Option<Vec<String>>,
    ) -> Result<SupervisorResult> {
        // 1. Analyze goal and generate plan
        let plan = self.analyze_goal(goal)?;

        // 2. Assign tasks to agents
        let assignments = self.assign_tasks(&plan, agents_hint)?;

        // 3. Execute plan
        let results = self.execute_plan(assignments.clone()).await?;

        // 4. Aggregate results
        let aggregated = self.aggregate_results(results);

        Ok(SupervisorResult {
            goal: goal.to_string(),
            plan,
            assignments,
            results: aggregated,
        })
    }

    /// Analyze a goal and generate an execution plan
    pub fn analyze_goal(&self, goal: &str) -> Result<Plan> {
        planner::analyze_goal(goal)
    }

    /// Assign tasks from a plan to agents
    pub fn assign_tasks(
        &self,
        plan: &Plan,
        agents_hint: Option<Vec<String>>,
    ) -> Result<Vec<Assignment>> {
        assigner::assign_tasks(plan, agents_hint)
    }

    /// Execute a plan with the given assignments
    pub async fn execute_plan(&self, assignments: Vec<Assignment>) -> Result<Vec<TaskResult>> {
        executor::execute_plan(assignments, self.config.strategy).await
    }

    /// Aggregate results from multiple tasks
    pub fn aggregate_results(&self, results: Vec<TaskResult>) -> AggregatedResult {
        aggregator::aggregate_results(results, self.config.merge_strategy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_supervisor_coordinate_goal() {
        let config = SupervisorConfig::default();
        let supervisor = Supervisor::new(config);

        let result = supervisor
            .coordinate_goal(
                "Implement secure auth",
                Some(vec![
                    "Security".to_string(),
                    "Backend".to_string(),
                    "Frontend".to_string(),
                ]),
            )
            .await
            .unwrap();

        assert_eq!(result.goal, "Implement secure auth");
        assert!(!result.plan.steps.is_empty());
        assert_eq!(result.assignments.len(), result.plan.steps.len());
        assert_eq!(
            result.results.individual_results.len(),
            result.assignments.len()
        );
    }

    #[tokio::test]
    async fn test_supervisor_with_different_strategies() {
        let strategies = vec![
            CoordinationStrategy::Sequential,
            CoordinationStrategy::Parallel,
            CoordinationStrategy::Hybrid,
        ];

        for strategy in strategies {
            let config = SupervisorConfig {
                strategy,
                ..Default::default()
            };
            let supervisor = Supervisor::new(config);

            let result = supervisor
                .coordinate_goal("Test goal", Some(vec!["Agent1".to_string()]))
                .await
                .unwrap();

            assert!(!result.results.individual_results.is_empty());
        }
    }
}
