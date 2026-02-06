# Infrastructure Fixes Log (2026-02-06)

## Overview

Addressed infrastructure debt, compilation errors, and workflow warnings as part of the Phase 3 handover process.

## Changes

### 1. CLI Compilation Fixes

- Added `rmcp` workspace dependency to `codex-cli/Cargo.toml`.
- Fixed `codex-cli/src/chrome_cmd.rs`:
  - Resolved `unresolved import rmcp`.
  - Corrected field access from `content.content` to `content.raw` for `Annotated<RawContent>`.
  - Fixed `rmcp::model::ClientCapabilities` initialization (missing `elicitation`).
  - Fixed `rmcp::model::Implementation` initialization (missing `icons`, `title`, `website_url`).
  - Updated `ProtocolVersion` conversion.
  - Removed unused `mcp_types::RequestId`.

### 2. MCP Server Warnings

- Fixed `codex-mcp-server/tests/common/lib.rs`:
  - Fixed import path for `create_shell_command_sse_response` in tests.
  - Resolved unused import warnings in `tests/suite/codex_tool.rs`.
  - Suppressed unused code warnings (`#[allow(unused)]`) in:
    - `src/external_mcp_manager.rs`
    - `src/webhook_tool.rs`
    - `src/webhook_tool_handler.rs`
    - `src/external_mcp_tool_handler.rs`

### 3. GitHub Workflows

- Updated `rust-ci.yml`:
  - Removed invalid `version` input from `cargo-shear` field.
  - Replaced deprecated `save-always: true` with explicit `actions/cache/restore` and `actions/cache/save` steps.

## Verification

- Running `cargo check --workspace --all-targets` to ensure clean build.
