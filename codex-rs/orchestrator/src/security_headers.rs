//! Security headers for HTTP responses
//!
//! Provides utilities for adding security headers to HTTP responses.
//! Note: Currently orchestrator uses custom transport (UDS/Pipe/TCP),
//! but this module prepares for future HTTP support.

use std::collections::HashMap;

/// Security headers configuration
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    /// Enable HSTS (Strict-Transport-Security)
    pub hsts_enabled: bool,
    /// HSTS max-age in seconds
    pub hsts_max_age: u64,
    /// Include subdomains in HSTS
    pub hsts_include_subdomains: bool,
    /// Content Security Policy
    pub content_security_policy: Option<String>,
    /// Enable X-Frame-Options
    pub x_frame_options: bool,
    /// Enable X-Content-Type-Options
    pub x_content_type_options: bool,
    /// Enable X-XSS-Protection
    pub x_xss_protection: bool,
    /// Referrer Policy
    pub referrer_policy: Option<String>,
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            hsts_enabled: true,
            hsts_max_age: 31536000, // 1 year
            hsts_include_subdomains: true,
            content_security_policy: Some(
                "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';".to_string(),
            ),
            x_frame_options: true,
            x_content_type_options: true,
            x_xss_protection: true,
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
        }
    }
}

/// Security headers manager
pub struct SecurityHeaders {
    config: SecurityHeadersConfig,
}

impl SecurityHeaders {
    /// Create a new security headers manager
    pub fn new(config: SecurityHeadersConfig) -> Self {
        Self { config }
    }

    /// Get all security headers as a map
    pub fn get_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        // HSTS
        if self.config.hsts_enabled {
            let mut hsts_value = format!("max-age={}", self.config.hsts_max_age);
            if self.config.hsts_include_subdomains {
                hsts_value.push_str("; includeSubDomains");
            }
            headers.insert("Strict-Transport-Security".to_string(), hsts_value);
        }

        // X-Content-Type-Options
        if self.config.x_content_type_options {
            headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        }

        // X-Frame-Options
        if self.config.x_frame_options {
            headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
        }

        // X-XSS-Protection
        if self.config.x_xss_protection {
            headers.insert("X-XSS-Protection".to_string(), "1; mode=block".to_string());
        }

        // Content-Security-Policy
        if let Some(ref csp) = self.config.content_security_policy {
            headers.insert("Content-Security-Policy".to_string(), csp.clone());
        }

        // Referrer-Policy
        if let Some(ref policy) = self.config.referrer_policy {
            headers.insert("Referrer-Policy".to_string(), policy.clone());
        }

        headers
    }

    /// Get security headers as a string (for logging/debugging)
    pub fn get_headers_string(&self) -> String {
        let headers = self.get_headers();
        headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
