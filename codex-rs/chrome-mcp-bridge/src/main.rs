mod bridge;
mod tools;

use anyhow::Result;
use bridge::BridgeServer;
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mode = env::args().nth(1).unwrap_or_else(|| "stdio".to_string());
    
    match mode.as_str() {
        "stdio" => {
            BridgeServer::run_stdio().await?;
        }
        "http" => {
            let port = env::args()
                .nth(2)
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(8788);
            BridgeServer::run_http(port).await?;
        }
        _ => {
            eprintln!("Usage: codex-chrome-mcp-bridge [stdio|http] [port]");
            eprintln!("  stdio: Run as stdio MCP server (default)");
            eprintln!("  http <port>: Run as streamable HTTP MCP server (default port: 8788)");
            std::process::exit(1);
        }
    }

    Ok(())
}
