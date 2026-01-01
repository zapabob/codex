mod cli_bridge;
mod message;

use anyhow::{Context, Result};
use message::{read_message, write_response, NativeResponse};
use std::io;
use tracing_subscriber;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    loop {
        match read_message() {
            Ok(msg) => {
                let response = handle_message(msg).await;
                if let Err(e) = response {
                    eprintln!("Error handling message: {}", e);
                }
            }
            Err(e) => {
                if e.to_string().contains("Failed to read message length") {
                    break;
                }
                eprintln!("Error reading message: {}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn handle_message(msg: message::NativeMessage) -> Result<()> {
    let response = match msg.r#type.as_str() {
        "ping" => {
            NativeResponse::success(
                msg.id.clone(),
                "ping.response".to_string(),
                serde_json::json!({ "message": "pong" }),
            )
        }
        "deep_research.request" => {
            let query = msg
                .payload
                .get("query")
                .and_then(|v| v.as_str())
                .context("Missing query in payload")?
                .to_string();

            let options = msg.payload.get("options").and_then(|v| v.as_object());
            let depth = options
                .and_then(|o| o.get("depth"))
                .and_then(|v| v.as_u64())
                .map(|d| d as u8);
            let breadth = options
                .and_then(|o| o.get("breadth"))
                .and_then(|v| v.as_u64())
                .map(|b| b as u8);

            match cli_bridge::handle_deep_research(query, depth, breadth).await {
                Ok(data) => NativeResponse::success(
                    msg.id.clone(),
                    "deep_research.response".to_string(),
                    data,
                ),
                Err(e) => NativeResponse::error(
                    msg.id.clone(),
                    "deep_research.response".to_string(),
                    e.to_string(),
                ),
            }
        }
        "nl_command.request" => {
            let utterance = msg
                .payload
                .get("utterance")
                .and_then(|v| v.as_str())
                .context("Missing utterance in payload")?
                .to_string();

            let origin = msg.origin.and_then(|o| {
                serde_json::from_value::<codex_core::chrome::ChromeOrigin>(o).ok()
            });

            match cli_bridge::handle_nl_command(utterance, origin) {
                Ok(data) => NativeResponse::success(
                    msg.id.clone(),
                    "nl_command.response".to_string(),
                    data,
                ),
                Err(e) => NativeResponse::error(
                    msg.id.clone(),
                    "nl_command.response".to_string(),
                    e.to_string(),
                ),
            }
        }
        "codegen.request" => {
            NativeResponse::error(
                msg.id.clone(),
                "codegen.response".to_string(),
                "Code generation not yet implemented".to_string(),
            )
        }
        "dom.read.request" => {
            let selector = msg.payload.get("selector").and_then(|v| v.as_str()).map(|s| s.to_string());
            let max_chars = msg.payload
                .get("max_chars")
                .and_then(|v| v.as_u64())
                .map(|c| c as usize)
                .unwrap_or(5000);

            match cli_bridge::handle_dom_read(selector, max_chars) {
                Ok(data) => NativeResponse::success(
                    msg.id.clone(),
                    "dom.read.response".to_string(),
                    data,
                ),
                Err(e) => NativeResponse::error(
                    msg.id.clone(),
                    "dom.read.response".to_string(),
                    e.to_string(),
                ),
            }
        }
        "console.get_logs.request" => {
            let level = msg.payload.get("level").and_then(|v| v.as_str()).map(|s| s.to_string());
            let filter = msg.payload.get("filter").and_then(|v| v.as_str()).map(|s| s.to_string());
            let limit = msg.payload
                .get("limit")
                .and_then(|v| v.as_u64())
                .map(|l| l as usize)
                .unwrap_or(50);

            match cli_bridge::handle_console_logs(level, filter, limit) {
                Ok(data) => NativeResponse::success(
                    msg.id.clone(),
                    "console.get_logs.response".to_string(),
                    data,
                ),
                Err(e) => NativeResponse::error(
                    msg.id.clone(),
                    "console.get_logs.response".to_string(),
                    e.to_string(),
                ),
            }
        }
        "network.get_logs.request" => {
            let filter = msg.payload.get("filter").and_then(|v| v.as_str()).map(|s| s.to_string());
            let limit = msg.payload
                .get("limit")
                .and_then(|v| v.as_u64())
                .map(|l| l as usize)
                .unwrap_or(50);

            match cli_bridge::handle_network_logs(filter, limit) {
                Ok(data) => NativeResponse::success(
                    msg.id.clone(),
                    "network.get_logs.response".to_string(),
                    data,
                ),
                Err(e) => NativeResponse::error(
                    msg.id.clone(),
                    "network.get_logs.response".to_string(),
                    e.to_string(),
                ),
            }
        }
        _ => NativeResponse::error(
            msg.id.clone(),
            format!("{}.response", msg.r#type),
            format!("Unknown message type: {}", msg.r#type),
        ),
    };

    write_response(&response)?;
    Ok(())
}
