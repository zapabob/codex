use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use codex_core::AuthManager;
use codex_core::config::Config;
use codex_core::models_manager::manager::ModelsManager;
use codex_otel::OtelManager;

use crate::app_event_sender::AppEventSender;
use crate::bottom_pane::FeedbackAudience;
use crate::tui::FrameRequester;

use super::user_message::UserMessage;

/// Common initialization parameters shared by all `ChatWidget` constructors.
pub(crate) struct ChatWidgetInit {
    pub(crate) config: Config,
    pub(crate) frame_requester: FrameRequester,
    pub(crate) app_event_tx: AppEventSender,
    pub(crate) initial_user_message: Option<UserMessage>,
    pub(crate) enhanced_keys_supported: bool,
    pub(crate) auth_manager: Arc<AuthManager>,
    pub(crate) models_manager: Arc<ModelsManager>,
    pub(crate) feedback: codex_feedback::CodexFeedback,
    pub(crate) is_first_run: bool,
    pub(crate) feedback_audience: FeedbackAudience,
    pub(crate) model: Option<String>,
    // Shared latch so we only warn once about invalid status-line item IDs.
    pub(crate) status_line_invalid_items_warned: Arc<AtomicBool>,
    pub(crate) otel_manager: OtelManager,
}
