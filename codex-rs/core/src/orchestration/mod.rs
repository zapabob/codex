//! AI orchestration and sub-agent coordination
//!
//! Provides central orchestration with conflict resolution and git worktree
//! parallel development modes.

pub mod auto_orchestrator;
pub mod collaboration_store;
pub mod conflict_resolver;
pub mod development_mode;
pub mod error_handler;
pub mod parallel_execution;
pub mod plan_orchestrator;
pub mod qc_logger;
pub mod qc_merger;
pub mod resource_manager;
pub mod task_analyzer;
pub mod worktree_manager;

pub use auto_orchestrator::AutoOrchestrator;
pub use collaboration_store::CollaborationStore;
pub use conflict_resolver::{ConflictResolver, FileEditTracker, MergeStrategy};
pub use development_mode::{DevelopmentMode, DevelopmentModeSelector, ImplementationLog};
pub use error_handler::ErrorHandler;
pub use parallel_execution::{AgentResult, AgentTask, AgentType, ParallelOrchestrator};
pub use plan_orchestrator::PlanOrchestrator;
pub use qc_logger::QcLogger;
pub use qc_merger::QcMerger;
pub use resource_manager::{ResourceCapacity, ResourceManager, SystemStats};
pub use task_analyzer::{TaskAnalysis, TaskAnalyzer};
pub use worktree_manager::{WorktreeInfo, WorktreeManager};
