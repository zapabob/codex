# Codex Project Overview

## Purpose
Codex is an AI-Native Operating System with 4D Git Visualization & VR/AR Support. It's a Rust-based AI development platform that integrates multiple AI models, MCP servers, and provides advanced features like sub-agent orchestration, deep research capabilities, and cross-platform compatibility.

## Tech Stack
- **Language**: Rust 2024 edition
- **Build System**: Cargo workspace with multiple crates
- **Key Dependencies**: 
  - Async runtime (tokio, async-trait)
  - MCP (Model Context Protocol) servers
  - OpenTelemetry for tracing
  - TUI libraries (ratatui)
  - HTTP clients (reqwest)
- **Platforms**: Windows, macOS, Linux, VR/AR support

## Extended Features (zapabob/codex)
- **Sub-Agent System**: Multi-agent orchestration with parallel execution (2.6x speedup)
- **Deep Research Engine**: Multi-source research with citation management (45x speedup)
- **MCP Server Extensions**: 7 additional tools and enhanced integration
- **Orchestration Framework**: Advanced async execution patterns
- **Cross-platform Sandboxing**: Enhanced Windows, macOS, Linux support

## Code Structure
- `core/`: Core functionality and orchestration
- `cli/`: Command-line interface
- `tui/`: Terminal user interface
- `app-server/`: Backend server
- `mcp-server/`: MCP server implementation
- `deep-research/`: Research engine
- `agents/`: Sub-agent system
- Various utility crates and integrations

## Development Commands
- `just fmt`: Format code with rustfmt
- `just fix`: Run clippy with auto-fix
- `just clippy`: Run clippy linting
- `just test`: Run tests with cargo-nextest
- `cargo check --all-features`: Check compilation
- `cargo build --release`: Release build
- `cargo install --path codex-rs/cli --force`: Install CLI

## Windows-Specific Notes
- Use PowerShell instead of bash
- Commands like `cd codex-rs && cargo check --all-features` should be split (PowerShell doesn't support &&)
- Build artifacts go to `target/` directory (should be in .gitignore)
- Use `cargo install` for installing binaries