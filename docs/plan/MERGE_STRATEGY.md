# Upstream Merge Strategy

## Goal

Keep this fork close to `openai/codex` by default, preserve only fork-only value that upstream still does not offer, and move legacy GUI functionality onto official plugin and app-server seams.

Current target:

- baseline tag: `rust-v0.121.0`
- released: April 15, 2026
- post-tag hardening window: through April 17, 2026 on `upstream/main`

## Security Gate

Treat the writable-root hardening from `fix: reduce writable root (#17947)` as mandatory. Do not preserve any fork change that reintroduces writable-root drift or equivalent sandbox-boundary regressions.

## Driver

Use [`scripts/upstream_sync.py`](/C:/Users/downl/Desktop/codex-main/scripts/upstream_sync.py) as the only merge entrypoint.

Default analysis run:

```powershell
python scripts/upstream_sync.py
```

Analysis plus no-commit merge:

```powershell
python scripts/upstream_sync.py --merge
```

The driver:

1. fetches refs and tags
2. verifies the baseline and upstream refs
3. creates an integration branch ref from the configured base branch
4. diffs `rust-v0.121.0` against `upstream/main`
5. classifies paths
6. optionally runs `git merge --no-commit --no-ff`
7. resolves conflicts through [`scripts/resolve_merge_conflicts.py`](/C:/Users/downl/Desktop/codex-main/scripts/resolve_merge_conflicts.py)
8. writes Markdown and JSON reports

## Classification Policy

### `upstream-first`

Use upstream by default for:

- `codex-rs/**`
- CLI, protocol, app-server, and plugin seams
- repo-local marketplace and plugin infrastructure

### `upstream-plus-reinject`

Only reinject local value for:

- fork-only orchestration behavior
- retained DeepResearch backend logic
- other fork code that still has no upstream equivalent

### `plugin-migrate`

Move legacy GUI behavior to plugins for:

- `gui/**`
- `codex-gui-x/**`
- `codex-rs/gui/**`
- `codex-rs/tauri-gui/**`

### `retire-after-parity`

Delete after parity instead of preserving:

- virtual OS surfaces
- custom computer-operation flows
- custom OS-control surfaces

### `keep-fork`

Keep local-only operational assets where upstream does not own the surface:

- `_docs/**`
- local sync and maintenance scripts
- local agent definitions and skills

## Plugin Migration Path

Repo-local migration now lives at:

- [`.agents/plugins/marketplace.json`](/C:/Users/downl/Desktop/codex-main/.agents/plugins/marketplace.json)
- [`plugins/zapabob-legacy-suite/.codex-plugin/plugin.json`](/C:/Users/downl/Desktop/codex-main/plugins/zapabob-legacy-suite/.codex-plugin/plugin.json)

Use the official plugin APIs:

- `plugin/list`
- `plugin/read`
- `plugin/install`
- mention path `plugin://zapabob-legacy-suite@zapabob-repo-local`

## Verification

Use `scripts/upstream_sync.py` reports as the authoritative closeout signal for this fork. Legacy verify scripts remain convenience helpers only.

On native Windows, final full-workspace verification should be run in a session with Developer Mode or equivalent symlink privilege. If `cargo test --workspace` stops in `v8` build setup with `os error 1314`, treat that as an environment prerequisite blocker and resume the full validation in a privileged Windows session.

At minimum, verify:

- sync classification tests in `scripts/test/test_upstream_sync.py`
- app-server plugin discovery tests
- app-server plugin read tests
- CLI `gui-x` deprecation tests

Recommended commands:

```powershell
python -m unittest scripts.test.test_upstream_sync
cd codex-rs
cargo test -p codex-cli gui_x
cargo test -p codex-app-server plugin_list_discovers_repo_local_migration_bundle -- --exact
cargo test -p codex-app-server plugin_read_returns_repo_local_migration_bundle_details -- --exact
```
