# Codex Zapabob

Codex Zapabob is a Windows-ready fork of OpenAI Codex CLI. It tracks the official
Codex release line while keeping the local extensions that matter for long-running
agent work: goals, skills, plugin mentions, Git4D/VR-AR workflows, app-server
helpers, Python SDK goal APIs, and fast local build/install scripts.

> Current release line: `v3.4.0` (2026-07-16)
> Official base: `rust-v0.144.5` plus upstream main `cbc83d961e8132bfff4d340ab8342d181b79e95e`

<!-- version-sync:start -->
> **Current release:** v3.4.0 (2026-07-16) · canonical source `VERSION` · fork/upstream mapping in `version-metadata.json`.
> Legacy v2.x release notes are archived under `releases/legacy/v2.x/RELEASE_NOTES.md`.
<!-- version-sync:end -->

## What This Release Carries

This line was rebased onto the latest official Codex release observed on
2026-07-16 and keeps the custom behavior that is not yet equivalent upstream.
When an upstream feature is equivalent, the upstream implementation is preferred;
when the fork still has a practical advantage, the fork behavior is kept behind
the current official API shape.

- Bumps the fork semantic version to `3.4.0` because this line adds official upstream capabilities while keeping backward-compatible zapabob extension behavior.
- latest official release: `rust-v0.144.5`
- release publication time: 2026-07-16T02:54:48Z
- latest observed upstream main commit: `cbc83d961e8132bfff4d340ab8342d181b79e95e`
- latest observed upstream main commit time: 2026-07-16T00:00:00Z

## Zapabob Extensions

The fork keeps the additions that are useful for serious local operation:
thread goals and goal status UI, skill discovery and skill mention popups,
plugin catalog and plugin mention UX, Git4D and VR/AR helper skills, Python SDK
goal operations, app-server protocol helpers, release metadata sync, and
Windows-first build/install automation.

The extension rule is simple: keep official Codex behavior as the base, keep
custom behavior only where it adds durable local value, and expose custom APIs
through the closest current official surface.

## Build And Install Locally

For a fast 4-core Windows release build:

```powershell
cd codex-rs
cargo +1.95.0 build --release -p codex-cli -j 4
```

After the build, run the repository helper from the repository root. It stops
Codex CLI processes outside the Microsoft Store app directory, then atomically
replaces the installed CLI binary while leaving Codex App running:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\install_with_kill.ps1 `
  -SourcePath .\codex-rs\target\release\codex.exe `
  -TargetPath "$HOME\.cargo\bin\codex.exe" `
  -ProcessNames codex `
  -ExcludePathPrefixes 'C:\Program Files\WindowsApps\OpenAI.Codex_' `
  -Force
```

Verify the installed binary with `codex --version` and `codex --help`.

## Upstream Sync Workflow

The merge is intentionally scripted so future upstream pulls are repeatable:

```bash
python scripts/resolve_merge_conflicts.py --rule "*=upstream" --paths <conflicted-files>
node scripts/sync-version.mjs
node scripts/sync-version.mjs --check
```

Use upstream when the feature is equivalent. Keep a fork patch only when it
preserves a documented Zapabob advantage, and then adapt it to the latest
official API before release.

## Repository Map

- `codex-rs/`: Rust workspace for CLI, TUI, app-server, protocol, state, tools,
  skills, plugins, and supervisor crates.
- `sdk/python/`: Python SDK and app-server/goal client examples.
- `plugins/zapabob-legacy-suite/`: local skill and MCP compatibility bundle.
- `tools/`: fast build, copy/install, and release helper scripts.
- `scripts/`: upstream sync, version sync, release preparation, and migration tools.
- `site/github-pages/`: static GitHub Pages guide.

## Verification Checklist

Before publishing a release, run the targeted Rust checks for changed crates,
sync the version files, build with `-j 4`, verify `codex --version`, and confirm
the intended branch and tag state before publishing release artifacts.

This repository is licensed under the [Apache-2.0 License](LICENSE).
