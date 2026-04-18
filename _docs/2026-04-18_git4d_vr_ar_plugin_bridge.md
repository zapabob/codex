# Git4D / VR-AR Plugin Bridge Implementation Log

## Overview

Implemented the first bridge slice for Git4D and VR/AR plugin migration on top of the existing `zapabob-legacy-suite` plugin.

This slice does not attempt to restore the old immersive GUI wholesale. Instead it:

- exposes a clearer Git4D capability/session contract in `codex-rs/gui`
- restores the plugin-local MCP server implementation as tracked source
- teaches the plugin to prefer live GUI bridge endpoints when available and degrade cleanly when they are not

## Background / Requirements

- Keep `zapabob-legacy-suite` as the migration bridge instead of creating a new plugin
- Prefer official Codex app/app-server/plugin seams
- Make Git4D usable through plugin-triggered launch/session/capability routes first
- Keep VR/AR optional and fall back to desktop/text-first behavior when device support is missing

## Assumptions / Decisions

- `codex-rs/gui` remains a migration-era service, so this run only tightened its API contract rather than trying to wire a new front-end
- The plugin MCP server is the practical first place to surface live Git4D status while GUI work remains partial
- The repo-wide `*.py` ignore had to be punched through for the plugin server entrypoint so the bridge can be versioned

## Changed Files

- `.gitignore`
- `README.md`
- `codex-rs/gui/README.md`
- `codex-rs/gui/src/api/git4d.rs`
- `codex-rs/gui/src/main.rs`
- `plugins/zapabob-legacy-suite/.codex-plugin/plugin.json`
- `plugins/zapabob-legacy-suite/README.md`
- `plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py`
- `plugins/zapabob-legacy-suite/skills/git4d/SKILL.md`
- `plugins/zapabob-legacy-suite/skills/vr-ar/SKILL.md`

## Implementation Details

- Added `GET /api/visualization/git4d/capabilities/{mode}` to report requested mode, effective mode, platform, device availability, and fallback reason
- Enriched the Git4D launch response with `requestedMode`, `effectiveMode`, `fallbackReason`, `eventsPath`, and `capabilityPath`
- Enriched the Git4D session list with `eventsPath`
- Added lightweight helper tests for the capability-response mapping logic in `codex-rs/gui/src/api/git4d.rs`
- Restored `plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py` as tracked source and extended it so:
  - `git4d_repo_summary` can inspect live bridge sessions, capability status, and optional launch attempts
  - `vr_ar_capability_report` prefers the GUI capability endpoint and falls back to local heuristics when the GUI bridge is unavailable
- Updated plugin metadata and README/skill text to describe the plugin as an official-surface bridge instead of a pure fallback bundle

## Commands Run

```powershell
git worktree add C:\Users\downl\.codex\worktrees\codex-main-git4d-vr-ar-plugin-bridge -b codex/git4d-vr-ar-plugin-bridge
cargo check --manifest-path C:\Users\downl\.codex\worktrees\codex-main-git4d-vr-ar-plugin-bridge\codex-rs\gui\Cargo.toml
rustfmt codex-rs/gui/src/api/git4d.rs codex-rs/gui/src/main.rs
git diff --check
python -m py_compile plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py
python - <<handler smoke via importlib>>
```

## Test / Verification Results

- `python -m py_compile plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py`
  - Passed
- direct handler smoke for `legacy_suite_mcp.py`
  - `TOOLS` exported as `deepresearch_brief`, `git4d_repo_summary`, `vr_ar_capability_report`
  - `handle_git4d_repo_summary(...)` returned `ok=True` and produced `# Git4D Bridge Summary`
  - `handle_vr_ar_capability_report(...)` returned `ok=True` and produced `# VR or AR Capability Report`
- `git check-ignore plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py`
  - Confirmed `not-ignored` after the `.gitignore` exception was added
- `git diff --check`
  - No whitespace errors; only LF->CRLF warnings in the Windows working copy
- `cargo check --manifest-path codex-rs/gui/Cargo.toml`
  - Failed before compilation because `codex-rs/gui` currently believes it is in the `codex-rs` workspace while not being listed in `workspace.members`

## Residual Risks

- `codex-rs/gui` is still outside the active workspace, so the Rust-side bridge API changes could not be compile-verified in this run without broader workspace surgery
- `codex-core` still contains pre-existing Git4D/VR-AR module exposure seams that were not widened here
- No current front-end consumes the new capability/session fields yet; this run only established the backend/plugin contract

## Recommended Next Actions

- Decide whether `codex-rs/gui` should be brought back into `codex-rs` workspace membership or explicitly excluded as legacy
- Add a small front-end surface that consumes `capabilities/{mode}` and `sessions`
- Once the GUI crate has a supported build path again, run targeted Rust tests for `codex-rs/gui`
