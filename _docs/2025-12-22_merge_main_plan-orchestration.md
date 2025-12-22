# Merge log: main <= merge-upstream-2025-12-20

Date: 2025-12-22
Branch: main
Source: merge-upstream-2025-12-20

Summary
- Fast-forwarded main to include upstream merge plus plan execution mode enablement.
- Plan orchestration now routes by execution mode (single/orchestrated/competition).
- DevelopmentMode now maps to plan ExecutionMode.

Key commits included
- 86d9efdca Enable plan execution modes
- 2de8ad799 Merge upstream/main

Notes
- This merge pulled in large upstream changes and new files across codex-rs/tui/tui2 and related tooling.

Follow-up work (2025-12-22)
- Synced TUI approval overlay, command popup, rate limit rendering, and CLI flags to match current API expectations.
- Updated TUI exec handling, interrupt queue, status card, and tests for new protocol structures.
- Updated MCP server event handling to match current EventMsg variants.
- Formatting ran via cargo fmt (just fmt still fails on Windows shell).
- Clippy/tests incomplete due to disk full during build; see CLI output for details.
