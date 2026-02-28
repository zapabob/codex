# QA Build Verification Draft (2026-02-27)

## Scope
- QA-only verification artifacts (no product code edits).
- Target commands:
  - `cargo build -p codex-cli --features custom-features -j 6` in `codex-rs`
  - `npm run build` in `codex-gui-x`

## Verification Runner
- Script path: `logs/run_build_verification.ps1`
- Current script status: timeout support added for concurrent-build safety; Windows GUI invocation fixed to use `npm.cmd` (not yet re-run after this fix).

## Captured Runs

### Run A (scripted)
- Artifact dir: `logs/build/verify_20260227_223213`
- Summary: `logs/build/verify_20260227_223213/summary.md`
- Results:
  - Rust build: `exit=124`, `timedOut=true`, stderr shows `Blocking waiting for file lock on artifact directory`.
  - GUI build: `exit=1`, failed in runner due `Start-Process npm` Win32 invocation issue (script bug fixed afterward).

### Run B (manual command capture)
- Artifact dir: `logs/build/verify_20260227_222446`
- Results:
  - Rust build (timeout-guarded capture): `exit=124`, timed out after 120s with repeated cargo lock waits.
    - Log: `logs/build/verify_20260227_222446/cargo_build_codex-cli.log`
  - GUI build (direct): `exit=2`, TypeScript compile failed.
    - Log: `logs/build/verify_20260227_222446/npm_build_codex-gui-x.log`
    - Error count: 33 TS errors
    - First failures include:
      - `src/App.tsx` lazy import/default export typing mismatches (`TS2322`)
      - `src/components/atoms/Button.tsx` and `Card.tsx` motion/MUI prop typing overload failures (`TS2769`)
      - multiple unused symbol/type-only import issues (`TS6133`, `TS1484`)

## Commands Executed and Outcomes
1. `powershell -NoProfile -ExecutionPolicy Bypass -File logs/run_build_verification.ps1`
   - Initial script parse issue was corrected (`param` placement).
2. `npm run build` (in `codex-gui-x`, direct execution for evidence capture)
   - Outcome: `exit=2`, TS compile failures logged.
3. Timeout-guarded cargo capture for `cargo build -p codex-cli --features custom-features -j 6` (in `codex-rs`)
   - Outcome: `exit=124`, lock wait + timeout.
4. `powershell -NoProfile -ExecutionPolicy Bypass -File logs/run_build_verification.ps1 -CargoTimeoutSec 120 -GuiTimeoutSec 300`
   - Outcome in Run A: generated summary/logs; cargo timed out; GUI step hit runner invocation bug (`npm` vs `npm.cmd`), now patched.

## Ready-To-Run After Fixes
- Use:
  - `powershell -NoProfile -ExecutionPolicy Bypass -File logs/run_build_verification.ps1 -CargoTimeoutSec 120 -GuiTimeoutSec 300`
- Expected outputs:
  - `logs/build/verify_<timestamp>/summary.md`
  - `logs/build/verify_<timestamp>/summary.json`
  - `logs/build/verify_<timestamp>/cargo_build_codex-cli.log`
  - `logs/build/verify_<timestamp>/npm_build_codex-gui-x.log`
