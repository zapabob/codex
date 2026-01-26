//! Plan manager
//!
//! High-level API for creating, updating, approving, and exporting plan.
//!
//! ## Integration with Official Plan Mode
//!
//! This custom Plan module works in conjunction with the official Plan Mode collaboration
//! template (`codex-rs/core/templates/collaboration_mode/plan.md`). The official template
//! provides the conversational prompt structure for LLM interactions, while this module
//! manages the structured `PlanBlock` data that results from those interactions.
//!
//! ### Official Plan Mode Improvements (commits #9877, #9874)
//!
//! The official Plan Mode follows a 2-phase conversational approach:
//!
//! **PHASE 1 — Intent chat**: Gather goal, success criteria, audience, scope, constraints,
//! current state, and key preferences/tradeoffs. Bias toward questions over guessing.
//!
//! **PHASE 2 — Implementation chat**: Once intent is stable, gather decision-complete spec:
//! approach, interfaces, data flow, edge cases, testing criteria, rollout/monitoring.
//!
//! Key principles:
//! - Ask many questions, but only those that materially change the spec/plan
//! - Batch questions (4-10) per `request_user_input` call
//! - Explore discoverable facts first before asking
//! - Ask preferences/tradeoffs early with 2-4 options + recommended default
//!
//! ### Custom Plan Features
//!
//! This module extends the official Plan Mode with advanced features:
//! - **Budget management**: Token and time budgets per step and session
//! - **Execution logging**: Detailed logs of plan execution with events
//! - **Orchestration**: Support for single, orchestrated, and competition execution modes
//! - **Research integration**: Deep research capabilities for plan creation
//! - **Quality assurance**: QC analysis and quality gates
//! - **State management**: Drafting, approval, execution, and completion states

use super::persist::PlanPersister;
use super::policy::ApprovalRole;
use super::policy::PolicyEnforcer;
use super::policy::PrivilegedOperation;
use super::schema::PlanBlock;
use super::schema::ResearchBlock;
use super::schema::Risk;
use super::schema::WorkItem;
use super::state::PlanState;
use super::state::StateTransitionError;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use thiserror::Error;

/// Errors from Plan manager operations
#[derive(Debug, Error)]
pub enum ManagerError {
    #[error("Plan not found: {id}")]
    NotFound { id: String },

    #[error("Plan cannot be modified in current state: {state}")]
    CannotModify { state: String },

    #[error("State transition error: {0}")]
    StateTransition(#[from] StateTransitionError),

    #[error("Persistence error: {0}")]
    Persistence(#[from] std::io::Error),

    #[error("Policy violation: {0}")]
    Policy(#[from] super::policy::PolicyError),
}

/// Plan manager for high-level operations
pub struct PlanManager {
    /// In-memory Plan store
    plans: Arc<RwLock<HashMap<String, PlanBlock>>>,

    /// Persister for saving Plans
    persister: PlanPersister,

    /// Policy enforcer
    policy_enforcer: PolicyEnforcer,
}

impl PlanManager {
    /// Create a new Plan manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            plans: Arc::new(RwLock::new(HashMap::new())),
            persister: PlanPersister::new()?,
            policy_enforcer: PolicyEnforcer::default(),
        })
    }

    /// Create a new Plan manager with custom persister and policy
    pub fn with_config(persister: PlanPersister, policy_enforcer: PolicyEnforcer) -> Self {
        Self {
            plans: Arc::new(RwLock::new(HashMap::new())),
            persister,
            policy_enforcer,
        }
    }

    /// Create a new Plan
    ///
    /// Creates a new PlanBlock in Drafting state. The Plan should be populated through
    /// interactions following the official Plan Mode 2-phase approach:
    ///
    /// 1. **Intent chat**: Populate `goal`, `assumptions`, and `clarifying_questions`
    /// 2. **Implementation chat**: Populate `approach`, `work_items`, `risks`, and `eval`
    ///
    /// See `codex-rs/core/templates/collaboration_mode/plan.md` for the official
    /// conversational prompt structure.
    ///
    /// # Arguments
    ///
    /// * `goal` - High-level goal for the plan (from Phase 1)
    /// * `title` - Descriptive title for the plan
    /// * `created_by` - Optional identifier for the plan creator
    ///
    /// # Returns
    ///
    /// The unique Plan ID that can be used to retrieve or update the plan.
    #[allow(non_snake_case)]
    pub fn create_Plan(
        &self,
        goal: String,
        title: String,
        created_by: Option<String>,
    ) -> Result<String> {
        let mut bp = PlanBlock::new(goal, title);
        bp.created_by = created_by;
        bp.state = PlanState::Inactive.start_drafting()?;

        let id = bp.id.clone();

        // Store in memory
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(id.clone(), bp.clone());
        }

        // Persist to disk
        self.persister
            .save_json(&bp)
            .context("Failed to persist Plan")?;

        Ok(id)
    }

    /// Get a Plan by ID
    #[allow(non_snake_case)]
    pub fn get_Plan(&self, id: &str) -> Result<PlanBlock> {
        // Try memory first
        {
            let plans = self.plans.read().unwrap();
            if let Some(bp) = plans.get(id) {
                return Ok(bp.clone());
            }
        }

        // Try loading from disk
        let bp = self
            .persister
            .load_json(id)
            .map_err(|_| ManagerError::NotFound { id: id.to_string() })?;

        // Cache in memory
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(id.to_string(), bp.clone());
        }

        Ok(bp)
    }

    /// Update a Plan (creates new version if scope changes)
    #[allow(non_snake_case)]
    pub fn update_Plan(&self, id: &str, update_fn: impl FnOnce(&mut PlanBlock)) -> Result<String> {
        let mut bp = self.get_Plan(id)?;

        // Check if Plan can be modified
        if !bp.state.can_modify() {
            return Err(ManagerError::CannotModify {
                state: bp.state.name().to_string(),
            }
            .into());
        }

        // Apply updates
        update_fn(&mut bp);
        bp.touch();

        // Save
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(bp.id.clone(), bp.clone());
        }

        self.persister.save_json(&bp)?;

        Ok(bp.id.clone())
    }

    /// Submit Plan for approval
    pub fn submit_for_approval(&self, id: &str) -> Result<()> {
        let mut bp = self.get_Plan(id)?;
        bp.state = bp.state.submit_for_approval()?;
        bp.touch();

        // Save
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(bp.id.clone(), bp.clone());
        }

        self.persister.save_json(&bp)?;
        Ok(())
    }

    /// Approve a Plan
    #[allow(non_snake_case)]
    pub fn approve_Plan(
        &self,
        id: &str,
        approver: String,
        approver_role: ApprovalRole,
    ) -> Result<()> {
        // Check policy
        self.policy_enforcer
            .enforce(PrivilegedOperation::ShellExec, Some(approver_role), None)?;

        let mut bp = self.get_Plan(id)?;
        bp.state = bp.state.approve(approver)?;
        bp.touch();

        // Save
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(bp.id.clone(), bp.clone());
        }

        self.persister.save_json(&bp)?;
        Ok(())
    }

    /// Reject a Plan
    #[allow(non_snake_case)]
    pub fn reject_Plan(&self, id: &str, reason: String, rejector: Option<String>) -> Result<()> {
        let mut bp = self.get_Plan(id)?;
        bp.state = bp.state.reject(reason, rejector)?;
        bp.touch();

        // Save
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(bp.id.clone(), bp.clone());
        }

        self.persister.save_json(&bp)?;
        Ok(())
    }

    /// Supersede a Plan with a new version
    #[allow(non_snake_case)]
    pub fn supersede_Plan(&self, id: &str, new_id: String) -> Result<()> {
        let mut bp = self.get_Plan(id)?;
        bp.state = bp.state.supersede(new_id)?;
        bp.touch();

        // Save
        {
            let mut plans = self.plans.write().unwrap();
            plans.insert(bp.id.clone(), bp.clone());
        }

        self.persister.save_json(&bp)?;
        Ok(())
    }

    /// Export Plan to markdown and JSON
    #[allow(non_snake_case)]
    pub fn export_Plan(&self, id: &str) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
        let bp = self.get_Plan(id)?;
        self.persister.export(&bp).context("Failed to export Plan")
    }

    /// Add work item to Plan
    pub fn add_work_item(&self, id: &str, work_item: WorkItem) -> Result<()> {
        self.update_Plan(id, |bp| {
            bp.add_work_item(work_item);
        })?;
        Ok(())
    }

    /// Add risk to Plan
    pub fn add_risk(&self, id: &str, risk: Risk) -> Result<()> {
        self.update_Plan(id, |bp| {
            bp.add_risk(risk);
        })?;
        Ok(())
    }

    /// Add research results to Plan
    pub fn add_research(&self, id: &str, research: ResearchBlock) -> Result<()> {
        self.update_Plan(id, |bp| {
            bp.set_research(research);
        })?;
        Ok(())
    }

    /// List all Plan IDs
    #[allow(non_snake_case)]
    pub fn list_Plans(&self) -> Result<Vec<String>> {
        self.persister.list_plans().context("Failed to list Plans")
    }

    /// Delete a Plan (soft delete - marks as superseded)
    #[allow(non_snake_case)]
    pub fn delete_Plan(&self, id: &str) -> Result<()> {
        self.supersede_Plan(id, "deleted".to_string())
    }
}

impl Default for PlanManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default PlanManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_manager() -> (PlanManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let markdown_dir = temp_dir.path().join("markdown");
        let json_dir = temp_dir.path().join("json");

        let persister = PlanPersister::with_dirs(markdown_dir, json_dir).unwrap();
        let manager = PlanManager::with_config(persister, PolicyEnforcer::default());

        (manager, temp_dir)
    }

    #[test]
    fn test_create_and_get_plan() {
        let (manager, _temp) = create_test_manager();

        let id = manager
            .create_Plan(
                "Test goal".to_string(),
                "test-bp".to_string(),
                Some("user1".to_string()),
            )
            .unwrap();

        let bp = manager.get_Plan(&id).unwrap();
        assert_eq!(bp.goal, "Test goal");
        assert!(matches!(bp.state, PlanState::Drafting));
    }

    #[test]
    fn test_approval_flow() {
        let (manager, _temp) = create_test_manager();

        let id = manager
            .create_Plan("Test".to_string(), "test".to_string(), None)
            .unwrap();

        // Submit for approval
        manager.submit_for_approval(&id).unwrap();
        let bp = manager.get_Plan(&id).unwrap();
        assert!(matches!(bp.state, PlanState::Pending { .. }));

        // Approve
        manager
            .approve_Plan(&id, "reviewer".to_string(), ApprovalRole::Maintainer)
            .unwrap();
        let bp = manager.get_Plan(&id).unwrap();
        assert!(matches!(bp.state, PlanState::Approved { .. }));
        assert!(bp.can_execute());
    }

    #[test]
    fn test_rejection_flow() {
        let (manager, _temp) = create_test_manager();

        let id = manager
            .create_Plan("Test".to_string(), "test".to_string(), None)
            .unwrap();

        manager.submit_for_approval(&id).unwrap();

        // Reject
        manager
            .reject_Plan(&id, "Not ready".to_string(), Some("reviewer".to_string()))
            .unwrap();

        let bp = manager.get_Plan(&id).unwrap();
        assert!(matches!(bp.state, PlanState::Rejected { .. }));
    }

    #[test]
    fn test_cannot_modify_approved() {
        let (manager, _temp) = create_test_manager();

        let id = manager
            .create_Plan("Test".to_string(), "test".to_string(), None)
            .unwrap();

        manager.submit_for_approval(&id).unwrap();
        manager
            .approve_Plan(&id, "reviewer".to_string(), ApprovalRole::Maintainer)
            .unwrap();

        // Try to update
        let result = manager.update_Plan(&id, |bp| {
            bp.goal = "Modified goal".to_string();
        });

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err().downcast::<ManagerError>().unwrap(),
            ManagerError::CannotModify { .. }
        ));
    }

    #[test]
    fn test_add_work_item() {
        let (manager, _temp) = create_test_manager();

        let id = manager
            .create_Plan("Test".to_string(), "test".to_string(), None)
            .unwrap();

        let work_item = WorkItem {
            name: "Task 1".to_string(),
            files_touched: vec!["file.rs".to_string()],
            diff_contract: "patch".to_string(),
            tests: vec!["test_file".to_string()],
        };

        manager.add_work_item(&id, work_item).unwrap();

        let bp = manager.get_Plan(&id).unwrap();
        assert_eq!(bp.work_items.len(), 1);
        assert_eq!(bp.work_items[0].name, "Task 1");
    }

    #[test]
    fn test_list_plans() {
        let (manager, _temp) = create_test_manager();

        let id1 = manager
            .create_Plan("Test 1".to_string(), "test-1".to_string(), None)
            .unwrap();
        let id2 = manager
            .create_Plan("Test 2".to_string(), "test-2".to_string(), None)
            .unwrap();

        let ids = manager.list_Plans().unwrap();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
    }
}
