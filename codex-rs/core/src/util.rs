use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use rand::Rng;
use tracing::debug;
use tracing::error;

const INITIAL_DELAY_MS: u64 = 200;
const BACKOFF_FACTOR: f64 = 2.0;

/// Emit structured feedback metadata as key/value pairs.
///
/// This logs a tracing event with `target: "feedback_tags"`. If
/// `codex_feedback::CodexFeedback::metadata_layer()` is installed, these fields are captured and
/// later attached as tags when feedback is uploaded.
///
/// Values are wrapped with [`tracing::field::DebugValue`], so the expression only needs to
/// implement [`std::fmt::Debug`].
///
/// Example:
///
/// ```rust
/// codex_core::feedback_tags!(model = "gpt-5", cached = true);
/// codex_core::feedback_tags!(provider = provider_id, request_id = request_id);
/// ```
#[macro_export]
macro_rules! feedback_tags {
    ($( $key:ident = $value:expr ),+ $(,)?) => {
        ::tracing::info!(
            target: "feedback_tags",
            $( $key = ::tracing::field::debug(&$value) ),+
        );
    };
}

pub(crate) fn backoff(attempt: u64) -> Duration {
    let exp = BACKOFF_FACTOR.powi(attempt.saturating_sub(1) as i32);
    let base = (INITIAL_DELAY_MS as f64 * exp) as u64;
    let jitter = rand::rng().random_range(0.9..1.1);
    Duration::from_millis((base as f64 * jitter) as u64)
}

pub(crate) fn error_or_panic(message: impl std::string::ToString) {
    use crate::security::mask_secrets;
    let masked_message = mask_secrets(&message.to_string());
    // Always log as error, never panic (even in debug mode)
    // This prevents unexpected crashes in development environments
    error!("{}", masked_message);
}

pub(crate) fn try_parse_error_message(text: &str) -> String {
    use crate::security::mask_secrets;
    let masked_text = mask_secrets(text);
    debug!("Parsing server error response: {}", masked_text);
    let json = serde_json::from_str::<serde_json::Value>(text).unwrap_or_default();
    if let Some(error) = json.get("error")
        && let Some(message) = error.get("message")
        && let Some(message_str) = message.as_str()
    {
        return mask_secrets(message_str);
    }
    if text.is_empty() {
        return "Unknown error".to_string();
    }
    masked_text
}

pub fn resolve_path(base: &Path, path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.clone()
    } else {
        base.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_error_message() {
        let text = r#"{
  "error": {
    "message": "Your refresh token has already been used to generate a new access token. Please try signing in again.",
    "type": "invalid_request_error",
    "param": null,
    "code": "refresh_token_reused"
  }
}"#;
        let message = try_parse_error_message(text);
        assert_eq!(
            message,
            "Your refresh token has already been used to generate a new access token. Please try signing in again."
        );
    }

    #[test]
    fn test_try_parse_error_message_no_error() {
        let text = r#"{"message": "test"}"#;
        let message = try_parse_error_message(text);
        assert_eq!(message, r#"{"message": "test"}"#);
    }

    #[test]
    fn feedback_tags_macro_compiles() {
        #[derive(Debug)]
        struct OnlyDebug;

        feedback_tags!(model = "gpt-5", cached = true, debug_only = OnlyDebug);
    }
}
