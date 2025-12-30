# バックグラウンドビルドとインストールスクリプト
# 進捗をログファイルに記録

$ErrorActionPreference = "Continue"
$logPath = ".cursor\debug.log"

function Write-DebugLog {
    param([string]$location, [string]$message, [hashtable]$data = @{})
    $logEntry = @{
        id = "log_$(Get-Date -Format 'yyyyMMddHHmmss')_$(New-Guid)"
        timestamp = [DateTimeOffset]::Now.ToUnixTimeMilliseconds()
        location = $location
        message = $message
        data = $data
        sessionId = "debug-session"
        runId = "build-install"
    } | ConvertTo-Json -Compress
    Add-Content -Path $logPath -Value $logEntry -Encoding UTF8 -ErrorAction SilentlyContinue
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Codex 高速差分ビルド & インストール" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

Set-Location codex-rs

$buildStart = Get-Date
Write-DebugLog -location "build-install-background.ps1:1" -message "ビルド開始" -data @{startTime = $buildStart.ToString("yyyy-MM-dd HH:mm:ss")}

Write-Host "Building release binary (incremental build)..." -ForegroundColor Yellow
Write-Host "  This may take several minutes..." -ForegroundColor Gray
Write-Host ""

# ビルド実行
cargo build --release -p codex-cli

$buildEnd = Get-Date
$buildDuration = ($buildEnd - $buildStart).TotalSeconds
Write-DebugLog -location "build-install-background.ps1:2" -message "ビルド完了" -data @{duration = $buildDuration; exitCode = $LASTEXITCODE}

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    Set-Location ..
    exit $LASTEXITCODE
}

Write-Host "✅ Build completed in $([math]::Round($buildDuration, 2)) seconds" -ForegroundColor Green
Write-Host ""

# インストール
$installStart = Get-Date
Write-DebugLog -location "build-install-background.ps1:3" -message "インストール開始" -data @{startTime = $installStart.ToString("yyyy-MM-dd HH:mm:ss")}

Write-Host "Installing binary (force overwrite)..." -ForegroundColor Yellow
cargo install --path cli --force

$installEnd = Get-Date
$installDuration = ($installEnd - $installStart).TotalSeconds
Write-DebugLog -location "build-install-background.ps1:4" -message "インストール完了" -data @{duration = $installDuration; exitCode = $LASTEXITCODE}

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Installation failed" -ForegroundColor Red
    Set-Location ..
    exit $LASTEXITCODE
}

Set-Location ..

Write-Host "✅ Installation completed!" -ForegroundColor Green
Write-Host ""

# バージョン確認
codex --version
Write-DebugLog -location "build-install-background.ps1:5" -message "ビルド・インストール完了"
