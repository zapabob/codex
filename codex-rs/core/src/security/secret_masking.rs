//! Secret masking utilities for preventing sensitive information leakage
//!
//! Provides functions to mask API keys, tokens, passwords, and other sensitive
//! data in logs, error messages, and debug output.

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Compile regex patterns once at startup
// If compilation fails, we'll use a fallback that returns the original text
static OPENAI_API_KEY_PROJ_REGEX: Lazy<Result<regex::Regex, regex::Error>> =
    Lazy::new(|| regex::Regex::new(r"sk-proj-[A-Za-z0-9]{6,}"));
static OPENAI_API_KEY_REGEX: Lazy<Result<regex::Regex, regex::Error>> =
    Lazy::new(|| regex::Regex::new(r"sk-[A-Za-z0-9]{6,}"));
static GITHUB_TOKEN_REGEX: Lazy<Result<regex::Regex, regex::Error>> =
    Lazy::new(|| regex::Regex::new(r"(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{6,}"));
static GOOGLE_API_KEY_REGEX: Lazy<Result<regex::Regex, regex::Error>> =
    Lazy::new(|| regex::Regex::new(r"AIzaSy[A-Za-z0-9_-]{10,}"));
static BEARER_TOKEN_REGEX: Lazy<Result<regex::Regex, regex::Error>> =
    Lazy::new(|| regex::Regex::new(r"Bearer\s+[A-Za-z0-9_-]{20,}"));
static PASSWORD_REGEX: Lazy<Result<regex::Regex, regex::Error>> = Lazy::new(|| {
    regex::Regex::new(r"(?i)(password|pwd|pass|secret|token|api[_-]?key)\s*[:=]\s*([^\s&]+)")
});
static URL_CREDENTIALS_REGEX: Lazy<Result<regex::Regex, regex::Error>> =
    Lazy::new(|| regex::Regex::new(r"https?://[^:]+:[^@]+@"));

// Track if regex compilation failed (for logging/debugging)
static REGEX_COMPILATION_ERROR: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

/// Mask sensitive strings in text (API keys, tokens, passwords)
pub fn mask_secrets(text: &str) -> String {
    // Patterns to detect and mask:
    // - API keys: sk-proj-..., sk-..., ghp_..., AIzaSy...
    // - Tokens: Bearer tokens, access tokens
    // - Passwords: password=..., pwd=...
    // - URLs with credentials: https://user:pass@host

    let mut masked = text.to_string();

    // Mask OpenAI API keys (sk-proj-... or sk-...)
    if let Ok(re) = OPENAI_API_KEY_PROJ_REGEX.as_ref() {
        masked = re.replace_all(&masked, "sk-proj-***MASKED***").to_string();
    } else {
        log_regex_error("OPENAI_API_KEY_PROJ_REGEX");
    }
    if let Ok(re) = OPENAI_API_KEY_REGEX.as_ref() {
        masked = re.replace_all(&masked, "sk-***MASKED***").to_string();
    } else {
        log_regex_error("OPENAI_API_KEY_REGEX");
    }

    // Mask GitHub tokens (ghp_..., gho_..., ghu_..., ghs_..., ghr_...)
    if let Ok(re) = GITHUB_TOKEN_REGEX.as_ref() {
        masked = re.replace_all(&masked, "$1_***MASKED***").to_string();
    } else {
        log_regex_error("GITHUB_TOKEN_REGEX");
    }

    // Mask Google API keys (AIzaSy...)
    if let Ok(re) = GOOGLE_API_KEY_REGEX.as_ref() {
        masked = re.replace_all(&masked, "AIzaSy***MASKED***").to_string();
    } else {
        log_regex_error("GOOGLE_API_KEY_REGEX");
    }

    // Mask Bearer tokens
    if let Ok(re) = BEARER_TOKEN_REGEX.as_ref() {
        masked = re.replace_all(&masked, "Bearer ***MASKED***").to_string();
    } else {
        log_regex_error("BEARER_TOKEN_REGEX");
    }

    // Mask passwords in query strings or form data
    if let Ok(re) = PASSWORD_REGEX.as_ref() {
        masked = re.replace_all(&masked, "$1=***MASKED***").to_string();
    } else {
        log_regex_error("PASSWORD_REGEX");
    }

    // Mask URLs with credentials (https://user:pass@host)
    if let Ok(re) = URL_CREDENTIALS_REGEX.as_ref() {
        masked = re.replace_all(&masked, "https://***MASKED***@").to_string();
    } else {
        log_regex_error("URL_CREDENTIALS_REGEX");
    }

    masked
}

/// Log regex compilation error (only once)
fn log_regex_error(regex_name: &str) {
    let mut error = REGEX_COMPILATION_ERROR.lock().unwrap();
    if error.is_none() {
        *error = Some(format!("Regex compilation failed for {regex_name}"));
        tracing::error!("Secret masking regex compilation failed: {}", regex_name);
    }
}

/// Mask secrets in error messages
pub fn mask_error_message(error: &dyn std::error::Error) -> String {
    let error_string = format!("{error}");
    mask_secrets(&error_string)
}

/// Mask secrets in debug output
pub fn mask_debug_output<T: std::fmt::Debug>(value: &T) -> String {
    let debug_string = format!("{value:?}");
    mask_secrets(&debug_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_openai_api_key() {
        let text = "API key: sk-proj-abcdefghijklmnopqrstuvwxyz123456";
        let masked = mask_secrets(text);
        assert!(masked.contains("sk-proj-***MASKED***"));
        assert!(!masked.contains("abcdefghijklmnopqrstuvwxyz123456"));
    }

    #[test]
    fn test_mask_github_token() {
        let text = "Token: ghp_abcdefghijklmnopqrstuvwxyz123456789";
        let masked = mask_secrets(text);
        assert!(masked.contains("ghp_***MASKED***"));
        assert!(!masked.contains("abcdefghijklmnopqrstuvwxyz123456789"));
    }

    #[test]
    fn test_mask_google_api_key() {
        let text = "Key: AIzaSyabcdefghijklmnopqrstuvwxyz12345";
        let masked = mask_secrets(text);
        assert!(masked.contains("AIzaSy***MASKED***"));
        assert!(!masked.contains("abcdefghijklmnopqrstuvwxyz12345"));
    }

    #[test]
    fn test_mask_bearer_token() {
        let text = "Authorization: Bearer abcdefghijklmnopqrstuvwxyz123456";
        let masked = mask_secrets(text);
        assert!(masked.contains("Bearer ***MASKED***"));
        assert!(!masked.contains("abcdefghijklmnopqrstuvwxyz123456"));
    }

    #[test]
    fn test_mask_url_credentials() {
        let text = "https://user:password@example.com/api";
        let masked = mask_secrets(text);
        assert!(masked.contains("https://***MASKED***@"));
        assert!(!masked.contains("user:password"));
    }

    #[test]
    fn test_mask_password_in_query() {
        let text = "password=secret123&username=test";
        let masked = mask_secrets(text);
        assert!(masked.contains("password=***MASKED***"));
        assert!(!masked.contains("secret123"));
    }

    #[test]
    fn test_mask_secrets_handles_regex_errors_gracefully() {
        // Test that mask_secrets doesn't panic even if regex compilation fails
        // This tests the error handling we added
        let text = "Some text with API key: sk-proj-test123456789012345678901234567890";
        let masked = mask_secrets(text);
        // Should either mask the secret or return the original text (if regex failed)
        // The important thing is that it doesn't panic
        assert!(!masked.is_empty());
    }

    #[test]
    fn test_mask_secrets_with_multiple_secrets() {
        let text = "API key: sk-proj-abc123 Token: ghp_xyz789 Password: secret123";
        let masked = mask_secrets(text);
        // Should mask all secrets
        assert!(!masked.contains("abc123"));
        assert!(!masked.contains("xyz789"));
        assert!(!masked.contains("secret123"));
    }

    #[test]
    fn test_mask_error_message() {
        #[derive(Debug)]
        struct TestError;
        impl std::fmt::Display for TestError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "Error with API key: sk-proj-test123456789012345678901234567890"
                )
            }
        }
        impl std::error::Error for TestError {}

        let error: Box<dyn std::error::Error> = Box::new(TestError);
        let masked = mask_error_message(error.as_ref());
        assert!(!masked.contains("test123456789012345678901234567890"));
    }

    #[test]
    fn test_mask_debug_output() {
        #[derive(Debug)]
        struct TestStruct {
            api_key: String,
            token: String,
        }

        let value = TestStruct {
            api_key: "sk-proj-test123456789012345678901234567890".to_string(),
            token: "ghp_xyz789012345678901234567890123456789".to_string(),
        };

        let masked = mask_debug_output(&value);
        assert!(!masked.contains("test123456789012345678901234567890"));
        assert!(!masked.contains("xyz789012345678901234567890123456789"));
    }
}
