# 高速開発ワークフロー自動化スクリプト
param(
    [string]$InstallPath = "$env:USERPROFILE\.cargo\bin\codex.exe",
    [string]$BuildProfile = "dev-fast",
    [switch]$NoBuild
)

$ErrorActionPreference = "Stop"

Write-Host "=== Codex CLI 高速インストール ===" -ForegroundColor Cyan

# 1. ビルド（オプション）
if (-not $NoBuild) {
    Write-Host "[1/4] 差分ビルド実行..." -ForegroundColor Yellow
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    
    cargo build -p codex-cli --profile $BuildProfile 2>&1 | ForEach-Object {
        if ($_ -match "Compiling|Finished") { Write-Host "  $_" -ForegroundColor Gray }
    }
    
    $sw.Stop()
    Write-Host "  ビルド時間: $($sw.Elapsed.TotalSeconds)秒" -ForegroundColor Green
}

# 2. プロセス確認・終了
Write-Host "[2/4] 既存プロセスをチェック..." -ForegroundColor Yellow
$process = Get-Process -Name "codex" -ErrorAction SilentlyContinue
if ($process) {
    Write-Host "  プロセス終了: PID $($process.Id)" -ForegroundColor Red
    $process | Stop-Process -Force
    Start-Sleep -Milliseconds 500
}

# 3. バイナリコピー
Write-Host "[3/4] バイナリをデプロイ..." -ForegroundColor Yellow
$source = "target\$BuildProfile\codex.exe"

if (-not (Test-Path $source)) {
    Write-Error "ビルド出力が見つかりません: $source"
    exit 1
}

# バックアップ作成
if (Test-Path $InstallPath) {
    $backup = "$InstallPath.backup"
    Copy-Item $InstallPath $backup -Force
    Write-Host "  バックアップ作成" -ForegroundColor Gray
}

# アトミックコピー
Copy-Item $source $InstallPath -Force
Write-Host "  コピー完了: $InstallPath" -ForegroundColor Green

# 4. 検証
Write-Host "[4/4] インストール検証..." -ForegroundColor Yellow
& $InstallPath --version 2>&1 | ForEach-Object {
    if ($_ -match "error") { 
        Write-Error "インストールに問題があります: $_"
    } else {
        Write-Host "  バージョン: $_" -ForegroundColor Green
    }
}

Write-Host "`n完了！所要時間を大幅に短縮しました。" -ForegroundColor Cyan
