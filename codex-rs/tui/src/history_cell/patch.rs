
use crate::diff_render::display_path_for;
use crate::render::line_utils::prefix_lines;
use codex_protocol::file_change::FileChange;
use ratatui::prelude::*;
use ratatui::style::Stylize;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use super::HistoryCell;

#[derive(Debug)]
pub(crate) struct PatchHistoryCell {
    changes: HashMap<PathBuf, FileChange>,
    cwd: PathBuf,
}

impl HistoryCell for PatchHistoryCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
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
                FileChange::Delete => "deleted".red(),
                FileChange::New(content) => {
                    format!("new file, {} lines", content.lines().count()).green()
                }
                FileChange::Modify(hunks) => {
                    let mut insertions = 0;
                    let mut deletions = 0;
                    for hunk in hunks {
                        for line in &hunk.lines {
                            match line {
                                crate::diff_view::DiffLine::Add(_) => insertions += 1,
                                crate::diff_view::DiffLine::Delete(_) => deletions += 1,
                                _ => {}
                            }
                        }
                    }
                    format!("{} lines changed (+{insertions}, -{deletions})", hunks.len()).dim()
                }
            };
            file_lines.push(vec![display_path.bold(), " ".into(), summary].into());
        }

        // 4. Prefix the file list with the tree structure
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
