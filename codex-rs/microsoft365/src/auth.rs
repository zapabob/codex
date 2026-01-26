//! OAuth 2.0 authentication management for Microsoft 365

use anyhow::Context;
use anyhow::Result;
use codex_keyring_store::DefaultKeyringStore;
use codex_keyring_store::KeyringStore;
use oauth2::AuthUrl;
use oauth2::AuthorizationCode;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::CsrfToken;
use oauth2::RedirectUrl;
use oauth2::Scope;
use oauth2::TokenResponse;
use oauth2::TokenUrl;
use oauth2::basic::BasicClient;
use oauth2::EndpointSet;
use oauth2::EndpointNotSet;
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use tracing::info;
use tracing::warn;

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
    /// OAuth client (fully configured with auth_uri and token_uri endpoints)
    /// Type state pattern: EndpointSet for auth_uri and token_uri, EndpointNotSet for others
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    /// HTTP client for async requests
    http_client: ReqwestClient,
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
        let auth_url =
            format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/authorize");
        let token_url = format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token");

        let auth_url_parsed = AuthUrl::new(auth_url).context("Invalid auth URL")?;
        let token_url_parsed = TokenUrl::new(token_url).context("Invalid token URL")?;
        let redirect_url_parsed = RedirectUrl::new(redirect_url).context("Invalid redirect URL")?;

        // Build client with all required endpoints set
        // The type state pattern ensures authorize_url and exchange_code are only available
        // when auth_uri and token_uri endpoints are configured
        // After setting endpoints, the type becomes BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>
        let client = BasicClient::new(ClientId::new(client_id.clone()))
            .set_client_secret(ClientSecret::new("".to_string())) // Public client
            .set_auth_uri(auth_url_parsed)
            .set_token_uri(token_url_parsed)
            .set_redirect_uri(redirect_url_parsed);

        // Create HTTP client with redirect policy to prevent SSRF
        let http_client = ReqwestClient::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .context("Failed to create HTTP client")?;

        let keyring = DefaultKeyringStore;

        Ok(Self {
            client,
            http_client,
            keyring,
            client_id,
            tenant_id,
        })
    }

    /// Get authorization URL for OAuth flow
    pub fn get_authorization_url(&self, scopes: Vec<String>) -> Result<(String, CsrfToken)> {
        // In oauth2 v5.0.0, authorize_url returns a builder that must be configured
        let mut auth_request = self.client.authorize_url(CsrfToken::new_random);
        
        for scope in scopes {
            auth_request = auth_request.add_scope(Scope::new(scope));
        }

        let (auth_url, csrf_token) = auth_request.url();

        Ok((auth_url.to_string(), csrf_token))
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        scopes: Vec<String>,
    ) -> Result<TokenInfo> {
        // In oauth2 v5.0.0, exchange_code returns a builder that must be configured
        // The scopes parameter is not used in the exchange, but we keep it for API compatibility
        let _scopes: Vec<Scope> = scopes.into_iter().map(Scope::new).collect();

        let token_result = self
            .client
            .exchange_code(code)
            .add_extra_param("client_id", &self.client_id)
            .request_async(&self.http_client)
            .await
            .context("Failed to exchange authorization code")?;

        let expires_at = token_result.expires_in().map(|duration: std::time::Duration| {
            chrono::Utc::now() + chrono::Duration::seconds(duration.as_secs() as i64)
        });

        let token_info = TokenInfo {
            access_token: token_result.access_token().secret().clone(),
            refresh_token: token_result.refresh_token().map(|t: &oauth2::RefreshToken| t.secret().clone()),
            expires_at,
            scopes: _scopes.iter().map(|s| s.to_string()).collect(),
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
        let tenant_id = &self.tenant_id;
        let account = format!("{tenant_id}:token");
        let value = serde_json::to_string(token)?;
        self.keyring.save(service, &account, &value)?;
        Ok(())
    }

    /// Load token from keyring
    async fn load_token(&self) -> Result<TokenInfo> {
        let service = "microsoft365";
        let tenant_id = &self.tenant_id;
        let account = format!("{tenant_id}:token");
        let value = self
            .keyring
            .load(service, &account)?
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
