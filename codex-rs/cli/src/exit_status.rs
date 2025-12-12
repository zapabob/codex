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

/// Handle exit status from a command execution
pub fn handle_exit_status(exit_code: Option<i32>) -> ExitCode {
    match exit_code {
        Some(0) => ExitCode::Success,
        Some(1) => ExitCode::Error,
        Some(2) => ExitCode::ConfigError,
        Some(3) => ExitCode::NetworkError,
        Some(4) => ExitCode::AuthError,
        _ => ExitCode::Error,
    }
}

/// Handle exit status from std::process::ExitStatus
pub fn handle_process_exit_status(status: std::process::ExitStatus) -> ExitCode {
    if status.success() {
        ExitCode::Success
    } else {
        status
            .code()
            .map(|code| match code {
                1 => ExitCode::Error,
                2 => ExitCode::ConfigError,
                3 => ExitCode::NetworkError,
                4 => ExitCode::AuthError,
                _ => ExitCode::Error,
            })
            .unwrap_or(ExitCode::Error)
    }
}
