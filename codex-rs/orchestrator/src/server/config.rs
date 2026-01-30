use crate::audit::AuditLoggerConfig;
use crate::rate_limit::RateLimitConfig;
use crate::replay_protection::ReplayProtectionConfig;
use crate::transport::TransportConfig;
use std::path::PathBuf;

/// Orchestrator configuration
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    pub queue_capacity: usize,
    pub transport_config: TransportConfig,
    pub codex_dir: PathBuf,
    pub total_token_budget: u64,
    pub warning_threshold: u64,
    pub per_agent_limit: u64,
    pub rate_limit_config: Option<RateLimitConfig>,
    pub replay_protection_config: Option<ReplayProtectionConfig>,
    pub audit_logger_config: Option<AuditLoggerConfig>,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            queue_capacity: 1024,
            transport_config: TransportConfig::default(),
            codex_dir: dirs::home_dir().unwrap_or_default().join(".codex"),
            total_token_budget: 100_000,
            warning_threshold: 80_000,
            per_agent_limit: 20_000,
            rate_limit_config: Some(RateLimitConfig::default()),
            replay_protection_config: Some(ReplayProtectionConfig::default()),
            audit_logger_config: Some(AuditLoggerConfig::default()),
        }
    }
}
