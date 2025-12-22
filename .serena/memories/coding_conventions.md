# Coding Conventions & Style Guide

## Rust Code Style
- **Edition**: Rust 2024
- **Formatting**: Use `cargo fmt` with `imports_granularity=Item`
- **Linting**: Use `cargo clippy --all-features --tests`
- **Auto-fix**: Use `cargo clippy --fix --allow-dirty` for automatic fixes

## Naming Conventions
- **Crates**: Prefixed with `codex-` (e.g., `codex-core`, `codex-tui`)
- **Functions**: snake_case
- **Types/Structs**: PascalCase
- **Constants**: SCREAMING_SNAKE_CASE
- **Modules**: snake_case

## Code Quality Rules
- **Clippy**: All warnings must pass (zero error policy)
- **Tests**: Use `cargo nextest` for faster testing
- **Documentation**: Include doc comments for public APIs
- **Error Handling**: Use `anyhow::Result` for application errors

## Async Programming
- Use `tokio` runtime
- Prefer async traits and functions
- Use `async move` when capturing variables
- Handle cancellation with `CancellationToken`

## TUI Style (ratatui)
- Use Stylize trait helpers: `.red()`, `.green()`, `.bold()`, etc.
- Prefer `Line::from(vec![...])` over manual construction
- Use computed styles for dynamic styling
- Avoid hardcoded white color, use default foreground

## Testing
- Use `pretty_assertions` for better diff output
- Prefer deep equality comparisons (`assert_eq!` on entire objects)
- Use `core_test_support` utilities for integration tests
- Mock SSE responses using `responses.rs` helpers

## Build Optimization
- Use incremental builds with `CARGO_TARGET_DIR`
- Enable `RUSTC_WRAPPER=sccache` when available
- Use workspace-level dependencies to avoid duplication
- Split large builds into smaller packages when possible

## Windows-Specific Considerations
- Use forward slashes `/` or double backslashes `\\` in paths
- Handle case-insensitive filesystem
- Use appropriate line endings (CRLF for Windows files)
- Test sandboxing and permission systems thoroughly

## Git Workflow
- Use feature branches for development
- Squash commits before merging
- Keep commit messages descriptive but concise
- Use `.gitignore` to exclude build artifacts (`target/`, `*.tmp`)

## Performance Guidelines
- Use `Arc` and `Mutex` for shared mutable state
- Prefer `HashMap` over `BTreeMap` unless ordered iteration needed
- Use `Cow` for strings that may be borrowed or owned
- Profile with `cargo flamegraph` when optimizing