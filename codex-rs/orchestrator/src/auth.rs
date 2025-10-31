//! HMAC-based authentication for orchestrator protocol.

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::path::PathBuf;

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Path to secret file
    pub secret_path: PathBuf,
    
    /// Maximum allowed clock skew in seconds
    pub max_skew_seconds: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            secret_path: PathBuf::from(".codex/secret"),
            max_skew_seconds: 300, // 5 minutes
        }
    }
}

/// Authentication manager
pub struct AuthManager {
    secret: Vec<u8>,
    config: AuthConfig,
}

impl AuthManager {
    /// Create a new auth manager, generating a secret if needed
    pub async fn new(config: AuthConfig) -> Result<Self> {
        let secret = Self::load_or_generate_secret(&config.secret_path).await?;
        Ok(Self { secret, config })
    }
    
    /// Load existing secret or generate a new one
    async fn load_or_generate_secret(path: &PathBuf) -> Result<Vec<u8>> {
        if path.exists() {
            // Load existing secret
            let encoded = tokio::fs::read_to_string(path)
                .await
                .context("Failed to read secret file")?;
            
            BASE64.decode(encoded.trim())
                .context("Failed to decode secret")
        } else {
            // Generate new secret
            use sha2::digest::generic_array::GenericArray;
            use sha2::digest::typenum::U32;
            
            let mut secret = GenericArray::<u8, U32>::default();
            // Use urandom for cryptographic randomness
            getrandom::getrandom(&mut secret)
                .map_err(|e| anyhow::anyhow!("Failed to generate random secret: {}", e))?;
            
            let encoded = BASE64.encode(&secret);
            
            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            
            // Write with restrictive permissions
            tokio::fs::write(path, encoded.as_bytes()).await?;
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let metadata = tokio::fs::metadata(path).await?;
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o600);
                tokio::fs::set_permissions(path, permissions).await?;
            }
            
            Ok(secret.to_vec())
        }
    }
    
    /// Sign a message with HMAC-SHA256
    pub fn sign(&self, message: &str, timestamp: &DateTime<Utc>) -> String {
        let payload = format!("{}{}", message, timestamp.to_rfc3339());
        let mut mac = hmac::Hmac::<Sha256>::new_from_slice(&self.secret)
            .expect("HMAC can take key of any size");
        
        use hmac::Mac;
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        BASE64.encode(result.into_bytes())
    }
    
    /// Verify a message signature
    pub fn verify(
        &self,
        message: &str,
        timestamp: &DateTime<Utc>,
        signature: &str,
    ) -> Result<()> {
        // Check timestamp is within skew window
        let now = Utc::now();
        let diff = (now - *timestamp).num_seconds().abs();
        
        if diff > self.config.max_skew_seconds {
            anyhow::bail!(
                "Timestamp skew too large: {} seconds (max {})",
                diff,
                self.config.max_skew_seconds
            );
        }
        
        // Verify signature
        let expected = self.sign(message, timestamp);
        if signature != expected {
            anyhow::bail!("Invalid signature");
        }
        
        Ok(())
    }
    
    /// Rotate the secret (for manual rotation)
    pub async fn rotate_secret(&mut self) -> Result<()> {
        // Generate new secret
        use sha2::digest::generic_array::GenericArray;
        use sha2::digest::typenum::U32;
        
        let mut new_secret = GenericArray::<u8, U32>::default();
        getrandom::getrandom(&mut new_secret)
            .map_err(|e| anyhow::anyhow!("Failed to generate random secret: {}", e))?;
        
        // Backup old secret
        let backup_path = self.config.secret_path.with_extension("bak");
        let old_encoded = BASE64.encode(&self.secret);
        tokio::fs::write(&backup_path, old_encoded.as_bytes()).await?;
        
        // Write new secret
        let new_encoded = BASE64.encode(&new_secret);
        tokio::fs::write(&self.config.secret_path, new_encoded.as_bytes()).await?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = tokio::fs::metadata(&self.config.secret_path).await?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o600);
            tokio::fs::set_permissions(&self.config.secret_path, permissions).await?;
        }
        
        // Update in-memory secret
        self.secret = new_secret.to_vec();
        
        Ok(())
    }
}

/// Request with authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedRequest {
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_secret_generation() {
        let temp_dir = TempDir::new().unwrap();
        let secret_path = temp_dir.path().join("secret");
        
        let config = AuthConfig {
            secret_path: secret_path.clone(),
            max_skew_seconds: 300,
        };
        
        let auth = AuthManager::new(config).await.unwrap();
        
        // Secret file should exist
        assert!(secret_path.exists());
        
        // Should be able to sign and verify
        let now = Utc::now();
        let message = "test message";
        let signature = auth.sign(message, &now);
        
        auth.verify(message, &now, &signature).unwrap();
    }
    
    #[tokio::test]
    async fn test_signature_verification_fails_with_wrong_signature() {
        let temp_dir = TempDir::new().unwrap();
        let config = AuthConfig {
            secret_path: temp_dir.path().join("secret"),
            max_skew_seconds: 300,
        };
        
        let auth = AuthManager::new(config).await.unwrap();
        
        let now = Utc::now();
        let message = "test message";
        let wrong_signature = "wrong signature";
        
        assert!(auth.verify(message, &now, wrong_signature).is_err());
    }
    
    #[tokio::test]
    async fn test_timestamp_skew_check() {
        let temp_dir = TempDir::new().unwrap();
        let config = AuthConfig {
            secret_path: temp_dir.path().join("secret"),
            max_skew_seconds: 10,
        };
        
        let auth = AuthManager::new(config).await.unwrap();
        
        let old_time = Utc::now() - chrono::Duration::seconds(20);
        let message = "test message";
        let signature = auth.sign(message, &old_time);
        
        assert!(auth.verify(message, &old_time, &signature).is_err());
    }
}
