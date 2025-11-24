//! Codex TUI - Terminal User Interface

use std::io::{self, Stdout, stdout};

use color_eyre::eyre::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};

mod app;
mod ui;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create app
    let mut app = App::new().await?;

    // Run the application
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    restore_terminal(&mut terminal)?;

    // Print result
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => return Ok(()),
                    crossterm::event::KeyCode::Esc => return Ok(()),
                    _ => app.handle_key(key),
                }
            }
        }

        // Update app state
        app.tick().await?;
    }
}
