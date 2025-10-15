//! Auto-orchestration module for ClaudeCode-style autonomous agent coordination.
//!
//! This module provides automatic task analysis and sub-agent orchestration,
//! enabling Codex to transparently delegate complex tasks to specialized agents
//! without explicit user intervention.

pub mod auto_orchestrator;
pub mod collaboration_store;
pub mod conflict_resolver;
pub mod error_handler;
pub mod task_analyzer;

pub use auto_orchestrator::AutoOrchestrator;
pub use auto_orchestrator::ExecutionPlan;
pub use auto_orchestrator::OrchestratedResult;
pub use auto_orchestrator::PlannedTask;
pub use collaboration_store::CollaborationStore;
pub use conflict_resolver::ConflictResolver;
pub use conflict_resolver::EditToken;
pub use conflict_resolver::FileEditTracker;
pub use conflict_resolver::MergeStrategy;
pub use error_handler::AgentError;
pub use error_handler::ErrorHandler;
pub use error_handler::ErrorResolution;
pub use error_handler::FallbackStrategy;
pub use error_handler::RetryPolicy;
pub use task_analyzer::TaskAnalysis;
pub use task_analyzer::TaskAnalyzer;
