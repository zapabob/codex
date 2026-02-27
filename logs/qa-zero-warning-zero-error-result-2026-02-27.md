# QA Zero-Warning / Zero-Error Result Sheet (2026-02-27)

## Metadata
- QA owner: `codex-agent`
- Date: `2026-02-28`
- Repo root: `C:\Users\downl\Desktop\codex-main`
- Baseline commit SHA: `dbf6542d9edb9f2b171826fc4c0db333911e218f`
- Build command: `cargo build -p codex-cli --features custom-features -j 6`
- Working directory: `codex-rs`
- Build env overrides used for stable Windows execution:
  - `RUSTC_WRAPPER=''`
  - `CARGO_INCREMENTAL=1`
  - `CARGO_TARGET_DIR=target/codex-cli-fast`

## Execution Record

| Run | Start | End | Exit code | Warning count | Error count | Verdict |
|---|---|---|---:|---:|---:|---|
| full-build | 2026-02-28 02:54 JST | 2026-02-28 03:13 JST | 0 | 0 | 0 | PASS |
| incremental-build | 2026-02-28 03:14 JST | 2026-02-28 03:20 JST | 0 | 0 | 0 | PASS |

## Evidence Paths
- full-build combined log: `logs/build-codex-cli-custom-features-j6-final.log`
- incremental-build combined log: `logs/build-codex-cli-custom-features-j6-incremental.log`

## Verification Rules
- PASS only if all conditions are true:
  - exit code is `0`
  - warning count is `0`
  - error count is `0`

## Final Status
- Overall status: `PASS`
- Notes:
  - First successful run was a cold build in dedicated target dir.
  - Second run in same target dir completed in `6m12s` (incremental).
