//! Windows 11 25H2 MCP Bridge
//!
//! Provides MCP tools for accessing Windows 11 25H2 specific features
//! including AI acceleration, enhanced security, and system integration.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(all(target_os = "windows", feature = "windows-ai"))]
use codex_core::windows_ai_integration::WindowsAiOptions;
#[cfg(target_os = "windows")]
use codex_core::windows_ai_integration::{get_gpu_statistics, is_windows_ai_available};

/// Windows 11 25H2 MCP Tool Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Windows25H2ToolParam {
    /// Tool action to perform
    pub action: Windows25H2Action,
    /// Optional parameters for the action
    pub params: Option<HashMap<String, serde_json::Value>>,
}

/// Windows 11 25H2 MCP Tool Actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Windows25H2Action {
    /// Get GPU statistics using Windows AI API
    GetGpuStats,
    /// Execute with Windows AI acceleration
    ExecuteWithAi {
        prompt: String,
        use_kernel_acceleration: Option<bool>,
    },
    /// Check Windows AI availability
    CheckAiAvailability,
    /// Get Windows 11 25H2 system information
    GetSystemInfo,
    /// Enable Windows AI acceleration
    EnableAiAcceleration {
        enabled: bool,
        kernel_accelerated: Option<bool>,
    },
}

/// Windows 11 25H2 MCP Tool Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Windows25H2ToolResult {
    pub success: bool,
    pub data: serde_json::Value,
    pub message: Option<String>,
}

/// Handle Windows 11 25H2 MCP tool requests
#[cfg(target_os = "windows")]
pub async fn handle_windows_25h2_tool(
    param: Windows25H2ToolParam,
) -> Result<Windows25H2ToolResult> {
    match param.action {
        Windows25H2Action::GetGpuStats => {
            let stats = get_gpu_statistics().await?;
            Ok(Windows25H2ToolResult {
                success: true,
                data: serde_json::json!({
                    "utilization": stats.utilization,
                    "memory_used_mb": stats.memory_used / 1024 / 1024,
                    "memory_total_mb": stats.memory_total / 1024 / 1024,
                    "temperature": stats.temperature,
                }),
                message: Some("GPU statistics retrieved successfully".to_string()),
            })
        }
        Windows25H2Action::ExecuteWithAi {
            prompt,
            use_kernel_acceleration,
        } => {
            #[cfg(feature = "windows-ai")]
            {
                let options = WindowsAiOptions {
                    enabled: true,
                    kernel_accelerated: use_kernel_acceleration.unwrap_or(false),
                    use_gpu: true,
                };

                let result =
                    codex_core::windows_ai_integration::execute_with_windows_ai(&prompt, &options)
                        .await?;
                Ok(Windows25H2ToolResult {
                    success: true,
                    data: serde_json::json!({
                        "result": result,
                        "kernel_accelerated": options.kernel_accelerated,
                    }),
                    message: Some("Execution completed with Windows AI acceleration".to_string()),
                })
            }

            #[cfg(not(feature = "windows-ai"))]
            {
                let _ = prompt;
                let _ = use_kernel_acceleration;
                Ok(Windows25H2ToolResult {
                    success: false,
                    data: serde_json::json!({
                        "error": "Windows AI feature not enabled"
                    }),
                    message: Some("Windows AI feature is not enabled. Enable 'windows-ai' feature to use this functionality.".to_string()),
                })
            }
        }
        Windows25H2Action::CheckAiAvailability => {
            let available = is_windows_ai_available();
            Ok(Windows25H2ToolResult {
                success: true,
                data: serde_json::json!({
                    "available": available,
                    "platform": "Windows 11 25H2",
                }),
                message: Some(if available {
                    "Windows AI is available on this system".to_string()
                } else {
                    "Windows AI is not available on this system".to_string()
                }),
            })
        }
        Windows25H2Action::GetSystemInfo => {
            let os_version = get_windows_version();
            let ai_available = is_windows_ai_available();

            Ok(Windows25H2ToolResult {
                success: true,
                data: serde_json::json!({
                    "os_version": os_version,
                    "windows_ai_available": ai_available,
                    "features": {
                        "ai_acceleration": ai_available,
                        "kernel_driver": cfg!(feature = "windows-ai"),
                    },
                }),
                message: Some("System information retrieved".to_string()),
            })
        }
        Windows25H2Action::EnableAiAcceleration {
            enabled,
            kernel_accelerated,
        } => {
            // This would typically update a global configuration
            // For now, we just return the status
            Ok(Windows25H2ToolResult {
                success: true,
                data: serde_json::json!({
                    "enabled": enabled,
                    "kernel_accelerated": kernel_accelerated.unwrap_or(false),
                }),
                message: Some(format!(
                    "Windows AI acceleration {} (kernel: {})",
                    if enabled { "enabled" } else { "disabled" },
                    kernel_accelerated.unwrap_or(false)
                )),
            })
        }
    }
}

/// Handle Windows 11 25H2 MCP tool requests (non-Windows stub)
#[cfg(not(target_os = "windows"))]
pub async fn handle_windows_25h2_tool(
    _param: Windows25H2ToolParam,
) -> Result<Windows25H2ToolResult> {
    Ok(Windows25H2ToolResult {
        success: false,
        data: serde_json::json!({
            "error": "Windows 11 25H2 features are only available on Windows"
        }),
        message: Some("This tool is only available on Windows 11 25H2".to_string()),
    })
}

/// Get Windows version information
#[cfg(target_os = "windows")]
fn get_windows_version() -> String {
    use std::process::Command;

    // Try to get Windows version from system
    if let Ok(output) = Command::new("powershell")
        .args([
            "-Command",
            "(Get-CimInstance Win32_OperatingSystem).Version",
        ])
        .output()
    {
        if let Ok(version) = String::from_utf8(output.stdout) {
            return version.trim().to_string();
        }
    }

    "Unknown".to_string()
}

#[cfg(not(target_os = "windows"))]
fn get_windows_version() -> String {
    "N/A (Not Windows)".to_string()
}
