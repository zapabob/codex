# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Codex is an AI coding assistant platform forked from OpenAI/codex with extensive enhancements (zapabob extensions). The core is a Rust workspace (`codex-rs/`) with 69 crates, plus a Next.js GUI (`gui/`), a Node.js CLI (`codex-cli/`), and supporting services.

**Version**: 2.16.0 | **Rust edition**: 2024 | **Toolchain**: 1.93.0

## Build & Development Commands

### Rust (primary — all commands run from `codex-rs/`)

```bash
# Task runner (just) — working directory is automatically set to codex-rs/
just fmt                        # Format code (cargo fmt with imports_granularity=Item)
just clippy                     # Lint all crates
just fix -p <crate>             # Auto-fix clippy issues for a specific crate
just test                       # Run all tests with cargo-nextest
just install                    # Fetch dependencies, show active toolchain
just codex                      # Run codex from source (cargo run --bin codex)
just exec "prompt"              # Run codex exec mode

# Direct cargo commands (from codex-rs/)
cargo build --release -p codex-cli          # Release build of CLI
cargo install --path cli --force            # Install globally
cargo test -p codex-tui                     # Test a single crate
cargo test --all-features -p codex-core     # Test with all features
cargo nextest run --no-fail-fast            # Fast test runner (preferred)
cargo clippy -p codex-cli -- -D warnings    # Lint a specific crate

# Snapshot tests (TUI uses insta)
cargo test -p codex-tui                     # Generate .snap.new files
cargo insta pending-snapshots -p codex-tui  # Review pending
cargo insta accept -p codex-tui             # Accept snapshots

# Dependency checks
cargo shear                                 # Find unused dependencies
cargo deny check                            # License/security audit
```

### Windows fast-build scripts (from codex-rs/)

```powershell
.\ultra-fast-build-install.ps1   # Fastest incremental build
.\clean-build-install.ps1        # Clean rebuild
```

### Bazel (alternative build system)

```bash
just bazel-codex                # Run codex via Bazel
just bazel-test                 # Run all Bazel tests
just bazel-remote-test          # Run tests with remote execution
just build-for-release          # Remote release builds
```

### GUI (Next.js — from gui/)

```bash
npm run dev          # Dev server (0.0.0.0)
npm run build        # Production build
npm run lint         # ESLint
npm test             # Playwright E2E tests
npm run test:headed  # Headed browser tests
```

### Root-level formatting (from repo root)

```bash
pnpm run format      # Prettier check (JSON, MD, YAML, JS)
pnpm run format:fix  # Prettier auto-fix
```

## Architecture

### Rust Workspace (`codex-rs/`)

The workspace follows a layered architecture. Key crates and their roles:

- **`cli`** — Binary entry point (`codex`). Subcommands: interactive TUI, `exec`, `delegate`, `agent-create`, `research`, `resume`, etc.
- **`tui`** — Terminal UI built with ratatui. Renders chat, execution cells, slash commands. Uses insta for snapshot testing.
- **`core`** — Central orchestration engine. Contains:
  - `agents/runtime.rs` — Sub-agent execution runtime
  - `codex.rs` — Main Codex session lifecycle
  - `orchestration/` — Resource management, task coordination
  - `tools/` — Tool specs and execution (shell, file ops, etc.)
  - `mcp_dynamic_loader/` — Dynamic MCP server loading
  - `models_manager/` — Model family detection, presets, capabilities
  - `sandboxing/` — Platform-specific sandbox (macOS: Seatbelt, Linux: Landlock+seccomp, Windows: Job Objects)
- **`backend-client`** — HTTP client for OpenAI API. Handles SSE streaming, retries, auth.
- **`protocol`** — Wire protocol types shared between CLI/TUI and app-server.
- **`app-server`** — WebSocket/SSE server for GUI communication. Implements `app-server-protocol`.
- **`app-server-protocol`** — Schema definitions (v2 protocol) for GUI<->server communication. Has `write_schema_fixtures` binary to regenerate vendored schemas.
- **`mcp-server`** — Codex as an MCP server for external tool integration.
- **`config`** — Configuration types (`~/.codex/config.toml` schema). Has `codex-write-config-schema` binary.
- **`state`** — SQLite-backed session/state persistence (via sqlx).
- **`exec` / `exec-server`** — Command execution layer.
- **`deep-research`** — Multi-source research with citations.
- **`supervisor`** — Agent supervision and lifecycle management.
- **`windows-sandbox-rs`** / `linux-sandbox` / `process-hardening` — Platform sandboxing.
- **`utils/`** — Shared utilities (paths, git, caching, sanitization, fuzzy-match, etc.).

### Data Flow

```
User → CLI/TUI → Core Runtime → Backend Client → OpenAI API
                      ↓
              MCP Servers ← Tool Execution ← Sandbox Layer
                      ↓
              App Server → GUI (WebSocket/SSE)
```

### GUI (`gui/`)

Next.js 14 app with React, MUI 7, Three.js for 3D visualization, WebXR for VR/AR support. Communicates with the Rust app-server over WebSocket.

### Sub-Agent System (`.codex/agents/`)

YAML-defined specialized agents (code-reviewer, test-gen, sec-audit, researcher, etc.) with token budgets and sandbox policies. Executed via `codex delegate` or `codex delegate-parallel`.

## Coding Conventions

### Rust

- **Clippy is strict**: `unwrap_used` and `expect_used` are denied (allowed in tests). See workspace lints in `codex-rs/Cargo.toml`.
- Use inline format args: `println!("User {name}")` not `println!("User {}", name)` (`uninlined_format_args = "deny"`).
- Prefer iterators/method references over explicit loops/closures (`redundant_closure_for_method_calls = "deny"`).
- TUI colors: Use ANSI colors only. `Color::Rgb`, `Color::Indexed`, hardcoded white/black/yellow are disallowed in clippy.toml.
- Enhanced thresholds: max 5 function args, cognitive complexity 15, max 300 lines/function, error size 128 bytes.
- `unsafe`, `transmute`, `null`, `uninitialized`, `zeroed` are disallowed.
- Format: `cargo fmt` with `edition = "2024"`. The `imports_granularity = "Item"` flag is passed via `just fmt` but not in rustfmt.toml (unstable).

### TypeScript/JavaScript

- Prettier: printWidth=80, semi=true, trailingComma="all", tabWidth=2.
- No `any` types, no `var`. Use `const` default, `async/await` over `.then()`.

### Integration tests

- Use `pretty_assertions::assert_eq` for better diffs.
- Use `core_test_support::responses` helpers for mock SSE responses.
- TUI: snapshot tests with `insta`.

## Sandbox Modes

Three security levels available via `--sandbox` flag:

| Mode | Description |
|------|-------------|
| `read-only` | Default. Read files only. |
| `workspace-write` | Write within workspace directory. |
| `danger-full-access` | Full system access (requires explicit opt-in). |

## Environment Variables

```
OPENAI_API_KEY     — Required for API access
RUST_LOG           — Log level (e.g., "info", "debug")
CODEX_CONFIG_PATH  — Override config location (default: ~/.codex/config.toml)
```

## Regenerating Schemas

```bash
just write-config-schema         # Regenerate config.toml JSON schema
just write-app-server-schema     # Regenerate app-server protocol schema
```

## CI

GitHub Actions workflows run: multi-platform Rust builds (Linux/macOS/Windows, x64/ARM64), clippy, fmt, nextest, cargo-deny, codespell, Bazel tests, Playwright GUI tests, and security audits. The CI test profile (`cargo-profile ci-test`) uses `opt-level = 0` and `debug = 1` for faster compilation.
