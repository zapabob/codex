//! Application state and logic

use crate::app_backtrack::BacktrackState;
use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use crate::bottom_pane::ApprovalRequest;
use crate::chatwidget::ChatWidget;
use crate::diff_render::DiffSummary;
use crate::exec_command::strip_bash_lc_and_escape;
use crate::file_search::FileSearchManager;
use crate::history_cell::HistoryCell;
use crate::model_migration::ModelMigrationOutcome;
use crate::model_migration::migration_copy_for_config;
use crate::model_migration::run_model_migration_prompt;
use crate::pager_overlay::Overlay;
use crate::render::highlight::highlight_bash_to_lines;
use crate::render::renderable::Renderable;
use crate::resume_picker::ResumeSelection;
use crate::skill_error_prompt::SkillErrorPromptOutcome;
use crate::skill_error_prompt::run_skill_error_prompt;
use crate::tui;
use crate::tui::TuiEvent;
use crate::update_action::UpdateAction;
use codex_ansi_escape::ansi_escape_line;
use codex_app_server_protocol::AuthMode;
use codex_core::ai_orchestrator;
use codex_core::mcp_integration_manager;
use codex_core::AuthManager;
use codex_core::ConversationManager;
use codex_core::config::Config;
use codex_core::config::edit::ConfigEditsBuilder;
#[cfg(target_os = "windows")]
use codex_core::features::Feature;
use codex_core::openai_models::model_presets::HIDE_GPT_5_1_CODEX_MAX_MIGRATION_PROMPT_CONFIG;
use codex_core::openai_models::model_presets::HIDE_GPT5_1_MIGRATION_PROMPT_CONFIG;
use codex_core::openai_models::models_manager::ModelsManager;
use codex_core::protocol::EventMsg;
use codex_core::protocol::FinalOutput;
use codex_core::protocol::Op;
use codex_core::protocol::SessionSource;
use codex_core::protocol::SkillLoadOutcomeInfo;
use codex_core::protocol::TokenUsage;
use codex_core::skills::SkillError;
use codex_protocol::ConversationId;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ModelUpgrade;
use codex_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    /// AI Orchestrator
    pub orchestrator: Option<ai_orchestrator::AIOrchestrator>,
    /// MCP Integration Manager
    pub mcp_manager: Option<mcp_integration_manager::McpIntegrationManager>,
    /// Git Lock Manager for parallel development
    pub git_lock_manager: Option<Arc<codex_core::git_lock_manager::GitLockManager>>,
    /// Conflict Detector
    pub conflict_detector: Option<Arc<Mutex<Box<dyn codex_core::git_lock_manager::ConflictDetectorTrait + Send + Sync>>>>,
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
    /// Development mode orchestration
    DevelopmentMode {
        mode: ai_orchestrator::DevelopmentMode,
        active_servers: Vec<String>,
        agent_status: std::collections::BTreeMap<String, String>,
    },
    /// Settings
    Settings,
    /// Git Lock Management
    GitLockManager {
        locks: Vec<codex_core::git_lock_manager::LockEntry>,
        conflicts: Vec<codex_core::git_lock_manager::LockConflict>,
    },
}

impl App {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            state: AppState::MainMenu,
            ui: UiState::new(),
            should_quit: false,
            legacy_app: None,
            legacy_event_rx: None,
            orchestrator: None,
            mcp_manager: None,
            git_lock_manager: None,
            conflict_detector: None,
        })
    }

    pub async fn handle_key(&mut self, tui: &mut Tui, key: KeyEvent) {
        match self.state {
            AppState::MainMenu => self.handle_main_menu_key(key),
            AppState::Conversation { .. } => self.handle_conversation_key(tui, key).await,
            AppState::PlanManager => self.handle_plan_key(key),
            AppState::QualityControl => self.handle_qc_key(key),
            AppState::Settings => self.handle_settings_key(key),
            AppState::DevelopmentMode { .. } => self.handle_development_mode_key(key),
            AppState::GitLockManager { .. } => self.handle_git_lock_key(key),
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
            crossterm::event::KeyCode::Char('5') => {
                // Initialize GitLockManager state
                self.state = AppState::GitLockManager {
                    locks: Vec::new(),
                    conflicts: Vec::new(),
                };
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

    fn handle_git_lock_key(&mut self, key: KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Char('m') => {
                self.state = AppState::MainMenu;
            }
            crossterm::event::KeyCode::Char('r') => {
                // Refresh lock status
                // TODO: Implement refresh logic
            }
            _ => {}
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

    fn handle_development_mode_key(&mut self, _key: KeyEvent) {
        // TODO: Implement development mode key handling
        // For now, just ignore
    }
}
