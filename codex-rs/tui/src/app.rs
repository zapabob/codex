//! Application state and logic

use std::sync::Arc;

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use tokio::sync::Mutex;

use crate::ui::UiState;

pub struct App {
    /// Application state
    pub state: AppState,
    /// UI state
    pub ui: UiState,
    /// Should quit
    pub should_quit: bool,
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
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state {
            AppState::MainMenu => self.handle_main_menu_key(key),
            AppState::Conversation { .. } => self.handle_conversation_key(key),
            AppState::PlanManager => self.handle_plan_key(key),
            AppState::QualityControl => self.handle_qc_key(key),
            AppState::Settings => self.handle_settings_key(key),
        }
    }

    fn handle_main_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Char('1') => {
                self.state = AppState::Conversation { conversation_id: None };
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

    fn handle_conversation_key(&mut self, _key: KeyEvent) {
        // Handle conversation input
    }

    fn handle_plan_key(&mut self, _key: KeyEvent) {
        // Handle plan management
    }

    fn handle_qc_key(&mut self, _key: KeyEvent) {
        // Handle quality control
    }

    fn handle_settings_key(&mut self, _key: KeyEvent) {
        // Handle settings
    }

    pub async fn tick(&mut self) -> Result<()> {
        // Update application state
        Ok(())
    }
}