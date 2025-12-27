# Simple GUI Launcher Script
# Usage: .\scripts\launch-gui-simple.ps1

$ErrorActionPreference = "Stop"

Write-Host "🚀 Codex GUI を起動中..." -ForegroundColor Cyan

# Check if codex command is available
$codexCmd = Get-Command codex -ErrorAction SilentlyContinue
if (-not $codexCmd) {
    Write-Host "❌ codex コマンドが見つかりません" -ForegroundColor Red
    Write-Host "   先に codex をインストールしてください: cargo install --path codex-rs/cli" -ForegroundColor Yellow
    exit 1
}

# Launch GUI using codex gui command
Write-Host "📱 GUI を起動します..." -ForegroundColor Yellow
codex gui
