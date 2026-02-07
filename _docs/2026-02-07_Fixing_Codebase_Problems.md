# Codebase Hygiene Fixes - 2026-02-07

## Overview

Resolved several code quality and maintenance issues across the codebase, including missing metadata, linter warnings in CI/CD, and dead code.

## Changes

### VS Code Extensions

- **`extensions/package.json`**: Added missing `icon` properties to the `codex-agents` views (`agentsList`, `agentStatus`, `researchResults`) using standard codicons (`$(beaker)`, `$(dashboard)`, `$(book)`).
- **`extensions/windsurf-extension/package.json`**: Verified activation events are clean.

### GitHub Workflows

- **`.github/workflows/issue-labeler.yml`**: Refactored to map `secrets.CODEX_OPENAI_API_KEY` to an environment variable at the **step level**.
- **`.github/workflows/rust-release.yml`**:
  - Resolved a structural error by removing the unsupported **workflow-level** `env` block for secrets.
  - Consolidated sign/deploy secrets into **Job-level** `env` blocks for both `build` and `release` jobs.
  - Reverted to standard **dot notation** (`secrets.KEY`) as the scoping fix treats the root cause of the invalid context access.

### Rust TUI

- **`codex-rs/tui/src/history_cell/tests.rs`**:
  - Removed unused `image_block` function.
  - Removed unused imports `mcp_types::ContentBlock` and `mcp_types::ImageContent`.

## Status

All fixes implemented and verified structurally. Any remaining "Context access might be invalid" warnings in the IDE are benign false positives from static analysis.
