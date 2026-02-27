# Implementation Log - 2026-02-27

## Scope
- Aligned Rust workspace modules with upstream API changes while preserving custom CLI features.
- Restored build consistency for `codex-cli` with `--features custom-features`.
- Fixed GUI production build failure in `codex-gui-x`.
- Increased subagent capacity defaults and performed semantic version bump.
- Executed parallel subagent workflow (`backend`, `qa`, `MILSPEC`, `gui`) and archived verification artifacts.

## Key Changes
- Synced API-drifted crates with upstream references: `codex-rs/{app-server,exec,mcp-server,tui}`.
- Added compatibility exports in `codex-rs/core/src/lib.rs` (`protocol`, `InitialHistory`).
- Updated custom/runtime integration points:
  - `codex-rs/supervisor/src/codex_executor.rs`
  - `codex-rs/app-server/src/codex_message_processor.rs`
  - `codex-rs/tui/src/bottom_pane/feedback_view.rs`
  - `codex-rs/cli/src/{agent_create_cmd.rs,delegate_cmd.rs,parallel_delegate_cmd.rs}`
- Feature wiring for custom CLI in `codex-rs/core/Cargo.toml`.
- Subagent scale changes:
  - `DEFAULT_AGENT_MAX_THREADS`: `12 -> 24` (`codex-rs/core/src/config/mod.rs`)
  - `max_parallel_agents` default: `5 -> 10` (`codex-rs/supervisor/src/types.rs`)
- SemVer bump:
  - `codex-rs/Cargo.toml`: `2.19.0 -> 2.19.1`
  - `package.json`: `2.19.0 -> 2.19.1`
  - `codex-gui-x/package.json`: `2.19.0 -> 2.19.1`
  - `VERSION`: `created -> 2.19.1`

## Build Verification
- Rust (6-core):
  - Command: `cargo build -p codex-cli --features custom-features -j 6`
  - Environment: `RUSTC_WRAPPER=''`, `CARGO_INCREMENTAL=1`, `CARGO_TARGET_DIR=target/codex-cli-fast`
  - Result: **PASS** (cold build: `19m53s`, incremental rebuild: `6m12s`)
  - Log evidence:
    - `logs/build-codex-cli-custom-features-j6-final.log`
    - `logs/build-codex-cli-custom-features-j6-incremental.log`
  - Warning/Error status: **0 warnings / 0 errors**
- GUI:
  - Command: `npm run build` in `codex-gui-x`
  - Result: **PASS** (`vite build complete`)
  - Additional fix: removed UTF-8 BOM from `codex-gui-x/package.json` to resolve PostCSS JSON parse failure.

## Notes
- `codex gui-x` launch path is present in `codex-rs/cli/src/main.rs` and remains available.
- MILSPEC and QA companion reports are stored under `logs/` and `docs/` for auditability.
