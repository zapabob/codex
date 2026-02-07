# Codebase Hygiene Fixes - 2026-02-07

## Overview

Resolved several code quality and maintenance issues across the codebase, including missing metadata, linter warnings in CI/CD, and dead code.

## Changes

### VS Code Extensions

- **`extensions/package.json`**: Added missing `icon` properties to the `codex-agents` views (`agentsList`, `agentStatus`, `researchResults`) using standard codicons (`$(beaker)`, `$(dashboard)`, `$(book)`).
- **`extensions/windsurf-extension/package.json`**: Verified activation events are clean.

### GitHub Workflows

- **`.github/workflows/issue-labeler.yml`**: Refactored to map `secrets.CODEX_OPENAI_API_KEY` to an environment variable and reference it via `${{ env.CODEX_OPENAI_API_KEY }}` to resolve "Context access might be invalid" warnings.
- **`.github/workflows/rust-release.yml`**: Refactored signing and deployment steps to map secrets (`AZURE_TRUSTED_SIGNING_*`, `APPLE_*`, `DEV_WEBSITE_VERCEL_DEPLOY_HOOK_URL`) to environment variables, resolving persistent linter warnings.

### Rust TUI

- **`codex-rs/tui/src/history_cell/tests.rs`**:
  - Removed unused `image_block` function.
  - Removed unused imports `mcp_types::ContentBlock` and `mcp_types::ImageContent`.

## Status

All fixes implemented and verified structurally.
