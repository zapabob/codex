# Codex v3.1.0 Release Notes

> Current release document for the v3.1.0 line.
> Legacy v2.x release notes are archived at `releases/legacy/v2.x/RELEASE_NOTES.md`.

## Release Channels

- Stable channel: `v3.1.0-stable.0` from `release/3.1.0-stable`
- Mainline channel: `v3.1.0` from `main`
- Primary asset: Windows `tar.gz` bundle containing `codex.exe`, `README.md`, `LICENSE`, and `VERSION`

## Canonical Versioning

- Canonical source: root `VERSION`
- Fork version: `3.1.0`
- Upstream base: `rust-v0.121.0`
- Release date: 2026-04-18

## What Changed In v3.1.0

### Product positioning

- Rewrote the root README around a public-facing TL;DR, release-channel guidance, architecture snapshot, and migration status.
- Clarified that the official Codex surfaces are the baseline product story for this fork: `codex`, `codex app`, `codex app-server`, and plugins.
- Tightened the public explanation of what remains fork-specific and what is intentionally being retired or migrated.

### Fork-only highlights

- DeepResearch remains available as a plugin-facing workflow on top of the official Codex surfaces.
- Git4D remains positioned as an optional visualization capability instead of a permanent core fork seam.
- VR and AR remain opt-in capabilities with graceful fallback when no device or WebXR path is available.
- Repo-local plugin marketplace support remains the main vehicle for shipping distinctive zapabob functionality without carrying a permanently divergent core.

### Release alignment

- Aligned the root package manifests and release-visible metadata to `3.1.0`.
- Published both a stable branch release and a mainline release for the same release line.
- Standardized the Windows release bundle naming around the tag-specific `tar.gz` artifact.

### Upstream-first operations

- Kept `scripts/upstream_sync.py` as the authoritative sync and closeout driver.
- Preserved the native Windows verification note that full `cargo test --workspace` can still be gated by the `v8` symlink privilege prerequisite.
