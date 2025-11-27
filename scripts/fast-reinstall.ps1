# Fast Reinstall Script
# Kills processes, builds, and installs Codex

param(
    [switch]$SkipBuild,
    [switch]$SkipInstall
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Fast Reinstall Started..." -ForegroundColor Cyan

# 1. Kill Processes
Write-Host "🔪 Killing processes..." -ForegroundColor Yellow
$processes = @("codex", "codex-gui", "codex-service", "Codex")
foreach ($proc in $processes) {
    Get-Process -Name $proc -ErrorAction SilentlyContinue | Stop-Process -Force
}
Start-Sleep -Seconds 1

# 2. Build
if (-not $SkipBuild) {
    Write-Host "🔨 Building Codex..." -ForegroundColor Yellow
    
    # Rust Build (Incremental)
    Push-Location "$PSScriptRoot/../codex-rs"
    try {
        # Using sccache if available is handled by config, just run cargo build
        # We use release for "overwrite install" as requested, but maybe dev is faster?
        # User said "overwrite install", usually implies the final binary.
        # Let's stick to the build-unified.ps1 logic but simplified or just call it.
        # Calling build-unified.ps1 is safer to ensure all artifacts are there.
        & .\build-unified.ps1 -Release -SkipClean
    }
    finally {
        Pop-Location
    }
}

# 3. Install
if (-not $SkipInstall) {
    Write-Host "📦 Installing..." -ForegroundColor Yellow
    Push-Location "$PSScriptRoot/../codex-rs"
    try {
        & .\install-unified.ps1
    }
    finally {
        Pop-Location
    }
}

Write-Host "✅ Fast Reinstall Complete!" -ForegroundColor Green
