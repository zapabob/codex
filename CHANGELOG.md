# Changelog

## v3.2.0 - 2026-05-22

### Changed

- Synchronized the fork with official Codex `rust-v0.133.0` and upstream main `24faf49b2a70f8813e522afb0e701add9b15b0bd`.
- Bumped the fork semantic version from `3.1.0` to `3.2.0` across release-visible manifests and Rust workspace metadata.
- Rewrote the root `README.md` around the v3.2.0 upstream base, Git4D app-server bridge, release channels, and verification commands.
- Updated Git4D capability detection so AR/VR requests fall back to desktop mode when no `OPENXR_RUNTIME_JSON` runtime is configured.
- Replaced VR/AR initialization stdout writes with tracing so app-server JSON-RPC output stays protocol-clean.

### Added

- Python overlay merge driver at `scripts/upstream_overlay_merge.py` with Markdown and JSON reports in `_docs/`.
- Official app-server Git4D methods for capabilities, session start, session list, session watch, and session unwatch.
- App-server integration coverage for Git4D capability fallback, session round trips, buffered watch replay, and unwatch behavior.
- Official `thread/search` protocol and backing rollout/thread-store search support.
- Official plugin hook behavior with the separate plugin-hooks feature flag removed upstream.

### Security

- Reviewed the official Codex advisory `GHSA-w5fx-fh39-j5rw` during the upstream sync.
- Pulled the official dependency and lockfile refresh surface from the latest upstream workspace before applying zapabob-specific reinjection.

## v3.1.0 - 2026-04-18

### Changed

- Rewrote the root `README.md` around a public-facing TL;DR, release-channel guidance, architecture snapshot, and migration status.
- Promoted `scripts/upstream_sync.py` as the authoritative upstream-first sync and release-closeout driver for this fork.
- Clarified that the official Codex surfaces now define the product baseline: `codex`, `codex app`, `codex app-server`, and plugins.
- Documented the native Windows verification boundary around the `v8` symlink privilege prerequisite so release status is not confused with a repo regression.

### Added

- Stable release channel `v3.1.0-stable.0` from `release/3.1.0-stable`.
- Mainline release channel `v3.1.0` from `main`.
- Tracked repo-local marketplace at [`.agents/plugins/marketplace.json`](/C:/Users/downl/Desktop/codex-main/.agents/plugins/marketplace.json).
- Repo-local plugin bundle at [`plugins/zapabob-legacy-suite/.codex-plugin/plugin.json`](/C:/Users/downl/Desktop/codex-main/plugins/zapabob-legacy-suite/.codex-plugin/plugin.json).

### Deprecated

- `codex gui-x` no longer launches the legacy GUI stack.
- Legacy GUI trees remain in the repository only until plugin parity is verified.
- Fork-only virtual OS, computer-operation, and OS-control surfaces are deprecated.

## Historical Release Notes

- legacy release notes: `releases/legacy/v2.x/RELEASE_NOTES.md`
- legacy changelog: `releases/legacy/v2.x/CHANGELOG.md`
