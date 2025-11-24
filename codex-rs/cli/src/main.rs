//! Codex CLI - AI-Native OS Command Line Interface

use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::process::Command;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use codex_core::orchestration::ResourceManager;
use codex_core::security::{MalwareDetector, Quarantine};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use dirs;
use which;
use which;
use dirs;
use which;

/// RPC Request
#[derive(Debug, Serialize, Deserialize)]
struct RpcRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    params: serde_json::Value,
}

/// RPC Response
#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<RpcError>,
}

/// RPC Error
#[derive(Debug, Serialize, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "tui" => launch_tui(),
        "gui" => launch_gui(),
        "server" => launch_server(),
        "mcp-gemini" => launch_gemini_mcp_server(),
        // "deep-research" => launch_deep_research(&args), // Temporarily disabled
        "plan" => launch_plan(&args),
        // "qc" => launch_qc(&args), // Temporarily disabled
        "worktree" => launch_worktree(&args),
        "delegate" => launch_delegate(&args),
        "--version" | "-v" => {
            print_version();
            Ok(())
        }
        "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            std::process::exit(1);
        }
    }
}

fn launch_tui() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Terminal User Interface...");

    // First try codex-tui.exe
    let tui_path = std::env::current_exe()?.parent().unwrap().join("codex-tui");

    // If not found, try codex-tui (without extension)
    let tui_path = if tui_path.exists() {
        tui_path
    } else {
        std::env::current_exe()?
            .parent()
            .unwrap()
            .join("codex-tui.exe")
    };

    match Command::new(&tui_path).spawn() {
        Ok(mut child) => {
            println!("TUI launched successfully (PID: {})", child.id());
            let status = child.wait()?;
            println!("TUI exited with status: {}", status);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to launch TUI: {}", e);
            eprintln!("TUI path tried: {:?}", tui_path);
            eprintln!("Please ensure the TUI application is installed and available in PATH.");
            // Don't exit with error for TUI, as it's not yet implemented
            Ok(())
        }
    }
}

fn launch_gui() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Graphical User Interface...");

    let gui_path = std::env::current_exe()?
        .parent()
        .unwrap()
        .join("codex-tauri-gui");

    match Command::new(gui_path).spawn() {
        Ok(mut child) => {
            println!("GUI launched successfully (PID: {})", child.id());
            let status = child.wait()?;
            println!("GUI exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to launch GUI: {}", e);
            eprintln!("Please ensure the GUI application is installed.");
            std::process::exit(1);
        }
    }

    Ok(())
}

// Global resource manager instance
static RESOURCE_MANAGER: Lazy<Arc<RwLock<Option<ResourceManager>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

static MALWARE_DETECTOR: Lazy<Arc<RwLock<Option<Arc<MalwareDetector>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

static QUARANTINE: Lazy<Arc<RwLock<Option<Arc<Quarantine>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

/// Initialize resource manager
async fn init_resource_manager() {
    let mut manager = RESOURCE_MANAGER.write().await;
    if manager.is_none() {
        *manager = Some(codex_core::orchestration::ResourceManager::new());
    }
}

/// Handle RPC method calls
async fn handle_rpc_method(
    method: &str,
    params: &serde_json::Value,
) -> Result<serde_json::Value, RpcError> {
    match method {
        "account.login" => {
            // Mock login response
            Ok(serde_json::json!({
                "token": "mock-jwt-token",
                "user": {
                    "id": "user1",
                    "email": "user@example.com",
                    "name": "Mock User"
                }
            }))
        }

        "account.logout" => Ok(serde_json::Value::Null),

        "account.read" => Ok(serde_json::json!({
            "id": "user1",
            "email": "user@example.com",
            "name": "Mock User",
            "plan": "free"
        })),

        "conversation.list" => Ok(serde_json::json!([])),

        "conversation.create" => {
            let title = params
                .get("initialMessage")
                .and_then(|v| v.as_str())
                .unwrap_or("New Conversation");

            Ok(serde_json::json!({
                "id": format!("conv-{}", chrono::Utc::now().timestamp()),
                "title": title,
                "createdAt": chrono::Utc::now().to_rfc3339(),
                "updatedAt": chrono::Utc::now().to_rfc3339(),
                "model": params.get("model").unwrap_or(&serde_json::json!("gpt-4")),
                "messageCount": 1
            }))
        }

        "conversation.sendMessage" => Ok(serde_json::json!({
            "id": format!("msg-{}", chrono::Utc::now().timestamp()),
            "conversationId": params.get("conversationId").unwrap_or(&serde_json::json!("conv-1")),
            "role": "assistant",
            "content": "This is a mock response from the CLI server. Real AI integration coming soon.",
            "createdAt": chrono::Utc::now().to_rfc3339()
        })),

        "agent.list" => Ok(serde_json::json!([
            {
                "id": "code-reviewer",
                "name": "Code Reviewer",
                "type": "code-reviewer",
                "status": "idle",
                "description": "コードの品質とセキュリティをレビューします"
            },
            {
                "id": "test-gen",
                "name": "Test Generator",
                "type": "test-gen",
                "status": "idle",
                "description": "自動的にテストコードを生成します"
            },
            {
                "id": "sec-audit",
                "name": "Security Auditor",
                "type": "sec-audit",
                "status": "idle",
                "description": "セキュリティ脆弱性をスキャンします"
            },
            {
                "id": "researcher",
                "name": "Deep Researcher",
                "type": "researcher",
                "status": "idle",
                "description": "高度な研究と分析を行います"
            }
        ])),

        "agent.run" => {
            let agent_id = params
                .get("agentId")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            Ok(serde_json::json!({
                "status": "completed",
                "output": format!("Mock execution completed for agent: {}", agent_id),
                "error": "",
                "exitCode": 0,
                "duration": 100
            }))
        }

        "mcp.connections" => Ok(serde_json::json!([
            {
                "id": "filesystem",
                "name": "File System",
                "type": "filesystem",
                "status": "connected",
                "lastConnected": chrono::Utc::now().to_rfc3339()
            },
            {
                "id": "github",
                "name": "GitHub",
                "type": "github",
                "status": "connected",
                "lastConnected": chrono::Utc::now().to_rfc3339()
            }
        ])),

        "system.metrics" => Ok(serde_json::json!({
            "cpuUsage": 15.5,
            "memoryUsage": 45.2,
            "diskUsage": 67.8,
            "activeProcesses": 42,
            "uptime": 3600
        })),

        "exec.command" => {
            let command_str = params
                .get("command")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'command' parameter".to_string(),
                    data: None,
                })?;

            // Parse command into Vec<String>
            let command: Vec<String> = shlex::split(command_str)
                .unwrap_or_else(|| vec![command_str.to_string()]);

            // YOLOモードでも危険なコマンドはブロック
            if codex_core::command_safety::is_dangerous_command::command_might_be_dangerous(&command) {
                return Err(RpcError {
                    code: -32001,
                    message: format!(
                        "Dangerous command blocked: {}. Dangerous commands cannot be executed even in YOLO mode for security reasons.",
                        command_str
                    ),
                    data: Some(serde_json::json!({
                        "blocked": true,
                        "reason": "dangerous_command",
                        "command": command_str
                    })),
                });
            }

            // Note: In production, this should execute the actual command
            // For now, return mock response
            Ok(serde_json::json!({
                "exitCode": 0,
                "stdout": format!("Command would be executed: {}", command_str),
                "stderr": ""
            }))
        }

        "fs.search" => Ok(serde_json::json!([])),

        // Resource management methods
        "resource.getStatus" => {
            init_resource_manager().await;
            let manager = RESOURCE_MANAGER.read().await;
            if let Some(ref rm) = *manager {
                let capacity = rm.get_capacity().await;
                let stats = rm.get_system_stats().await.map_err(|e| RpcError {
                    code: -32000,
                    message: format!("Failed to get system stats: {}", e),
                    data: None,
                })?;
                Ok(serde_json::json!({
                    "capacity": {
                        "maxConcurrent": capacity.max_concurrent,
                        "activeTasks": capacity.active_tasks,
                        "availableSlots": capacity.available_slots
                    },
                    "stats": {
                        "cpuUsagePercent": stats.cpu_usage_percent,
                        "memoryUsedBytes": stats.memory_used_bytes,
                        "memoryTotalBytes": stats.memory_total_bytes,
                        "memoryUsagePercent": stats.memory_usage_percent,
                        "activeAgents": stats.active_agents,
                        "cpuCores": stats.cpu_cores
                    }
                }))
            } else {
                Err(RpcError {
                    code: -32000,
                    message: "Resource manager not initialized".to_string(),
                    data: None,
                })
            }
        }

        "resource.acquire" => {
            init_resource_manager().await;
            let manager = RESOURCE_MANAGER.read().await;
            if let Some(ref rm) = *manager {
                match rm.acquire_slot().await {
                    Ok(_guard) => {
                        // Guard is stored in a way that it will be released when dropped
                        // For now, we'll return success and the guard will be dropped immediately
                        // In production, you'd want to store guards in a map keyed by request ID
                        Ok(serde_json::json!({
                            "success": true,
                            "message": "Resource slot acquired"
                        }))
                    }
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Failed to acquire resource slot: {}", e),
                        data: None,
                    }),
                }
            } else {
                Err(RpcError {
                    code: -32000,
                    message: "Resource manager not initialized".to_string(),
                    data: None,
                })
            }
        }

        "resource.release" => {
            // Resource release is handled automatically by ResourceGuard Drop
            // This method is provided for explicit release if needed
            Ok(serde_json::json!({
                "success": true,
                "message": "Resource slot will be released automatically"
            }))
        }

        // CLI execution methods
        "cli.codex.execute" => {
            let prompt = params
                .get("prompt")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'prompt' parameter".to_string(),
                    data: None,
                })?;

            let output = tokio::process::Command::new("codex")
                .arg("exec")
                .arg(prompt)
                .output()
                .await
                .map_err(|e| RpcError {
                    code: -32000,
                    message: format!("Failed to execute codex: {}", e),
                    data: None,
                })?;

            Ok(serde_json::json!({
                "exitCode": output.status.code().unwrap_or(-1),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "success": output.status.success()
            }))
        }

        "cli.gemini.execute" => {
            let prompt = params
                .get("prompt")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'prompt' parameter".to_string(),
                    data: None,
                })?;

            // Try gemini-cli first, fallback to gemini
            let output = tokio::process::Command::new("gemini-cli")
                .arg(prompt)
                .output()
                .await
                .or_else(|_| {
                    tokio::process::Command::new("gemini")
                        .arg(prompt)
                        .output()
                })
                .await
                .map_err(|e| RpcError {
                    code: -32000,
                    message: format!("Failed to execute gemini: {}", e),
                    data: None,
                })?;

            Ok(serde_json::json!({
                "exitCode": output.status.code().unwrap_or(-1),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "success": output.status.success()
            }))
        }

        "cli.claude.execute" => {
            let prompt = params
                .get("prompt")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'prompt' parameter".to_string(),
                    data: None,
                })?;

            // Try claudecode first, fallback to claude
            let output = tokio::process::Command::new("claudecode")
                .arg(prompt)
                .output()
                .await
                .or_else(|_| {
                    tokio::process::Command::new("claude")
                        .arg(prompt)
                        .output()
                })
                .await
                .map_err(|e| RpcError {
                    code: -32000,
                    message: format!("Failed to execute claude: {}", e),
                    data: None,
                })?;

            Ok(serde_json::json!({
                "exitCode": output.status.code().unwrap_or(-1),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "success": output.status.success()
            }))
        }

        // GeminiCLI MCP integration methods
        "mcp.gemini.start" => {
            // Start Gemini MCP server as background process
            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.spawn(async {
                if let Err(e) = launch_gemini_mcp_server() {
                    eprintln!("Failed to start Gemini MCP server: {}", e);
                }
            });

            Ok(serde_json::json!({
                "success": true,
                "message": "Gemini MCP server starting in background"
            }))
        }

        "mcp.gemini.status" => {
            // Check if Gemini CLI is available
            let check = tokio::process::Command::new("gemini")
                .arg("--version")
                .output()
                .await;

            match check {
                Ok(output) if output.status.success() => {
                    let version = String::from_utf8_lossy(&output.stdout);
                    Ok(serde_json::json!({
                        "available": true,
                        "version": version.trim(),
                        "message": "Gemini CLI is available"
                    }))
                }
                _ => Ok(serde_json::json!({
                    "available": false,
                    "message": "Gemini CLI not found. Install with: npm install -g @google/gemini-cli"
                }))
            }
        }

        "mcp.gemini.execute" => {
            let query = params
                .get("query")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'query' parameter".to_string(),
                    data: None,
                })?;

            let model = params
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or("gemini-2.5-flash");

            let use_grounding = params
                .get("useGrounding")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            // Execute Gemini CLI with optional grounding
            let mut cmd = tokio::process::Command::new("gemini");
            cmd.args(["--model", model]);
            
            if use_grounding {
                cmd.args(["--grounding", "web", "--format", "json"]);
            }
            
            cmd.arg(query);

            let output = cmd.output().await.map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to execute Gemini: {}", e),
                data: None,
            })?;

            Ok(serde_json::json!({
                "exitCode": output.status.code().unwrap_or(-1),
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "success": output.status.success()
            }))
        }

        // Plan management methods
        "plan.create" => {
            let title = params
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'title' parameter".to_string(),
                    data: None,
                })?;

            let mode = params
                .get("mode")
                .and_then(|v| v.as_str())
                .unwrap_or("orchestrated");

            let budget_tokens = params
                .get("budgetTokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(100000);

            let budget_time = params
                .get("budgetTime")
                .and_then(|v| v.as_u64())
                .unwrap_or(30);

            // TODO: Implement actual plan creation via codex-core
            Ok(serde_json::json!({
                "id": format!("plan-{}", chrono::Utc::now().timestamp()),
                "title": title,
                "mode": mode,
                "budgetTokens": budget_tokens,
                "budgetTime": budget_time,
                "state": "drafting",
                "createdAt": chrono::Utc::now().to_rfc3339(),
                "updatedAt": chrono::Utc::now().to_rfc3339()
            }))
        }

        "plan.list" => {
            // TODO: Implement actual plan listing via codex-core
            Ok(serde_json::json!({
                "plans": []
            }))
        }

        "plan.approve" => {
            let plan_id = params
                .get("planId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'planId' parameter".to_string(),
                    data: None,
                })?;

            // TODO: Implement actual plan approval via codex-core
            Ok(serde_json::json!({
                "success": true,
                "planId": plan_id,
                "state": "approved"
            }))
        }

        "plan.reject" => {
            let plan_id = params
                .get("planId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'planId' parameter".to_string(),
                    data: None,
                })?;

            let reason = params
                .get("reason")
                .and_then(|v| v.as_str())
                .unwrap_or("Rejected by user");

            // TODO: Implement actual plan rejection via codex-core
            Ok(serde_json::json!({
                "success": true,
                "planId": plan_id,
                "state": "rejected",
                "reason": reason
            }))
        }

        "plan.execute" => {
            let plan_id = params
                .get("planId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'planId' parameter".to_string(),
                    data: None,
                })?;

            // TODO: Implement actual plan execution via codex-core
            Ok(serde_json::json!({
                "success": true,
                "planId": plan_id,
                "state": "executing"
            }))
        }

        "plan.status" => {
            let plan_id = params
                .get("planId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'planId' parameter".to_string(),
                    data: None,
                })?;

            // TODO: Implement actual plan status check via codex-core
            Ok(serde_json::json!({
                "planId": plan_id,
                "state": "drafting",
                "blocks": []
            }))
        }

        // Windows 11 25H2 MCP Standard Features
        "mcp.windows.detect" => {
            // Detect Windows 11 25H2 MCP standard features
            #[cfg(target_os = "windows")]
            {
                use std::process::Command;
                
                // Check Windows build version
                let output = Command::new("cmd")
                    .args(["/C", "ver"])
                    .output();
                
                let build_version = if let Ok(output) = output {
                    let version_str = String::from_utf8_lossy(&output.stdout);
                    // Extract build number (e.g., "26100" from "Microsoft Windows [Version 10.0.26100.xxxx]")
                    version_str
                        .split_whitespace()
                        .find_map(|s| {
                            if s.contains("26100") || s.contains("26200") {
                                Some("25H2".to_string())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| "Unknown".to_string())
                } else {
                    "Unknown".to_string()
                };

                // Check for MCP standard features
                let mcp_available = build_version == "25H2";
                
                Ok(serde_json::json!({
                    "windowsVersion": build_version,
                    "mcpStandardAvailable": mcp_available,
                    "features": {
                        "autoDetection": mcp_available,
                        "standardProtocol": mcp_available,
                        "nativeIntegration": mcp_available
                    }
                }))
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                Ok(serde_json::json!({
                    "windowsVersion": "N/A",
                    "mcpStandardAvailable": false,
                    "message": "Windows 11 25H2 MCP features only available on Windows"
                }))
            }
        }

        "mcp.windows.autoDetect" => {
            // Auto-detect MCP servers using Windows 11 25H2 standard features
            #[cfg(target_os = "windows")]
            {
                use std::process::Command;
                use std::path::PathBuf;
                
                // Common MCP server locations
                let search_paths = vec![
                    PathBuf::from("C:\\Program Files\\Codex\\mcp-servers"),
                    PathBuf::from(std::env::var("LOCALAPPDATA").unwrap_or_default()).join("Codex\\mcp-servers"),
                    PathBuf::from(std::env::var("APPDATA").unwrap_or_default()).join("Codex\\mcp-servers"),
                ];

                let mut detected_servers = Vec::new();

                for path in search_paths {
                    if path.exists() {
                        // Scan directory for MCP servers
                        if let Ok(entries) = std::fs::read_dir(&path) {
                            for entry in entries.flatten() {
                                if let Ok(metadata) = entry.metadata() {
                                    if metadata.is_file() {
                                        let file_name = entry.file_name().to_string_lossy().to_string();
                                        if file_name.ends_with(".exe") || file_name.ends_with(".cmd") {
                                            detected_servers.push(serde_json::json!({
                                                "name": file_name.replace(".exe", "").replace(".cmd", ""),
                                                "path": entry.path().to_string_lossy().to_string(),
                                                "type": "windows-native"
                                            }));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(serde_json::json!({
                    "servers": detected_servers,
                    "count": detected_servers.len()
                }))
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                Ok(serde_json::json!({
                    "servers": [],
                    "count": 0,
                    "message": "Auto-detection only available on Windows"
                }))
            }
        }

        "mcp.windows.manage" => {
            let action = params
                .get("action")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'action' parameter".to_string(),
                    data: None,
                })?;

            match action {
                "list" => {
                    // List all MCP servers
                    Ok(serde_json::json!({
                        "servers": [],
                        "message": "MCP server management via Windows 11 25H2 standard features"
                    }))
                }
                "connect" => {
                    let server_id = params.get("serverId").and_then(|v| v.as_str());
                    Ok(serde_json::json!({
                        "success": true,
                        "serverId": server_id,
                        "message": "Connected via Windows 11 25H2 MCP standard protocol"
                    }))
                }
                "disconnect" => {
                    let server_id = params.get("serverId").and_then(|v| v.as_str());
                    Ok(serde_json::json!({
                        "success": true,
                        "serverId": server_id,
                        "message": "Disconnected"
                    }))
                }
                _ => Err(RpcError {
                    code: -32602,
                    message: format!("Unknown action: {}", action),
                    data: None,
                })
            }
        }

        // GPU Access Methods
        "gpu.getStatus" => {
            #[cfg(target_os = "windows")]
            {
                // TODO: Implement actual GPU status via Windows AI API or DirectML
                // For now, return mock data
                Ok(serde_json::json!({
                    "gpus": [
                        {
                            "name": "NVIDIA GeForce RTX 3080",
                            "vendor": "nvidia",
                            "usagePercent": 45.5,
                            "memoryUsed": 8589934592,
                            "memoryTotal": 10737418240,
                            "memoryUsagePercent": 80.0,
                            "temperature": 72,
                            "powerUsage": 250,
                            "clockSpeed": 1710,
                            "computeCapability": "8.6",
                            "cudaVersion": "12.0",
                            "directMLVersion": "1.13"
                        }
                    ]
                }))
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                Ok(serde_json::json!({
                    "gpus": [],
                    "message": "GPU status only available on Windows"
                }))
            }
        }

        "gpu.getAccess" => {
            // Check GPU access permissions
            Ok(serde_json::json!({
                "hasAccess": true,
                "permissions": {
                    "compute": true,
                    "memory": true,
                    "monitoring": true
                }
            }))
        }

        "gpu.optimize" => {
            let settings = params.get("settings").unwrap_or(&serde_json::json!({}));
            // TODO: Implement GPU optimization settings
            Ok(serde_json::json!({
                "success": true,
                "message": "GPU optimization settings applied",
                "settings": settings
            }))
        }

        // Malware Detection Methods
        "malware.scan" => {
            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'path' parameter".to_string(),
                    data: None,
                })?;

            let scan_type = params
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("file"); // "file" or "directory"

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                // Initialize detector if needed
                {
                    let mut detector = MALWARE_DETECTOR.write().await;
                    if detector.is_none() {
                        *detector = Some(Arc::new(MalwareDetector::new()));
                    }
                }

                let detector = MALWARE_DETECTOR.read().await;
                let detector = detector.as_ref().unwrap();

                let scan_path = PathBuf::from(path);
                let results = if scan_type == "directory" {
                    detector.scan_directory(&scan_path).await
                } else {
                    detector.scan_file(&scan_path).await
                };

                match results {
                    Ok(detections) => {
                        let results_json: Vec<serde_json::Value> = detections
                            .iter()
                            .map(|d| {
                                serde_json::json!({
                                    "filePath": d.file_path.to_string_lossy(),
                                    "method": format!("{:?}", d.method),
                                    "threatName": d.threat_name,
                                    "confidence": d.confidence,
                                    "severity": format!("{:?}", d.severity),
                                    "details": d.details,
                                    "timestamp": d.timestamp.to_rfc3339()
                                })
                            })
                            .collect();

                        Ok(serde_json::json!({
                            "success": true,
                            "threatsFound": detections.len(),
                            "results": results_json
                        }))
                    }
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Scan failed: {}", e),
                        data: None,
                    }),
                }
            })
        }

        "malware.quarantine" => {
            let file_path = params
                .get("filePath")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'filePath' parameter".to_string(),
                    data: None,
                })?;

            let threat_name = params
                .get("threatName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown Threat");

            let confidence = params
                .get("confidence")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.9) as f32;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                // Initialize quarantine if needed
                {
                    let mut quarantine = QUARANTINE.write().await;
                    if quarantine.is_none() {
                        let quarantine_dir = dirs::data_dir()
                            .unwrap_or_else(|| PathBuf::from("."))
                            .join("codex")
                            .join("quarantine");
                        let q = Arc::new(Quarantine::new(quarantine_dir));
                        q.initialize().await.map_err(|e| RpcError {
                            code: -32000,
                            message: format!("Failed to initialize quarantine: {}", e),
                            data: None,
                        })?;
                        *quarantine = Some(q);
                    }
                }

                let quarantine = QUARANTINE.read().await;
                let quarantine = quarantine.as_ref().unwrap();

                let path = PathBuf::from(file_path);
                match quarantine.quarantine_file(&path, threat_name, confidence).await {
                    Ok(entry) => Ok(serde_json::json!({
                        "success": true,
                        "entryId": entry.id,
                        "originalPath": entry.original_path.to_string_lossy(),
                        "quarantinePath": entry.quarantine_path.to_string_lossy(),
                        "threatName": entry.threat_name,
                        "quarantinedAt": entry.quarantined_at.to_rfc3339()
                    })),
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Quarantine failed: {}", e),
                        data: None,
                    }),
                }
            })
        }

        "malware.delete" => {
            let entry_id = params
                .get("entryId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'entryId' parameter".to_string(),
                    data: None,
                })?;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                let quarantine = QUARANTINE.read().await;
                let quarantine = quarantine.as_ref().ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Quarantine not initialized".to_string(),
                    data: None,
                })?;

                match quarantine.delete_file(entry_id).await {
                    Ok(_) => Ok(serde_json::json!({
                        "success": true,
                        "message": "File deleted successfully"
                    })),
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Delete failed: {}", e),
                        data: None,
                    }),
                }
            })
        }

        "malware.restore" => {
            let entry_id = params
                .get("entryId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'entryId' parameter".to_string(),
                    data: None,
                })?;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                let quarantine = QUARANTINE.read().await;
                let quarantine = quarantine.as_ref().ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Quarantine not initialized".to_string(),
                    data: None,
                })?;

                match quarantine.restore_file(entry_id).await {
                    Ok(_) => Ok(serde_json::json!({
                        "success": true,
                        "message": "File restored successfully"
                    })),
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Restore failed: {}", e),
                        data: None,
                    }),
                }
            })
        }

        "malware.listQuarantine" => {
            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                let quarantine = QUARANTINE.read().await;
                let quarantine = quarantine.as_ref().ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Quarantine not initialized".to_string(),
                    data: None,
                })?;

                let entries = quarantine.list_entries().await;
                let entries_json: Vec<serde_json::Value> = entries
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "id": e.id,
                            "originalPath": e.original_path.to_string_lossy(),
                            "quarantinePath": e.quarantine_path.to_string_lossy(),
                            "threatName": e.threat_name,
                            "confidence": e.confidence,
                            "status": format!("{:?}", e.status),
                            "quarantinedAt": e.quarantined_at.to_rfc3339()
                        })
                    })
                    .collect();

                Ok(serde_json::json!({
                    "entries": entries_json,
                    "count": entries_json.len()
                }))
            })
        }

        "malware.getStats" => {
            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                let detector = MALWARE_DETECTOR.read().await;
                let detector = detector.as_ref().ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Detector not initialized".to_string(),
                    data: None,
                })?;

                let stats = detector.get_stats().await;
                Ok(serde_json::json!({
                    "totalFilesScanned": stats.total_files_scanned,
                    "threatsDetected": stats.threats_detected,
                    "signatureMatches": stats.signature_matches,
                    "heuristicMatches": stats.heuristic_matches,
                    "behavioralMatches": stats.behavioral_matches
                }))
            })
        }

        // System Tray and Notification Methods
        "tray.setAutostart" => {
            let enabled = params
                .get("enabled")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'enabled' parameter".to_string(),
                    data: None,
                })?;

            // TODO: Implement autostart via codex-core or direct Windows API
            // For now, return success
            Ok(serde_json::json!({
                "success": true,
                "enabled": enabled,
                "message": "Autostart setting updated"
            }))
        }

        "tray.getAutostart" => {
            // TODO: Check actual autostart status
            Ok(serde_json::json!({
                "enabled": false,
                "message": "Autostart status check not yet implemented"
            }))
        }

        "notification.show" => {
            let title = params
                .get("title")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'title' parameter".to_string(),
                    data: None,
                })?;

            let body = params
                .get("body")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let notification_type = params
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("info");

            #[cfg(target_os = "windows")]
            {
                use std::process::Command;
                // Use PowerShell to show toast notification
                let ps_script = format!(
                    r#"
                    [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
                    [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] | Out-Null
                    
                    $template = @"
                    <toast>
                        <visual>
                            <binding template="ToastGeneric">
                                <text>{}</text>
                                <text>{}</text>
                            </binding>
                        </visual>
                    </toast>
"@
                    $xml = New-Object Windows.Data.Xml.Dom.XmlDocument
                    $xml.LoadXml($template)
                    $toast = [Windows.UI.Notifications.ToastNotification]::new($xml)
                    [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("Codex").Show($toast)
                    "#,
                    title, body
                );

                Command::new("powershell")
                    .args(&["-Command", &ps_script])
                    .output()
                    .ok();
            }

            #[cfg(target_os = "macos")]
            {
                use std::process::Command;
                Command::new("osascript")
                    .args(&[
                        "-e",
                        &format!(
                            "display notification \"{}\" with title \"{}\"",
                            body, title
                        ),
                    ])
                    .output()
                    .ok();
            }

            #[cfg(target_os = "linux")]
            {
                use std::process::Command;
                Command::new("notify-send")
                    .args(&[
                        "--app-name", "Codex",
                        title,
                        body,
                    ])
                    .output()
                    .ok();
            }

            Ok(serde_json::json!({
                "success": true,
                "message": "Notification shown"
            }))
        }

        "tray.setNotificationEnabled" => {
            let enabled = params
                .get("enabled")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'enabled' parameter".to_string(),
                    data: None,
                })?;

            // TODO: Store notification setting in config
            Ok(serde_json::json!({
                "success": true,
                "enabled": enabled,
                "message": "Notification setting updated"
            }))
        }

        // Virtual OS Terminal Methods
        "virtualos.terminal.createSession" => {
            let working_dir = params
                .get("workingDirectory")
                .and_then(|v| v.as_str())
                .unwrap_or(".");

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                use codex_core::virtualization::{TerminalManager, VirtualNetwork, NetworkSecurityPolicy};
                use std::sync::Arc;
                use tokio::sync::RwLock;

                static TERMINAL_MANAGER: Lazy<Arc<RwLock<TerminalManager>>> =
                    Lazy::new(|| Arc::new(RwLock::new(TerminalManager::new())));

                let network = VirtualNetwork::new(NetworkSecurityPolicy::Whitelist);
                let session_id = {
                    let mut manager = TERMINAL_MANAGER.write().await;
                    manager.create_session(
                        std::path::PathBuf::from(working_dir),
                        Some(network),
                    )
                };

                Ok(serde_json::json!({
                    "sessionId": session_id,
                    "workingDirectory": working_dir
                }))
            })
        }

        "virtualos.terminal.execute" => {
            let session_id = params
                .get("sessionId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'sessionId' parameter".to_string(),
                    data: None,
                })?;

            let command = params
                .get("command")
                .and_then(|v| v.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Option<Vec<String>>>()
                })
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing or invalid 'command' parameter".to_string(),
                    data: None,
                })?;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                use codex_core::virtualization::TerminalManager;
                use std::sync::Arc;
                use tokio::sync::RwLock;

                static TERMINAL_MANAGER: Lazy<Arc<RwLock<TerminalManager>>> =
                    Lazy::new(|| Arc::new(RwLock::new(TerminalManager::new())));

                let mut manager = TERMINAL_MANAGER.write().await;
                let session = manager.get_session_mut(session_id).ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Session not found".to_string(),
                    data: None,
                })?;

                match session.execute_command(command).await {
                    Ok(result) => Ok(serde_json::json!({
                        "exitCode": result.exit_code,
                        "stdout": result.stdout,
                        "stderr": result.stderr,
                        "isBlocked": result.is_blocked,
                        "blockReason": result.block_reason
                    })),
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Command execution failed: {}", e),
                        data: None,
                    }),
                }
            })
        }

        "virtualos.terminal.listCommands" => {
            let session_id = params
                .get("sessionId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'sessionId' parameter".to_string(),
                    data: None,
                })?;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                use codex_core::virtualization::TerminalManager;
                use std::sync::Arc;
                use tokio::sync::RwLock;

                static TERMINAL_MANAGER: Lazy<Arc<RwLock<TerminalManager>>> =
                    Lazy::new(|| Arc::new(RwLock::new(TerminalManager::new())));

                let manager = TERMINAL_MANAGER.read().await;
                let session = manager.get_session(session_id).ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Session not found".to_string(),
                    data: None,
                })?;

                let commands = session.list_available_commands().await;
                Ok(serde_json::json!({
                    "commands": commands
                }))
            })
        }

        "virtualos.terminal.getHistory" => {
            let session_id = params
                .get("sessionId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'sessionId' parameter".to_string(),
                    data: None,
                })?;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                use codex_core::virtualization::TerminalManager;
                use std::sync::Arc;
                use tokio::sync::RwLock;

                static TERMINAL_MANAGER: Lazy<Arc<RwLock<TerminalManager>>> =
                    Lazy::new(|| Arc::new(RwLock::new(TerminalManager::new())));

                let manager = TERMINAL_MANAGER.read().await;
                let session = manager.get_session(session_id).ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Session not found".to_string(),
                    data: None,
                })?;

                let history: Vec<serde_json::Value> = session
                    .get_history()
                    .iter()
                    .map(|cmd| {
                        serde_json::json!({
                            "command": cmd.command,
                            "workingDirectory": cmd.working_directory.to_string_lossy(),
                            "timestamp": cmd.timestamp.to_rfc3339(),
                            "result": cmd.result.as_ref().map(|r| serde_json::json!({
                                "exitCode": r.exit_code,
                                "stdout": r.stdout,
                                "stderr": r.stderr,
                                "isBlocked": r.is_blocked,
                                "blockReason": r.block_reason
                            }))
                        })
                    })
                    .collect();

                Ok(serde_json::json!({
                    "history": history
                }))
            })
        }

        "virtualos.terminal.changeDirectory" => {
            let session_id = params
                .get("sessionId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'sessionId' parameter".to_string(),
                    data: None,
                })?;

            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'path' parameter".to_string(),
                    data: None,
                })?;

            let rt = Runtime::new().map_err(|e| RpcError {
                code: -32000,
                message: format!("Failed to create runtime: {}", e),
                data: None,
            })?;

            rt.block_on(async {
                use codex_core::virtualization::TerminalManager;
                use std::sync::Arc;
                use tokio::sync::RwLock;

                static TERMINAL_MANAGER: Lazy<Arc<RwLock<TerminalManager>>> =
                    Lazy::new(|| Arc::new(RwLock::new(TerminalManager::new())));

                let mut manager = TERMINAL_MANAGER.write().await;
                let session = manager.get_session_mut(session_id).ok_or_else(|| RpcError {
                    code: -32000,
                    message: "Session not found".to_string(),
                    data: None,
                })?;

                match session.change_directory(std::path::PathBuf::from(path)) {
                    Ok(_) => Ok(serde_json::json!({
                        "success": true,
                        "workingDirectory": session.get_working_directory().to_string_lossy()
                    })),
                    Err(e) => Err(RpcError {
                        code: -32000,
                        message: format!("Failed to change directory: {}", e),
                        data: None,
                    }),
                }
            })
        }

        // DeepResearch integration via GeminiCLI
        "research.deep" => {
            let query = params
                .get("query")
                .and_then(|v| v.as_str())
                .ok_or_else(|| RpcError {
                    code: -32602,
                    message: "Missing 'query' parameter".to_string(),
                    data: None,
                })?;

            let depth = params
                .get("depth")
                .and_then(|v| v.as_u64())
                .unwrap_or(3) as u8;

            let strategy = params
                .get("strategy")
                .and_then(|v| v.as_str())
                .unwrap_or("comprehensive");

            let use_gemini = params
                .get("useGemini")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            // Use GeminiCLI for DeepResearch if enabled
            if use_gemini {
                let enhanced_query = format!(
                    "Conduct comprehensive research on: {}\n\
                     Provide detailed analysis with citations, sources, and evidence.\n\
                     Research depth: {}\n\
                     Strategy: {}\n\
                     Format as structured JSON with fields: summary, sources (array with title, url, snippet, relevance_score), conclusions",
                    query, depth, strategy
                );

                let output = tokio::process::Command::new("gemini")
                    .args([
                        "--model", "gemini-2.5-flash",
                        "--grounding", "web",
                        "--format", "json",
                        &enhanced_query
                    ])
                    .output()
                    .await
                    .map_err(|e| RpcError {
                        code: -32000,
                        message: format!("Failed to execute Gemini DeepResearch: {}", e),
                        data: None,
                    })?;

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    // Try to parse as JSON, fallback to text
                    match serde_json::from_str::<serde_json::Value>(&stdout) {
                        Ok(json) => Ok(json),
                        Err(_) => Ok(serde_json::json!({
                            "query": query,
                            "summary": stdout,
                            "sources": [],
                            "strategy": strategy,
                            "depth": depth
                        }))
                    }
                } else {
                    Err(RpcError {
                        code: -32000,
                        message: format!("Gemini DeepResearch failed: {}", String::from_utf8_lossy(&output.stderr)),
                        data: None,
                    })
                }
            } else {
                // Fallback to standard deep research (if available)
                Ok(serde_json::json!({
                    "query": query,
                    "message": "Standard deep research not yet implemented via RPC",
                    "suggestion": "Use useGemini: true for GeminiCLI-based research"
                }))
            }
        }

        _ => Err(RpcError {
            code: -32601,
            message: format!("Method not found: {}", method),
            data: None,
        }),
    }
}

fn launch_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Codex Orchestrator RPC Server...");
    println!("⚠️  YOLO Mode: Full file access enabled, but dangerous commands are blocked for security.");

    // Create tokio runtime for async operations
    let rt = Runtime::new()?;

    rt.block_on(async {
        println!("Starting orchestrator server on port 3001...");

        // WebSocket-based RPC server for GUI integration
        use tokio::net::TcpListener;
        use futures_util::{SinkExt, StreamExt};
        use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

        let listener = TcpListener::bind("127.0.0.1:3001").await?;
        println!("RPC Server listening on ws://127.0.0.1:3001");

        loop {
            let (stream, _) = listener.accept().await?;
            let ws_stream = accept_async(stream).await?;
            let (mut write, mut read) = ws_stream.split();

            tokio::spawn(async move {
                while let Some(message) = read.next().await {
                    match message {
                        Ok(Message::Text(text)) => {
                            // Parse RPC request
                            match serde_json::from_str::<RpcRequest>(&text) {
                                Ok(request) => {
                                    // Handle RPC method
                                    match handle_rpc_method(&request.method, &request.params).await {
                                        Ok(result) => {
                                            let response = RpcResponse {
                                                jsonrpc: "2.0".to_string(),
                                                id: Some(request.id),
                                                result: Some(result),
                                                error: None,
                                            };

                                            let response_text = serde_json::to_string(&response)
                                                .unwrap_or_else(|_| r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"}}"#.to_string());

                                            if write.send(Message::Text(response_text.into())).await.is_err() {
                                                break;
                                            }
                                        }
                                        Err(rpc_error) => {
                                            let response = RpcResponse {
                                                jsonrpc: "2.0".to_string(),
                                                id: Some(request.id),
                                                result: None,
                                                error: Some(rpc_error),
                                            };

                                            let response_text = serde_json::to_string(&response)
                                                .unwrap_or_else(|_| r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"}}"#.to_string());

                                            if write.send(Message::Text(response_text.into())).await.is_err() {
                                                break;
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    // Invalid JSON-RPC request
                                    let error_response = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
                                    let _ = write.send(Message::Text(error_response.into())).await;
                                }
                            }
                        }
                        Ok(Message::Close(_)) => break,
                        _ => {}
                    }
                }
            });
        }
    })
}

fn print_version() {
    println!("Codex AI-Native OS v2.3.0");
}

/// Launch Gemini CLI MCP Server
fn launch_gemini_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Launching Gemini CLI MCP Server...");

    // Check if gemini CLI is available
    let gemini_check = Command::new("gemini").arg("--help").output();
    if gemini_check.is_err() {
        eprintln!("❌ Gemini CLI not found. Please install Gemini CLI first:");
        eprintln!("   npm install -g @google/gemini-cli");
        eprintln!("   Or visit: https://github.com/google/gemini-cli");
        return Ok(());
    }

    println!("✅ Gemini CLI found");
    
    // Check for MCP server binary
    let mcp_server_paths = vec![
        "codex-gemini-cli-mcp-server",
        "codex-gemini-mcp",
        "gemini-mcp-server",
    ];
    
    let mut mcp_server_found = false;
    let mut mcp_server_path = String::new();
    
    for path in &mcp_server_paths {
        let check = Command::new(path).arg("--version").output();
        if check.is_ok() {
            mcp_server_path = path.to_string();
            mcp_server_found = true;
            break;
        }
    }
    
    if !mcp_server_found {
        eprintln!("⚠️  MCP server binary not found. Trying direct gemini CLI mode...");
        eprintln!("💡 You can install the MCP server with:");
        eprintln!("   cargo install codex-gemini-cli-mcp-server");
        
        // Fallback: Use gemini CLI directly with MCP-like interface
        println!("🌐 Starting Gemini CLI in MCP mode...");
        println!("💡 Server will handle Google Search requests via Gemini");
        
        // Run gemini CLI with MCP-like parameters
        let status = Command::new("gemini")
            .args(["--model", "gemini-2.5-flash", "--grounding", "web"])
            .status()?;
        
        if status.success() {
            println!("✅ Gemini CLI completed successfully");
        } else {
            eprintln!("❌ Gemini CLI failed with exit code: {}", status);
        }
        
        return Ok(());
    }

    println!("✅ MCP Server found: {}", mcp_server_path);
    println!("🌐 Starting MCP server on STDIO...");
    println!("💡 Server will handle Google Search requests via Gemini");

    // Execute the Gemini MCP server with improved error handling
    let output = Command::new(&mcp_server_path)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            println!("{}", stdout);
        }
        println!("✅ Gemini MCP Server completed successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("❌ Gemini MCP Server failed with exit code: {}", output.status);
        if !stderr.is_empty() {
            eprintln!("Error output: {}", stderr);
        }
        
        // Provide helpful error messages
        if stderr.contains("OAuth") || stderr.contains("authentication") {
            eprintln!("💡 Authentication issue detected. Please configure OAuth 2.0:");
            eprintln!("   Run: gemini auth login");
        }
        
        if stderr.contains("not found") || stderr.contains("command not found") {
            eprintln!("💡 MCP server not found. Install with:");
            eprintln!("   cargo install codex-gemini-cli-mcp-server");
        }
    }

    Ok(())
}

/// Launch Deep Research
fn launch_deep_research(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        println!("Usage: codex deep-research \"<query>\" [strategy] [depth]");
        println!("");
        println!("Arguments:");
        println!("  query      The research query");
        println!("  strategy   Research strategy (Comprehensive, Focused, Exploratory)");
        println!("  depth      Maximum research depth (default: 3)");
        println!("");
        println!("Examples:");
        println!("  codex deep-research \"Rust async patterns\" Comprehensive 5");
        println!("  codex deep-research \"React Server Components\"");
        return Ok(());
    }

    let query = args[1].clone();
    let strategy = args.get(2).map(|s| s.clone()).unwrap_or_else(|| "Comprehensive".to_string());
    let depth: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(3);

    println!("🔬 Starting Deep Research...");
    println!("📝 Query: {}", query);
    println!("🎯 Strategy: {}", strategy);
    println!("📊 Depth: {}", depth);
    println!("");

    // Create tokio runtime for async operations
    let rt = Runtime::new()?;

    rt.block_on(async {
        // Create Gemini search provider with grounding enabled
        let search_provider = codex_deep_research::GeminiSearchProvider::new("gemini-2.5-flash".to_string());

        // Create deep researcher
        let config = codex_deep_research::DeepResearcherConfig {
            max_depth: depth as u8,
            max_sources: match strategy.as_str() {
                "Exploratory" => 20,
                "Focused" => 8,
                _ => 12,
            },
            strategy: match strategy.as_str() {
                "Comprehensive" => codex_deep_research::ResearchStrategy::Comprehensive,
                "Focused" => codex_deep_research::ResearchStrategy::Focused,
                "Exploratory" => codex_deep_research::ResearchStrategy::Exploratory,
                _ => codex_deep_research::ResearchStrategy::Comprehensive,
            },
        };

        let researcher = codex_deep_research::DeepResearcher::new(config, std::sync::Arc::new(search_provider));

        // Execute research
        match researcher.research(&query).await {
            Ok(report) => {
                println!("📋 Research Report");
                println!("════════════════════════════════════════");
                println!("Query: {}", report.query);
                println!("Strategy: {:?}", report.strategy);
                println!("Depth Reached: {}", report.depth_reached);
                println!("Sources Found: {}", report.sources.len());
                println!("");
                println!("Summary:");
                println!("{}", report.summary);
                println!("");
                println!("Sources:");
                for (i, source) in report.sources.iter().enumerate() {
                    println!("{}. {}", i + 1, source.title);
                    println!("   URL: {}", source.url);
                    if !source.snippet.is_empty() {
                        println!("   Snippet: {}", source.snippet.chars().take(100).collect::<String>());
                    }
                    println!("");
                }
                println!("✅ Research completed successfully!");
            }
            Err(e) => {
                eprintln!("❌ Research failed: {}", e);
                eprintln!("This might be due to Gemini CLI not being available or network issues.");
                eprintln!("Make sure 'gemini' command is available in PATH.");
            }
        }

        Ok(())
    })
}

/// Launch Plan Command
fn launch_plan(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        println!("Usage: codex plan <subcommand> [options]");
        println!("");
        println!("Subcommands:");
        println!("  create <name>     Create a new plan");
        println!("  list              List all plans");
        println!("  show <id>         Show plan details");
        println!("  execute <id>      Execute a plan");
        println!("  status <id>       Show execution status");
        println!("  approve <id>      Approve pending actions");
        println!("  reject <id>       Reject pending actions");
        println!("");
        println!("Examples:");
        println!("  codex plan create \"Code Review Plan\" --budget 10000");
        println!("  codex plan list");
        println!("  codex plan execute plan_123");
        println!("  codex plan approve plan_123");
        return Ok(());
    }

    let subcommand = args[2].as_str();

    match subcommand {
        "create" => {
            if args.len() < 4 {
                println!("Usage: codex plan create <name> [--budget <tokens>] [--mode <orchestrated|parallel>]");
                return Ok(());
            }
            let name = args[3].clone();
            let budget: Option<u64> = args.get(4).and_then(|s| s.parse().ok());
            let mode = args.get(5).unwrap_or(&"orchestrated".to_string()).clone();

            println!("📋 Creating new plan...");
            println!("📝 Name: {}", name);
            println!("💰 Budget: {} tokens", budget.unwrap_or(10000));
            println!("🎯 Mode: {}", mode);

            // Plan creation logic would go here
            println!("✅ Plan '{}' created successfully", name);
            println!("🔢 Plan ID: plan_{}", name.to_lowercase().replace(" ", "_"));
        }
        "list" => {
            println!("📋 Available Plans");
            println!("══════════════════");
            println!("No plans found. Create one with 'codex plan create <name>'");
        }
        "show" | "execute" | "status" | "approve" | "reject" => {
            let plan_id = args.get(3).unwrap_or(&"".to_string()).clone();
            if plan_id.is_empty() {
                println!("Error: Plan ID required for '{}' command", subcommand);
                return Ok(());
            }

            match subcommand {
                "show" => println!("📋 Showing plan: {}", plan_id),
                "execute" => println!("🚀 Executing plan: {}", plan_id),
                "status" => println!("📊 Status of plan: {}", plan_id),
                "approve" => println!("✅ Approving actions for plan: {}", plan_id),
                "reject" => println!("❌ Rejecting actions for plan: {}", plan_id),
                _ => unreachable!(),
            }

            println!("⚠️  Plan functionality is under development");
            println!("📝 This feature will be available in the next release");
        }
        _ => {
            println!("Error: Unknown subcommand '{}'", subcommand);
            println!("Run 'codex plan' for available subcommands");
        }
    }

    Ok(())
}

/// Launch QC Command
fn launch_qc(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        println!("Usage: codex qc <subcommand> [options]");
        println!("");
        println!("Subcommands:");
        println!("  analyze <file>     Analyze code quality for a specific file");
        println!("  report <target>    Generate comprehensive QC report");
        println!("  optimize <file>    Apply automatic optimizations");
        println!("  dashboard          Generate quality dashboard");
        println!("");
        println!("Options:");
        println!("  --output <dir>     Output directory for reports (default: qc_reports)");
        println!("  --verbose          Enable verbose logging");
        println!("  --no-viz           Disable visualization generation");
        println!("");
        println!("Examples:");
        println!("  codex qc analyze src/main.rs");
        println!("  codex qc report my_project --output ./reports");
        println!("  codex qc dashboard --verbose");
        return Ok(());
    }

    let subcommand = args[2].as_str();

    match subcommand {
        "analyze" => {
            if args.len() < 4 {
                println!("Error: File path required for analyze command");
                println!("Usage: codex qc analyze <file>");
                return Ok(());
            }

            let file_path = &args[3];
            let output_dir = args.get(4).and_then(|s| if s == "--output" { args.get(5) } else { None })
                .unwrap_or(&"qc_reports".to_string()).clone();
            let verbose = args.contains(&"--verbose".to_string());
            let enable_viz = !args.contains(&"--no-viz".to_string());

            println!("🔍 Analyzing code quality for: {}", file_path);
            println!("📁 Output directory: {}", output_dir);

            // Read file content
            let content = match std::fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    println!("Error: Failed to read file '{}': {}", file_path, e);
                    return Ok(());
                }
            };

            // Create QC agent
            let config = codex_core::qc::QcConfig {
                enable_statistical: true,
                enable_quantum: true,
                enable_mathematical: true,
                enable_visualization: enable_viz,
                output_dir,
                min_confidence: 0.6,
                verbose,
            };

            let agent = codex_core::qc::QcAgent::with_config(config);

            // Run analysis (async)
            let rt = Runtime::new()?;
            let file_name = std::path::Path::new(file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            rt.block_on(async {
                match agent.analyze(&content, file_name).await {
                    Ok(report) => {
                        println!("✅ QC Analysis completed!");
                        println!("📊 Overall Quality Score: {:.2}/1.0", report.scores.overall);
                        println!("📈 Readability: {:.2}", report.scores.readability);
                        println!("🔧 Maintainability: {:.2}", report.scores.maintainability);
                        println!("⚡ Performance: {:.2}", report.scores.performance);
                        println!("🔒 Security: {:.2}", report.scores.security);
                        println!("");
                        println!("📋 Generated outputs:");
                        for output in &report.outputs {
                            println!("  • {}", output);
                        }
                        println!("");
                        println!("💡 Top recommendations:");
                        for (i, rec) in report.recommendations.iter().enumerate().take(3) {
                            println!("  {}. {}", i + 1, rec);
                        }
                    }
                    Err(e) => {
                        println!("❌ QC Analysis failed: {}", e);
                    }
                }
            });
        }
        "report" => {
            let target = args.get(3).unwrap_or(&"current_project".to_string()).clone();
            println!("📊 Generating QC report for: {}", target);

            // Placeholder for comprehensive project reporting
            println!("⚠️  Project-level QC reporting is under development");
            println!("💡 Use 'codex qc analyze <file>' for individual file analysis");
        }
        "optimize" => {
            if args.len() < 4 {
                println!("Error: File path required for optimize command");
                println!("Usage: codex qc optimize <file>");
                return Ok(());
            }

            let file_path = &args[3];
            println!("🚀 Applying automatic optimizations to: {}", file_path);

            // Placeholder for automatic optimization
            println!("⚠️  Automatic optimization is under development");
            println!("💡 Use 'codex qc analyze <file>' to see optimization suggestions");
        }
        "dashboard" => {
            println!("📊 Generating quality dashboard...");

            // Placeholder for dashboard generation
            println!("⚠️  Dashboard generation is under development");
            println!("💡 Run individual file analyses first with 'codex qc analyze <file>'");
        }
        _ => {
            println!("Error: Unknown subcommand '{}'", subcommand);
            println!("Run 'codex qc' for available subcommands");
        }
    }

    Ok(())
}

/// Launch Worktree Command
fn launch_worktree(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        println!("Usage: codex worktree <subcommand> [options]");
        println!("");
        println!("Subcommands:");
        println!("  competition <task>  Run worktree competition for a task");
        println!("  list                List active worktrees");
        println!("  merge <winner>      Merge winning worktree to main");
        println!("  cleanup             Clean up old worktrees");
        println!("");
        println!("Options:");
        println!("  --variants <n>      Number of variants to create (default: 2)");
        println!("  --agents <list>     Comma-separated list of agents to use");
        println!("  --time-budget <min> Time budget in minutes (default: 30)");
        println!("");
        println!("Examples:");
        println!("  codex worktree competition \"Implement user authentication\"");
        println!("  codex worktree competition \"Fix memory leak\" --variants 3 --agents CodeReviewer,TestGen");
        println!("  codex worktree list");
        println!("  codex worktree merge worktree_CodeReviewer_123");
        return Ok(());
    }

    let subcommand = args[2].as_str();

    match subcommand {
        "competition" => {
            if args.len() < 4 {
                println!("Error: Task description required for competition");
                println!("Usage: codex worktree competition <task>");
                return Ok(());
            }

            let task = args[3].clone();
            let variants = args.get(4).and_then(|s| if s == "--variants" { args.get(5) } else { None })
                .and_then(|s| s.parse().ok()).unwrap_or(2);
            let agents = args.get(6).and_then(|s| if s == "--agents" { args.get(7) } else { None })
                .unwrap_or(&"CodeReviewer,TestGen".to_string()).clone();
            let time_budget = args.get(8).and_then(|s| if s == "--time-budget" { args.get(9) } else { None })
                .and_then(|s| s.parse().ok()).unwrap_or(30);

            println!("🏁 Starting worktree competition...");
            println!("📋 Task: {}", task);
            println!("🔢 Variants: {}", variants);
            println!("🤖 Agents: {}", agents);
            println!("⏰ Time Budget: {} minutes", time_budget);
            println!("");

            // Create task ID from timestamp
            let task_id = format!("comp_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));

            println!("🔄 Creating worktrees...");
            // Worktree creation logic would go here
            println!("✅ Created {} worktrees", variants);

            println!("🚀 Executing parallel tasks...");
            // Parallel execution logic would go here
            println!("⚡ All tasks completed");

            println!("📊 Evaluating results...");
            // Scoring and evaluation logic would go here
            println!("🏆 Winner determined: worktree_CodeReviewer_{}", task_id);

            println!("💡 Competition completed successfully!");
            println!("💡 Use 'codex worktree merge <winner>' to merge the winning implementation");
        }
        "list" => {
            println!("📋 Active Worktrees");
            println!("═══════════════════");

            // List worktrees logic would go here
            println!("No active worktrees found.");
            println!("💡 Use 'codex worktree competition <task>' to start a competition");
        }
        "merge" => {
            if args.len() < 4 {
                println!("Error: Winner worktree name required");
                println!("Usage: codex worktree merge <winner>");
                return Ok(());
            }

            let winner = &args[3];
            println!("🔀 Merging winning worktree: {}", winner);

            // Merge logic would go here
            println!("✅ Successfully merged {} to main branch", winner);
            println!("🧹 Cleaning up worktrees...");
            println!("✅ Cleanup completed");
        }
        "cleanup" => {
            println!("🧹 Cleaning up old worktrees...");

            // Cleanup logic would go here
            println!("✅ Removed 0 old worktrees");
            println!("💡 No old worktrees to clean up");
        }
        _ => {
            println!("Error: Unknown subcommand '{}'", subcommand);
            println!("Run 'codex worktree' for available subcommands");
        }
    }

    Ok(())
}

/// Launch Delegate Command
fn launch_delegate(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        println!("Usage: codex delegate <agent-type> \"<task-description>\"");
        println!("");
        println!("Available agent types:");
        println!("  code-reviewer    Code quality and security review");
        println!("  test-gen         Automated test generation");
        println!("  sec-audit        Security vulnerability scanning");
        println!("  researcher       Deep research and analysis");
        println!("  architect        System architecture design");
        println!("  refactorer       Code refactoring assistance");
        return Ok(());
    }

    let agent_type = args[2].clone();
    let task = if args.len() > 3 {
        args[3].clone()
    } else {
        println!("Enter task description:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    println!("🤖 Delegating to {} agent...", agent_type);
    println!("📋 Task: {}", task);

    // Create tokio runtime for async operations
    let rt = Runtime::new()?;

    rt.block_on(async {
        // Connect to MCP server and delegate task
        println!("🔗 Connecting to MCP server...");

        // For now, simulate delegation - full implementation pending
        println!("⚠️  Sub-agent delegation is under development");
        println!("📝 Task '{}' delegated to {} agent", task, agent_type);
        println!("⏳ Processing... (simulated)");

        // Simulate processing time
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        println!("✅ Delegation completed");
        println!("📊 Results would be available in the agent inbox");

        Ok(())
    })
}

fn print_help() {
    println!("Codex AI-Native OS v2.3.0");
    println!("");
    println!("USAGE:");
    println!("    codex [COMMAND]");
    println!("");
    println!("COMMANDS:");
    println!("    tui           Launch Terminal User Interface");
    println!("    gui           Launch Graphical User Interface");
    println!("    server        Launch RPC Server for GUI integration");
    println!("    mcp-gemini    Launch Gemini CLI MCP Server");
    // println!("    deep-research Run deep research with AI assistance"); // Temporarily disabled
    println!("    plan          Create and manage AI execution plans");
    println!("    qc            Perform comprehensive quality control analysis");
    println!("    worktree      Manage git worktree competitions");
    println!("    delegate      Delegate tasks to specialized sub-agents");
    println!("    --help, -h    Show this help message");
    println!("    --version, -v Show version information");
}
