# Production Logging Setup Script for Rust2024
# 本番環境向けログ設定スクリプト

param(
    [string]$LogLevel = "info",
    [string]$LogFormat = "json",
    [string]$LogDir = "$env:CODEX_HOME\logs"
)

$ErrorActionPreference = "Stop"

Write-Host "📊 Production Logging Setup" -ForegroundColor Cyan
Write-Host "===========================" -ForegroundColor Cyan
Write-Host ""

# Create log directory if it doesn't exist
if (-not (Test-Path $LogDir)) {
    New-Item -ItemType Directory -Path $LogDir -Force | Out-Null
    Write-Host "✅ Created log directory: $LogDir" -ForegroundColor Green
}

# Set environment variables for production logging
$env:RUST_LOG = "$LogLevel"
$env:CODEX_LOG_DIR = $LogDir
$env:CODEX_LOG_FORMAT = $LogFormat

Write-Host "Log Configuration:" -ForegroundColor Yellow
Write-Host "  Level: $LogLevel" -ForegroundColor Gray
Write-Host "  Format: $LogFormat" -ForegroundColor Gray
Write-Host "  Directory: $LogDir" -ForegroundColor Gray
Write-Host ""

Write-Host "Environment variables set:" -ForegroundColor Yellow
Write-Host "  RUST_LOG=$env:RUST_LOG" -ForegroundColor Gray
Write-Host "  CODEX_LOG_DIR=$env:CODEX_LOG_DIR" -ForegroundColor Gray
Write-Host "  CODEX_LOG_FORMAT=$env:CODEX_LOG_FORMAT" -ForegroundColor Gray
Write-Host ""

Write-Host "✅ Production logging setup completed" -ForegroundColor Green
Write-Host ""
Write-Host "To use structured JSON logging, set:" -ForegroundColor Cyan
Write-Host "  `$env:RUST_LOG='info'" -ForegroundColor Gray
Write-Host "  `$env:CODEX_LOG_FORMAT='json'" -ForegroundColor Gray
