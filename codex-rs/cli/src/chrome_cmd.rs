use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use codex_core::chrome::{
    ChromeConstraints, ChromeNlRequest, ChromeNlResponse, ChromeOrigin, parse_nl_command,
};
use serde::{Deserialize, Serialize};
use std::io::Read;

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

    let response = match parse_nl_command(ChromeNlRequest {
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
    use codex_cli::research_cmd::run_research_command;

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
    let (mut stdin, mut stdout) = spawn_native_host().await?;

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

    send_message_to_host(&mut stdin, &message).await?;
    let response = receive_message_from_host(&mut stdout).await?;

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
            anyhow::bail!("DOM read failed: {}", error);
        }
    } else {
        anyhow::bail!("Invalid response format");
    }

    Ok(())
}

async fn run_console(args: ChromeConsoleArgs) -> Result<()> {
    eprintln!("Note: Console log retrieval requires the Chrome extension to be active.");
    eprintln!("The extension will handle the request via Native Messaging Host.");
    eprintln!("");
    eprintln!("To use this feature:");
    eprintln!("1. Ensure the Chrome extension is installed and active");
    eprintln!("2. Open a webpage in Chrome");
    eprintln!("3. Use the extension popup to view console logs");
    eprintln!("");
    eprintln!("Request parameters:");
    eprintln!("  Level filter: {:?}", args.level);
    eprintln!("  Message filter: {:?}", args.filter);
    eprintln!("  Limit: {}", args.limit);
    Ok(())
}

async fn run_network(args: ChromeNetworkArgs) -> Result<()> {
    let (mut stdin, mut stdout) = spawn_native_host().await?;

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

    send_message_to_host(&mut stdin, &message).await?;
    let response = receive_message_from_host(&mut stdout).await?;

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
            anyhow::bail!("Network log retrieval failed: {}", error);
        }
    } else {
        anyhow::bail!("Invalid response format");
    }

    Ok(())
}
