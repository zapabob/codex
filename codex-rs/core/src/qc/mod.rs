//! QC Orchestrator Module
//!
//! Handles pre-merge quality checks for AI-assisted multi-worktree development

mod logger;
mod orchestrator;
mod profiles;
mod worktree;

pub use logger::QcLogger;
pub use orchestrator::QcOrchestrator;
pub use orchestrator::QcRecommendation;
pub use orchestrator::QcResult;
pub use profiles::TestProfile;
pub use profiles::TestProfileConfig;
pub use worktree::WorktreeInfo;
