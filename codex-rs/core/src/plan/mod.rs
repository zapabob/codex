//! plan mode implementation
//!
//! Provides read-only planning phase with approval gates, budget enforcement,
//! and multiple execution strategies (single, orchestrated, competition).
//!
//! ## Integration with Official Plan Mode
//!
//! This custom Plan module extends the official Plan Mode collaboration template
//! (`codex-rs/core/templates/collaboration_mode/plan.md`) with advanced features
//! while maintaining alignment with the official 2-phase conversational approach.
//!
//! ### Official Plan Mode Structure
//!
//! The official Plan Mode (improved in commits #9877, #9874) uses a conversational
//! 2-phase approach:
//!
//! 1. **PHASE 1 — Intent chat**: Understand what the user actually wants
//!    - Goal + success criteria, audience, in/out of scope, constraints, current state
//!    - Key preferences/tradeoffs
//!    - Bias toward questions over guessing
//!
//! 2. **PHASE 2 — Implementation chat**: Technical spec and implementation plan
//!    - Approach, interfaces (APIs/schemas/I/O), data flow
//!    - Edge cases/failure modes, testing + acceptance criteria
//!    - Rollout/monitoring, migrations/compat constraints
//!
//! ### Key Principles from Official Improvements
//!
//! - **Ask a lot, but never ask trivia**: Questions must materially change the spec/plan,
//!   confirm/lock assumptions, or choose between meaningful tradeoffs
//! - **Batch questions**: 4-10 questions per `request_user_input` call to keep momentum
//! - **Two kinds of unknowns**:
//!   1. Discoverable facts: Explore first (≥2 targeted searches) before asking
//!   2. Preferences/tradeoffs: Ask early with 2-4 options + recommended default
//! - **Finalization rule**: Only output final plan when remaining unknowns are low-impact
//!   and explicitly listed as assumptions
//!
//! ### Custom Plan Features
//!
//! This module adds advanced capabilities beyond the official template:
//!
//! - **Budget management**: Token and time budgets with per-step and session caps
//! - **Execution logging**: Detailed event-based logs of plan execution
//! - **Orchestration**: Support for single-agent, orchestrated, and competition modes
//! - **Research integration**: Deep research capabilities for informed planning
//! - **Quality assurance**: QC analysis with quality gates and compliance tracking
//! - **State management**: Full lifecycle from Drafting → Pending → Approved → Executing → Completed
//! - **Persistence**: JSON and Markdown export with versioning support

pub mod budget;
pub mod execution_log;
pub mod executor;
pub mod manager;
pub mod persist;
pub mod policy;
pub mod research_integration;
pub mod schema;
pub mod state;

pub use budget::BudgetError;
pub use budget::BudgetTracker;
pub use budget::BudgetUsage;
pub use budget::format_usage;
pub use executor::ExecutionEvent;
pub use executor::ExecutionResult;
pub use executor::PlanExecutor;
pub use executor::TestResult;
pub use manager::ManagerError;
pub use manager::PlanManager;
pub use persist::PlanPersister;
pub use policy::ApprovalRole;
pub use policy::PermissionTier;
pub use policy::PlanPolicy;
pub use policy::PolicyEnforcer;
pub use policy::PolicyError;
pub use policy::PrivilegedOperation;
pub use research_integration::ResearchApprovalDialog;
pub use research_integration::ResearchIntegration;
pub use research_integration::ResearchRequest;
pub use schema::Budget;
pub use schema::EvalCriteria;
pub use schema::ExecutionMode;
pub use schema::PlanBlock;
pub use schema::QcAnalysisResults;
pub use schema::QcComplianceStatus;
pub use schema::QcQualityBlock;
pub use schema::QcQualityRequirements;
pub use schema::QcQualityScores;
pub use schema::ResearchBlock;
pub use schema::ResearchSource;
pub use schema::Risk;
pub use schema::WorkItem;
pub use state::PlanState;
pub use state::StateTransitionError;
