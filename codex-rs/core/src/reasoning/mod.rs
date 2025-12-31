//! Reasoning engine for deep inference chains
//!
//! Provides Ultrathink Mode with multi-step deep reasoning chains,
//! dependency management, and result verification.

pub mod chain;
pub mod config;

pub use chain::ReasoningChain;
pub use chain::ReasoningStep;
pub use chain::ReasoningResult;
pub use config::ReasoningConfig;
