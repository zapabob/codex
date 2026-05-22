# Codex v3.2.0 Release Notes

> Current release document for the v3.2.0 line.
> Legacy v2.x release notes are archived at `releases/legacy/v2.x/RELEASE_NOTES.md`.

## Release Channels

- Stable channel: `v3.2.0-stable.0` from `release/3.2.0-stable`
- Mainline channel: `v3.2.0` from `main`
- Primary asset: Windows `tar.gz` bundle containing the executable, `README.md`, `LICENSE`, `VERSION`, release notes, and merge evidence
- Windows asset: `codex-v3.2.0-windows-x86_64.tar.gz`
- Windows asset SHA256: `E2508EC70DC888A0DA4AC1813124F7F9C3D5F65FF6F73067CFBFF5630207FF59`
- GitHub Pages: <https://zapabob.github.io/codex/>

## Canonical Versioning

- Canonical source: root `VERSION`
- Fork version: `3.2.0`
- Upstream release base: `rust-v0.133.0`
- Upstream main commit: `b14f11d3d2ca048bdae1872ef66087a2ce3f6b0c`
- Release date: 2026-05-22

## What Changed In v3.2.0

### Official Codex imports

- Adopted official Codex changes through `rust-v0.133.0` and upstream main `b14f11d3d2ca048bdae1872ef66087a2ce3f6b0c`, observed on 2026-05-22.
- Brought in Python SDK login, account, turn lifecycle, retry, streaming, and example updates.
- Brought in app-server protocol additions for plugin sharing, marketplace upgrade/remove, hooks, model provider capabilities, goals, process notifications, and Windows sandbox readiness.
- Brought in default-on goals, case-insensitive `thread/search`, and plugin hook behavior after the upstream feature flag removal.
- Brought in parallel read-only MCP tool calls, compact large tool-schema support, local `$ref`/`$defs` tool-schema support, extension-tool conversation history, and Node managed-proxy environment propagation.
- Brought in TUI startup, session picker, permission, status, and update-flow changes.
- Brought in Windows sandbox, V8, Bazel, CI, packaging, and release hardening changes.

### Zapabob extension retention

- Preserved Git4D as an optional app-server bridge instead of a separate product path.
- Preserved AR/VR capability handling with deterministic desktop fallback when no XR runtime is configured.
- Preserved repo-local plugin marketplace support for zapabob extension delivery.
- Preserved DeepResearch as a plugin-facing workflow layered onto official Codex surfaces.

### Protocol and tests

- Added `git4d/capabilities/read`.
- Added `git4d/session/start`.
- Added `git4d/session/list`.
- Added `git4d/session/watch`.
- Added `git4d/session/unwatch`.
- Added app-server integration tests covering fallback, session lifecycle, buffered event replay, and unwatch behavior.
- Kept VR/AR initialization off stdout so JSON-RPC framing remains valid.

### Security and maintenance

- Reviewed official advisory `GHSA-w5fx-fh39-j5rw` while syncing.
- Kept dependency and lockfile updates aligned with the official workspace before applying fork-specific versioning.
- Kept the fork semantic version at `3.2.0` to signal a compatible feature release, not a breaking major release.

## Verification Snapshot

Commands run during this release line:

```powershell
python -m py_compile scripts\upstream_overlay_merge.py scripts\resolve_merge_conflicts.py
cargo metadata --no-deps --format-version 1
cargo fmt --all
cargo test -p codex-app-server-protocol git4d -- --nocapture
cargo check -p codex-core -j 6
cargo check -p codex-app-server -j 6
cargo check -p codex-tui -j 6
cargo test -p codex-app-server git4d -- --nocapture
cargo build --release -p codex-cli -j 6
H:\codex-main-release-target-3.2.0\release\codex.exe --version
```

Release build result:

- `codex-cli 3.2.0`
- `releases/codex-v3.2.0-windows-x86_64.tar.gz`
- size: 85,263,562 bytes
- SHA256: `E2508EC70DC888A0DA4AC1813124F7F9C3D5F65FF6F73067CFBFF5630207FF59`
- GitHub Pages status: built and serving `Codex v3.2.0` from `gh-pages`

`just bazel-lock-update` and `just bazel-lock-check` passed. The full `just argument-comment-lint` command is blocked here by Bazel's Windows test-toolchain resolution, so the changed crates were checked with the prebuilt argument-comment linter.

Additional release gates are recorded in `_docs/2026-05-22_v3.2.0_upstream_sync_git4d_release.md`.
