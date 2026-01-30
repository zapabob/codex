//! Configuration management for orchestrator
//!
//! Loads and validates security configuration from config-secure.toml

use crate::audit::AuditLoggerConfig;
use crate::rate_limit::RateLimitConfig;
use crate::replay_protection::ReplayProtectionConfig;
use crate::security_headers::SecurityHeadersConfig;
use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

/// Security configuration from config-secure.toml
#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limiting: Option<RateLimitingConfig>,
    /// Replay protection configuration
    #[serde(default)]
    pub replay_protection: Option<ReplayProtectionConfigToml>,
    /// Audit logging configuration
    #[serde(default)]
    pub audit: Option<AuditConfig>,
}

/// Rate limiting configuration (TOML)
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Maximum requests per second
    #[serde(default = "default_max_requests_per_sec")]
    pub max_requests_per_sec: f64,
    /// Burst size
    #[serde(default = "default_burst_size")]
    pub burst_size: usize,
    /// Sliding window size in seconds
    #[serde(default = "default_window_seconds")]
    pub window_seconds: u64,
}

fn default_window_seconds() -> u64 {
    1
}

fn default_true() -> bool {
    true
}

fn default_max_requests_per_sec() -> f64 {
    10.0
}

fn default_burst_size() -> usize {
    20
}

/// Replay protection configuration (TOML)
#[derive(Debug, Clone, Deserialize)]
pub struct ReplayProtectionConfigToml {
    /// Enable replay protection
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Nonce store type
    #[serde(default = "default_nonce_store")]
    pub nonce_store: String,
    /// Redis URL (if using Redis)
    pub redis_url: Option<String>,
    /// Timestamp tolerance in seconds
    #[serde(default = "default_timestamp_tolerance")]
    pub timestamp_tolerance_sec: u64,
}

fn default_nonce_store() -> String {
    "memory".to_string()
}

fn default_timestamp_tolerance() -> u64 {
    300
}

/// Audit configuration (TOML)
#[derive(Debug, Clone, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Log directory
    #[serde(default = "default_log_dir")]
    pub log_dir: String,
    /// Log format
    #[serde(default = "default_log_format")]
    pub format: String,
    /// Include MCP calls
    #[serde(default = "default_true")]
    pub include_mcp_calls: bool,
    /// Include agent messages
    #[serde(default = "default_true")]
    pub include_agent_messages: bool,
    /// Include security events
    #[serde(default = "default_true")]
    pub include_security_events: bool,
    /// Include tool arguments
    #[serde(default = "default_true")]
    pub include_tool_args: bool,
    /// Log rotation
    #[serde(default = "default_rotation")]
    pub rotation: String,
    /// Retention days
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
}

fn default_log_dir() -> String {
    "~/.codex/audit-logs".to_string()
}

fn default_log_format() -> String {
    "json".to_string()
}

fn default_rotation() -> String {
    "daily".to_string()
}

fn default_retention_days() -> u32 {
    90
}

/// Load security configuration from config-secure.toml
pub fn load_security_config(config_path: &PathBuf) -> Result<SecurityConfig> {
    let content =
        std::fs::read_to_string(config_path).context("Failed to read config-secure.toml")?;

    let config: SecurityConfig =
        toml::from_str(&content).context("Failed to parse config-secure.toml")?;

    Ok(config)
}

/// Convert TOML rate limiting config to internal config
pub fn to_rate_limit_config(toml_config: &Option<RateLimitingConfig>) -> Option<RateLimitConfig> {
    toml_config.as_ref().map(|c| RateLimitConfig {
        enabled: c.enabled,
        max_requests_per_sec: c.max_requests_per_sec,
        burst_size: c.burst_size,
        window_seconds: c.window_seconds,
    })
}

/// Convert TOML replay protection config to internal config
pub fn to_replay_protection_config(
    toml_config: &Option<ReplayProtectionConfigToml>,
) -> Option<ReplayProtectionConfig> {
    toml_config.as_ref().map(|c| ReplayProtectionConfig {
        timestamp_tolerance_sec: c.timestamp_tolerance_sec,
        nonce_store: c.nonce_store.clone(),
        redis_url: c.redis_url.clone(),
    })
}

/// Convert TOML audit config to internal config
pub fn to_audit_logger_config(toml_config: &Option<AuditConfig>) -> Option<AuditLoggerConfig> {
    toml_config.as_ref().map(|c| AuditLoggerConfig {
        enabled: c.enabled,
        log_dir: PathBuf::from(&c.log_dir),
        format: c.format.clone(),
        include_mcp_calls: c.include_mcp_calls,
        include_agent_messages: c.include_agent_messages,
        include_security_events: c.include_security_events,
        include_tool_args: c.include_tool_args,
        rotation: c.rotation.clone(),
        retention_days: c.retention_days,
    })
}
