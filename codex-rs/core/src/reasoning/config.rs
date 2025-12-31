//! Configuration for Ultrathink Mode reasoning

use serde::Deserialize;
use serde::Serialize;

/// Configuration for reasoning chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    /// Maximum depth of the reasoning chain
    pub max_chain_depth: usize,
    /// Timeout in seconds for the entire reasoning process
    pub timeout_seconds: u64,
    /// Maximum number of reasoning steps
    pub max_steps: usize,
    /// Enable result verification
    pub enable_verification: bool,
    /// Enable counter-evidence checking
    pub enable_counter_evidence: bool,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}

/// Resource limits for reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum tokens per step
    pub max_tokens_per_step: u64,
    /// Maximum total tokens
    pub max_total_tokens: u64,
    /// Maximum concurrent reasoning steps
    pub max_concurrent_steps: usize,
}

impl Default for ReasoningConfig {
    fn default() -> Self {
        Self {
            max_chain_depth: 10,
            timeout_seconds: 300,
            max_steps: 50,
            enable_verification: true,
            enable_counter_evidence: true,
            resource_limits: ResourceLimits {
                max_tokens_per_step: 10000,
                max_total_tokens: 100000,
                max_concurrent_steps: 3,
            },
        }
    }
}

impl ReasoningConfig {
    /// Create a new reasoning config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a config with custom depth
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.max_chain_depth = depth;
        self
    }

    /// Create a config with custom timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
}
