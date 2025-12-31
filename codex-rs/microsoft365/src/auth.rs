//! OAuth 2.0 authentication management for Microsoft 365

use anyhow::{Context, Result};
use codex_keyring_store::DefaultKeyringStore;
use codex_keyring_store::KeyringStore;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};

/// OAuth 2.0 token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Access token
    pub access_token: String,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
    /// Token expiration timestamp
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Scopes granted
    pub scopes: Vec<String>,
}

/// Microsoft 365 authentication manager
pub struct AuthManager {
    /// OAuth client
    client: BasicClient,
    /// Keyring store for secure token storage
    keyring: DefaultKeyringStore,
    /// Client ID
    client_id: String,
    /// Tenant ID
    tenant_id: String,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(
        client_id: String,
        tenant_id: String,
        redirect_url: String,
        _codex_home: PathBuf,
    ) -> Result<Self> {
        let auth_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
            tenant_id
        );
        let token_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            tenant_id
        );

        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new("".to_string())), // Public client
            AuthUrl::new(auth_url).context("Invalid auth URL")?,
            Some(TokenUrl::new(token_url).context("Invalid token URL")?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).context("Invalid redirect URL")?);

        let keyring = DefaultKeyringStore;

        Ok(Self {
            client,
            keyring,
            client_id,
            tenant_id,
        })
    }

    /// Get authorization URL for OAuth flow
    pub fn get_authorization_url(&self, scopes: Vec<String>) -> Result<(String, CsrfToken)> {
        let scopes: Vec<Scope> = scopes
            .iter()
            .map(|s| Scope::new(s.clone()))
            .collect();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter())
            .url();

        Ok((auth_url.to_string(), csrf_token))
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        scopes: Vec<String>,
    ) -> Result<TokenInfo> {
        let scopes: Vec<Scope> = scopes
            .iter()
            .map(|s| Scope::new(s.clone()))
            .collect();

        let token_result = self
            .client
            .exchange_code(code)
            .add_extra_param("client_id", &self.client_id)
            .request_async(async_http_client)
            .await
            .context("Failed to exchange authorization code")?;

        let expires_at = token_result
            .expires_in()
            .map(|duration| chrono::Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64));

        let token_info = TokenInfo {
            access_token: token_result.access_token().secret().clone(),
            refresh_token: token_result.refresh_token().map(|t| t.secret().clone()),
            expires_at,
            scopes: scopes.iter().map(|s| s.to_string()).collect(),
        };

        // Store token securely
        self.save_token(&token_info).await?;

        info!("Successfully authenticated with Microsoft 365");
        Ok(token_info)
    }

    /// Refresh access token
    pub async fn refresh_token(&self) -> Result<TokenInfo> {
        let token_info = self.load_token().await?;

        let _refresh_token = token_info
            .refresh_token
            .as_ref()
            .context("No refresh token available")?;

        // In a real implementation, this would use the refresh token to get a new access token
        // For now, we'll return the existing token
        warn!("Token refresh not fully implemented, returning existing token");
        Ok(token_info)
    }

    /// Save token to keyring
    async fn save_token(&self, token: &TokenInfo) -> Result<()> {
        let service = "microsoft365";
        let account = format!("{}:token", self.tenant_id);
        let value = serde_json::to_string(token)?;
        self.keyring.save(service, &account, &value)?;
        Ok(())
    }

    /// Load token from keyring
    async fn load_token(&self) -> Result<TokenInfo> {
        let service = "microsoft365";
        let account = format!("{}:token", self.tenant_id);
        let value = self.keyring.load(service, &account)?
            .ok_or_else(|| anyhow::anyhow!("Token not found in keyring"))?;
        let token: TokenInfo = serde_json::from_str(&value)?;
        Ok(token)
    }

    /// Check if token is valid (not expired)
    pub async fn is_token_valid(&self) -> Result<bool> {
        let token = self.load_token().await?;
        if let Some(expires_at) = token.expires_at {
            Ok(chrono::Utc::now() < expires_at)
        } else {
            Ok(true) // No expiration, assume valid
        }
    }

    /// Get access token (refresh if needed)
    pub async fn get_access_token(&self) -> Result<String> {
        if !self.is_token_valid().await? {
            self.refresh_token().await?;
        }
        let token = self.load_token().await?;
        Ok(token.access_token)
    }
}
