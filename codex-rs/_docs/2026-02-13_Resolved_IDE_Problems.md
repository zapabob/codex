# 2026-02-13_Resolve_IDE_Problems

## Abstract

Resolved compilation errors and lint warnings across multiple crates (`codex-backend-client`, `codex-core`, `codex-mcp-server`) as identified by the IDE.

## Changes

### 1. codex-backend-client

- **File**: `backend-client/src/client.rs`
- **Issue**: "cannot move out of shared reference" on lines 316, 320, and 331.
- **Fix**: Cloned the boxed values before mapping them.

### 2. codex-core

- **File**: `core/src/mcp_dynamic_loader/tests.rs`
- **Issue**: Incorrect usage of enum variant `SandboxPolicy::ReadOnly` as a unit variant.
- **Fix**: Updated to use `SandboxPolicy::new_read_only_policy()` constructor.

### 3. codex-mcp-server

- **File**: `mcp-server/src/message_processor.rs`
- **Issue**: Unused import `rmcp::model::CallToolResult`.
- **Fix**: Removed the import.
- **File**: `mcp-server/src/outgoing_message.rs`
- **Issue**: Unused `cwd` variables in tests.
- **Fix**: Prefixed with `_` to suppress warnings.

## Verification

- **Command**: `cargo check -p codex-backend-client -p codex-core -p codex-mcp-server`
- **Result**: Success (Exit code: 0)
