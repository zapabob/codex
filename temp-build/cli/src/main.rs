//! Codex CLI - AI-Native OS Command Line Interface

use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::process::Command;
use tokio::runtime::Runtime;

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
            // Note: In production, this should have proper security checks
            Ok(serde_json::json!({
                "exitCode": 0,
                "stdout": "Command executed successfully (mock)",
                "stderr": ""
            }))
        }

        "fs.search" => Ok(serde_json::json!([])),

        _ => Err(RpcError {
            code: -32601,
            message: format!("Method not found: {}", method),
            data: None,
        }),
    }
}

fn launch_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Codex Orchestrator RPC Server...");

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

fn print_help() {
    println!("Codex AI-Native OS v2.3.0");
    println!("");
    println!("USAGE:");
    println!("    codex [COMMAND]");
    println!("");
    println!("COMMANDS:");
    println!("    tui         Launch Terminal User Interface");
    println!("    gui         Launch Graphical User Interface");
    println!("    server      Launch RPC Server for GUI integration");
    println!("    --help, -h  Show this help message");
    println!("    --version, -v Show version information");
}
