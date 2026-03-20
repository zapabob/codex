# Repository Relationship: `zapabob/codex` ↔ `openai/codex`

## Purpose

This repository is operated as a fork that continuously evaluates and selectively imports upstream changes from `openai/codex`. The goal is to make that relationship reproducible, auditable, and machine-readable.

## Canonical Remote Definitions

The repository treats the following remotes as canonical:

| Remote | URL | Purpose |
| --- | --- | --- |
| `origin` | `https://github.com/zapabob/codex.git` | Fork where custom integration work lands. |
| `upstream` | `https://github.com/openai/codex.git` | Official source of upstream branch and release tags. |

### Branch Tracking Rule

- The only tracked upstream branch for synchronization is `upstream/main`.
- The integration target branch in this fork is `origin/main`.
- Local `main` must track `origin/main`.

### Tag Tracking Rule

Upstream tags are tracked under two explicit patterns:

1. **Primary release lineage**: `rust-v*`
2. **Secondary compatibility lineage**: `v*`

The primary lineage is the authoritative release stream for synchronization decisions. The secondary lineage is fetched for historical compatibility and documentation lookup.

To avoid ambiguity with local release tags, upstream tags are fetched into `refs/upstream-tags/*`.

## Machine-Readable Upstream Intake Record

The authoritative intake record is stored in `releases/upstream-sync.json`.

Required fields:

- `remotes.origin.url`
- `remotes.upstream.url`
- `remotes.upstream.tracked_branch`
- `remotes.upstream.release_tag_policy`
- `sync.target_branch`
- `sync.merge_strategy`
- `sync.recorded_at`
- `sync.source.repository`
- `sync.source.branch`
- `sync.source.commit`
- `sync.source.tag`

### Current Recorded Intake

| Field | Value |
| --- | --- |
| `sync.source.repository` | `https://github.com/openai/codex.git` |
| `sync.source.branch` | `main` |
| `sync.source.commit` | `668330acc12b8907ecd82bc15148e0a627246783` |
| `sync.source.tag` | `null` |
| `sync.recorded_at` | `2026-03-19T20:08:57Z` |

`sync.source.tag` is `null` when the imported upstream commit does not have an exact upstream tag. In that case, the commit hash is the authoritative provenance record.

## Reproducible Sync Workflow

The supported workflow is `scripts/sync-upstream.sh`, with `just` wrappers for convenience.

### Configure only

```bash
./scripts/sync-upstream.sh --configure-only
```

This command:

- sets or updates `origin` and `upstream`
- fixes the remote fetch refspecs
- ensures `main` tracks `origin/main`

### Refresh metadata without merge

```bash
./scripts/sync-upstream.sh --dry-run
```

This command:

- fetches `origin` and `upstream`
- refreshes `releases/upstream-sync.json`
- does not modify the working tree with a merge commit

### Import upstream into `main`

```bash
./scripts/sync-upstream.sh
```

This command is the supported equivalent of:

```bash
git fetch upstream --prune
git merge --no-ff upstream/main
```

In addition to the merge, it refreshes the machine-readable intake record.

## Conflict Policy for Custom Extensions

The repository maintains path-specific conflict handling rules for custom subsystems.

| Area | Default decision | Reinjection rule |
| --- | --- | --- |
| `codex-rs/deep-research/` | Preserve custom implementation unless upstream ships equivalent research orchestration and provider abstraction. | Adopt upstream interfaces first, then re-inject provider breadth, ranking, or workflow features that remain uniquely valuable. |
| `codex-rs/supervisor/` | Preserve custom supervision lifecycle logic until upstream provides equivalent orchestration controls. | Prefer upstream lifecycle primitives when equivalent, then re-add only missing resilience, observability, or policy hooks. |
| `.codex/skills/` | Preserve custom skill catalog and workflow assets. | If upstream adds an equivalent skill or capability, adopt the official packaging/layout and re-inject only custom prompts, templates, or automation that still differentiate the workflow. |
| Git4D / VR modules | Treat as fork-owned extensions by default because they are outside current upstream scope. | Re-inject only the extension-specific UX or hardware integration once an upstream base exists. |

### General Resolution Order

1. **Security and correctness first**: upstream security fixes and correctness fixes are accepted before evaluating custom behavior.
2. **Official base before custom layering**: when upstream offers a sufficient base implementation, land that implementation first.
3. **Minimal reinjection**: re-apply only the custom delta that provides a verified advantage.
4. **Preserve provenance**: update `releases/upstream-sync.json`, `CHANGELOG.md`, and release notes whenever upstream intake changes.

## Official-Equivalent Adoption Rule

The decision rule is:

> If the official implementation is functionally equivalent for the supported use case, adopt the official implementation and re-inject only the advantage that the custom implementation uniquely provides.

A change is considered **functionally equivalent** when all of the following are true:

1. The upstream implementation covers the required user-visible behavior.
2. The upstream implementation satisfies the repository's security and maintenance baseline.
3. The remaining custom delta can be expressed as a small additive patch, adapter, configuration layer, or follow-up commit.

A custom implementation should remain fork-specific only when at least one of the following is true:

- upstream has no equivalent feature
- upstream lacks a required integration surface
- removing the custom implementation would regress a supported workflow
- the custom feature is intentionally fork-scoped (for example Git4D or VR-specific modules)

## Documentation Rule

Do not describe the repository as merely “officially synced.” Instead, record synchronization using:

- imported upstream tag (or `null`)
- imported upstream commit hash
- recorded intake date in UTC

That data must match `releases/upstream-sync.json`.
