//! Secret masking utilities for preventing sensitive information leakage
//!
//! Provides functions to mask API keys, tokens, passwords, and other sensitive
//! data in logs, error messages, and debug output.

/// Mask sensitive strings in text (API keys, tokens, passwords)
pub fn mask_secrets(text: &str) -> String {
    // Patterns to detect and mask:
    // - API keys: sk-proj-..., sk-..., ghp_..., AIzaSy...
    // - Tokens: Bearer tokens, access tokens
    // - Passwords: password=..., pwd=...
    // - URLs with credentials: https://user:pass@host
    
    let mut masked = text.to_string();
    
    // Mask OpenAI API keys (sk-proj-... or sk-...)
    masked = regex::Regex::new(r"sk-proj-[A-Za-z0-9]{32,}")
        .unwrap()
        .replace_all(&masked, "sk-proj-***MASKED***")
        .to_string();
    masked = regex::Regex::new(r"sk-[A-Za-z0-9]{32,}")
        .unwrap()
        .replace_all(&masked, "sk-***MASKED***")
        .to_string();
    
    // Mask GitHub tokens (ghp_..., gho_..., ghu_..., ghs_..., ghr_...)
    masked = regex::Regex::new(r"(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36,}")
        .unwrap()
        .replace_all(&masked, "$1_***MASKED***")
        .to_string();
    
    // Mask Google API keys (AIzaSy...)
    masked = regex::Regex::new(r"AIzaSy[A-Za-z0-9_-]{35}")
        .unwrap()
        .replace_all(&masked, "AIzaSy***MASKED***")
        .to_string();
    
    // Mask Bearer tokens
    masked = regex::Regex::new(r"Bearer\s+[A-Za-z0-9_-]{20,}")
        .unwrap()
        .replace_all(&masked, "Bearer ***MASKED***")
        .to_string();
    
    // Mask passwords in query strings or form data
    masked = regex::Regex::new(r"(?i)(password|pwd|pass|secret|token|api[_-]?key)\s*[:=]\s*([^\s&]+)")
        .unwrap()
        .replace_all(&masked, "$1=***MASKED***")
        .to_string();
    
    // Mask URLs with credentials (https://user:pass@host)
    masked = regex::Regex::new(r"https?://[^:]+:[^@]+@")
        .unwrap()
        .replace_all(&masked, "https://***MASKED***@")
        .to_string();
    
    masked
}

/// Mask secrets in error messages
pub fn mask_error_message(error: &dyn std::error::Error) -> String {
    let error_string = format!("{}", error);
    mask_secrets(&error_string)
}

/// Mask secrets in debug output
pub fn mask_debug_output<T: std::fmt::Debug>(value: &T) -> String {
    let debug_string = format!("{:?}", value);
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
}
