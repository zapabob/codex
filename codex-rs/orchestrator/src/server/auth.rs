use crate::auth::{AuthError, AuthManager};
use crate::rpc::{RpcError, RpcRequest};
use crate::server::OrchestratorServer;
use crate::transport::TransportInfo;
use std::sync::Arc;

impl OrchestratorServer {
    /// Check if a method requires authentication
    pub(crate) fn requires_auth(method: &str) -> bool {
        // Read-only methods that don't require auth
        let read_only_methods = ["status.get", "tokens.getBudget"];

        !read_only_methods.contains(&method)
    }

    /// Verify authentication for RPC request
    pub(crate) async fn verify_auth(
        auth_manager: &Arc<AuthManager>,
        request: &RpcRequest,
        _conn_info: Option<&TransportInfo>,
    ) -> Result<(), RpcError> {
        // Try OAuth token first
        if let Some(ref token) = request.auth_token {
            match auth_manager.verify_oauth_token(token).await {
                Ok(_) => return Ok(()),
                Err(AuthError::TokenExpired) => {
                    return Err(RpcError {
                        code: 401,
                        message: "Token expired".to_string(),
                        data: None,
                    });
                }
                Err(e) => {
                    // Fall through to API key check
                    tracing::debug!("OAuth token verification failed: {e}");
                }
            }
        }

        // Try API key
        if let Some(ref api_key) = request.api_key {
            match auth_manager.verify_api_key(api_key).await {
                Ok(_) => return Ok(()),
                Err(AuthError::ApiKeyExpired) => {
                    return Err(RpcError {
                        code: 401,
                        message: "API key expired".to_string(),
                        data: None,
                    });
                }
                Err(e) => {
                    // Mask secrets in error message
                    let error_msg = codex_core::security::secret_masking::mask_secrets(&format!(
                        "Authentication failed: {e}"
                    ));
                    return Err(RpcError {
                        code: 401,
                        message: if cfg!(debug_assertions) {
                            error_msg
                        } else {
                            "Authentication failed".to_string()
                        },
                        data: None,
                    });
                }
            }
        }

        // No authentication provided
        Err(RpcError {
            code: 401,
            message: "Authentication required".to_string(),
            data: Some(serde_json::json!({
                "error": "No authentication token or API key provided"
            })),
        })
    }
}
