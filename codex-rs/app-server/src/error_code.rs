pub(crate) const INVALID_REQUEST_ERROR_CODE: i64 = -32600;
pub(crate) const INTERNAL_ERROR_CODE: i64 = -32603;
pub(crate) const OVERLOADED_ERROR_CODE: i64 = -32001;

// --- Fork custom additions ---
#![allow(dead_code)]
pub type ErrorCode = i64;
pub const ERROR_INVALID_REQUEST: ErrorCode = 400;
pub const ERROR_NOT_FOUND: ErrorCode = 404;
pub const ERROR_INTERNAL_ERROR: ErrorCode = 500;
pub const INVALID_REQUEST_ERROR_CODE: ErrorCode = 400;
pub const INTERNAL_ERROR_CODE: ErrorCode = 500;
// --- End fork additions ---
