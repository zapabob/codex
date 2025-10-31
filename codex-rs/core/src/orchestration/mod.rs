//! Auto-orchestration module for ClaudeCode-style autonomous agent coordination.
//!
//! This module provides automatic task analysis and sub-agent orchestration,
//! enabling Codex to transparently delegate complex tasks to specialized agents
//! without explicit user intervention.

pub mod auto_orchestrator;
pub mod collaboration_store;
pub mod conflict_resolver;
pub mod error_handler;
pub mod status_provider;
pub mod task_analyzer;
pub mod token_tracker;

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
pub use status_provider::AgentStatus;
pub use status_provider::LockStatus;
pub use status_provider::OrchestratorStatus;
pub use status_provider::StatusProvider;
pub use status_provider::TokenStatus;
pub use task_analyzer::TaskAnalysis;
pub use task_analyzer::TaskAnalyzer;
pub use token_tracker::PairSession;
pub use token_tracker::TokenBudget;
pub use token_tracker::TokenTracker;
pub use token_tracker::TokenUsage;
