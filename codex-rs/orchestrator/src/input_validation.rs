//! Input validation utilities for RPC requests
//!
//! Provides validation functions for RPC parameters to prevent injection attacks
//! and ensure data integrity.

use std::path::PathBuf;

/// Maximum string length for RPC parameters
pub const MAX_STRING_LENGTH: usize = 1_000_000; // 1MB
/// Maximum path length
pub const MAX_PATH_LENGTH: usize = 4096;
/// Maximum number of items in arrays
pub const MAX_ARRAY_LENGTH: usize = 10_000;

/// Input validation error
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("String too long: max {max} characters, got {got}")]
    StringTooLong { max: usize, got: usize },
    #[error("Path too long: max {max} characters, got {got}")]
    PathTooLong { max: usize, got: usize },
    #[error("Array too long: max {max} items, got {got}")]
    ArrayTooLong { max: usize, got: usize },
    #[error("Invalid characters in string: {0}")]
    InvalidCharacters(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),
}

/// Validate and sanitize a string parameter
pub fn validate_string(s: &str, max_length: Option<usize>) -> Result<String, ValidationError> {
    let max_len = max_length.unwrap_or(MAX_STRING_LENGTH);
    if s.len() > max_len {
        return Err(ValidationError::StringTooLong {
            max: max_len,
            got: s.len(),
        });
    }

    // Check for null bytes and other dangerous characters
    if s.contains('\0') {
        return Err(ValidationError::InvalidCharacters(
            "Null bytes not allowed".to_string(),
        ));
    }

    // Check for control characters (except newline, tab, carriage return)
    if s.chars()
        .any(|c| c.is_control() && c != '\n' && c != '\t' && c != '\r')
    {
        return Err(ValidationError::InvalidCharacters(
            "Control characters not allowed".to_string(),
        ));
    }

    Ok(s.to_string())
}

/// Validate and normalize a path
pub fn validate_path(path: &PathBuf, allowed_base: &[PathBuf]) -> Result<PathBuf, ValidationError> {
    // Check path length
    let path_str = path.to_string_lossy();
    if path_str.len() > MAX_PATH_LENGTH {
        return Err(ValidationError::PathTooLong {
            max: MAX_PATH_LENGTH,
            got: path_str.len(),
        });
    }

    // Normalize path (resolve .. and .)
    let normalized = dunce::canonicalize(path)
        .map_err(|e| ValidationError::InvalidPath(format!("Failed to normalize path: {e}")))?;

    // Check for path traversal
    let normalized_str = normalized.to_string_lossy();
    if normalized_str.contains("..") || normalized_str.contains("~") {
        return Err(ValidationError::PathTraversal(
            "Path traversal detected".to_string(),
        ));
    }

    // Check if path is within allowed base directories
    let is_allowed = allowed_base.iter().any(|base| normalized.starts_with(base));

    if !is_allowed {
        return Err(ValidationError::InvalidPath(format!(
            "Path not in allowed directories: {}",
            normalized_str
        )));
    }

    Ok(normalized)
}

/// Validate an array parameter
pub fn validate_array<T>(arr: &[T], max_length: Option<usize>) -> Result<(), ValidationError> {
    let max_len = max_length.unwrap_or(MAX_ARRAY_LENGTH);
    if arr.len() > max_len {
        return Err(ValidationError::ArrayTooLong {
            max: max_len,
            got: arr.len(),
        });
    }
    Ok(())
}

/// Sanitize a string by removing dangerous characters
pub fn sanitize_string(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || c == &'\n' || c == &'\t' || c == &'\r')
        .filter(|c| *c != '\0')
        .collect()
}

/// Validate JSON value for dangerous content
pub fn validate_json_value(value: &serde_json::Value) -> Result<(), ValidationError> {
    match value {
        serde_json::Value::String(s) => {
            validate_string(s, None)?;
        }
        serde_json::Value::Array(arr) => {
            validate_array(arr, None)?;
            for item in arr {
                validate_json_value(item)?;
            }
        }
        serde_json::Value::Object(obj) => {
            if obj.len() > MAX_ARRAY_LENGTH {
                return Err(ValidationError::ArrayTooLong {
                    max: MAX_ARRAY_LENGTH,
                    got: obj.len(),
                });
            }
            for (key, val) in obj {
                validate_string(key, Some(256))?; // Key length limit
                validate_json_value(val)?;
            }
        }
        _ => {
            // Numbers, booleans, null are safe
        }
    }
    Ok(())
}
