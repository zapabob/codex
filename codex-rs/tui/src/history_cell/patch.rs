use super::HistoryCell;
use crate::diff_render::display_path_for;
use crate::render::line_utils::prefix_lines;
use codex_protocol::protocol::FileChange;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct PatchHistoryCell {
    changes: HashMap<PathBuf, FileChange>,
    cwd: PathBuf,
}

impl HistoryCell for PatchHistoryCell {
    fn display_lines(&self, _width: u16) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = Vec::new();

        // 1. Header line
        let file_count = self.changes.len();
        let header_text = if file_count == 1 {
            "Proposed edition to 1 file".to_string()
        } else {
            format!("Proposed editions to {file_count} files")
        };
        lines.push(vec!["• ".dim(), header_text.bold()].into());

        // 2. Sort files by path for deterministic output
        let mut changes: Vec<_> = self.changes.iter().collect();
        changes.sort_by(|(ka, _), (kb, _)| ka.cmp(kb));

        // 3. Render each file logic
        let mut file_lines: Vec<Line<'static>> = Vec::new();
        for (path, change) in changes {
            let display_path = display_path_for(path, &self.cwd);
            let summary = match change {
                FileChange::Delete { .. } => "deleted".red(),
                FileChange::Add { content } => {
                    format!("new file, {} lines", content.lines().count()).green()
                }
                FileChange::Update { unified_diff, .. } => {
                    let (insertions, deletions) =
                        crate::diff_render::calculate_add_remove_from_diff(unified_diff);
                    format!("modified (+{insertions}, -{deletions})").dim()
                }
            };
            file_lines.push(vec![display_path.bold(), " ".into(), summary].into());
        }

        lines.extend(prefix_lines(file_lines, "  └ ".dim(), "    ".into()));

        lines
    }
}

/// Create a new `PendingPatch` cell that lists the file‑level summary of
/// a proposed patch. The summary lines should already be formatted (e.g.
/// "A path/to/file.rs").
pub(crate) fn new_patch_event(
    changes: HashMap<PathBuf, FileChange>,
    cwd: &Path,
) -> PatchHistoryCell {
    PatchHistoryCell {
        changes,
        cwd: cwd.to_path_buf(),
    }
}
