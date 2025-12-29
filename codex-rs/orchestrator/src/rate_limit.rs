//! Rate limiting implementation for orchestrator
//!
//! Provides sliding window rate limiting to prevent DDoS attacks.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per second
    pub max_requests_per_sec: f64,
    /// Burst size (maximum requests allowed in a short burst)
    pub burst_size: usize,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_sec: 10.0,
            burst_size: 20,
        }
    }
}

/// Rate limiter entry for a client
#[derive(Debug, Clone)]
struct RateLimitEntry {
    /// Timestamps of recent requests (sliding window)
    requests: Vec<SystemTime>,
    /// Last cleanup time
    last_cleanup: SystemTime,
}

/// Rate limiter
pub struct RateLimiter {
    /// Rate limit configuration
    config: RateLimitConfig,
    /// Client entries (IP or token -> entry)
    entries: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if a request should be allowed
    pub async fn check(&self, client_id: &str) -> Result<(), RateLimitError> {
        let now = SystemTime::now();
        let window_duration = Duration::from_secs(1);

        let mut entries = self.entries.write().await;
        let entry = entries.entry(client_id.to_string()).or_insert_with(|| {
            RateLimitEntry {
                requests: Vec::new(),
                last_cleanup: now,
            }
        });

        // Clean up old requests outside the window
        let cutoff = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .checked_sub(window_duration)
            .unwrap();
        let cutoff_time = SystemTime::UNIX_EPOCH + cutoff;

        entry.requests.retain(|&time| time > cutoff_time);

        // Check burst limit
        if entry.requests.len() >= self.config.burst_size {
            return Err(RateLimitError::BurstExceeded);
        }

        // Check rate limit
        let requests_per_sec = entry.requests.len() as f64;
        if requests_per_sec >= self.config.max_requests_per_sec {
            return Err(RateLimitError::RateExceeded);
        }

        // Add current request
        entry.requests.push(now);

        // Periodic cleanup (every 60 seconds)
        if now
            .duration_since(entry.last_cleanup)
            .unwrap()
            .as_secs()
            >= 60
        {
            entry.last_cleanup = now;
            // Clean up entries with no recent requests
            entries.retain(|_, e| {
                e.requests
                    .iter()
                    .any(|&time| time > cutoff_time)
            });
        }

        Ok(())
    }

    /// Get remaining requests for a client
    pub async fn remaining(&self, client_id: &str) -> usize {
        let now = SystemTime::now();
        let window_duration = Duration::from_secs(1);
        let cutoff = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .checked_sub(window_duration)
            .unwrap();
        let cutoff_time = SystemTime::UNIX_EPOCH + cutoff;

        let entries = self.entries.read().await;
        if let Some(entry) = entries.get(client_id) {
            let recent_requests = entry
                .requests
                .iter()
                .filter(|&&time| time > cutoff_time)
                .count();
            let max_requests = self.config.max_requests_per_sec as usize;
            max_requests.saturating_sub(recent_requests)
        } else {
            self.config.max_requests_per_sec as usize
        }
    }
}

/// Rate limit error
#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded: {0} requests per second")]
    RateExceeded,
    #[error("Burst limit exceeded: {0} requests")]
    BurstExceeded,
}
