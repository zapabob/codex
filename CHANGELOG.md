# Changelog

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
