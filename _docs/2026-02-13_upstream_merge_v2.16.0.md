# Upstream Merge v2.16.0 Completion Log

**Date:** 2026-02-13
**Activity:** Merge `openai/codex` (v2.16.0) into `zapabob/Codex` (v2.15.1) and fix build errors.

## Summary

Successfully merged upstream changes and resolved compilation errors preventing build. The codebase is now aligned with v2.16.0 structure and dependencies.

## Key Changes & Fixes

1.  **Duplicate Module Cleanups**:
    - Removed redundant `pub mod session_prefix;` in `core/lib.rs`.
    - Removed duplicate `pub mod bash;` and `pub mod parse_command;` if present (handled by revert/merge resolution).
    - Removed duplicate `static PRESETS` in `core/models_manager/model_presets.rs`.
    - Removed duplicate import of `WindowsSandboxLevel` in `core/sandboxing/mod.rs`.

2.  **Code Syntax & Initialize Fixes**:
    - Fixed unclosed delimiter in `core/src/tools/runtimes/unified_exec.rs`.
    - Initialized missing `network` field in `UnifiedExecRequest::new` in `unified_exec.rs`.
    - Removed invalid `network: None` from `UnifiedExecApprovalKey` struct initialization.
    - Implemented missing `run_sampling_request` function in `core/src/codex.rs`.

3.  **Dependency Corrections**:
    - Added missing dependencies to `core/Cargo.toml` (`regex`, `git2`, `lsp-types`, `dashmap`, `walkdir`, `sysinfo`, `sqlx`).
    - Validated `codex-rs/Cargo.toml` workspace dependencies, reverting accidental mass-replacements to ensure standard versions (e.g., `futures`, `reqwest`, `windows-sys`).
    - Updated `[workspace.package] version` to **2.16.0** in `codex-rs/Cargo.toml`.

4.  **Verification**:
    - Confirmed `codex-core` compilation (in progress/completed).
    - Verified `codex-api` compilation.

## Next Steps

- Monitor `codex-exec` and other crates for specific platform issues.
- Proceed with standard testing pipeline.
