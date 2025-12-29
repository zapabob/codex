//! Replay attack protection for orchestrator
//!
//! Prevents replay attacks by tracking nonces and timestamps.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Replay protection configuration
#[derive(Debug, Clone)]
pub struct ReplayProtectionConfig {
    /// Timestamp tolerance in seconds (requests older than this are rejected)
    pub timestamp_tolerance_sec: u64,
    /// Nonce store type ("memory" or "redis")
    pub nonce_store: String,
    /// Redis URL (if using Redis)
    pub redis_url: Option<String>,
}

impl Default for ReplayProtectionConfig {
    fn default() -> Self {
        Self {
            timestamp_tolerance_sec: 300, // 5 minutes
            nonce_store: "memory".to_string(),
            redis_url: None,
        }
    }
}

/// Nonce entry
#[derive(Debug, Clone)]
struct NonceEntry {
    /// When the nonce was first seen
    first_seen: SystemTime,
    /// When the nonce expires
    expires_at: SystemTime,
}

/// Replay protection manager
pub struct ReplayProtection {
    /// Configuration
    config: ReplayProtectionConfig,
    /// Nonce store (nonce -> entry)
    nonces: Arc<RwLock<HashSet<String>>>,
    /// Nonce entries with expiration (for cleanup)
    nonce_entries: Arc<RwLock<std::collections::HashMap<String, NonceEntry>>>,
}

impl ReplayProtection {
    /// Create a new replay protection manager
    pub fn new(config: ReplayProtectionConfig) -> Self {
        Self {
            config,
            nonces: Arc::new(RwLock::new(HashSet::new())),
            nonce_entries: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Verify a request nonce and timestamp
    pub async fn verify(
        &self,
        nonce: &str,
        timestamp: SystemTime,
    ) -> Result<(), ReplayProtectionError> {
        let now = SystemTime::now();

        // Check timestamp tolerance
        let time_diff = now
            .duration_since(timestamp)
            .map_err(|_| ReplayProtectionError::InvalidTimestamp)?;

        if time_diff.as_secs() > self.config.timestamp_tolerance_sec {
            return Err(ReplayProtectionError::TimestampTooOld);
        }

        // Check if nonce was already used
        let mut nonces = self.nonces.write().await;
        if nonces.contains(nonce) {
            return Err(ReplayProtectionError::NonceReused);
        }

        // Add nonce to store
        let expires_at = now + Duration::from_secs(self.config.timestamp_tolerance_sec);
        nonces.insert(nonce.to_string());

        // Store entry for cleanup
        let mut entries = self.nonce_entries.write().await;
        entries.insert(
            nonce.to_string(),
            NonceEntry {
                first_seen: now,
                expires_at,
            },
        );

        Ok(())
    }

    /// Clean up expired nonces
    pub async fn cleanup(&self) {
        let now = SystemTime::now();
        let mut nonces = self.nonces.write().await;
        let mut entries = self.nonce_entries.write().await;

        // Remove expired entries
        entries.retain(|nonce, entry| {
            if entry.expires_at < now {
                nonces.remove(nonce);
                false
            } else {
                true
            }
        });
    }
}

/// Replay protection error
#[derive(Debug, thiserror::Error)]
pub enum ReplayProtectionError {
    #[error("Nonce already used (replay attack detected)")]
    NonceReused,
    #[error("Timestamp is too old (outside tolerance window)")]
    TimestampTooOld,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
}
