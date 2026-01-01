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
    eprintln!("Note: DOM reading requires the Chrome extension to be active.");
    eprintln!("The extension will handle the request via Native Messaging Host.");
    eprintln!("");
    eprintln!("To use this feature:");
    eprintln!("1. Ensure the Chrome extension is installed and active");
    eprintln!("2. Open a webpage in Chrome");
    eprintln!("3. Use the extension popup to read DOM, or");
    eprintln!("4. Use: codex chrome parse --utterance \"read DOM\" --url <URL>");
    eprintln!("");
    eprintln!("Request parameters:");
    eprintln!("  Selector: {:?}", args.selector);
    eprintln!("  Max chars: {}", args.max_chars);
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
    eprintln!("Note: Network request monitoring requires the Chrome extension to be active.");
    eprintln!("The extension will handle the request via Native Messaging Host.");
    eprintln!("");
    eprintln!("To use this feature:");
    eprintln!("1. Ensure the Chrome extension is installed and active");
    eprintln!("2. Open a webpage in Chrome");
    eprintln!("3. Use the extension popup to monitor network requests");
    eprintln!("");
    eprintln!("Request parameters:");
    eprintln!("  URL filter: {:?}", args.filter);
    eprintln!("  Limit: {}", args.limit);
    Ok(())
}
