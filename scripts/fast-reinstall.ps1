param(
    [switch]$SkipBuild,
    [switch]$SkipInstall,
    [string]$InstallDir = "$env:USERPROFILE\.cargo\bin"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$workspaceRoot = Join-Path $repoRoot "codex-rs"
$installHelper = Join-Path $PSScriptRoot "install_with_kill.ps1"
$sourceBinary = Join-Path $workspaceRoot "target\release\codex.exe"
$targetBinary = Join-Path $InstallDir "codex.exe"
$codexAppPrefix = "C:\Program Files\WindowsApps\OpenAI.Codex_"

Write-Host "[*] Fast reinstall started" -ForegroundColor Cyan

if (-not $SkipBuild) {
    Write-Host "[*] Building codex-cli release binary" -ForegroundColor Yellow
    Push-Location $workspaceRoot
    try {
        cargo build --release -p codex-cli
    }
    finally {
        Pop-Location
    }
}

if (-not $SkipInstall) {
    Write-Host "[*] Installing codex.exe with path-aware process filtering" -ForegroundColor Yellow
    & $installHelper `
        -SourcePath $sourceBinary `
        -TargetPath $targetBinary `
        -ProcessNames @("codex", "codex-tui", "codex-gui", "opencode") `
        -ExcludePathPrefixes @($codexAppPrefix) `
        -Force
}

Write-Host "[OK] Fast reinstall complete" -ForegroundColor Green
