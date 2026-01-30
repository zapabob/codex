use super::HistoryCell;
use crate::markdown::append_markdown;
use crate::wrapping::{RtOptions, word_wrap_lines};
use ratatui::prelude::*;
use ratatui::style::Style;
use ratatui::style::Stylize;

#[derive(Debug)]
pub(crate) struct ReasoningSummaryCell {
    _header: String,
    content: String,
    transcript_only: bool,
}

impl ReasoningSummaryCell {
    pub(crate) fn new(header: String, content: String, transcript_only: bool) -> Self {
        Self {
            _header: header,
            content,
            transcript_only,
        }
    }

    fn lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = Vec::new();
        append_markdown(
            &self.content,
            Some((width as usize).saturating_sub(2)),
            &mut lines,
        );
        let summary_style = Style::default().dim().italic();
        let summary_lines = lines
            .into_iter()
            .map(|mut line| {
                line.spans = line
                    .spans
                    .into_iter()
                    .map(|span| span.patch_style(summary_style))
                    .collect();
                line
            })
            .collect::<Vec<_>>();

        word_wrap_lines(
            &summary_lines,
            RtOptions::new(width as usize)
                .initial_indent("• ".dim().into())
                .subsequent_indent("  ".into()),
        )
    }
}

impl HistoryCell for ReasoningSummaryCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        if self.transcript_only {
            Vec::new()
        } else {
            self.lines(width)
        }
    }

    fn desired_height(&self, width: u16) -> u16 {
        if self.transcript_only {
            0
        } else {
            self.lines(width).len() as u16
        }
    }

    fn transcript_lines(&self, width: u16) -> Vec<Line<'static>> {
        self.lines(width)
    }

    fn desired_transcript_height(&self, width: u16) -> u16 {
        self.lines(width).len() as u16
    }
}

pub(crate) fn new_reasoning_summary_block(full_reasoning_buffer: String) -> Box<dyn HistoryCell> {
    let full_reasoning_buffer = full_reasoning_buffer.trim();
    if let Some(open) = full_reasoning_buffer.find("**") {
        let after_open = &full_reasoning_buffer[(open + 2)..];
        if let Some(close) = after_open.find("**") {
            let after_close_idx = open + 2 + close + 2;
            // if we don't have anything beyond `after_close_idx`
            // then we don't have a summary to inject into history
            if after_close_idx < full_reasoning_buffer.len() {
                let header_buffer = full_reasoning_buffer[..after_close_idx].to_string();
                let summary_buffer = full_reasoning_buffer[after_close_idx..].to_string();
                return Box::new(ReasoningSummaryCell::new(
                    header_buffer,
                    summary_buffer,
                    false,
                ));
            }
        }
    }
    Box::new(ReasoningSummaryCell::new(
        "".to_string(),
        full_reasoning_buffer.to_string(),
        true,
    ))
}
