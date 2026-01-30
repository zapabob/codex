// use crate::audit::AuditLogger;
use crate::server::OrchestratorServer;
use crate::server::config::OrchestratorConfig;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

impl OrchestratorServer {
    /// Validate path to prevent directory traversal attacks
    /// Ensures the path is within allowed base directories
    pub(crate) fn validate_path_against_base(
        path: &Path,
        base_dirs: &[PathBuf],
    ) -> Result<PathBuf, String> {
        // Canonicalize to resolve symlinks and normalize
        let canonical = path
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize path: {e}"))?;

        // Check if path is within any allowed base directory
        for base_dir in base_dirs {
            if let Ok(base_canonical) = base_dir.canonicalize() {
                if canonical.starts_with(&base_canonical) {
                    return Ok(canonical);
                }
            }
        }

        Err(format!(
            "Path access denied: path must be within allowed base directories. Requested: {:?}, Allowed bases: {:?}",
            canonical, base_dirs
        ))
    }

    /// Get allowed base directories for file operations
    pub(crate) fn get_allowed_base_directories(config: &OrchestratorConfig) -> Vec<PathBuf> {
        let mut allowed = vec![config.codex_dir.clone()];

        // Add current working directory if available
        if let Ok(cwd) = std::env::current_dir() {
            allowed.push(cwd);
        }

        // Add home directory
        if let Some(home) = dirs::home_dir() {
            allowed.push(home);
        }

        allowed
    }

    /// Publish event to subscribers
    pub(crate) async fn publish_event(
        topic: &str,
        _data: serde_json::Value,
        subscribers: &Arc<RwLock<HashMap<String, Vec<String>>>>,
    ) {
        let subscribers = subscribers.read().await;
        if let Some(connection_ids) = subscribers.get(topic) {
            // In a real implementation, we would send events to these connections
            // For now, we just log that an event would be sent
            tracing::debug!(
                "Publishing event to topic '{}' for {} subscribers",
                topic,
                connection_ids.len()
            );
        }
    }
}
