//! Secure error handling utilities
//!
//! Provides secure error message formatting that masks sensitive information
//! and prevents information leakage in production environments.

use crate::rpc::RpcError;
use codex_core::security::secret_masking::mask_secrets;

/// Production mode flag (set via environment variable or config)
static PRODUCTION_MODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Set production mode (hides detailed error messages)
pub fn set_production_mode(enabled: bool) {
    PRODUCTION_MODE.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

/// Check if in production mode
pub fn is_production_mode() -> bool {
    PRODUCTION_MODE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Create a secure RPC error that masks sensitive information
pub fn create_secure_rpc_error(
    code: i32,
    user_message: &str,
    internal_error: Option<&dyn std::error::Error>,
) -> RpcError {
    let message = if is_production_mode() {
        // In production, show generic message
        user_message.to_string()
    } else {
        // In development, include masked error details
        if let Some(err) = internal_error {
            let masked = mask_secrets(&format!("{}: {}", user_message, err));
            masked
        } else {
            user_message.to_string()
        }
    };

    RpcError {
        code,
        message,
        data: None,
    }
}

/// Create a secure error message for logging (always masks secrets)
pub fn create_secure_log_message(
    context: &str,
    error: &dyn std::error::Error,
) -> String {
    let error_str = format!("{}: {}", context, error);
    mask_secrets(&error_str)
}

/// Validate error message doesn't leak sensitive information
pub fn sanitize_error_message(msg: &str) -> String {
    mask_secrets(msg)
}

/// Common error messages (generic, no information leakage)
pub mod messages {
    pub const AUTHENTICATION_FAILED: &str = "Authentication failed";
    pub const AUTHORIZATION_FAILED: &str = "Authorization failed";
    pub const INVALID_REQUEST: &str = "Invalid request";
    pub const INTERNAL_ERROR: &str = "Internal server error";
    pub const RATE_LIMIT_EXCEEDED: &str = "Rate limit exceeded";
    pub const SESSION_EXPIRED: &str = "Session expired";
    pub const INVALID_PARAMETERS: &str = "Invalid parameters";
    pub const RESOURCE_NOT_FOUND: &str = "Resource not found";
    pub const OPERATION_FAILED: &str = "Operation failed";
}
