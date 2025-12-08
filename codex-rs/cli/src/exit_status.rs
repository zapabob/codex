//! Exit status handling for CLI

/// Exit codes for the CLI application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Success
    Success = 0,
    /// General error
    Error = 1,
    /// Configuration error
    ConfigError = 2,
    /// Network error
    NetworkError = 3,
    /// Authentication error
    AuthError = 4,
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> i32 {
        code as i32
    }
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> std::process::ExitCode {
        std::process::ExitCode::from(code as u8)
    }
}
