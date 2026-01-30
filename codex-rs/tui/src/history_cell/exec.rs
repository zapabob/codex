use super::HistoryCell;
use super::basic::{CompositeHistoryCell, PlainHistoryCell, PrefixedWrappedHistoryCell};
use crate::exec_command::strip_bash_lc_and_escape;
use crate::live_wrap::take_prefix_by_width;
use crate::render::line_utils::push_owned_lines;
use crate::text_formatting::truncate_text;
use crate::wrapping::{RtOptions, word_wrap_line, word_wrap_lines};
use ratatui::prelude::*;
use ratatui::style::Stylize;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub(crate) struct AgentMessageCell {
    lines: Vec<Line<'static>>,
    is_first_line: bool,
}

impl AgentMessageCell {
    pub(crate) fn new(lines: Vec<Line<'static>>, is_first_line: bool) -> Self {
        Self {
            lines,
            is_first_line,
        }
    }
}

impl HistoryCell for AgentMessageCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        word_wrap_lines(
            &self.lines,
            RtOptions::new(width as usize)
                .initial_indent(if self.is_first_line {
                    "• ".dim().into()
                } else {
                    "  ".into()
                })
                .subsequent_indent("  ".into()),
        )
    }

    fn is_stream_continuation(&self) -> bool {
        !self.is_first_line
    }
}

#[derive(Debug)]
pub(crate) struct UnifiedExecInteractionCell {
    command_display: Option<String>,
    stdin: String,
}

impl UnifiedExecInteractionCell {
    pub(crate) fn new(command_display: Option<String>, stdin: String) -> Self {
        Self {
            command_display,
            stdin,
        }
    }
}

impl HistoryCell for UnifiedExecInteractionCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        if width == 0 {
            return Vec::new();
        }
        let wrap_width = width as usize;

        let mut header_spans = vec!["↳ ".dim(), "Interacted with background terminal".bold()];
        if let Some(command) = &self.command_display
            && !command.is_empty()
        {
            header_spans.push(" · ".dim());
            header_spans.push(command.clone().dim());
        }
        let header = Line::from(header_spans);

        let mut out: Vec<Line<'static>> = Vec::new();
        let header_wrapped = word_wrap_line(&header, RtOptions::new(wrap_width));
        push_owned_lines(&header_wrapped, &mut out);

        let input_lines: Vec<Line<'static>> = if self.stdin.is_empty() {
            vec![vec!["(waited)".dim()].into()]
        } else {
            self.stdin
                .lines()
                .map(|line| Line::from(line.to_string()))
                .collect()
        };

        let input_wrapped = word_wrap_lines(
            input_lines,
            RtOptions::new(wrap_width)
                .initial_indent(Line::from("  └ ".dim()))
                .subsequent_indent(Line::from("    ".dim())),
        );
        out.extend(input_wrapped);
        out
    }

    fn desired_height(&self, width: u16) -> u16 {
        self.display_lines(width).len() as u16
    }
}

pub(crate) fn new_unified_exec_interaction(
    command_display: Option<String>,
    stdin: String,
) -> UnifiedExecInteractionCell {
    UnifiedExecInteractionCell::new(command_display, stdin)
}

#[derive(Debug)]
struct UnifiedExecProcessesCell {
    processes: Vec<String>,
}

impl UnifiedExecProcessesCell {
    fn new(processes: Vec<String>) -> Self {
        Self { processes }
    }
}

impl HistoryCell for UnifiedExecProcessesCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        if width == 0 {
            return Vec::new();
        }

        let wrap_width = width as usize;
        let max_processes = 16usize;
        let mut out: Vec<Line<'static>> = Vec::new();
        out.push(vec!["Background terminals".bold()].into());
        out.push("".into());

        if self.processes.is_empty() {
            out.push("  • No background terminals running.".italic().into());
            return out;
        }

        let prefix = "  • ";
        let prefix_width = UnicodeWidthStr::width(prefix);
        let truncation_suffix = " [...]";
        let truncation_suffix_width = UnicodeWidthStr::width(truncation_suffix);
        let mut shown = 0usize;
        for command in &self.processes {
            if shown >= max_processes {
                break;
            }
            let (snippet, snippet_truncated) = {
                let (first_line, has_more_lines) = match command.split_once('\n') {
                    Some((first, _)) => (first, true),
                    None => (command.as_str(), false),
                };
                let max_graphemes = 80;
                let mut graphemes = first_line.grapheme_indices(true);
                if let Some((byte_index, _)) = graphemes.nth(max_graphemes) {
                    (first_line[..byte_index].to_string(), true)
                } else {
                    (first_line.to_string(), has_more_lines)
                }
            };
            if wrap_width <= prefix_width {
                out.push(Line::from(prefix.dim()));
                shown += 1;
                continue;
            }
            let budget = wrap_width.saturating_sub(prefix_width);
            let mut needs_suffix = snippet_truncated;
            if !needs_suffix {
                let (_, remainder, _) = take_prefix_by_width(&snippet, budget);
                if !remainder.is_empty() {
                    needs_suffix = true;
                }
            }
            if needs_suffix && budget > truncation_suffix_width {
                let available = budget.saturating_sub(truncation_suffix_width);
                let (truncated, _, _) = take_prefix_by_width(&snippet, available);
                out.push(vec![prefix.dim(), truncated.cyan(), truncation_suffix.dim()].into());
            } else {
                let (truncated, _, _) = take_prefix_by_width(&snippet, budget);
                out.push(vec![prefix.dim(), truncated.cyan()].into());
            }
            shown += 1;
        }

        let remaining = self.processes.len().saturating_sub(shown);
        if remaining > 0 {
            let more_text = format!("... and {remaining} more running");
            if wrap_width <= prefix_width {
                out.push(Line::from(prefix.dim()));
            } else {
                let budget = wrap_width.saturating_sub(prefix_width);
                let (truncated, _, _) = take_prefix_by_width(&more_text, budget);
                out.push(vec![prefix.dim(), truncated.dim()].into());
            }
        }

        out
    }

    fn desired_height(&self, width: u16) -> u16 {
        self.display_lines(width).len() as u16
    }
}

pub(crate) fn new_unified_exec_processes_output(processes: Vec<String>) -> CompositeHistoryCell {
    let command = PlainHistoryCell::new(vec!["/ps".magenta().into()]);
    let summary = UnifiedExecProcessesCell::new(processes);
    CompositeHistoryCell::new(vec![Box::new(command), Box::new(summary)])
}

fn truncate_exec_snippet(full_cmd: &str) -> String {
    let mut snippet = match full_cmd.split_once('\n') {
        Some((first, _)) => format!("{first} ..."),
        None => full_cmd.to_string(),
    };
    snippet = truncate_text(&snippet, 80);
    snippet
}

fn exec_snippet(command: &[String]) -> String {
    let full_cmd = strip_bash_lc_and_escape(command);
    truncate_exec_snippet(&full_cmd)
}

pub fn new_approval_decision_cell(
    command: Vec<String>,
    decision: codex_core::protocol::ReviewDecision,
) -> Box<dyn HistoryCell> {
    use codex_core::protocol::ReviewDecision::*;

    let (symbol, summary): (Span<'static>, Vec<Span<'static>>) = match decision {
        Approved => {
            let snippet = Span::from(exec_snippet(&command)).dim();
            (
                "✔ ".green(),
                vec![
                    "You ".into(),
                    "approved".bold(),
                    " codex to run ".into(),
                    snippet,
                    " this time".bold(),
                ],
            )
        }
        ApprovedExecpolicyAmendment { .. } => {
            let snippet = Span::from(exec_snippet(&command)).dim();
            (
                "✔ ".green(),
                vec![
                    "You ".into(),
                    "approved".bold(),
                    " codex to run ".into(),
                    snippet,
                    " and applied the execpolicy amendment".bold(),
                ],
            )
        }
        ApprovedForSession => {
            let snippet = Span::from(exec_snippet(&command)).dim();
            (
                "✔ ".green(),
                vec![
                    "You ".into(),
                    "approved".bold(),
                    " codex to run ".into(),
                    snippet,
                    " every time this session".bold(),
                ],
            )
        }
        Denied => {
            let snippet = Span::from(exec_snippet(&command)).dim();
            (
                "✗ ".red(),
                vec![
                    "You ".into(),
                    "did not approve".bold(),
                    " codex to run ".into(),
                    snippet,
                ],
            )
        }
        Abort => {
            let snippet = Span::from(exec_snippet(&command)).dim();
            (
                "✗ ".red(),
                vec![
                    "You ".into(),
                    "canceled".bold(),
                    " the request to run ".into(),
                    snippet,
                ],
            )
        }
    };

    Box::new(PrefixedWrappedHistoryCell::new(
        Line::from(summary),
        symbol,
        "  ",
    ))
}
