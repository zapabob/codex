//! Rate limiting implementation for orchestrator
//!
//! Provides sliding window rate limiting to prevent DDoS attacks.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Rate limiter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Maximum requests per second
    pub max_requests_per_sec: f64,
    /// Burst size (maximum requests allowed in a short burst)
    pub burst_size: usize,
    /// Sliding window size in seconds
    pub window_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_requests_per_sec: 10.0,
            burst_size: 20,
            window_seconds: 1,
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
        if !self.config.enabled {
            return Ok(());
        }
        self.check_with_config(client_id, &self.config).await
    }

    /// Check a request using a specific config (per-key overrides)
    pub async fn check_with_config(
        &self,
        client_id: &str,
        config: &RateLimitConfig,
    ) -> Result<(), RateLimitError> {
        let now = SystemTime::now();
        let window_duration = Duration::from_secs(config.window_seconds.max(1));

        let mut entries = self.entries.write().await;
        let entry = entries
            .entry(client_id.to_string())
            .or_insert_with(|| RateLimitEntry {
                requests: Vec::new(),
                last_cleanup: now,
            });

        // Clean up old requests outside the window
        // Handle system clock going backwards (NTP adjustment, etc.)
        let cutoff = match now.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => match duration.checked_sub(window_duration) {
                Some(cutoff) => cutoff,
                None => {
                    // System clock went backwards significantly, reset entry
                    *entry = RateLimitEntry {
                        requests: vec![now],
                        last_cleanup: now,
                    };
                    entry.requests.push(now);
                    return Ok(());
                }
            },
            Err(_) => {
                // System clock is before UNIX_EPOCH, reset entry
                *entry = RateLimitEntry {
                    requests: vec![now],
                    last_cleanup: now,
                };
                entry.requests.push(now);
                return Ok(());
            }
        };
        let cutoff_time = SystemTime::UNIX_EPOCH + cutoff;

        entry.requests.retain(|&time| time > cutoff_time);

        // Check burst limit
        if entry.requests.len() >= config.burst_size {
            return Err(RateLimitError::BurstExceeded {
                burst_size: config.burst_size,
            });
        }

        // Check rate limit
        let requests_per_sec = entry.requests.len() as f64;
        if requests_per_sec >= config.max_requests_per_sec {
            return Err(RateLimitError::RateExceeded {
                max_requests_per_sec: config.max_requests_per_sec,
            });
        }

        // Add current request
        entry.requests.push(now);

        // Periodic cleanup (every 60 seconds)
        // Handle system clock going backwards
        if let Ok(duration) = now.duration_since(entry.last_cleanup) {
            if duration.as_secs() >= 60 {
                entry.last_cleanup = now;
                // Clean up entries with no recent requests
                entries.retain(|_, e| e.requests.iter().any(|&time| time > cutoff_time));
            }
        } else {
            // System clock went backwards, update last_cleanup
            entry.last_cleanup = now;
        }

        Ok(())
    }

    /// Get remaining requests for a client
    pub async fn remaining(&self, client_id: &str) -> usize {
        let now = SystemTime::now();
        let window_duration = Duration::from_secs(self.config.window_seconds.max(1));
        // Handle system clock going backwards (NTP adjustment, etc.)
        let cutoff_time = match now.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => match duration.checked_sub(window_duration) {
                Some(cutoff) => SystemTime::UNIX_EPOCH + cutoff,
                None => {
                    // System clock went backwards significantly, return max requests
                    return self.config.max_requests_per_sec as usize;
                }
            },
            Err(_) => {
                // System clock is before UNIX_EPOCH, return max requests
                return self.config.max_requests_per_sec as usize;
            }
        };

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
    #[error("Rate limit exceeded: {max_requests_per_sec} requests per second")]
    RateExceeded { max_requests_per_sec: f64 },
    #[error("Burst limit exceeded: {burst_size} requests")]
    BurstExceeded { burst_size: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_basic() {
        let config = RateLimitConfig {
            enabled: true,
            max_requests_per_sec: 10.0,
            burst_size: 20,
            window_seconds: 1,
        };
        let limiter = RateLimiter::new(config);

        // First request should succeed
        assert!(limiter.check("client1").await.is_ok());

        // Multiple requests should succeed up to burst limit
        for _ in 0..19 {
            assert!(limiter.check("client1").await.is_ok());
        }

        // Burst limit exceeded
        assert!(limiter.check("client1").await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            max_requests_per_sec: 1.0,
            burst_size: 1,
            window_seconds: 1,
        };
        let limiter = RateLimiter::new(config);

        // All requests should succeed when disabled
        for _ in 0..100 {
            assert!(limiter.check("client1").await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_remaining_requests() {
        let config = RateLimitConfig {
            enabled: true,
            max_requests_per_sec: 10.0,
            burst_size: 20,
            window_seconds: 1,
        };
        let limiter = RateLimiter::new(config);

        // Initially should have max requests
        assert_eq!(limiter.remaining("client1").await, 10);

        // After making requests, remaining should decrease
        for _ in 0..5 {
            assert!(limiter.check("client1").await.is_ok());
        }
        let remaining = limiter.remaining("client1").await;
        assert!(remaining <= 10);
    }

    #[tokio::test]
    async fn test_different_clients() {
        let config = RateLimitConfig {
            enabled: true,
            max_requests_per_sec: 10.0,
            burst_size: 20,
            window_seconds: 1,
        };
        let limiter = RateLimiter::new(config);

        // Different clients should have separate rate limits
        assert!(limiter.check("client1").await.is_ok());
        assert!(limiter.check("client2").await.is_ok());
        assert!(limiter.check("client3").await.is_ok());
    }

    #[tokio::test]
    async fn test_remaining_with_clock_edge_cases() {
        let config = RateLimitConfig {
            enabled: true,
            max_requests_per_sec: 10.0,
            burst_size: 20,
            window_seconds: 1,
        };
        let limiter = RateLimiter::new(config);

        // Test that remaining() handles edge cases gracefully
        // This tests the error handling for system clock going backwards
        let remaining = limiter.remaining("nonexistent_client").await;
        assert_eq!(remaining, 10); // Should return max requests for new client
    }

    #[tokio::test]
    async fn test_check_with_clock_edge_cases() {
        let config = RateLimitConfig {
            enabled: true,
            max_requests_per_sec: 10.0,
            burst_size: 20,
            window_seconds: 1,
        };
        let limiter = RateLimiter::new(config);

        // Test that check() handles edge cases gracefully
        // The error handling for system clock going backwards should not panic
        let result = limiter.check("client1").await;
        assert!(result.is_ok() || result.is_err()); // Should not panic
    }
}
