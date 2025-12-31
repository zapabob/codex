//! Audit logging for orchestrator
//!
//! Provides structured audit logging for security events, authentication,
//! and RPC requests.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: AuditEventType,
    /// User ID (if authenticated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Method name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Event data
    pub data: serde_json::Value,
    /// IP address (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// Audit event type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    /// Authentication success
    AuthSuccess,
    /// Authentication failure
    AuthFailure,
    /// RPC request received
    RpcRequest,
    /// RPC response sent
    RpcResponse,
    /// Security event
    SecurityEvent,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// Replay attack detected
    ReplayAttack,
    /// Session started
    SessionStart,
    /// Session ended
    SessionEnd,
}

/// Audit logger configuration
#[derive(Debug, Clone)]
pub struct AuditLoggerConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Log directory
    pub log_dir: PathBuf,
    /// Log format ("json" or "text")
    pub format: String,
    /// Include MCP calls
    pub include_mcp_calls: bool,
    /// Include agent messages
    pub include_agent_messages: bool,
    /// Include security events
    pub include_security_events: bool,
    /// Include tool arguments
    pub include_tool_args: bool,
    /// Log rotation ("daily" or "none")
    pub rotation: String,
    /// Retention days
    pub retention_days: u32,
}

impl Default for AuditLoggerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_dir: PathBuf::from("~/.codex/audit-logs"),
            format: "json".to_string(),
            include_mcp_calls: true,
            include_agent_messages: true,
            include_security_events: true,
            include_tool_args: true,
            rotation: "daily".to_string(),
            retention_days: 90,
        }
    }
}

/// Audit logger
pub struct AuditLogger {
    /// Configuration
    config: AuditLoggerConfig,
    /// Current log file handle
    log_file: Arc<RwLock<Option<tokio::fs::File>>>,
    /// Current log file path
    current_log_path: Arc<RwLock<PathBuf>>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub async fn new(config: AuditLoggerConfig) -> Result<Self, AuditLoggerError> {
        use codex_core::security::secret_masking::mask_secrets;

        let log_dir = if config.log_dir.starts_with("~") {
            dirs::home_dir()
                .ok_or(AuditLoggerError::HomeDirNotFound)?
                .join(&config.log_dir.strip_prefix("~").unwrap())
        } else {
            config.log_dir.clone()
        };

        // Create log directory if it doesn't exist
        tokio::fs::create_dir_all(&log_dir)
            .await
            .map_err(|e| AuditLoggerError::IoError(e.to_string()))?;

        // Get current log file path
        let log_filename = if config.rotation == "daily" {
            format!("audit-{}.log", Utc::now().format("%Y-%m-%d"))
        } else {
            "audit.log".to_string()
        };
        let log_path = log_dir.join(log_filename);

        // Open log file
        let log_file = if log_path.exists() {
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(&log_path)
                .await
                .map_err(|e| AuditLoggerError::IoError(e.to_string()))?
        } else {
            tokio::fs::File::create(&log_path)
                .await
                .map_err(|e| AuditLoggerError::IoError(e.to_string()))?
        };

        Ok(Self {
            config,
            log_file: Arc::new(RwLock::new(Some(log_file))),
            current_log_path: Arc::new(RwLock::new(log_path)),
        })
    }

    /// Log an audit event
    pub async fn log(&self, entry: AuditLogEntry) -> Result<(), AuditLoggerError> {
        if !self.config.enabled {
            return Ok(());
        }

        // Mask secrets in data
        let mut masked_entry = entry.clone();
        let data_str = serde_json::to_string(&masked_entry.data)
            .map_err(|e| AuditLoggerError::SerializationError(e.to_string()))?;
        let masked_data_str = codex_core::security::secret_masking::mask_secrets(&data_str);
        masked_entry.data = serde_json::from_str(&masked_data_str)
            .unwrap_or_else(|_| serde_json::json!({ "error": "failed to parse masked data" }));

        // Format log entry
        let log_line = if self.config.format == "json" {
            serde_json::to_string(&masked_entry)
                .map_err(|e| AuditLoggerError::SerializationError(e.to_string()))?
        } else {
            format!(
                "[{}] {:?} - user: {:?}, method: {:?}, data: {}",
                masked_entry.timestamp,
                masked_entry.event_type,
                masked_entry.user_id,
                masked_entry.method,
                masked_data_str
            )
        };

        // Write to log file
        let mut file_guard = self.log_file.write().await;
        if let Some(ref mut file) = *file_guard {
            file.write_all(log_line.as_bytes())
                .await
                .map_err(|e| AuditLoggerError::IoError(e.to_string()))?;
            file.write_all(b"\n")
                .await
                .map_err(|e| AuditLoggerError::IoError(e.to_string()))?;
            file.flush()
                .await
                .map_err(|e| AuditLoggerError::IoError(e.to_string()))?;
        }

        Ok(())
    }

    /// Log authentication success
    pub async fn log_auth_success(
        &self,
        user_id: String,
        method: String,
        ip_address: Option<String>,
    ) -> Result<(), AuditLoggerError> {
        self.log(AuditLogEntry {
            timestamp: Utc::now(),
            event_type: AuditEventType::AuthSuccess,
            user_id: Some(user_id),
            request_id: None,
            method: Some(method),
            data: serde_json::json!({}),
            ip_address,
        })
        .await
    }

    /// Log authentication failure
    pub async fn log_auth_failure(
        &self,
        reason: String,
        method: String,
        ip_address: Option<String>,
    ) -> Result<(), AuditLoggerError> {
        self.log(AuditLogEntry {
            timestamp: Utc::now(),
            event_type: AuditEventType::AuthFailure,
            user_id: None,
            request_id: None,
            method: Some(method),
            data: serde_json::json!({ "reason": reason }),
            ip_address,
        })
        .await
    }

    /// Log RPC request
    pub async fn log_rpc_request(
        &self,
        request_id: String,
        method: String,
        user_id: Option<String>,
        params: serde_json::Value,
        ip_address: Option<String>,
    ) -> Result<(), AuditLoggerError> {
        self.log(AuditLogEntry {
            timestamp: Utc::now(),
            event_type: AuditEventType::RpcRequest,
            user_id,
            request_id: Some(request_id),
            method: Some(method),
            data: params,
            ip_address,
        })
        .await
    }

    /// Log security event
    pub async fn log_security_event(
        &self,
        event: String,
        details: serde_json::Value,
        user_id: Option<String>,
        ip_address: Option<String>,
    ) -> Result<(), AuditLoggerError> {
        self.log(AuditLogEntry {
            timestamp: Utc::now(),
            event_type: AuditEventType::SecurityEvent,
            user_id,
            request_id: None,
            method: None,
            data: serde_json::json!({
                "event": event,
                "details": details
            }),
            ip_address,
        })
        .await
    }
}

/// Audit logger error
#[derive(Debug, thiserror::Error)]
pub enum AuditLoggerError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Home directory not found")]
    HomeDirNotFound,
}
