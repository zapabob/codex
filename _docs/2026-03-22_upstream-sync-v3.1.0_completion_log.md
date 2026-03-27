# Upstream Sync v3.1.0 Completion Log

- **Date**: 2026-03-22
- **Workspace**: `C:\Users\downl\Desktop\codex-main-upstream-sync`
- **Branch**: `codex/upstream-sync-2026-03-22`
- **Target upstream commit**: `cf0223887fb84a9bd57986c21f3d7eadfce249a3`
- **Objective**: Continue the upstream sync, restore broken merge areas with upstream-first semantics, preserve custom platform features, and unify the product version to `v3.1.0`.

## Summary

This pass focused on three things:

1. Restoring `exec-server` to a valid upstream-shaped implementation.
2. Unifying the product version across Rust, web, Tauri, and release-facing metadata to `3.1.0`.
3. Removing remaining merge-conflict artifacts from build and lint infrastructure that would block further stabilization work.

At the end of this pass:

- `cargo check -p codex-exec-server` succeeds.
- Non-documentation merge markers have been cleared from the actively used build/lint paths checked during this pass.
- The repository-wide version source and major product-facing manifests now point at `3.1.0`.
- Broader workspace validation is still in progress and not yet fully closed out.

## Implementation Details

### 1. `exec-server` restoration

The highest-priority blocker was `codex-rs/exec-server/src/server/handler.rs`.
It had been left in a hybrid state where the newer upstream delegation layer and the older local process-management implementation were both present in one file.

Actions taken:

- Replaced `codex-rs/exec-server/src/server/handler.rs` with the upstream delegation-based shape.
- Removed re-export duplication in `codex-rs/exec-server/src/lib.rs`.
- Fixed filesystem bootstrap wiring in `codex-rs/exec-server/src/server/filesystem.rs`.

Outcome:

- `exec-server` now builds again as `codex-exec-server v3.1.0`.
- The immediate syntax and type-collision errors in this subsystem are resolved.

### 2. Product version unified to `v3.1.0`

The repository had drift between multiple version authorities:

- Rust workspace was still effectively `0.0.0`.
- Root `VERSION` was `3.0.1`.
- Root, GUI, Tauri, and protocol-client package manifests still showed `3.0.0`.
- Some top-level docs still showed either `2.16.0` or `3.0.0`.

Actions taken:

- Updated `codex-rs/Cargo.toml` workspace package version to `3.1.0`.
- Updated root `VERSION` to `3.1.0`.
- Updated `version-metadata.json`:
  - `fork_version` -> `3.1.0`
  - `upstream_base` -> `3.1.0`
  - `release_date` -> `2026-03-22`
- Updated manifests:
  - `package.json`
  - `gui/package.json`
  - `packages/protocol-client/package.json`
  - `codex-rs/tauri-gui/package.json`
  - `codex-rs/tauri-gui/src-tauri/tauri.conf.json`
- Updated major release-facing docs:
  - `README.md`
  - `CHANGELOG.md`
  - `releases/RELEASE_NOTES.md`
  - `CLAUDE.md`
  - `AGENTS.md`

Notes:

- Display-facing references were aligned to `v3.1.0`.
- Internal package/crate versions were aligned to `3.1.0`.
- Protocol/schema/database/setup-version style counters were intentionally not changed.

### 3. Build/lint merge artifact cleanup

Several non-code infrastructure files still contained unresolved conflict markers and would have interfered with downstream checks.

Actions taken:

- Resolved `MODULE.bazel` to the newer LLVM dependency line.
- Resolved `MODULE.bazel.lock` to the matching LLVM lock entries.
- Resolved `justfile` conflict around `argument-comment-lint` and retained both:
  - prebuilt path
  - source path
- Resolved `tools/argument-comment-lint/run.sh`.
- Replaced `tools/argument-comment-lint/src/bin/argument-comment-lint.rs` with the clean upstream implementation.
- Cleaned `tools/argument-comment-lint/README.md`.
- Reconciled the root `package.json` conflict while preserving custom scripts:
  - `audit`
  - `security:check`
  - `deps:update`
  - `version:sync`
  - `version:check`
  - `write-hooks-schema`

## Files Updated In This Pass

### Core stabilization

- `codex-rs/exec-server/src/server/handler.rs`
- `codex-rs/exec-server/src/lib.rs`
- `codex-rs/exec-server/src/server/filesystem.rs`

### Version unification

- `codex-rs/Cargo.toml`
- `VERSION`
- `version-metadata.json`
- `package.json`
- `gui/package.json`
- `packages/protocol-client/package.json`
- `codex-rs/tauri-gui/package.json`
- `codex-rs/tauri-gui/src-tauri/tauri.conf.json`
- `README.md`
- `CHANGELOG.md`
- `releases/RELEASE_NOTES.md`
- `CLAUDE.md`
- `AGENTS.md`

### Build and lint infrastructure

- `MODULE.bazel`
- `MODULE.bazel.lock`
- `justfile`
- `tools/argument-comment-lint/run.sh`
- `tools/argument-comment-lint/src/bin/argument-comment-lint.rs`
- `tools/argument-comment-lint/README.md`

## Validation Results

### Successful

- `cargo check -p codex-exec-server`
  - Result: success
  - Notes: completed with warnings only, no blocking errors.

- Merge marker scan excluding documentation/vendor/template false positives
  - Result: non-documentation active build/lint paths cleared.

### In progress / not fully closed

- `cargo run --bin codex -- --version`
  - Status: started, but still in a large rebuild when this log was written.
  - Intended verification: final emitted CLI version should be `3.1.0`.

- Broader targeted workspace check:
  - Command family:
    - `cargo check -p codex-core -p codex-cli -p codex-mcp-server -p codex-deep-research -p codex-tui -p codex-tui-app-server`
  - Status: initiated after `exec-server` recovery, not fully closed during this pass.

## Remaining Work

1. Let the broad workspace compile finish and record the next real blocker, if any.
2. Confirm `cargo run --bin codex -- --version` prints `3.1.0`.
3. Run:
   - `just fmt`
   - `just argument-comment-lint`
   - `just clippy`
4. If dependency/lock drift is still present after the broader pass, run:
   - `just bazel-lock-update`
   - `just bazel-lock-check`
5. Decide whether to normalize remaining historical documentation merge markers under `docs/`.
   - They are not currently part of the active build path, but they should eventually be cleaned for repository hygiene.

## Notes For The Next Pass

- `git status` on this branch/worktree is still heavily modified because this is an in-progress upstream integration branch; do not treat the dirty state as accidental.
- `AGENTS.md` was rewritten into a clean conflict-free working form to remove merge markers and align top-level guidance with `3.1.0`.
- The new `v3.1.0` versioning source of truth should be considered:
  - root `VERSION`
  - `version-metadata.json`
  - Rust workspace package version in `codex-rs/Cargo.toml`

## Next Pass Progress Update

### Validation sequencing applied

- Switched to serialized Cargo execution to avoid package/build-directory lock contention.
- Killed stale overlapping `cargo` processes from earlier parallel attempts before restarting `cargo run --bin codex -- --version`.
- Re-ran version verification as the single active Cargo-heavy command.

### Additional blockers fixed in this pass

- `codex-rs/features/src/lib.rs`
  - Removed stale `crate::auth`, `crate::config`, and `CONFIG_TOML_FILE` imports left over from the old core-era implementation.
  - Restored upstream `apps_enabled_for_auth` visibility/signature.
  - Removed the reinjected legacy body fragment from `from_sources`.
  - Restored upstream `normalize_dependencies`.
  - Cleaned the test module footer back to plain `#[cfg(test)] mod tests;`.

- `codex-rs/core/src/lib.rs`
  - Removed `pub mod auth;` because `codex_login` is already re-exported as `auth`.
  - This fixed the missing-module error for `core/src/auth.rs`.

- `codex-rs/core/src/exec.rs`
  - Removed a duplicated `consume_output`/`consume_truncated_output` splice that left the file with an unclosed delimiter.
  - Restored the upstream `consume_output` function header.

- `codex-rs/core/src/exec_policy.rs`
  - Removed a reinjected legacy `runtime_sandbox_provides_safety` branch fragment inside `render_decision_for_unmatched_command`.
  - Kept the upstream Windows `environment_lacks_sandbox_protections` logic and closed the unmatched delimiter.

### Validation status after those fixes

- `cargo run --bin codex -- --version`
  - Advanced past:
    - `codex-features`
    - `codex-exec-server`
  - Then advanced into `codex-core`, surfacing and clearing blockers in:
    - `core/src/lib.rs`
    - `core/src/exec.rs`
    - `core/src/exec_policy.rs`
  - Status at the time of this update:
    - still running as the single serialized Cargo job after the latest `exec_policy` fix
    - no confirmation yet that the final emitted CLI version string has reached `3.1.0`

### Current guidance

- Continue using exactly one Cargo-heavy command at a time until `cargo run --bin codex -- --version` completes.
- Once version output is confirmed, run the targeted workspace check:
  - `cargo check -p codex-core -p codex-cli -p codex-mcp-server -p codex-deep-research -p codex-tui -p codex-tui-app-server`
- Do not reopen already fixed adapter-layer merges in `features`, `core/src/lib.rs`, `core/src/exec.rs`, or `core/src/exec_policy.rs` unless a new compiler error proves further semantic drift.

### Latest blocker after `mcp_tool_call` recovery

- `cargo run --bin codex -- --version`
  - Advanced through:
    - `codex-features`
    - `codex-exec-server`
    - `codex-core` initialization and approval-path modules
  - Current blocking file:
    - `codex-rs/core/src/plugins/manager.rs`
  - Current failure shape:
    - multiple mismatched and unclosed delimiters
    - affected regions include plugin install flow, startup task wiring, and test helpers near the bottom of the file
  - Interpretation:
    - this is another merge splice in a high-touch core module, not a new versioning failure
    - the serialized `cargo run --bin codex -- --version` workflow is still the correct driver because it is surfacing the next real blocker cleanly

### `plugins/manager.rs` recovery progress

- Repaired [manager.rs](C:\Users\downl\Desktop\codex-main-upstream-sync\codex-rs\core\src\plugins\manager.rs) upstream-first in these areas:
  - removed duplicate `Feature` imports and duplicate `FEATURED_PLUGIN_IDS_CACHE_TTL`
  - removed stale `STASHED:` fragments from the marketplace/plugin type definitions
  - restored `PluginsManager` method flow for:
    - `featured_plugin_ids_for_config`
    - `install_plugin`
    - `install_plugin_with_remote_sync`
    - `install_resolved_plugin`
    - `sync_plugins_from_remote(..., additive_only: bool)`
    - `list_marketplaces_for_config`
    - `read_plugin_for_config`
    - `maybe_start_plugin_startup_tasks_for_config`
  - removed inline broken `mod tests { ... }` content and restored the footer to the external test module:
    - `#[cfg(test)]`
    - `#[path = "manager_tests.rs"]`
    - `mod tests;`

### Disk-space workaround for serialized validation

- After `manager.rs` recovery, `cargo run --bin codex -- --version` no longer failed on the original delimiter errors in that file.
- The next failure was environmental rather than semantic:
  - `os error 112` while writing `.rmeta` files under `codex-rs/target/debug`
- Observed disk state:
  - `C:` free space was about `6.69 GB`
  - `codex-rs/target/debug` was about `15.53 GB`
  - `codex-rs/target/debug/incremental` was about `7.30 GB`
- Deletion of local build artifacts was blocked by policy, so the validation flow was redirected instead of cleaning in place.
- Active workaround:
  - build is now being rerun with `CARGO_TARGET_DIR=F:\codex-targets\codex-main-upstream-sync`
  - `F:` had about `95 GB` free, so this avoids the `C:` capacity ceiling without mutating repo-tracked state

---

_Generated during the 2026-03-22 upstream sync completion pass._
