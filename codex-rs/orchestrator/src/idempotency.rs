//! Idempotency cache for deduplicating requests.

use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Idempotency cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub response: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Idempotency cache
pub struct IdempotencyCache {
    /// In-memory cache
    cache: Arc<DashMap<String, CacheEntry>>,
    
    /// Time window for deduplication (default 10 minutes)
    window_minutes: i64,
}

impl IdempotencyCache {
    /// Create a new idempotency cache
    pub fn new(window_minutes: i64) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            window_minutes,
        }
    }
    
    /// Check if a request with this idempotency key exists
    pub fn get(&self, key: &str) -> Option<serde_json::Value> {
        if let Some(entry) = self.cache.get(key) {
            // Check if entry is still within window
            let now = Utc::now();
            let age = now - entry.created_at;
            
            if age.num_minutes() < self.window_minutes {
                return Some(entry.response.clone());
            } else {
                // Entry expired, remove it
                drop(entry);
                self.cache.remove(key);
            }
        }
        None
    }
    
    /// Store a response for an idempotency key
    pub fn put(&self, key: String, response: serde_json::Value) {
        let entry = CacheEntry {
            key: key.clone(),
            response,
            created_at: Utc::now(),
        };
        
        self.cache.insert(key, entry);
    }
    
    /// Clean up expired entries
    pub fn cleanup(&self) {
        let now = Utc::now();
        let cutoff = now - Duration::minutes(self.window_minutes);
        
        self.cache.retain(|_, entry| entry.created_at > cutoff);
    }
    
    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for IdempotencyCache {
    fn default() -> Self {
        Self::new(10) // 10 minutes default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_put_and_get() {
        let cache = IdempotencyCache::new(10);
        
        let key = "test-key";
        let response = serde_json::json!({"result": "success"});
        
        cache.put(key.to_string(), response.clone());
        
        let retrieved = cache.get(key);
        assert_eq!(retrieved, Some(response));
    }
    
    #[test]
    fn test_cache_expiration() {
        let cache = IdempotencyCache::new(0); // 0 minutes = immediate expiration
        
        let key = "test-key";
        let response = serde_json::json!({"result": "success"});
        
        cache.put(key.to_string(), response.clone());
        
        // Should not retrieve expired entry
        let retrieved = cache.get(key);
        assert_eq!(retrieved, None);
    }
    
    #[test]
    fn test_cache_cleanup() {
        let cache = IdempotencyCache::new(10);
        
        // Add some entries
        for i in 0..5 {
            cache.put(
                format!("key-{}", i),
                serde_json::json!({"id": i}),
            );
        }
        
        assert_eq!(cache.len(), 5);
        
        cache.cleanup();
        
        // All entries should still be present (not expired)
        assert_eq!(cache.len(), 5);
    }
}
