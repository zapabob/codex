// Gemini authentication module
// Supports both API key and OAuth 2.0 with PKCE

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;

use crate::default_client::{CodexHttpClient, CodexRequestBuilder};

/// Gemini API key environment variable
pub const GEMINI_API_KEY_ENV_VAR: &str = "GEMINI_API_KEY";

/// Credential source for Gemini authentication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CredentialSource {
    /// API key authentication (Google AI Studio)
    ApiKey { key: String },
    /// OAuth 2.0 authentication with PKCE
    OAuth {
        access_token: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        refresh_token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        expiry: Option<DateTime<Utc>>,
    },
}

/// Gemini credentials container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeminiCredentials {
    pub source: CredentialSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh: Option<DateTime<Utc>>,
}

/// Gemini authentication provider
pub struct GeminiAuthProvider {
    codex_home: std::path::PathBuf,
    client: CodexHttpClient,
}

impl GeminiAuthProvider {
    /// Create a new Gemini auth provider
    pub fn new(codex_home: std::path::PathBuf, client: CodexHttpClient) -> Self {
        Self {
            codex_home,
            client,
        }
    }

    /// Resolve credentials from environment, config, or secure storage
    /// Priority: env > .codex/config.yaml > secure storage
    pub fn resolve_credentials(&self) -> std::io::Result<Option<GeminiCredentials>> {
        // 1. Check environment variable
        if let Some(api_key) = read_gemini_api_key_from_env() {
            return Ok(Some(GeminiCredentials {
                source: CredentialSource::ApiKey { key: api_key },
                last_refresh: None,
            }));
        }

        // 2. Check config file
        if let Some(creds) = self.load_from_config()? {
            return Ok(Some(creds));
        }

        // 3. Check secure storage (auth.json)
        if let Some(creds) = self.load_from_storage()? {
            return Ok(Some(creds));
        }

        Ok(None)
    }

    /// Attach authentication to an HTTP request
    pub fn attach_auth(
        &self,
        request: CodexRequestBuilder,
        credentials: &GeminiCredentials,
    ) -> CodexRequestBuilder {
        match &credentials.source {
            CredentialSource::ApiKey { key } => {
                // Gemini API key is passed as x-goog-api-key header
                request.header("x-goog-api-key", key)
            }
            CredentialSource::OAuth { access_token, .. } => {
                // OAuth uses Bearer token
                request.header("Authorization", format!("Bearer {}", access_token))
            }
        }
    }

    /// Load credentials from config file
    fn load_from_config(&self) -> std::io::Result<Option<GeminiCredentials>> {
        let config_path = self.codex_home.join("config.toml");
        if !config_path.exists() {
            return Ok(None);
        }

        let config_content = std::fs::read_to_string(&config_path)?;
        let config: toml::Value = toml::from_str(&config_content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Look for gemini.api_key in config
        if let Some(gemini_section) = config.get("gemini") {
            if let Some(api_key) = gemini_section.get("api_key").and_then(|v| v.as_str()) {
                return Ok(Some(GeminiCredentials {
                    source: CredentialSource::ApiKey {
                        key: api_key.to_string(),
                    },
                    last_refresh: None,
                }));
            }
        }

        Ok(None)
    }

    /// Load credentials from secure storage (auth.json)
    fn load_from_storage(&self) -> std::io::Result<Option<GeminiCredentials>> {
        use super::storage::{create_auth_storage, AuthCredentialsStoreMode};

        let storage = create_auth_storage(self.codex_home.clone(), AuthCredentialsStoreMode::Auto);
        let auth_data = storage.load()?;

        Ok(auth_data.and_then(|data| data.gemini_credentials))
    }

    /// Save credentials to secure storage
    pub fn save_credentials(&self, credentials: &GeminiCredentials) -> std::io::Result<()> {
        use super::storage::{create_auth_storage, AuthCredentialsStoreMode, AuthDotJson};

        let storage = create_auth_storage(self.codex_home.clone(), AuthCredentialsStoreMode::Auto);
        
        // Load existing auth data or create new
        let mut auth_data = storage.load()?.unwrap_or_else(|| AuthDotJson {
            openai_api_key: None,
            gemini_credentials: None,
            tokens: None,
            last_refresh: None,
        });

        auth_data.gemini_credentials = Some(credentials.clone());
        storage.save(&auth_data)
    }

    /// Refresh OAuth token if needed
    pub async fn refresh_token_if_needed(
        &self,
        credentials: &mut GeminiCredentials,
    ) -> std::io::Result<bool> {
        let should_refresh = match &credentials.source {
            CredentialSource::OAuth {
                expiry,
                refresh_token,
                ..
            } => {
                // Check if token is expired or will expire soon (within 5 minutes)
                if let Some(expiry_time) = expiry {
                    let now = Utc::now();
                    let buffer = chrono::Duration::minutes(5);
                    
                    if *expiry_time > now + buffer {
                        return Ok(false); // Token still valid
                    }
                }

                refresh_token.clone()
            }
            CredentialSource::ApiKey { .. } => return Ok(false), // API keys don't expire
        };

        // Refresh token
        if let Some(refresh_token) = should_refresh {
            self.refresh_oauth_token(credentials, &refresh_token).await?;
            Ok(true)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No refresh token available",
            ))
        }
    }

    /// Perform OAuth token refresh
    async fn refresh_oauth_token(
        &self,
        credentials: &mut GeminiCredentials,
        refresh_token: &str,
    ) -> std::io::Result<()> {
        // Google OAuth 2.0 token endpoint
        let token_url = "https://oauth2.googleapis.com/token";

        let client_id = env::var("GEMINI_OAUTH_CLIENT_ID").map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "GEMINI_OAUTH_CLIENT_ID environment variable not set",
            )
        })?;
        
        let body = serde_json::json!({
            "client_id": client_id,
            "refresh_token": refresh_token,
            "grant_type": "refresh_token",
        });

        let response = self
            .client
            .post(token_url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        if !response.status().is_success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Token refresh failed: {}", response.status()),
            ));
        }

        let token_response: OAuthTokenResponse = response
            .json()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        // Update credentials
        let expiry = Utc::now() + chrono::Duration::seconds(token_response.expires_in as i64);
        credentials.source = CredentialSource::OAuth {
            access_token: token_response.access_token,
            refresh_token: Some(refresh_token.to_string()),
            expiry: Some(expiry),
        };
        credentials.last_refresh = Some(Utc::now());

        // Save updated credentials
        self.save_credentials(credentials)?;

        Ok(())
    }
}

/// Read Gemini API key from environment
pub fn read_gemini_api_key_from_env() -> Option<String> {
    env::var(GEMINI_API_KEY_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_credential_source_serialization() {
        let api_key = CredentialSource::ApiKey {
            key: "test-key".to_string(),
        };
        let json = serde_json::to_string(&api_key).unwrap();
        assert!(json.contains("api_key"));
        assert!(json.contains("test-key"));

        let oauth = CredentialSource::OAuth {
            access_token: "access".to_string(),
            refresh_token: Some("refresh".to_string()),
            expiry: None,
        };
        let json = serde_json::to_string(&oauth).unwrap();
        assert!(json.contains("oauth"));
        assert!(json.contains("access"));
    }

    #[test]
    fn test_gemini_credentials() {
        let creds = GeminiCredentials {
            source: CredentialSource::ApiKey {
                key: "test".to_string(),
            },
            last_refresh: None,
        };

        let json = serde_json::to_string(&creds).unwrap();
        let deserialized: GeminiCredentials = serde_json::from_str(&json).unwrap();
        assert_eq!(creds, deserialized);
    }

    #[test]
    fn test_attach_auth_api_key() {
        let dir = tempdir().unwrap();
        let client = crate::default_client::create_client();
        let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client.clone());

        let creds = GeminiCredentials {
            source: CredentialSource::ApiKey {
                key: "test-api-key".to_string(),
            },
            last_refresh: None,
        };

        let request = client.get("https://example.com");
        let request = provider.attach_auth(request, &creds);
        
        // We can't easily inspect the headers, but we can verify it doesn't panic
        drop(request);
    }

    #[test]
    fn test_attach_auth_oauth() {
        let dir = tempdir().unwrap();
        let client = crate::default_client::create_client();
        let provider = GeminiAuthProvider::new(dir.path().to_path_buf(), client.clone());

        let creds = GeminiCredentials {
            source: CredentialSource::OAuth {
                access_token: "test-access-token".to_string(),
                refresh_token: None,
                expiry: None,
            },
            last_refresh: None,
        };

        let request = client.get("https://example.com");
        let request = provider.attach_auth(request, &creds);
        
        // We can't easily inspect the headers, but we can verify it doesn't panic
        drop(request);
    }
}
