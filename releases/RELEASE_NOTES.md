# Codex v3.1.0 Release Notes

> **Current release document** for the v3.1.0 line.
> Legacy v2.x release notes are archived at `releases/legacy/v2.x/RELEASE_NOTES.md`.

## Canonical Versioning

- **Canonical source**: root `VERSION`
- **Fork version**: `3.1.0`
- **Upstream base**: `3.1.0`
- **Release date**: 2026-03-22

## What changed in v3.1.0

### Version governance

- Root `VERSION` is now the single source of truth for release-visible versioning.
- `version-metadata.json` defines `fork_version` and `upstream_base` so tooling can distinguish fork releases from upstream alignment.
- `scripts/sync-version.mjs` regenerates synced version displays and validates drift with `--check`.

### Repository docs and manifests

- Synced the root `package.json`, Rust workspace version, and `packages/protocol-client/package.json` to v3.1.0.
- Rebuilt the root changelog and release notes as **current release** documents for the v3.x line.
- Archived the older v2.x release notes so the latest release is unambiguous.

## Sync procedure

```bash
# 1) edit VERSION (and version-metadata.json upstream_base if needed)
node scripts/sync-version.mjs

# 2) verify no drift remains
node scripts/sync-version.mjs --check
```
