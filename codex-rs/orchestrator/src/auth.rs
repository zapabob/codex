//! Authentication and authorization for orchestrator
//!
//! Provides OAuth 2.0 token verification, API key authentication,
//! and role-based access control (RBAC).

use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::decode_header;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Authentication manager for orchestrator
#[derive(Clone)]
pub struct AuthManager {
    /// Path to codex home directory
    codex_home: PathBuf,
    /// OAuth 2.0 issuer URL for token verification
    oauth_issuer: Option<String>,
    /// OAuth 2.0 audience
    oauth_audience: Option<String>,
    /// Token cache (token -> TokenInfo)
    token_cache: Arc<RwLock<HashMap<String, TokenInfo>>>,
    /// API keys (hashed_key -> ApiKeyInfo)
    api_keys: Arc<RwLock<HashMap<String, ApiKeyInfo>>>,
    /// API keys file path
    api_keys_path: PathBuf,
}

/// Token information cached for verification
#[derive(Debug, Clone)]
struct TokenInfo {
    /// Token expiration time
    expires_at: SystemTime,
    /// Token scopes
    scopes: Vec<String>,
    /// Token subject (user ID)
    subject: String,
}

/// API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiKeyInfo {
    /// Key name/description
    name: String,
    /// Key creation time
    created_at: DateTime<Utc>,
    /// Key expiration time (None = never expires)
    expires_at: Option<DateTime<Utc>>,
    /// Allowed scopes
    scopes: Vec<String>,
    /// Rate limit (requests per second)
    rate_limit: Option<f64>,
    /// Last used time
    last_used: Option<DateTime<Utc>>,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject (user ID)
    sub: String,
    /// Issuer
    iss: String,
    /// Audience
    aud: String,
    /// Expiration time
    exp: i64,
    /// Issued at
    iat: i64,
    /// Scopes (space-separated or array)
    #[serde(default)]
    scope: String,
    /// Roles (optional)
    #[serde(default)]
    roles: Vec<String>,
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    /// Authenticated user ID
    pub user_id: String,
    /// User roles
    pub roles: Vec<String>,
    /// Token scopes
    pub scopes: Vec<String>,
}

/// Authentication error
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("API key expired")]
    ApiKeyExpired,
    #[error("Insufficient permissions: required {required:?}, got {got:?}")]
    InsufficientPermissions {
        required: Vec<String>,
        got: Vec<String>,
    },
    #[error("Token verification failed: {0}")]
    VerificationFailed(String),
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(codex_home: &Path) -> Result<Self> {
        let codex_home = codex_home.to_path_buf();
        let api_keys_path = codex_home.join("api_keys.json");

        // Load existing API keys
        let api_keys = if api_keys_path.exists() {
            let content =
                std::fs::read_to_string(&api_keys_path).context("Failed to read API keys file")?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(Self {
            codex_home,
            oauth_issuer: None,   // Can be set from config
            oauth_audience: None, // Can be set from config
            token_cache: Arc::new(RwLock::new(HashMap::new())),
            api_keys: Arc::new(RwLock::new(api_keys)),
            api_keys_path,
        })
    }

    /// Set OAuth 2.0 issuer URL
    pub fn set_oauth_issuer(&mut self, issuer: String) {
        self.oauth_issuer = Some(issuer);
    }

    /// Set OAuth 2.0 audience
    pub fn set_oauth_audience(&mut self, audience: String) {
        self.oauth_audience = Some(audience);
    }

    /// Verify OAuth 2.0 Bearer token
    pub async fn verify_oauth_token(&self, token: &str) -> Result<AuthResult, AuthError> {
        // Check cache first
        {
            let cache = self.token_cache.read().await;
            if let Some(info) = cache.get(token) {
                if info.expires_at > SystemTime::now() {
                    return Ok(AuthResult {
                        user_id: info.subject.clone(),
                        roles: vec![], // Roles should be in token claims
                        scopes: info.scopes.clone(),
                    });
                }
            }
        }

        // Decode token header to get algorithm
        let header = decode_header(token)
            .map_err(|e| AuthError::VerificationFailed(format!("Invalid token header: {e}")))?;

        // For now, we'll use a simple validation approach
        // In production, you should fetch the public key from the issuer's JWKS endpoint
        let validation = Validation::new(Algorithm::RS256);

        // Decode token (without verification for now - in production, verify with public key)
        // TODO: Implement proper JWT verification with JWKS
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(b"dummy"), &validation)
            .map_err(|e| AuthError::VerificationFailed(format!("Token decode failed: {e}")))?;

        let claims = token_data.claims;

        // Verify expiration
        let exp_timestamp = claims.exp as u64;
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if exp_timestamp < now {
            return Err(AuthError::TokenExpired);
        }

        // Verify issuer if configured
        if let Some(ref expected_issuer) = self.oauth_issuer {
            if claims.iss != *expected_issuer {
                return Err(AuthError::InvalidToken("Invalid issuer".to_string()));
            }
        }

        // Verify audience if configured
        if let Some(ref expected_audience) = self.oauth_audience {
            if claims.aud != *expected_audience {
                return Err(AuthError::InvalidToken("Invalid audience".to_string()));
            }
        }

        // Parse scopes
        let scopes: Vec<String> = claims
            .scope
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // Cache token info
        let expires_at = SystemTime::UNIX_EPOCH + Duration::from_secs(exp_timestamp);
        let token_info = TokenInfo {
            expires_at,
            scopes: scopes.clone(),
            subject: claims.sub.clone(),
        };

        {
            let mut cache = self.token_cache.write().await;
            cache.insert(token.to_string(), token_info);
        }

        Ok(AuthResult {
            user_id: claims.sub,
            roles: claims.roles,
            scopes,
        })
    }

    /// Verify API key
    pub async fn verify_api_key(&self, api_key: &str) -> Result<AuthResult, AuthError> {
        // Hash the provided API key
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hashed_key = format!("{:x}", hasher.finalize());

        // Look up API key
        let api_keys = self.api_keys.read().await;
        let key_info = api_keys.get(&hashed_key).ok_or(AuthError::InvalidApiKey)?;

        // Check expiration
        if let Some(expires_at) = key_info.expires_at {
            if expires_at < Utc::now() {
                return Err(AuthError::ApiKeyExpired);
            }
        }

        // Update last used time
        drop(api_keys);
        {
            let mut api_keys = self.api_keys.write().await;
            if let Some(key_info) = api_keys.get_mut(&hashed_key) {
                key_info.last_used = Some(Utc::now());
            }
        }

        Ok(AuthResult {
            user_id: format!("api_key:{}", hashed_key),
            roles: vec!["api_user".to_string()],
            scopes: key_info.scopes.clone(),
        })
    }

    /// Generate a new API key
    pub async fn generate_api_key(
        &self,
        name: String,
        scopes: Vec<String>,
        expires_in_days: Option<u32>,
    ) -> Result<String> {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key_bytes: [u8; 32] = rng.r#gen();
        let api_key = BASE64_STANDARD.encode(key_bytes);

        // Hash the key for storage
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hashed_key = format!("{:x}", hasher.finalize());

        // Create API key info
        let expires_at =
            expires_in_days.map(|days| Utc::now() + chrono::Duration::days(days as i64));

        let key_info = ApiKeyInfo {
            name,
            created_at: Utc::now(),
            expires_at,
            scopes,
            rate_limit: None,
            last_used: None,
        };

        // Store API key
        {
            let mut api_keys = self.api_keys.write().await;
            api_keys.insert(hashed_key, key_info);
        }

        // Save to file
        self.save_api_keys().await?;

        Ok(api_key)
    }

    /// Save API keys to file
    async fn save_api_keys(&self) -> Result<()> {
        let api_keys = self.api_keys.read().await;
        let content =
            serde_json::to_string_pretty(&*api_keys).context("Failed to serialize API keys")?;
        std::fs::write(&self.api_keys_path, content).context("Failed to write API keys file")?;
        Ok(())
    }

    /// Check if user has required scopes
    pub fn check_scopes(user_scopes: &[String], required: &[String]) -> bool {
        required.iter().all(|req| user_scopes.contains(req))
    }

    /// Check if user has required roles
    pub fn check_roles(user_roles: &[String], required: &[String]) -> bool {
        required.iter().any(|req| user_roles.contains(req))
    }

    /// Clean up expired tokens from cache
    pub async fn cleanup_token_cache(&self) {
        let now = SystemTime::now();
        let mut cache = self.token_cache.write().await;
        cache.retain(|_, info| info.expires_at > now);
    }
}

// Placeholder types for compatibility
pub struct AuthHeader;

pub struct HmacAuthenticator;

#[derive(Clone)]
pub struct CodexAuth;

impl AuthManager {
    /// Create from auth for testing (compatibility method)
    pub fn from_auth_for_testing<T>(_auth: T) -> Self {
        Self::new(Path::new("/tmp")).unwrap()
    }
}
