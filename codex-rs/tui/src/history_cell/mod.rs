pub mod basic;
pub mod exec;
pub mod mcp;
pub mod patch;
pub mod plan;

pub mod reasoning;
pub mod session;
#[cfg(test)]
mod tests;
pub mod update;
pub mod user;

use crate::render::renderable::Renderable;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Wrap};
use std::any::Any;
use unicode_width::UnicodeWidthStr;

pub(crate) use basic::{CompositeHistoryCell, PlainHistoryCell, PrefixedWrappedHistoryCell};
pub(crate) use basic::{
    FinalMessageSeparator, new_error_event, new_info_event, new_patch_apply_failure,
    new_review_status_line, new_view_image_tool_call, new_warning_event,
};
pub(crate) use exec::{
    AgentMessageCell, UnifiedExecInteractionCell, new_approval_decision_cell,
    new_unified_exec_interaction, new_unified_exec_processes_output,
};
pub(crate) use mcp::{
    McpToolCallCell, WebSearchCell, empty_mcp_output, new_active_mcp_tool_call,
    new_active_web_search_call, new_mcp_tools_output, new_web_search_call,
};
pub(crate) use patch::{PatchHistoryCell, new_patch_event};
pub(crate) use plan::{PlanUpdateCell, new_plan_update};
pub(crate) use reasoning::{ReasoningSummaryCell, new_reasoning_summary_block};
pub(crate) use session::{SessionHeaderHistoryCell, SessionInfoCell, new_session_info};
pub(crate) use update::{
    DeprecationNoticeCell, UpdateAvailableHistoryCell, new_deprecation_notice,
};
pub(crate) use user::{UserHistoryCell, new_user_prompt};

/// Represents an event to display in the conversation history. Returns its
/// `Vec<Line<'static>>` representation to make it easier to display in a
/// scrollable list.
pub(crate) trait HistoryCell: std::fmt::Debug + Send + Sync + Any {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>>;

    fn desired_height(&self, width: u16) -> u16 {
        Paragraph::new(Text::from(self.display_lines(width)))
            .wrap(Wrap { trim: false })
            .line_count(width)
            .try_into()
            .unwrap_or(0)
    }

    fn transcript_lines(&self, width: u16) -> Vec<Line<'static>> {
        self.display_lines(width)
    }

    fn desired_transcript_height(&self, width: u16) -> u16 {
        let lines = self.transcript_lines(width);
        // Workaround for ratatui bug: if there's only one line and it's whitespace-only, ratatui gives 2 lines.
        if let [line] = &lines[..]
            && line
                .spans
                .iter()
                .all(|s| s.content.chars().all(char::is_whitespace))
        {
            return 1;
        }

        Paragraph::new(Text::from(lines))
            .wrap(Wrap { trim: false })
            .line_count(width)
            .try_into()
            .unwrap_or(0)
    }

    fn is_stream_continuation(&self) -> bool {
        false
    }

    /// Returns a coarse "animation tick" when transcript output is time-dependent.
    ///
    /// The transcript overlay caches the rendered output of the in-flight active cell, so cells
    /// that include time-based UI (spinner, shimmer, etc.) should return a tick that changes over
    /// time to signal that the cached tail should be recomputed. Returning `None` means the
    /// transcript lines are stable, while returning `Some(tick)` during an in-flight animation
    /// allows the overlay to keep up with the main viewport.
    ///
    /// If a cell uses time-based visuals but always returns `None`, `Ctrl+T` can appear "frozen" on
    /// the first rendered frame even though the main viewport is animating.
    fn transcript_animation_tick(&self) -> Option<u64> {
        None
    }
}

impl Renderable for Box<dyn HistoryCell> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let lines = self.display_lines(area.width);
        let y = if area.height == 0 {
            0
        } else {
            let overflow = lines.len().saturating_sub(usize::from(area.height));
            u16::try_from(overflow).unwrap_or(u16::MAX)
        };
        Paragraph::new(Text::from(lines))
            .scroll((y, 0))
            .render(area, buf);
    }
    fn desired_height(&self, width: u16) -> u16 {
        HistoryCell::desired_height(self.as_ref(), width)
    }
}

impl dyn HistoryCell {
    pub(crate) fn as_any(&self) -> &dyn Any {
        self
    }

    pub(crate) fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Helper: returns a 2-character span (emoji + space).
pub(crate) fn padded_emoji(emoji: &'static str) -> Span<'static> {
    // We assume the emoji is 2 cells wide or followed by a space if 1 cell.
    // For TUI alignment, usually "X " is good.
    Span::from(format!("{emoji} "))
}

/// Helper to render content with an outer rounded white border.
pub(crate) fn with_border(inner_lines: Vec<Line<'static>>) -> Vec<Line<'static>> {
    let width = inner_lines.iter().map(|l| l.width()).max().unwrap_or(0);
    with_border_with_inner_width(inner_lines, width)
}

/// Helper to render content with an outer rounded white border, forcing a specific inner width.
pub(crate) fn with_border_with_inner_width(
    inner_lines: Vec<Line<'static>>,
    inner_width: usize,
) -> Vec<Line<'static>> {
    let mut out = Vec::new();
    let horizontal = "─".repeat(inner_width);

    // Top border: ╭─────╮
    out.push(Line::from(vec![
        "╭".into(),
        horizontal.as_str().into(),
        "╮".into(),
    ]));

    for line in inner_lines {
        // Pad line to inner_width
        let line_width = line.width();
        let padding = inner_width.saturating_sub(line_width);
        let mut spans = vec!["│".into()];
        spans.extend(line.spans);
        spans.push(" ".repeat(padding).into());
        spans.push("│".into());
        out.push(Line::from(spans));
    }

    // Bottom border: ╰─────╯
    out.push(Line::from(vec![
        "╰".into(),
        horizontal.as_str().into(),
        "╯".into(),
    ]));

    out
}
