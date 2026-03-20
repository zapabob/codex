# v3.0.1 Release Notes

## Highlights

This release standardizes how the fork tracks and imports changes from `openai/codex`.

- `origin` is canonically defined as `https://github.com/zapabob/codex.git`.
- `upstream` is canonically defined as `https://github.com/openai/codex.git`.
- The tracked upstream branch is fixed to `upstream/main`.
- The tracked upstream release tag conventions are fixed to `rust-v*` (primary) and `v*` (secondary compatibility).
- The repository now keeps a machine-readable upstream intake record in `releases/upstream-sync.json`.
- The reproducible sync entrypoint is `scripts/sync-upstream.sh` or `just sync-upstream`.

## Upstream Intake Record

| Field | Value |
| --- | --- |
| `source.repository` | `https://github.com/openai/codex.git` |
| `source.branch` | `main` |
| `source.commit` | `668330acc12b8907ecd82bc15148e0a627246783` |
| `source.tag` | `null` |
| `recorded_at` | `2026-03-19T20:08:57Z` |

## Conflict Policy Summary

When upstream and custom code overlap, the repository now uses a documented policy:

1. Adopt the official implementation when feature parity is sufficient.
2. Re-inject only the demonstrated custom advantage in a follow-up commit.
3. Preserve clearly custom-only areas until an official equivalent exists.
4. Keep the provenance record updated whenever upstream is imported.

See `docs/repository-relationship.md` for the full policy and path-specific rules.
