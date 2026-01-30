use crate::app_event::{AppEvent, FeedbackCategory};
use crate::app_event_sender::AppEventSender;
use ratatui::style::Stylize;
use ratatui::text::Span;

pub(crate) const BASE_ISSUE_URL: &str =
    "https://github.com/openai/codex/issues/new?template=2-bug-report.yml";

pub(crate) fn gutter() -> Span<'static> {
    "▌ ".cyan()
}

pub(crate) fn feedback_title_and_placeholder(category: FeedbackCategory) -> (String, String) {
    match category {
        FeedbackCategory::BadResult => (
            "Tell us more (bad result)".to_string(),
            "(optional) Write a short description to help us further".to_string(),
        ),
        FeedbackCategory::GoodResult => (
            "Tell us more (good result)".to_string(),
            "(optional) Write a short description to help us further".to_string(),
        ),
        FeedbackCategory::Bug => (
            "Tell us more (bug)".to_string(),
            "(optional) Write a short description to help us further".to_string(),
        ),
        FeedbackCategory::Other => (
            "Tell us more (other)".to_string(),
            "(optional) Write a short description to help us further".to_string(),
        ),
    }
}

pub(crate) fn feedback_classification(category: FeedbackCategory) -> &'static str {
    match category {
        FeedbackCategory::BadResult => "bad_result",
        FeedbackCategory::GoodResult => "good_result",
        FeedbackCategory::Bug => "bug",
        FeedbackCategory::Other => "other",
    }
}

pub(super) fn make_feedback_item(
    app_event_tx: AppEventSender,
    name: &str,
    description: &str,
    category: FeedbackCategory,
) -> crate::tui::bottom_pane::SelectionItem {
    let action: crate::tui::bottom_pane::SelectionAction =
        Box::new(move |_sender: &AppEventSender| {
            app_event_tx.send(AppEvent::OpenFeedbackConsent { category });
        });
    crate::tui::bottom_pane::SelectionItem {
        name: name.to_string(),
        description: Some(description.to_string()),
        actions: vec![action],
        dismiss_on_select: true,
        ..Default::default()
    }
}
