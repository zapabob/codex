//! Error codes for app server

/// Error code type
pub type ErrorCode = i64;

/// Common error codes
pub const ERROR_INVALID_REQUEST: ErrorCode = 400;
pub const ERROR_NOT_FOUND: ErrorCode = 404;
pub const ERROR_INTERNAL_ERROR: ErrorCode = 500;
pub const INVALID_REQUEST_ERROR_CODE: ErrorCode = 400;
pub const INTERNAL_ERROR_CODE: ErrorCode = 500;
