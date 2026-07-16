# Changelog

Current canonical version: **v3.3.0**.
Canonical source: `VERSION`. Fork/upstream disambiguation lives in `version-metadata.json`.

## Current Release — v3.3.0 (2026-06-25)

> This root changelog is the **current release line only**.
> Legacy v2.x history has been moved to `releases/legacy/v2.x/CHANGELOG.md` to make the latest release immediately obvious.

### Changed

- Adopted **root `VERSION`** as the single canonical version source for release-visible artifacts.
- Added a machine-readable version metadata file with `fork_version` and `upstream_base` for fork/upstream conflict resolution.
- Added generated sync automation for root/package manifests, workspace Cargo version, README display version, release notes, and changelog headers.

### Docs

- Split legacy **v2.x** history from the current **v3.x** release line.
- Marked the root release notes and changelog as the current release documents.
- Standardized the displayed release version across README badges and package metadata.

## Historical Release Lines

- **v2.x archive**: `releases/legacy/v2.x/CHANGELOG.md`
- **Legacy release notes**: `releases/legacy/v2.x/RELEASE_NOTES.md`
