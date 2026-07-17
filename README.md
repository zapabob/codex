# Codex Zapabob

Codex Zapabob is a Windows-ready fork of OpenAI Codex CLI built around three
practical advantages: GPT-5.6-family model and API support, local execution with
Ollama, and Git4D spatial code workflows for VR/AR through WebXR. It tracks the
official Codex release line while preserving goals, skills, plugin mentions,
app-server helpers, Python SDK goal APIs, and guarded Windows installation.

> Current release line: `v3.4.0` (2026-07-17)
> Official base: `rust-v0.144.5` plus upstream main `71448a29e7343b9613eaea620fcdbd196aed2af0`

<!-- version-sync:start -->
> **Current release:** v3.4.0 (2026-07-17) | canonical source `VERSION` | fork/upstream mapping in `version-metadata.json`.
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
- latest observed upstream main commit: `71448a29e7343b9613eaea620fcdbd196aed2af0`
- latest observed upstream main commit time: 2026-07-16T20:05:04Z

## Zapabob Extensions

The fork keeps the additions that are useful for serious local operation. Its
model surfaces and migration guidance understand the GPT-5.6 family while
following the current official API contract. The `codex-ollama` provider keeps
private and offline-capable local model workflows available. Git4D combines
repository time, author, branch, and path data with desktop, VR, and AR sessions
through the app-server bridge and WebXR-compatible clients.

Those capabilities sit alongside thread goals and goal status UI, skill and
plugin discovery, Python SDK goal operations, release metadata synchronization,
and guarded Windows build/install automation.

The extension rule is simple: keep official Codex behavior as the base, keep
custom behavior only where it adds durable local value, and expose custom APIs
through the closest current official surface.

## Build And Install Locally

One reproducible Windows release build command is:

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
