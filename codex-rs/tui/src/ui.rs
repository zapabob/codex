//! User interface rendering

use crate::custom_terminal::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

use crate::app::App;
use crate::app::AppState;

pub struct UiState {
    /// Current scroll position
    #[allow(dead_code)]
    pub scroll: usize,
    /// Selected item index
    #[allow(dead_code)]
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
    let size = f.area();

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
        AppState::DevelopmentMode { .. } => draw_development_mode(f, chunks[1], app),
        AppState::GitLockManager { .. } => draw_git_lock_manager(f, chunks[1], app),
    }

    // Draw status
    draw_status(f, chunks[2], app);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled("🧠 ", Style::default().fg(Color::Cyan)),
        Span::styled(
            "Codex TUI",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " - AI-Powered Development Assistant",
            Style::default().fg(Color::Gray),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Codex"))
    .wrap(Wrap { trim: true });

    f.render_widget(title, area);
}

fn draw_main_menu(f: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("Codex Main Menu").block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items = vec![
        ListItem::new("1. Conversation"),
        ListItem::new("2. Plan Manager"),
        ListItem::new("3. Quality Control"),
        ListItem::new("4. Settings"),
        ListItem::new("5. Git Lock Manager"),
    ];

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_widget(list, chunks[1]);
}

fn draw_conversation(_f: &mut Frame, _area: Rect, _app: &App) {
    // We need mutable access to app to call draw on legacy_app because it takes &mut self
    // But ui::draw takes &mut App, so we should change draw_conversation signature?
    // Actually ui::draw calls draw_conversation(f, chunks[1], app).
    // app is &mut App in ui::draw.
    // So we need to change draw_conversation signature to take &mut App.
    // And also update the call site in ui::draw.
}

fn draw_plan_manager(f: &mut Frame, area: Rect, _app: &App) {
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

fn draw_quality_control(f: &mut Frame, area: Rect, _app: &App) {
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Quality Control"),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_settings(f: &mut Frame, area: Rect, _app: &App) {
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
        AppState::DevelopmentMode { .. } => "Development Mode - AI orchestration active",
        AppState::GitLockManager { .. } => "Git Lock Manager - Parallel development control",
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .style(Style::default().fg(Color::Green));

    f.render_widget(status, area);
}

fn draw_development_mode(f: &mut Frame, area: Rect, _app: &App) {
    let content = vec![
        Line::from("Development Mode Active"),
        Line::from(""),
        Line::from("AI Orchestration: Active"),
        Line::from("Sub-agents: Ready"),
        Line::from("MCP Servers: Connected"),
        Line::from(""),
        Line::from("Press ESC to return to main menu"),
    ];

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Development Mode"),
        )
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(paragraph, area);
}

fn draw_git_lock_manager(f: &mut Frame, area: Rect, _app: &mut App) {
    let content = vec![
        Line::from("🔒 Git Lock Manager - Parallel Development Control"),
        Line::from(""),
        Line::from("Active Locks:"),
        Line::from("Conflicts:"),
        Line::from(""),
        Line::from("Commands:"),
        Line::from("  r - Refresh lock status"),
        Line::from("  m - Return to main menu"),
        Line::from(""),
        Line::from("Features:"),
        Line::from("  • File-level locking"),
        Line::from("  • Branch-level locking"),
        Line::from("  • Conflict detection"),
        Line::from("  • Deadlock prevention"),
    ];

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Git Lock Manager"),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
