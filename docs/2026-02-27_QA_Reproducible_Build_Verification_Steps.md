# QA Reproducible Build Verification Steps (2026-02-27)

## Scope
- Requested build command under test:
  - `cargo build -p codex-cli --features custom-features -j 6`
- Working directory:
  - `codex-rs`
- Goal:
  - Reproducible QA evidence for `exit code = 0`, `0 warnings`, and `0 errors`.

## Preconditions
1. Open PowerShell from repository root:
   - `C:\Users\downl\Desktop\codex-main`
2. Confirm toolchain availability:
   - `cargo --version`
   - `rustc --version`
3. Capture repository identity for reproducibility:
   - `git rev-parse HEAD`
   - `git status --short`

## Reproducible Execution Steps
1. Define run metadata and output path.
```powershell
$RunTag = Get-Date -Format "yyyyMMdd_HHmmss"
$OutDir = "logs/build/verify_$RunTag"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
```

2. Run the requested build command and capture stdout/stderr separately.
```powershell
Push-Location codex-rs
cargo build -p codex-cli --features custom-features -j 6 `
  1> "..\$OutDir\cargo_build_codex-cli.stdout.log" `
  2> "..\$OutDir\cargo_build_codex-cli.stderr.log"
$BuildExit = $LASTEXITCODE
Pop-Location
```

3. Produce one combined log file for audit/readability.
```powershell
$Combined = "$OutDir\cargo_build_codex-cli.log"
"## stdout" | Set-Content $Combined -Encoding UTF8
Get-Content "$OutDir\cargo_build_codex-cli.stdout.log" | Add-Content $Combined
"" | Add-Content $Combined
"## stderr" | Add-Content $Combined
Get-Content "$OutDir\cargo_build_codex-cli.stderr.log" | Add-Content $Combined
```

4. Count warning/error matches with fixed, repeatable patterns.
```powershell
$WarnCount = (Select-String -Path $Combined -Pattern "(^|\s)warning(\[|:)" -AllMatches).Matches.Count
$ErrCount = (Select-String -Path $Combined -Pattern "(^|\s)error(\[|:)" -AllMatches).Matches.Count
```

5. Write a machine-readable summary.
```powershell
$Summary = [ordered]@{
  run_tag = $RunTag
  command = "cargo build -p codex-cli --features custom-features -j 6"
  working_directory = "codex-rs"
  exit_code = $BuildExit
  warning_count = $WarnCount
  error_count = $ErrCount
  pass_zero_warning_zero_error = (($BuildExit -eq 0) -and ($WarnCount -eq 0) -and ($ErrCount -eq 0))
}
$Summary | ConvertTo-Json | Set-Content "$OutDir\summary.json" -Encoding UTF8
$Summary
```

## Pass/Fail Criteria
- PASS requires all:
  - Exit code is `0`
  - `warning_count = 0`
  - `error_count = 0`
- Otherwise: FAIL (attach captured logs and summary JSON as evidence).

## Required Artifacts
- `logs/build/verify_<timestamp>/cargo_build_codex-cli.stdout.log`
- `logs/build/verify_<timestamp>/cargo_build_codex-cli.stderr.log`
- `logs/build/verify_<timestamp>/cargo_build_codex-cli.log`
- `logs/build/verify_<timestamp>/summary.json`

