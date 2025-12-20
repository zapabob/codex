//! WSL path handling utilities

use std::path::Path;
use std::path::PathBuf;

/// Convert Windows path to WSL path
pub fn windows_to_wsl_path(windows_path: &Path) -> PathBuf {
    // Simple conversion - in real implementation, this would handle WSL path conversion
    windows_path.to_path_buf()
}

/// Convert WSL path to Windows path
pub fn wsl_to_windows_path(wsl_path: &Path) -> PathBuf {
    // Simple conversion - in real implementation, this would handle WSL path conversion
    wsl_path.to_path_buf()
}

/// Check if we're running under WSL
pub fn is_wsl() -> bool {
    std::env::var("WSL_DISTRO_NAME").is_ok()
}

/// Normalize path for WSL usage
pub fn normalize_for_wsl(path: &Path) -> PathBuf {
    if is_wsl() {
        windows_to_wsl_path(path)
    } else {
        path.to_path_buf()
    }
}
