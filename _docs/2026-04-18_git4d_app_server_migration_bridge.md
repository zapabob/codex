# 2026-04-18 Git4D App-Server Migration Bridge

## Goal

Move Git4D and VR/AR live bridging onto `codex app-server`, leave `codex-rs/gui` as a compatibility adapter, and keep `zapabob-legacy-suite` as the user-facing plugin entrypoint with app-server -> GUI -> local fallback ordering.

## Implemented

- Added experimental app-server protocol types in `codex-rs/app-server-protocol/src/protocol/git4d.rs`.
- Registered experimental methods and notification in `codex-rs/app-server-protocol/src/protocol/common.rs`:
  - `git4d/capabilities/read`
  - `git4d/session/start`
  - `git4d/session/list`
  - `git4d/session/watch`
  - `git4d/session/unwatch`
  - `git4d/session/event`
- Added `codex-rs/app-server/src/git4d_bridge.rs` and routed requests from `codex_message_processor.rs`.
- Extended `codex-rs/core/src/git4d_accelerated.rs` with:
  - `Git4DMode`
  - capability snapshots
  - session snapshots
  - sequenced replayable session events
  - canonical `launch_session`, `list_session_snapshots`, `get_session_replay_events`
- Switched `codex-rs/gui/src/api/git4d.rs` to a thin compatibility adapter over shared core metadata.
- Updated `plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py` to prefer:
  1. `CODEX_APP_SERVER_WS_URL`
  2. `CODEX_GUI_BASE_URL`
  3. local fallback
- Added Python smoke coverage in `plugins/zapabob-legacy-suite/servers/test_legacy_suite_mcp.py`.
- Updated docs and plugin skill text to describe app-server as the canonical live bridge.

## Verification

Successful:

- `cargo fmt --all`
- `python -m py_compile plugins/zapabob-legacy-suite/servers/legacy_suite_mcp.py plugins/zapabob-legacy-suite/servers/test_legacy_suite_mcp.py`
- `python -m unittest plugins/zapabob-legacy-suite/servers/test_legacy_suite_mcp.py`
- `git diff --check`

Attempted but incomplete on this machine:

- `cargo test -p codex-app-server-protocol git4d -- --nocapture`
- `cargo test -p codex-app-server-protocol git4d_capabilities_response_round_trip -- --exact --nocapture`

Both Rust test commands were run with:

- `CARGO_TARGET_DIR=F:\codex-main-git4d-target`
- `TEMP=F:\codex-temp`
- `TMP=F:\codex-temp`

Both timed out before completion. This is consistent with earlier disk and long-build pressure on the machine, even after moving target and temp output off `C:`.

## Residual Risk

- `codex-rs/gui` remains outside the main Rust workspace compile gate, so the GUI compatibility adapter is syntax-formatted and source-aligned but not separately compile-verified here.
- app-server Rust tests for the new Git4D bridge were added but not fully executed end-to-end because the local machine timed out during targeted Rust compilation.
- The plugin websocket path depends on either `websocket-client` or `websockets` being available in the Python environment; if neither is installed, it intentionally falls back to GUI or local behavior.
