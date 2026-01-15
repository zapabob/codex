set working-directory := "codex-rs"
set positional-arguments

# Display help
help:
    just -l

# `codex`
alias c := codex
codex *args:
    cargo run --bin codex -- "$@"

# `codex exec`
exec *args:
    cargo run --bin codex -- exec "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    cargo run --bin codex-file-search -- "$@"

# Build the CLI and run the app-server test client
app-server-test-client *args:
    cargo build -p codex-cli
    cargo run -p codex-app-server-test-client -- --codex-bin ./target/debug/codex "$@"

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item 2>/dev/null

fix *args:
    cargo clippy --fix --all-features --tests --allow-dirty "$@"

clippy:
    cargo clippy --all-features --tests "$@"

install:
    rustup show active-toolchain
    cargo fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install cargo-nextest` if you don't have it installed.
test:
    cargo nextest run --no-fail-fast

# Run the MCP server
mcp-server-run *args:
    cargo run -p codex-mcp-server -- "$@"

# Fast build system with incremental compilation
fast-build target="debug" *packages:
    python3 scripts/fast_build.py {{target}} {{packages}}

# Build and install with process killing
build-install *args:
    python3 scripts/build_and_install.py {{args}}

# Install with process killing (PowerShell)
install-kill source target:
    powershell -ExecutionPolicy Bypass -File scripts/install_with_kill.ps1 -SourcePath {{source}} -TargetPath {{target}} -Force

# Supervisor orchestrator (official Agents SDK)
supervisor *args:
    python3 tools/codex-supervisor/supervisor.py {{args}}

# Test supervisor package
test-supervisor:
    cd tools/codex-supervisor && python test_workflow.py

# Install supervisor as package
install-supervisor:
    cd tools/codex-supervisor && pip install -e .

# Test skills migration
test-skills:
    echo "Testing skills structure..."
    ls -la .codex/skills/
    echo "Testing architect skill..."
    cat .codex/skills/architect/SKILL.md | head -10

# Migrate agents to skills (official format)
migrate-agents:
    python3 scripts/migrate_to_skills.py
