//! Application state and logic

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;

use crate::app_event::AppEvent;
use crate::legacy_app::LegacyApp;
use crate::tui::Tui;
use crate::tui::TuiEvent;
use crate::ui::UiState;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct App {
    /// Application state
    pub state: AppState,
    /// UI state
    #[allow(dead_code)]
    pub ui: UiState,
    /// Should quit
    pub should_quit: bool,
    /// Legacy App (Chat Widget)
    pub legacy_app: Option<LegacyApp>,
    /// Legacy App Event Receiver
    pub legacy_event_rx: Option<UnboundedReceiver<AppEvent>>,
}

#[derive(Debug, Clone)]
pub enum AppState {
    /// Main menu
    MainMenu,
    /// Conversation view
    Conversation { conversation_id: Option<String> },
    /// Plan management
    PlanManager,
    /// Quality control
    QualityControl,
    /// Settings
    Settings,
}

impl App {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            state: AppState::MainMenu,
            ui: UiState::new(),
            should_quit: false,
            legacy_app: None,
            legacy_event_rx: None,
        })
    }

    pub async fn handle_key(&mut self, tui: &mut Tui, key: KeyEvent) {
        match self.state {
            AppState::MainMenu => self.handle_main_menu_key(key),
            AppState::Conversation { .. } => self.handle_conversation_key(tui, key).await,
            AppState::PlanManager => self.handle_plan_key(key),
            AppState::QualityControl => self.handle_qc_key(key),
            AppState::Settings => self.handle_settings_key(key),
        }
    }

    fn handle_main_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Char('1') => {
                self.state = AppState::Conversation {
                    conversation_id: None,
                };
            }
            crossterm::event::KeyCode::Char('2') => {
                self.state = AppState::PlanManager;
            }
            crossterm::event::KeyCode::Char('3') => {
                self.state = AppState::QualityControl;
            }
            crossterm::event::KeyCode::Char('4') => {
                self.state = AppState::Settings;
            }
            crossterm::event::KeyCode::Char('q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    async fn handle_conversation_key(&mut self, tui: &mut Tui, key: KeyEvent) {
        if let Some(legacy_app) = &mut self.legacy_app {
            // Check for exit key (e.g. Ctrl+b) to return to menu
            if key.code == crossterm::event::KeyCode::Char('b')
                && key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
            {
                self.state = AppState::MainMenu;
                return;
            }

            // Delegate to legacy app
            let _ = legacy_app.handle_tui_event(tui, TuiEvent::Key(key)).await;
        }
    }

    fn handle_plan_key(&mut self, _key: KeyEvent) {
        // Handle plan management
        if let crossterm::event::KeyCode::Char('m') = _key.code {
            self.state = AppState::MainMenu;
        }
    }

    fn handle_qc_key(&mut self, _key: KeyEvent) {
        // Handle quality control
        if let crossterm::event::KeyCode::Char('m') = _key.code {
            self.state = AppState::MainMenu;
        }
    }

    fn handle_settings_key(&mut self, _key: KeyEvent) {
        // Handle settings
        if let crossterm::event::KeyCode::Char('m') = _key.code {
            self.state = AppState::MainMenu;
        }
    }

    pub async fn tick(&mut self, tui: &mut Tui) -> Result<()> {
        // Update application state
        if let AppState::Conversation { .. } = self.state {
            if let Some(legacy_app) = &mut self.legacy_app {
                if let Some(rx) = &mut self.legacy_event_rx {
                    while let Ok(event) = rx.try_recv() {
                        legacy_app.handle_event(tui, event).await?;
                    }
                }
                // Handle TuiEvent::Draw for animations if needed, but main loop handles draw.
                // LegacyApp::handle_tui_event(Draw) is called in draw loop?
                // No, draw loop calls ui::draw.
                // We might need to call legacy_app.handle_tui_event(TuiEvent::Draw) here or in draw?
                // LegacyApp::handle_tui_event(Draw) does logic + drawing.
                // We should separate logic from drawing in LegacyApp if possible, or just call it here for logic
                // and let ui::draw handle the actual rendering if LegacyApp exposes a render method.

                // Looking at LegacyApp::handle_tui_event(Draw), it does:
                // 1. maybe_post_pending_notification
                // 2. handle_paste_burst_tick
                // 3. tui.draw(...)

                // We shouldn't call tui.draw here. We should extract the logic.
                // For now, let's assume LegacyApp's draw logic is coupled.
                // We can call a new method `tick_logic` on LegacyApp?

                // Let's just process events for now.
            }
        }
        Ok(())
    }
}
