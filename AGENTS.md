# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

- Crate names are prefixed with `codex-`. For example, the `core` folder's crate is named `codex-core`
- When using format! and you can inline variables into `{}`, always do that.
- Install any commands the repo relies on (for example `just`, `rg`, or `cargo-insta`) if they aren't already available before running instructions here.
- Never add or modify any code related to `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` or `CODEX_SANDBOX_ENV_VAR`.
  - You operate in a sandbox where `CODEX_SANDBOX_NETWORK_DISABLED=1` will be set whenever you use the `shell` tool. Any existing code that uses `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` was authored with this fact in mind. It is often used to early exit out of tests that the author knew you would not be able to run given your sandbox limitations.
  - Similarly, when you spawn a process using Seatbelt (`/usr/bin/sandbox-exec`), `CODEX_SANDBOX=seatbelt` will be set on the child process. Integration tests that want to run Seatbelt themselves cannot be run under Seatbelt, so checks for `CODEX_SANDBOX=seatbelt` are also often used to early exit out of tests, as appropriate.
- Always collapse if statements per <https://rust-lang.github.io/rust-clippy/master/index.html#collapsible_if>
- Always inline format! args when possible per <https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args>
- Use method references over closures when possible per <https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure_for_method_calls>
- Avoid bool or ambiguous `Option` parameters that force callers to write hard-to-read code such as `foo(false)` or `bar(None)`. Prefer enums, named methods, newtypes, or other idiomatic Rust API shapes when they keep the callsite self-documenting.
- When you cannot make that API change and still need a small positional-literal callsite in Rust, follow the `argument_comment_lint` convention:
  - Use an exact `/*param_name*/` comment before opaque literal arguments such as `None`, booleans, and numeric literals when passing them by position.
  - Do not add these comments for string or char literals unless the comment adds real clarity; those literals are intentionally exempt from the lint.
  - If you add one of these comments, the parameter name must exactly match the callee signature.
- When possible, make `match` statements exhaustive and avoid wildcard arms.
- When writing tests, prefer comparing the equality of entire objects over fields one by one.
- When making a change that adds or changes an API, ensure that the documentation in the `docs/` folder is up to date if applicable.
- If you change `ConfigToml` or nested config types, run `just write-config-schema` to update `codex-rs/core/config.schema.json`.
- If you change Rust dependencies (`Cargo.toml` or `Cargo.lock`), run `just bazel-lock-update` from the repo root to refresh `MODULE.bazel.lock`, and include that lockfile update in the same change.
- After dependency changes, run `just bazel-lock-check` from the repo root so lockfile drift is caught locally before CI.
- Bazel does not automatically make source-tree files available to compile-time Rust file access. If you add `include_str!`, `include_bytes!`, `sqlx::migrate!`, or similar build-time file or directory reads, update the crate's `BUILD.bazel` (`compile_data`, `build_script_data`, or test data) or Bazel may fail even when Cargo passes.
- Do not create small helper methods that are referenced only once.
- Avoid large modules:
  - Prefer adding new modules instead of growing existing ones.
  - Target Rust modules under 500 LoC, excluding tests.
  - If a file exceeds roughly 800 LoC, add new functionality in a new module instead of extending
    the existing file unless there is a strong documented reason not to.
  - This rule applies especially to high-touch files that already attract unrelated changes, such
    as `codex-rs/tui/src/app.rs`, `codex-rs/tui/src/bottom_pane/chat_composer.rs`,
    `codex-rs/tui/src/bottom_pane/footer.rs`, `codex-rs/tui/src/chatwidget.rs`,
    `codex-rs/tui/src/bottom_pane/mod.rs`, and similarly central orchestration modules.
  - When extracting code from a large module, move the related tests and module/type docs toward
    the new implementation so the invariants stay close to the code that owns them.
- When running Rust commands (e.g. `just fix` or `cargo test`) be patient with the command and never try to kill them using the PID. Rust lock can make the execution slow, this is expected.
  - If a file exceeds roughly 800 LoC, add new functionality in a new module instead of extending the existing file unless there is a strong documented reason not to.
  - This rule applies especially to high-touch files that already attract unrelated changes, such as `codex-rs/tui/src/app.rs`, `codex-rs/tui/src/bottom_pane/chat_composer.rs`, `codex-rs/tui/src/bottom_pane/footer.rs`, `codex-rs/tui/src/chatwidget.rs`, `codex-rs/tui/src/bottom_pane/mod.rs`, and similarly central orchestration modules.
  - When extracting code from a large module, move the related tests and module/type docs toward the new implementation so the invariants stay close to the code that owns them.


Codex is an AI coding assistant platform forked from OpenAI/codex with extensive enhancements (zapabob extensions). The core is a Rust workspace (`codex-rs/`) with 69 crates, plus a Next.js GUI (`gui/`), a Node.js CLI (`codex-cli/`), and supporting services.

**Version**: 3.1.0 | **Rust edition**: 2024 | **Toolchain**: 1.93.0

## Build & Development Commands

Also run `just argument-comment-lint` to ensure the codebase is clean of comment lint errors.

## The `codex-core` crate

Over time, the `codex-core` crate (defined in `codex-rs/core/`) has become bloated because it is the largest crate, so it is often easier to add something new to `codex-core` rather than refactor out the library code you need so your new code neither takes a dependency on, nor contributes to the size of, `codex-core`.

To that end: **resist adding code to codex-core**!

Particularly when introducing a new concept/feature/API, before adding to `codex-core`, consider whether:

- There is an existing crate other than `codex-core` that is an appropriate place for your new code to live.
- It is time to introduce a new crate to the Cargo workspace for your new functionality. Refactor existing code as necessary to make this happen.

Likewise, when reviewing code, do not hesitate to push back on PRs that would unnecessarily add code to `codex-core`.

## TUI style conventions

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

- When a change lands in `codex-rs/tui` and `codex-rs/tui_app_server` has a parallel implementation of the same behavior, reflect the change in `codex-rs/tui_app_server` too unless there is a documented reason not to.

- Use concise styling helpers from ratatui’s Stylize trait.
  - Basic spans: use `"text".into()`
  - Styled spans: use `"text".red()`, `"text".green()`, `"text".magenta()`, `"text".dim()`, etc.
  - Prefer these over constructing styles with `Span::styled` and `Style` directly.
  - Example: patch summary file lines
    - Desired: `vec!["  └ ".into(), "M".red(), " ".dim(), "tui/src/app.rs".dim()]`

# Dependency checks
cargo shear                                 # Find unused dependencies
cargo deny check                            # License/security audit
```

### Windows fast-build scripts (from codex-rs/)

```powershell
.\ultra-fast-build-install.ps1   # Fastest incremental build
.\clean-build-install.ps1        # Clean rebuild
```
