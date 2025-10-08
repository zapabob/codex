# Repository Guidelines

## Project Structure & Module Organization
- `codex-cli/` delivers the Node.js CLI; shipped binaries live in `codex-cli/bin/`.
- `codex-rs/` is the Rust workspace housing crates prefixed with `codex-` (for example, `codex-core`, `codex-tui`, `codex-mcp-server`). Integration assets such as `justfile`, `scripts/`, and documentation for Rust live alongside these crates.
- `sdk/typescript/` contains the TypeScript SDK with source in `src/`, Jest specs in `tests/`, and runnable examples in `samples/`.
- Human-facing docs are organized under `docs/`, while `_docs/` and `scripts/` hold internal automation and authoring utilities.

## Build, Test, and Development Commands
- Install dependencies with `pnpm install --frozen-lockfile` for JavaScript packages and `rustup target add`/`cargo fetch` via `just install` for Rust.
- Run the Rust CLI locally using `just codex -- <args>` or targeted binaries such as `just tui`.
- Format Rust code with `just fmt`; apply lint fixes using `just fix -p <crate>`. For TypeScript, prefer `pnpm --filter @openai/codex-sdk run lint` and `pnpm --filter @openai/codex-sdk run format`.
- Execute tests with `cargo test -p <crate>` (or `cargo test --all-features` when touching shared crates) and `pnpm --filter @openai/codex-sdk test` for the SDK.

## Coding Style & Naming Conventions
- Rust code follows standard `rustfmt` output and keeps crate and module names snake_case prefixed by `codex-`. Ratatui UI code should prefer Stylize helpers as captured in `codex-rs/tui/styles.md` and wrap text using utilities in `tui/src/wrapping.rs`.
- TypeScript uses ESLint + Prettier; stick to 2-space indentation, retain module boundaries, and export names in camelCase or PascalCase based on runtime or type definitions.
- Keep configuration files ASCII-only unless a pre-existing locale requires otherwise.

## Testing Guidelines
- Favor `cargo nextest` via `just test` for Rust integration passes; when modifying snapshot-driven TUI views, rerun `cargo test -p codex-tui` and review pending snapshots with `cargo insta pending-snapshots -p codex-tui` before accepting updates.
- TypeScript tests rely on Jest; trigger coverage with `pnpm --filter @openai/codex-sdk run coverage` to validate new features. Co-locate new specs under `tests/` mirroring the `src/` structure.
- Tests should use `pretty_assertions::assert_eq` in Rust and descriptive `describe`/`it` blocks in Jest; name files `<feature>_tests.rs` or `<feature>.test.ts` for clarity.

## Commit & Pull Request Guidelines
- Follow Conventional Commit prefixes (evident in history: `feat:`, `docs:`, `fix:`) and write messages in the imperative mood summarizing the change and scope.
- Ensure formatting and targeted tests pass before opening a PR; include links to related issues, highlight updated snapshots, and attach CLI/TUI screenshots when UI output changes.
- Keep PRs focused on a single logical change, note any feature flags or environment variables touched, and call out manual steps for reviewers.

## Multi-Agent & Deep Research Strategy
- Treat the primary reviewer agent as the coordinator and rely on `codex-supervisor` to fan out focused sub-agent work (for example, TypeScript CLI ownership, Rust core changes, and documentation updates).
- When a task requires broader context gathering, spin up the Deep Research flow (`codex-deep-research`) before coding so that implementation details are grounded in verified references.
- Ensure sub-agents exchange concise status updates through the supervisor so that findings from Deep Research or build tooling immediately inform coding decisions.
- Prefer automation over manual steps—add supervisor playbooks or reusable prompts whenever you notice repeatable Node.js or Rust workflows.

## CI/CD & PR Readiness
- Keep the repository green: run `just fmt`, `just fix -p <crate>`, `cargo test --all-features`, and `pnpm --filter @openai/codex-sdk test` before requesting reviews.
- Validate release builds locally with `just codex -- --version` and the relevant `cargo build --release` targets to confirm the supervisor and deep-research binaries ship correctly.
- Mirror CI expectations: if a workflow generates artifacts (for example, the CLI bundle or supervisor snapshots), reproduce the steps locally and attach summaries or diffs in the PR description.
- Capture the outcome of deep-research investigations in the PR body so reviewers can see the evidence behind architectural or dependency choices.

## Pull Request Handoff Checklist
- ✅ All Node.js and Rust formatting and linting jobs succeed locally and match CI settings.
- ✅ Test matrix covers supervisor, deep-research, and any newly introduced sub-agent integrations.
- ✅ PR description lists affected crates/packages, highlights CI/CD impacts, and links to research notes or design docs.
- ✅ Rollout or migration steps (if any) are scripted via `just` targets or documented so the release pipeline can apply them automatically.
