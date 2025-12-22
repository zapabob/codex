# Essential Commands for Codex Development

## Build & Check Commands
```powershell
# Change to codex-rs directory
cd codex-rs

# Check compilation (use --all-features for full check)
cargo check --all-features

# Build CLI
cargo build -p codex-cli

# Build TUI
cargo build -p codex-tui

# Release build
cargo build --release -p codex-cli -p codex-tui
```

## Code Quality Commands
```powershell
# Format code
just fmt
# or
cargo fmt -- --config imports_granularity=Item

# Run clippy (linting)
just clippy
# or
cargo clippy --all-features --tests

# Auto-fix clippy issues
just fix
# or
cargo clippy --fix --all-features --tests --allow-dirty
```

## Testing Commands
```powershell
# Run all tests (requires cargo-nextest)
just test
# or
cargo nextest run --no-fail-fast

# Run specific test
cargo test --package codex-core test_name

# Run integration tests
cargo test --test integration_test
```

## Installation Commands
```powershell
# Install CLI
cargo install --path codex-rs/cli --force

# Install TUI
cargo install --path codex-rs/tui --force

# Check installation
codex --version
codex --help
```

## Development Workflow Commands
```powershell
# Run codex CLI
just c --version
# or
cargo run --bin codex -- --version

# Run TUI
just tui
# or
cargo run --bin codex -- tui

# Run MCP server
just mcp-server-run
```

## Git & Project Management
```powershell
# Check git status
git status

# View differences
git diff merge-upstream-2025-12-20..origin/main

# Commit changes
git add .
git commit -m "fix: resolve build errors"

# Push to remote
git push origin merge-upstream-2025-12-20
```

## Windows PowerShell Specific Notes
- PowerShell doesn't support `&&` chaining like bash
- Use separate commands or `;` for sequencing
- Use `Get-ChildItem` instead of `ls`
- Use `Select-String` instead of `grep`
- Path separators: use backslashes `\` or forward slashes `/`

## Performance Optimization
```powershell
# Fast incremental builds
$env:CARGO_TARGET_DIR = "target/fast"
cargo build --release

# With sccache (if installed)
$env:RUSTC_WRAPPER = "sccache"
cargo build --release
```