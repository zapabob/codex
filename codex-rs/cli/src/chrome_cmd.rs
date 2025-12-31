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
