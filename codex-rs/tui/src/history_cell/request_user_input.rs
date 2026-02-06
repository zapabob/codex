use super::HistoryCell;
use crate::render::line_utils::prefix_lines;
use crate::style::user_message_style;
use codex_protocol::request_user_input::{RequestUserInputAnswer, RequestUserInputQuestion};
use ratatui::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct RequestUserInputResultCell {
    pub questions: Vec<RequestUserInputQuestion>,
    pub answers: HashMap<String, RequestUserInputAnswer>,
    pub interrupted: bool,
}

impl HistoryCell for RequestUserInputResultCell {
    fn display_lines(&self, _width: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let style = user_message_style();

        if self.interrupted {
            lines.push(
                Line::from("User input interrupted").style(Style::default().fg(Color::Yellow)),
            );
            return lines;
        }

        for (i, question) in self.questions.iter().enumerate() {
            if i > 0 {
                lines.push(Line::from(""));
            }

            // Question text
            lines.push(Line::from(vec![Span::styled(
                format!("? {}", question.question),
                style.bold(),
            )]));

            // Answer
            if let Some(answer) = self.answers.get(&question.id) {
                for ans_text in &answer.answers {
                    lines.push(Line::from(vec![
                        Span::from("  "),
                        Span::styled(ans_text.clone(), style.fg(Color::Cyan)),
                    ]));
                }
            } else {
                lines.push(Line::from(vec![
                    Span::from("  "),
                    Span::styled("(no answer)", style.dim()),
                ]));
            }
        }

        // Wrap in prefix
        let mut prefixed = prefix_lines(lines, "› ".bold().dim(), "  ".into());
        let mut result = vec![Line::from("").style(style)];
        result.append(&mut prefixed);
        result.push(Line::from("").style(style));
        result
    }
}
