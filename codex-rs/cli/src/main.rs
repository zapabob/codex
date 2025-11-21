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
        "mcp-gemini" => launch_gemini_mcp_server(),
        "deep-research" => launch_deep_research(&args),
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

/// Launch Gemini CLI MCP Server
fn launch_gemini_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Launching Gemini CLI MCP Server...");

    // Check if gemini CLI is available
    let gemini_check = Command::new("gemini").arg("--help").output();
    if gemini_check.is_err() {
        eprintln!("❌ Gemini CLI not found. Please install Gemini CLI first:");
        eprintln!("   npm install -g @google/gemini-cli");
        return Ok(());
    }

    println!("✅ Gemini CLI found");
    println!("🌐 Starting MCP server on STDIO...");
    println!("💡 Server will handle Google Search requests via Gemini");

    // Execute the Gemini MCP server
    let status = Command::new("codex-gemini-cli-mcp-server")
        .status()?;

    if status.success() {
        println!("✅ Gemini MCP Server completed successfully");
    } else {
        eprintln!("❌ Gemini MCP Server failed with exit code: {}", status);
    }

    Ok(())
}

/// Launch Deep Research
fn launch_deep_research(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        println!("Usage: codex deep-research \"<query>\" [strategy] [depth]");
        println!("");
        println!("Arguments:");
        println!("  query    Research query");
        println!("  strategy Comprehensive, Focused, or Exploratory (default: Comprehensive)");
        println!("  depth    Research depth 1-5 (default: 3)");
        println!("");
        println!("Examples:");
        println!("  codex deep-research \"Rust async patterns\"");
        println!("  codex deep-research \"React Server Components\" Focused 4");
        println!("  codex deep-research \"Modern web frameworks\" Exploratory 2");
        return Ok(());
    }

    let query = args[2].clone();
    let strategy = args.get(3).unwrap_or(&"Comprehensive".to_string()).clone();
    let depth: usize = args.get(4)
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);

    println!("🔬 Starting Deep Research...");
    println!("📝 Query: {}", query);
    println!("🎯 Strategy: {}", strategy);
    println!("📊 Depth: {}", depth);
    println!("");

    // Create tokio runtime for async operations
    let rt = Runtime::new()?;

    rt.block_on(async {
        use codex_deep_research::*;
        use std::sync::Arc;

        let config = DeepResearcherConfig {
            max_depth: depth,
            max_sources: match strategy.as_str() {
                "Exploratory" => 20,
                "Focused" => 8,
                _ => 12,
            },
            strategy: match strategy.as_str() {
                "Comprehensive" => ResearchStrategy::Comprehensive,
                "Focused" => ResearchStrategy::Focused,
                "Exploratory" => ResearchStrategy::Exploratory,
                _ => ResearchStrategy::Comprehensive,
            },
        };

        // Try Gemini provider first, fallback to MCP
        let provider: Arc<dyn ResearchProvider> = if std::env::var("GEMINI_API_KEY").is_ok() {
            println!("🤖 Using Gemini Search Provider");
            Arc::new(GeminiSearchProvider::new()?)
        } else {
            println!("🔍 Using MCP Search Provider");
            Arc::new(McpSearchProvider::new(SearchBackend::Google)?)
        };

        let researcher = DeepResearcher::new(config, provider);
        let report = researcher.research(&query).await?;

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
            if let Some(desc) = &source.description {
                println!("   Description: {}", desc);
            }
            println!("");
        }

        Ok(())
    })
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
    println!("    delegate      Delegate tasks to specialized sub-agents");
    println!("    --help, -h    Show this help message");
    println!("    --version, -v Show version information");
}
