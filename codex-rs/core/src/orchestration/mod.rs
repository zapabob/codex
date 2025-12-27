// Auto-orchestration and task analysis
pub mod auto_orchestrator;
pub mod development_mode;
pub mod task_analyzer;

// Blueprint orchestration
pub mod plan_orchestrator;

// Collaboration and conflict resolution
pub mod collaboration_store;
pub mod conflict_resolver;
pub mod qc_logger;
pub mod qc_merger;

// Error handling
pub mod error_handler;

// Parallel execution and worktree management (v1.3.0)
pub mod parallel_execution;
pub mod resource_manager;
pub mod worktree_manager;

// Re-export common types
pub use auto_orchestrator::AutoOrchestrator;
pub use auto_orchestrator::OrchestratedResult;
pub use auto_orchestrator::OrchestrationLogEntry;
pub use collaboration_store::CollaborationStore;
pub use collaboration_store::OrchestrationMetrics;
pub use conflict_resolver::ConflictResolver;
pub use conflict_resolver::MergeStrategy;
pub use error_handler::AgentError;
pub use error_handler::ErrorHandler;
pub use error_handler::ErrorResolution;
pub use error_handler::RetryPolicy;
pub use plan_orchestrator::PlanOrchestrator;
pub use resource_manager::ResourceCapacity;
pub use resource_manager::ResourceGuard;
pub use resource_manager::ResourceManager;
pub use resource_manager::SystemStats;
pub use task_analyzer::SkillTag;
pub use task_analyzer::TaskAnalysis;
pub use task_analyzer::TaskAnalyzer;
