# Simple codex-gui-x launcher via Codex CLI.
# Usage:
#   .\scripts\launch-gui-simple.ps1
#   .\scripts\launch-gui-simple.ps1 -Port 5174 -Attached

param(
    [string]$Host = "127.0.0.1",
    [int]$Port = 5173,
    [switch]$Attached
)

$ErrorActionPreference = "Stop"

Write-Host "Starting codex-gui-x via Codex CLI..." -ForegroundColor Cyan

$codexCmd = Get-Command codex -ErrorAction SilentlyContinue
if (-not $codexCmd) {
    Write-Host "Error: 'codex' command not found in PATH." -ForegroundColor Red
    Write-Host "Install it first (example): cargo install --path codex-rs/cli" -ForegroundColor Yellow
    exit 1
}

$args = @("gui-x", "--host", $Host, "--port", $Port)
if ($Attached) {
    $args += "--attached"
}

Write-Host ("Running: codex " + ($args -join " ")) -ForegroundColor Gray
& codex @args
