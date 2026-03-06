use anyhow::Context;
use anyhow::Result;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use codex_core::chrome::ChromeConstraints;
use codex_core::chrome::ChromeNlRequest;
use codex_core::chrome::ChromeNlResponse;
use codex_core::chrome::ChromeOrigin;
use codex_core::chrome::parse_nl_command;
use codex_rmcp_client::RmcpClient;
use futures::FutureExt;
use rmcp::model as mcp_model;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::process::ChildStdin;
use tokio::process::ChildStdout;
use tokio::time::timeout;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct ChromeCli {
    #[command(subcommand)]
    pub subcommand: ChromeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ChromeSubcommand {
    /// Parse natural language into a structured browser intent
    Parse(ChromeParseArgs),
    /// Run deep research query
    Research(ChromeResearchArgs),
    /// Read DOM from active tab
    Dom(ChromeDomArgs),
    /// Get console logs from active tab
    Console(ChromeConsoleArgs),
    /// Monitor network requests from active tab
    Network(ChromeNetworkArgs),
}

#[derive(Debug, Args)]
pub struct ChromeParseArgs {
    /// Read JSON request from stdin
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Natural language instruction
    #[arg(long)]
    pub utterance: Option<String>,

    /// Origin URL for domain safeguards
    #[arg(long)]
    pub url: Option<String>,

    /// Allowed intents (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub allowed_intents: Vec<String>,

    /// Intents that must require confirmation (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub require_confirmation_for: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ChromeResearchArgs {
    /// Research query
    pub query: String,

    /// Research depth (default: 3)
    #[arg(long, default_value_t = 3)]
    pub depth: u8,

    /// Research breadth (default: 10)
    #[arg(long, default_value_t = 10)]
    pub breadth: u8,
}

#[derive(Debug, Args)]
pub struct ChromeDomArgs {
    /// CSS selector to read (optional, reads entire page if not specified)
    #[arg(long)]
    pub selector: Option<String>,

    /// Maximum characters to read (default: 5000)
    #[arg(long, default_value_t = 5000)]
    pub max_chars: usize,
}

#[derive(Debug, Args)]
pub struct ChromeConsoleArgs {
    /// Filter logs by level (log, warn, error, info, debug)
    #[arg(long)]
    pub level: Option<String>,

    /// Filter logs by message content
    #[arg(long)]
    pub filter: Option<String>,

    /// Maximum number of logs to retrieve (default: 50)
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct ChromeNetworkArgs {
    /// Filter requests by URL pattern
    #[arg(long)]
    pub filter: Option<String>,

    /// Maximum number of requests to retrieve (default: 50)
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
}

#[derive(Debug, Deserialize)]
struct ChromeParseRequest {
    pub id: Option<String>,
    pub utterance: String,
    pub origin: Option<ChromeOrigin>,
    pub constraints: Option<ChromeConstraints>,
}

#[derive(Debug, Serialize)]
struct ChromeParseResponse {
    pub r#type: String,
    pub id: Option<String>,
    pub success: bool,
    pub data: Option<ChromeNlResponse>,
    pub error: Option<String>,
}

pub async fn run_chrome_command(cli: ChromeCli) -> Result<()> {
    match cli.subcommand {
        ChromeSubcommand::Parse(args) => run_parse(args),
        ChromeSubcommand::Research(args) => run_research(args).await,
        ChromeSubcommand::Dom(args) => run_dom(args).await,
        ChromeSubcommand::Console(args) => run_console(args).await,
        ChromeSubcommand::Network(args) => run_network(args).await,
    }
}

fn run_parse(args: ChromeParseArgs) -> Result<()> {
    let request = if args.json {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read stdin")?;
        serde_json::from_str::<ChromeParseRequest>(&buffer).context("Invalid JSON request")?
    } else {
        let utterance = args
            .utterance
            .clone()
            .context("--utterance is required unless --json is set")?;
        let constraints =
            if args.allowed_intents.is_empty() && args.require_confirmation_for.is_empty() {
                None
            } else {
                Some(ChromeConstraints {
                    allowed_intents: if args.allowed_intents.is_empty() {
                        None
                    } else {
                        Some(args.allowed_intents.clone())
                    },
                    require_confirmation_for: if args.require_confirmation_for.is_empty() {
                        None
                    } else {
                        Some(args.require_confirmation_for.clone())
                    },
                })
            };

        ChromeParseRequest {
            id: None,
            utterance,
            origin: args.url.map(|url| ChromeOrigin {
                tab_id: None,
                frame_id: None,
                url: Some(url),
            }),
            constraints,
        }
    };

    let response: ChromeParseResponse = match parse_nl_command(ChromeNlRequest {
        utterance: request.utterance,
        origin: request.origin,
        constraints: request.constraints,
    }) {
        Ok(intent) => ChromeParseResponse {
            r#type: "nl_command.response".to_string(),
            id: request.id,
            success: true,
            data: Some(intent),
            error: None,
        },
        Err(error) => ChromeParseResponse {
            r#type: "nl_command.response".to_string(),
            id: request.id,
            success: false,
            data: None,
            error: Some(error.to_string()),
        },
    };

    let output = serde_json::to_string_pretty(&response).context("Failed to serialize response")?;
    println!("{output}");
    Ok(())
}

async fn run_research(args: ChromeResearchArgs) -> Result<()> {
    use crate::research_cmd::run_research_command;

    run_research_command(
        args.query,
        args.depth,
        args.breadth,
        100_000, // budget
        false,   // citations
        None,    // mcp_url
        false,   // lightweight_fallback
        None,    // out
        false,   // use_gemini
        false,   // use_mcp
    )
    .await
}

async fn run_dom(args: ChromeDomArgs) -> Result<()> {
    // Try MCP bridge first, fall back to native messaging host
    if let Ok(result) = run_dom_via_mcp(&args).await {
        return result;
    }

    // Fallback to native messaging host
    let (mut stdin, mut stdout) = spawn_native_host()
        .await
        .context("Failed to start native messaging host. Please ensure codex-chrome-host is built and available in PATH or target/release directory")?;

    let message_id = Uuid::new_v4().to_string();
    let message = serde_json::json!({
        "version": "1.0",
        "id": message_id,
        "type": "dom.read.request",
        "origin": {},
        "payload": {
            "selector": args.selector,
            "max_chars": args.max_chars,
        }
    });

    send_message_to_host(&mut stdin, &message)
        .await
        .context("Failed to send message to native messaging host")?;

    let response = timeout(
        Duration::from_secs(30),
        receive_message_from_host(&mut stdout)
    )
    .await
    .context("Request timed out after 30 seconds. The extension may not be connected or the request may be taking too long")?
    .context("Failed to receive response from native messaging host")?;

    if let Some(success) = response.get("success").and_then(|s| s.as_bool()) {
        if success {
            if let Some(data) = response.get("data") {
                println!("{}", serde_json::to_string_pretty(data)?);
            } else {
                println!("DOM read successful (no data returned)");
            }
        } else {
            let error = response
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!(
                "DOM read failed: {}. Note: DOM reading requires the Chrome extension to be active and connected to the native messaging host.",
                error
            );
        }
    } else {
        anyhow::bail!("Invalid response format from native messaging host");
    }

    Ok(())
}

async fn run_dom_via_mcp(args: &ChromeDomArgs) -> Result<Result<()>> {
    let bridge_path = find_mcp_bridge_binary()?;

    let client =
        RmcpClient::new_stdio_client(OsString::from(&bridge_path), vec![], None, &[], None)
            .await
            .context("Failed to create MCP client")?;

    let init_params = mcp_model::InitializeRequestParams {
        meta: None,
        protocol_version: mcp_model::ProtocolVersion::V_2025_06_18,
        capabilities: mcp_model::ClientCapabilities {
            experimental: None,
            extensions: None,
            roots: None,
            sampling: None,
            elicitation: None,
            tasks: None,
        },
        client_info: mcp_model::Implementation {
            name: "codex-cli".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: None,
            icons: None,
            title: None,
            website_url: None,
        },
    };

    let send_elicitation: codex_rmcp_client::SendElicitation = Box::new(
        |_request_id: mcp_model::NumberOrString,
         _elicitation: mcp_model::CreateElicitationRequestParams| {
            async move { anyhow::bail!("Elicitation not supported") }.boxed()
        },
    );

    client
        .initialize(init_params, Some(Duration::from_secs(10)), send_elicitation)
        .await
        .context("Failed to initialize MCP client")?;

    let params = serde_json::json!({
        "selector": args.selector,
        "max_chars": args.max_chars,
    });

    let result = client
        .call_tool(
            "dom_read".to_string(),
            Some(params),
            Some(Duration::from_secs(30)),
        )
        .await
        .context("Failed to call dom_read tool")?;

    if let Some(content) = result.content.first() {
        if let mcp_model::RawContent::Text(text) = &content.raw {
            println!("{}", text.text);
        } else {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    } else {
        println!("DOM read completed (no content returned)");
    }

    Ok(Ok(()))
}

async fn run_console(args: ChromeConsoleArgs) -> Result<()> {
    // Try MCP bridge first, fall back to native messaging host
    if let Ok(result) = run_console_via_mcp(&args).await {
        return result;
    }

    // Fallback to native messaging host
    let (mut stdin, mut stdout) = spawn_native_host()
        .await
        .context("Failed to start native messaging host. Please ensure codex-chrome-host is built and available in PATH or target/release directory")?;

    let message_id = Uuid::new_v4().to_string();
    let message = serde_json::json!({
        "version": "1.0",
        "id": message_id,
        "type": "console.get_logs.request",
        "origin": {},
        "payload": {
            "level": args.level,
            "filter": args.filter,
            "limit": args.limit,
        }
    });

    send_message_to_host(&mut stdin, &message)
        .await
        .context("Failed to send message to native messaging host")?;

    let response = timeout(
        Duration::from_secs(30),
        receive_message_from_host(&mut stdout)
    )
    .await
    .context("Request timed out after 30 seconds. The extension may not be connected or the request may be taking too long")?
    .context("Failed to receive response from native messaging host")?;

    if let Some(success) = response.get("success").and_then(|s| s.as_bool()) {
        if success {
            if let Some(data) = response.get("data") {
                println!("{}", serde_json::to_string_pretty(data)?);
            } else {
                println!("Console logs retrieved (no data returned)");
            }
        } else {
            let error = response
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!(
                "Console log retrieval failed: {}. Note: Console log retrieval requires the Chrome extension to be active and connected to the native messaging host.",
                error
            );
        }
    } else {
        anyhow::bail!("Invalid response format from native messaging host");
    }

    Ok(())
}

async fn run_console_via_mcp(args: &ChromeConsoleArgs) -> Result<Result<()>> {
    let bridge_path = find_mcp_bridge_binary()?;

    let client =
        RmcpClient::new_stdio_client(OsString::from(&bridge_path), vec![], None, &[], None)
            .await
            .context("Failed to create MCP client")?;

    let init_params = mcp_model::InitializeRequestParams {
        meta: None,
        protocol_version: mcp_model::ProtocolVersion::V_2025_06_18,
        capabilities: mcp_model::ClientCapabilities {
            experimental: None,
            extensions: None,
            roots: None,
            sampling: None,
            elicitation: None,
            tasks: None,
        },
        client_info: mcp_model::Implementation {
            name: "codex-cli".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: None,
            icons: None,
            title: None,
            website_url: None,
        },
    };

    let send_elicitation: codex_rmcp_client::SendElicitation = Box::new(
        |_request_id: mcp_model::NumberOrString,
         _elicitation: mcp_model::CreateElicitationRequestParams| {
            async move { anyhow::bail!("Elicitation not supported") }.boxed()
        },
    );

    client
        .initialize(init_params, Some(Duration::from_secs(10)), send_elicitation)
        .await
        .context("Failed to initialize MCP client")?;

    let params = serde_json::json!({
        "level": args.level,
        "filter": args.filter,
        "limit": args.limit,
    });

    let result = client
        .call_tool(
            "console_get_logs".to_string(),
            Some(params),
            Some(Duration::from_secs(30)),
        )
        .await
        .context("Failed to call console_get_logs tool")?;

    if let Some(content) = result.content.first() {
        if let mcp_model::RawContent::Text(text) = &content.raw {
            println!("{}", text.text);
        } else {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    } else {
        println!("Console logs retrieved (no content returned)");
    }

    Ok(Ok(()))
}

async fn run_network(args: ChromeNetworkArgs) -> Result<()> {
    // Try MCP bridge first, fall back to native messaging host
    if let Ok(result) = run_network_via_mcp(&args).await {
        return result;
    }

    // Fallback to native messaging host
    let (mut stdin, mut stdout) = spawn_native_host()
        .await
        .context("Failed to start native messaging host. Please ensure codex-chrome-host is built and available in PATH or target/release directory")?;

    let message_id = Uuid::new_v4().to_string();
    let message = serde_json::json!({
        "version": "1.0",
        "id": message_id,
        "type": "network.get_logs.request",
        "origin": {},
        "payload": {
            "filter": args.filter,
            "limit": args.limit,
        }
    });

    send_message_to_host(&mut stdin, &message)
        .await
        .context("Failed to send message to native messaging host")?;

    let response = timeout(
        Duration::from_secs(30),
        receive_message_from_host(&mut stdout)
    )
    .await
    .context("Request timed out after 30 seconds. The extension may not be connected or the request may be taking too long")?
    .context("Failed to receive response from native messaging host")?;

    if let Some(success) = response.get("success").and_then(|s| s.as_bool()) {
        if success {
            if let Some(data) = response.get("data") {
                println!("{}", serde_json::to_string_pretty(data)?);
            } else {
                println!("Network logs retrieved (no data returned)");
            }
        } else {
            let error = response
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown error");
            anyhow::bail!(
                "Network log retrieval failed: {}. Note: Network log retrieval requires the Chrome extension to be active and connected to the native messaging host.",
                error
            );
        }
    } else {
        anyhow::bail!("Invalid response format from native messaging host");
    }

    Ok(())
}

async fn run_network_via_mcp(args: &ChromeNetworkArgs) -> Result<Result<()>> {
    let bridge_path = find_mcp_bridge_binary()?;

    let client =
        RmcpClient::new_stdio_client(OsString::from(&bridge_path), vec![], None, &[], None)
            .await
            .context("Failed to create MCP client")?;

    let init_params = mcp_model::InitializeRequestParams {
        meta: None,
        protocol_version: mcp_model::ProtocolVersion::V_2025_06_18,
        capabilities: mcp_model::ClientCapabilities {
            experimental: None,
            extensions: None,
            roots: None,
            sampling: None,
            elicitation: None,
            tasks: None,
        },
        client_info: mcp_model::Implementation {
            name: "codex-cli".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: None,
            icons: None,
            title: None,
            website_url: None,
        },
    };

    let send_elicitation: codex_rmcp_client::SendElicitation = Box::new(
        |_request_id: mcp_model::NumberOrString,
         _elicitation: mcp_model::CreateElicitationRequestParams| {
            async move { anyhow::bail!("Elicitation not supported") }.boxed()
        },
    );

    client
        .initialize(init_params, Some(Duration::from_secs(10)), send_elicitation)
        .await
        .context("Failed to initialize MCP client")?;

    let params = serde_json::json!({
        "filter": args.filter,
        "limit": args.limit,
    });

    let result = client
        .call_tool(
            "network_get_logs".to_string(),
            Some(params),
            Some(Duration::from_secs(30)),
        )
        .await
        .context("Failed to call network_get_logs tool")?;

    if let Some(content) = result.content.first() {
        if let mcp_model::RawContent::Text(text) = &content.raw {
            println!("{}", text.text);
        } else {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    } else {
        println!("Network logs retrieved (no content returned)");
    }

    Ok(Ok(()))
}

/// Find the MCP bridge binary path
fn find_mcp_bridge_binary() -> Result<PathBuf> {
    // Try to find in target/release directory (development)
    let current_exe = std::env::current_exe()?;
    let workspace_root = current_exe
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.join("codex-rs/target/release"));

    if let Some(release_dir) = workspace_root {
        #[cfg(target_os = "windows")]
        let binary_name = "codex-chrome-mcp-bridge.exe";
        #[cfg(not(target_os = "windows"))]
        let binary_name = "codex-chrome-mcp-bridge";

        let binary_path = release_dir.join(binary_name);
        if binary_path.exists() {
            return Ok(binary_path);
        }
    }

    // Try to find in PATH
    #[cfg(target_os = "windows")]
    let binary_name = "codex-chrome-mcp-bridge.exe";
    #[cfg(not(target_os = "windows"))]
    let binary_name = "codex-chrome-mcp-bridge";

    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(std::path::MAIN_SEPARATOR) {
            let candidate = PathBuf::from(dir).join(binary_name);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    anyhow::bail!(
        "MCP bridge binary not found. Please build it with: cargo build -p codex-chrome-mcp-bridge --release"
    )
}

/// Find the native messaging host binary path
fn find_native_host_binary() -> Result<PathBuf> {
    // Try to find in target/release directory (development)
    let current_exe = std::env::current_exe()?;
    let workspace_root = current_exe
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.join("codex-rs/target/release"));

    if let Some(release_dir) = workspace_root {
        #[cfg(target_os = "windows")]
        let binary_name = "codex-chrome-host.exe";
        #[cfg(not(target_os = "windows"))]
        let binary_name = "codex-chrome-host";

        let binary_path = release_dir.join(binary_name);
        if binary_path.exists() {
            return Ok(binary_path);
        }
    }

    // Try to find in PATH
    #[cfg(target_os = "windows")]
    let binary_name = "codex-chrome-host.exe";
    #[cfg(not(target_os = "windows"))]
    let binary_name = "codex-chrome-host";

    // Use which crate if available, otherwise try direct execution
    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(std::path::MAIN_SEPARATOR) {
            let candidate = PathBuf::from(dir).join(binary_name);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    anyhow::bail!(
        "Native messaging host binary not found. Please build it with: cargo build -p codex-chrome-host --release"
    )
}

/// Spawn the native messaging host process and return stdin/stdout handles
async fn spawn_native_host() -> Result<(ChildStdin, ChildStdout)> {
    let binary_path = find_native_host_binary()
        .context("Native messaging host binary not found. Please build it with: cargo build -p codex-chrome-host --release")?;

    let mut child = tokio::process::Command::new(&binary_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!(
                "Failed to spawn native messaging host from: {}",
                binary_path.display()
            )
        })?;

    let stdin = child
        .stdin
        .take()
        .context("Failed to take stdin from child process")?;
    let stdout = child
        .stdout
        .take()
        .context("Failed to take stdout from child process")?;

    // Spawn a task to wait for the child process and log errors
    let stderr = child.stderr.take();
    tokio::spawn(async move {
        if let Some(mut stderr) = stderr {
            let mut buffer = Vec::new();
            if let Ok(_) = tokio::io::AsyncReadExt::read_to_end(&mut stderr, &mut buffer).await {
                if !buffer.is_empty() {
                    if let Ok(err_str) = String::from_utf8(buffer) {
                        eprintln!("Native messaging host stderr: {}", err_str);
                    }
                }
            }
        }
        if let Err(e) = child.wait().await {
            eprintln!("Native messaging host process error: {}", e);
        }
    });

    Ok((stdin, stdout))
}

/// Send a message to the native messaging host
async fn send_message_to_host(stdin: &mut ChildStdin, message: &serde_json::Value) -> Result<()> {
    let json = serde_json::to_string(message).context("Failed to serialize message")?;
    let bytes = json.as_bytes();
    let len = bytes.len() as u32;

    // Write length prefix (4 bytes, little-endian)
    stdin
        .write_all(&len.to_le_bytes())
        .await
        .context("Failed to write message length")?;

    // Write message body
    stdin
        .write_all(bytes)
        .await
        .context("Failed to write message body")?;

    stdin.flush().await.context("Failed to flush stdin")?;

    Ok(())
}

/// Receive a message from the native messaging host
async fn receive_message_from_host(stdout: &mut ChildStdout) -> Result<serde_json::Value> {
    // Read length prefix (4 bytes, little-endian)
    let mut len_bytes = [0u8; 4];
    stdout
        .read_exact(&mut len_bytes)
        .await
        .context("Failed to read message length")?;

    let len = u32::from_le_bytes(len_bytes) as usize;
    if len == 0 {
        anyhow::bail!("Message length is zero");
    }
    if len > 1024 * 1024 {
        anyhow::bail!("Message too large: {} bytes", len);
    }

    // Read message body
    let mut buffer = vec![0u8; len];
    stdout
        .read_exact(&mut buffer)
        .await
        .context("Failed to read message body")?;

    let json_str = String::from_utf8(buffer).context("Invalid UTF-8 in message")?;
    let message: serde_json::Value =
        serde_json::from_str(&json_str).context("Failed to parse message JSON")?;

    Ok(message)
}
