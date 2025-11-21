//! Codex TUI - Terminal User Interface

use std::sync::Arc;
use color_eyre::eyre::Result;
use codex_core::AuthManager;
use codex_core::config::Config;
use codex_tui::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Load configuration
    let config = Config::load_or_default()?;
    let auth_manager = Arc::new(AuthManager::new(&config)?);

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Initialize terminal
    let mut terminal = codex_tui::tui::init()?;

    // Create app
    let mut app = App::new(
        auth_manager,
        config,
        None, // active_profile
        None, // initial_prompt
    )?;

    // Run the TUI application
    let exit_info = app.run(&mut terminal).await?;

    // Cleanup terminal
    codex_tui::tui::restore()?;

    // Print exit information
    if let Some(conversation_id) = exit_info.conversation_id {
        println!("Conversation saved: {}", conversation_id);
    }

    println!("Token usage: {}", exit_info.token_usage.total_tokens);

    Ok(())
}
