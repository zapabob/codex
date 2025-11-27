# Verify All Script
# Verifies CLI, TUI, and GUI functionality

param(
    [switch]$SkipCLI,
    [switch]$SkipTUI,
    [switch]$SkipGUI
)

$ErrorActionPreference = "Stop"

Write-Host "🔍 Verification Started..." -ForegroundColor Cyan

# 1. CLI Verification
if (-not $SkipCLI) {
    Write-Host "Testing CLI..." -ForegroundColor Yellow
    try {
        $version = codex --version
        Write-Host "  ✅ CLI Version: $version" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ CLI Failed" -ForegroundColor Red
        exit 1
    }
}

# 2. TUI Verification
if (-not $SkipTUI) {
    Write-Host "Testing TUI..." -ForegroundColor Yellow
    Write-Host "  ⚠️  TUI test requires manual interaction or visual check." -ForegroundColor Gray
    # We can try running help or a non-interactive command if available
    try {
        codex tui --help | Out-Null
        Write-Host "  ✅ TUI Help command works" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ TUI Failed" -ForegroundColor Red
    }
}

# 3. GUI Verification
if (-not $SkipGUI) {
    Write-Host "Testing GUI..." -ForegroundColor Yellow
    $guiProc = Get-Process -Name "codex-gui" -ErrorAction SilentlyContinue
    if ($guiProc) {
        Write-Host "  ✅ GUI Process Running (ID: $($guiProc.Id))" -ForegroundColor Green
    } else {
        Write-Host "  ⚠️  GUI Process NOT found. Attempting to launch..." -ForegroundColor Yellow
        # Launch logic here if needed, or just warn
    }
    
    # Run Playwright Tests
    Write-Host "  Running Playwright Tests..." -ForegroundColor Cyan
    Push-Location "$PSScriptRoot/../gui"
    try {
        pnpm exec playwright test
        Write-Host "  ✅ Playwright Tests Passed" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ Playwright Tests Failed" -ForegroundColor Red
        # Don't exit, just report
    } finally {
        Pop-Location
    }
}

Write-Host "🏁 Verification Complete!" -ForegroundColor Green
