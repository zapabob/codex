//! User interface rendering

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState};

pub struct UiState {
    /// Current scroll position
    pub scroll: usize,
    /// Selected item index
    pub selected: usize,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            scroll: 0,
            selected: 0,
        }
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Status
        ])
        .split(size);

    // Draw title
    draw_title(f, chunks[0]);

    // Draw content based on state
    match &app.state {
        AppState::MainMenu => draw_main_menu(f, chunks[1], app),
        AppState::Conversation { .. } => draw_conversation(f, chunks[1], app),
        AppState::PlanManager => draw_plan_manager(f, chunks[1], app),
        AppState::QualityControl => draw_quality_control(f, chunks[1], app),
        AppState::Settings => draw_settings(f, chunks[1], app),
    }

    // Draw status
    draw_status(f, chunks[2], app);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled("🧠 ", Style::default().fg(Color::Cyan)),
        Span::styled("Codex TUI", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(" - AI-Powered Development Assistant", Style::default().fg(Color::Gray)),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Codex"))
    .wrap(Wrap { trim: true });

    f.render_widget(title, area);
}

fn draw_main_menu(f: &mut Frame, area: Rect, app: &App) {
    let items = vec![
        ListItem::new("1. 💬 Start Conversation - Interactive AI chat"),
        ListItem::new("2. 📋 Plan Manager - Create and execute development plans"),
        ListItem::new("3. 🔍 Quality Control - Code analysis and optimization"),
        ListItem::new("4. ⚙️  Settings - Configure Codex preferences"),
        ListItem::new(""),
        ListItem::new("Press number to select, 'q' to quit"),
    ];

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}

fn draw_conversation(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from("🤖 AI Conversation Mode"),
        Line::from(""),
        Line::from("Type your message and press Enter to chat with AI"),
        Line::from("Use ↑/↓ to scroll through conversation history"),
        Line::from("Press 'm' to return to main menu"),
        Line::from(""),
        Line::from("[Conversation will appear here]"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Conversation"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_plan_manager(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from("📋 Plan Management"),
        Line::from(""),
        Line::from("• Create development plans"),
        Line::from("• Execute multi-step tasks"),
        Line::from("• Track progress and results"),
        Line::from("• Parallel execution support"),
        Line::from(""),
        Line::from("Press 'c' to create new plan, 'l' to list plans"),
        Line::from("Press 'm' to return to main menu"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Plan Manager"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_quality_control(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from("🔍 Quality Control Dashboard"),
        Line::from(""),
        Line::from("• Statistical code analysis"),
        Line::from("• Quantum optimization suggestions"),
        Line::from("• Mathematical performance modeling"),
        Line::from("• Visual quality reports"),
        Line::from(""),
        Line::from("Press 'a' to analyze current project"),
        Line::from("Press 'r' to view reports"),
        Line::from("Press 'm' to return to main menu"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Quality Control"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_settings(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from("⚙️  Settings"),
        Line::from(""),
        Line::from("• AI Model selection"),
        Line::from("• API key configuration"),
        Line::from("• Theme and appearance"),
        Line::from("• Performance settings"),
        Line::from("• Export/import preferences"),
        Line::from(""),
        Line::from("Press 'm' to return to main menu"),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_status(f: &mut Frame, area: Rect, app: &App) {
    let status_text = match &app.state {
        AppState::MainMenu => "Ready - Press number to select option",
        AppState::Conversation { conversation_id } => {
            if let Some(id) = conversation_id {
                &format!("Conversation active: {}", id)
            } else {
                "New conversation - Type to start chatting"
            }
        }
        AppState::PlanManager => "Plan Manager - Create or execute plans",
        AppState::QualityControl => "Quality Control - Analyze and optimize code",
        AppState::Settings => "Settings - Configure preferences",
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .style(Style::default().fg(Color::Green));

    f.render_widget(status, area);
}
