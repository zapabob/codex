# 2026-02-27 Implementation Log

## Request Scope
- Merge latest upstream official features, security updates, and bugfixes while preserving custom features.
- Resolve merge conflicts with `py -3` in one pass.
- Enable launching `codex-gui-x` from TUI/CLI.
- Refactor for official API changes while preserving custom features.
- Allow subagent -> subagent spawning and double default max subagent count.
- SemVer bump.
- Run with max subagents.
- Keep implementation logs.
- 6-core fast differential build.

## Completed Work
1. Upstream merge continuation
- Merge remained in progress and conflict-resolution state was continued.
- Conflict resolver script executed:
  - `py -3 scripts/resolve_merge_conflicts.py`
  - Result: `No merge conflicts found.`

2. Delegate-parallel enhancements
- Updated `codex-rs/cli/src/main.rs` subcommands (already present in working tree and retained):
  - `delegate-parallel`
  - `gui-x`
- Reworked `codex-rs/cli/src/parallel_delegate_cmd.rs`:
  - Added agent alias normalization for requested names:
    - `backend -> executor`
    - `qa -> code-reviewer`
    - `milspec -> sec-audit`
    - `gui -> ts-reviewer`
  - Kept fallback through `AgentAliases`.
  - Improved runtime logs and JSON output report:
    - `requested_agents`
    - `resolved_agents`
    - `results`

3. TUI/CLI -> codex-gui-x launch path
- Added `gui-x` command path in CLI (`codex-rs/cli/src/main.rs`) and launch function:
  - Runs `npm run dev -- --host <host> --port <port>` in `codex-gui-x`
  - Supports `--attached` mode and detached mode.

4. Subagent nesting and limits (2x default)
- Updated defaults in `codex-rs/core/src/config/mod.rs`:
  - `DEFAULT_AGENT_MAX_THREADS: Some(6) -> Some(12)`
  - `DEFAULT_AGENT_MAX_DEPTH: 1 -> 2`

5. SemVer bump
- Updated versions:
  - `package.json`: `2.17.0 -> 2.18.0`
  - `codex-gui-x/package.json`: `2.14.1 -> 2.18.0`
  - `codex-rs/Cargo.toml [workspace.package].version`: `0.0.0 -> 2.18.0`

6. Workspace repair for custom crates discovered during build
- Added missing Rust workspace members/dependencies in `codex-rs/Cargo.toml`:
  - members: `deep-research`, `microsoft365`, `supervisor`, `web-search`, `mcp-types`
  - workspace deps: `codex-deep-research`, `codex-microsoft365`, `codex-supervisor`, `codex-web-search`, `mcp-types`
  - external deps added for workspace resolution: `git2`, `slug`

7. GUI syntax fix found during build
- Fixed TS syntax error in:
  - `codex-gui-x/src/components/chat/MessageBubble.tsx`
  - missing `)` in streaming badge conditional.

## Build / Validation
1. Rust (6-core)
- Command:
  - `cargo build -p codex-cli --features custom-features -j 6`
- Status:
  - Failed.
- Main failure class:
  - Large upstream/custom API mismatch in `codex-rs/core` (missing/renamed modules, signatures, fields, and dependency declarations), beyond localized quick-fix scope.

2. GUI build
- Command:
  - `npm run build` in `codex-gui-x`
- Status:
  - Failed.
- Main failure class:
  - Large pre-existing TypeScript type and interface mismatch across many files; one direct syntax error was fixed in this run.

## Notes
- Merge conflict resolver (`py -3`) requirement was executed successfully.
- `delegate-parallel` and `gui-x` command paths are now implemented in CLI sources.
- Full monorepo green build still requires a dedicated cross-module API reconciliation pass in `codex-rs/core` and broad TS stabilization in `codex-gui-x`.

## Runtime Execution Attempt
- codex --version => codex-cli 2.17.0 (installed binary).
- Installed binary help does not expose delegate-parallel / gui-x yet, so direct runtime verification requires rebuilding the local source binary.
- Source rebuild currently fails in merged codex-rs/core due wide API mismatch errors (captured above).
