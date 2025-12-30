# ビルド・インストール状態確認スクリプト

$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " ビルド・インストール状態確認" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# ビルド済みバイナリの確認
$releaseBinary = "codex-rs\target\release\codex.exe"
$debugBinary = "codex-rs\target\debug\codex.exe"

if (Test-Path $releaseBinary) {
    $fileInfo = Get-Item $releaseBinary
    Write-Host "✅ Release binary found" -ForegroundColor Green
    Write-Host "   Path: $releaseBinary" -ForegroundColor Gray
    Write-Host "   Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Gray
    Write-Host "   Modified: $($fileInfo.LastWriteTime)" -ForegroundColor Gray
} else {
    Write-Host "❌ Release binary not found" -ForegroundColor Red
}

Write-Host ""

if (Test-Path $debugBinary) {
    $fileInfo = Get-Item $debugBinary
    Write-Host "✅ Debug binary found" -ForegroundColor Green
    Write-Host "   Path: $debugBinary" -ForegroundColor Gray
    Write-Host "   Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Gray
    Write-Host "   Modified: $($fileInfo.LastWriteTime)" -ForegroundColor Gray
} else {
    Write-Host "⚠️  Debug binary not found" -ForegroundColor Yellow
}

Write-Host ""

# インストール済みバイナリの確認
$installedBinary = "$env:USERPROFILE\.cargo\bin\codex.exe"

if (Test-Path $installedBinary) {
    $fileInfo = Get-Item $installedBinary
    Write-Host "✅ Installed binary found" -ForegroundColor Green
    Write-Host "   Path: $installedBinary" -ForegroundColor Gray
    Write-Host "   Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Gray
    Write-Host "   Modified: $($fileInfo.LastWriteTime)" -ForegroundColor Gray
    Write-Host ""
    
    # バージョン確認
    Write-Host "Version check:" -ForegroundColor Yellow
    $versionOutput = codex --version 2>&1
    Write-Host $versionOutput -ForegroundColor Cyan
} else {
    Write-Host "❌ Installed binary not found" -ForegroundColor Red
    Write-Host "   Expected: $installedBinary" -ForegroundColor Gray
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
