use crate::app_event::{AppEvent, FeedbackCategory};
use crate::app_event_sender::AppEventSender;
use ratatui::style::Stylize;
use ratatui::text::Line;

use super::utils::make_feedback_item;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;
use crate::bottom_pane::{SelectionAction, SelectionItem, SelectionViewParams};

// Build the selection popup params for feedback categories.
pub(crate) fn feedback_selection_params(app_event_tx: AppEventSender) -> SelectionViewParams {
    SelectionViewParams {
        title: Some("How was this?".to_string()),
        items: vec![
            make_feedback_item(
                app_event_tx.clone(),
                "bug",
                "Crash, error message, hang, or broken UI/behavior.",
                FeedbackCategory::Bug,
            ),
            make_feedback_item(
                app_event_tx.clone(),
                "bad result",
                "Output was off-target, incorrect, incomplete, or unhelpful.",
                FeedbackCategory::BadResult,
            ),
            make_feedback_item(
                app_event_tx.clone(),
                "good result",
                "Helpful, correct, high‑quality, or delightful result worth celebrating.",
                FeedbackCategory::GoodResult,
            ),
            make_feedback_item(
                app_event_tx,
                "other",
                "Slowness, feature suggestion, UX feedback, or anything else.",
                FeedbackCategory::Other,
            ),
        ],
        ..Default::default()
    }
}

/// Build the selection popup params shown when feedback is disabled.
pub(crate) fn feedback_disabled_params() -> SelectionViewParams {
    SelectionViewParams {
        title: Some("Sending feedback is disabled".to_string()),
        subtitle: Some("This action is disabled by configuration.".to_string()),
        footer_hint: Some(standard_popup_hint_line()),
        items: vec![SelectionItem {
            name: "Close".to_string(),
            dismiss_on_select: true,
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Build the upload consent popup params for a given feedback category.
pub(crate) fn feedback_upload_consent_params(
    app_event_tx: AppEventSender,
    category: FeedbackCategory,
    rollout_path: Option<std::path::PathBuf>,
) -> SelectionViewParams {
    let yes_action: SelectionAction = Box::new({
        let tx = app_event_tx.clone();
        move |sender: &AppEventSender| {
            let _ = sender;
            tx.send(AppEvent::OpenFeedbackNote {
                category,
                include_logs: true,
            });
        }
    });

    let no_action: SelectionAction = Box::new({
        let tx = app_event_tx;
        move |sender: &AppEventSender| {
            let _ = sender;
            tx.send(AppEvent::OpenFeedbackNote {
                category,
                include_logs: false,
            });
        }
    });

    // Build header listing files that would be sent if user consents.
    let mut header_lines: Vec<Box<dyn crate::render::renderable::Renderable>> = vec![
        Line::from("Upload logs?".bold()).into(),
        Line::from("").into(),
        Line::from("The following files will be sent:".dim()).into(),
        Line::from(vec!["  • ".into(), "codex-logs.log".into()]).into(),
    ];
    if let Some(path) = rollout_path.as_deref()
        && let Some(name) = path.file_name().map(|s| s.to_string_lossy().to_string())
    {
        header_lines.push(Line::from(vec!["  • ".into(), name.into()]).into());
    }

    SelectionViewParams {
        footer_hint: Some(standard_popup_hint_line()),
        items: vec![
            SelectionItem {
                name: "Yes".to_string(),
                description: Some(
                    "Share the current Codex session logs with the team for troubleshooting."
                        .to_string(),
                ),
                actions: vec![yes_action],
                dismiss_on_select: true,
                ..Default::default()
            },
            SelectionItem {
                name: "No".to_string(),
                description: Some("".to_string()),
                actions: vec![no_action],
                dismiss_on_select: true,
                ..Default::default()
            },
        ],
        header: Box::new(crate::render::renderable::ColumnRenderable::with(
            header_lines,
        )),
        ..Default::default()
    }
}
