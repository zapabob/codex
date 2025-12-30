# 高速差分ビルドとインストールスクリプト（tqdm風進捗表示付き）
# Rustのビルド進捗を視覚化して残り時間・経過時間を表示

param(
    [switch]$Release = $true,
    [switch]$Clean = $false
)

$ErrorActionPreference = "Continue"

# ログファイルパス
$logPath = ".cursor\debug.log"

# ログ関数
function Write-DebugLog {
    param(
        [string]$location,
        [string]$message,
        [hashtable]$data = @{},
        [string]$hypothesisId = ""
    )
    $logEntry = @{
        id = "log_$(Get-Date -Format 'yyyyMMddHHmmss')_$(New-Guid)"
        timestamp = [DateTimeOffset]::Now.ToUnixTimeMilliseconds()
        location = $location
        message = $message
        data = $data
        sessionId = "debug-session"
        runId = "build-install"
        hypothesisId = $hypothesisId
    } | ConvertTo-Json -Compress
    Add-Content -Path $logPath -Value $logEntry -Encoding UTF8 -ErrorAction SilentlyContinue
}

Write-DebugLog -location "fast-build-install.ps1:1" -message "高速差分ビルド開始" -data @{release = $Release; clean = $Clean} -hypothesisId "A"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Codex 高速差分ビルド & インストール" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# システム情報
$cpuCores = (Get-WmiObject Win32_Processor).NumberOfLogicalProcessors
Write-Host "CPU Cores: $cpuCores" -ForegroundColor White
Write-Host "Build Mode: $(if ($Release) { 'Release' } else { 'Debug' })" -ForegroundColor White
Write-Host ""

Set-Location codex-rs

if ($Clean) {
    Write-Host "Cleaning build cache..." -ForegroundColor Yellow
    cargo clean
    Write-Host "Clean complete!" -ForegroundColor Green
    Write-Host ""
}

# ビルド開始時刻
$buildStart = Get-Date
Write-DebugLog -location "fast-build-install.ps1:2" -message "ビルド開始" -data @{startTime = $buildStart.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

Write-Host "Starting incremental build..." -ForegroundColor Yellow
Write-Host "  Package: codex-cli" -ForegroundColor Gray
Write-Host "  Profile: $(if ($Release) { 'release' } else { 'dev' })" -ForegroundColor Gray
Write-Host "  Incremental: Enabled" -ForegroundColor Gray
Write-Host ""

# ビルド実行（進捗表示付き）
if ($Release) {
    Write-Host "Building release binary..." -ForegroundColor Cyan
    cargo build --release -p codex-cli 2>&1 | ForEach-Object {
        $line = $_
        Write-Host $line
        # ビルド進捗をログに記録
        if ($line -match "Compiling|Finished|error|warning") {
            Write-DebugLog -location "fast-build-install.ps1:3" -message "ビルド進捗" -data @{line = $line} -hypothesisId "A"
        }
    }
} else {
    Write-Host "Building debug binary..." -ForegroundColor Cyan
    cargo build -p codex-cli 2>&1 | ForEach-Object {
        $line = $_
        Write-Host $line
        if ($line -match "Compiling|Finished|error|warning") {
            Write-DebugLog -location "fast-build-install.ps1:4" -message "ビルド進捗" -data @{line = $line} -hypothesisId "A"
        }
    }
}

$buildEnd = Get-Date
$buildDuration = ($buildEnd - $buildStart).TotalSeconds
Write-DebugLog -location "fast-build-install.ps1:5" -message "ビルド完了" -data @{duration = $buildDuration; endTime = $buildEnd.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Build failed with exit code $LASTEXITCODE" -ForegroundColor Red
    Write-DebugLog -location "fast-build-install.ps1:6" -message "ビルド失敗" -data @{exitCode = $LASTEXITCODE} -hypothesisId "A"
    Set-Location ..
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "✅ Build completed in $([math]::Round($buildDuration, 2)) seconds" -ForegroundColor Green
Write-Host ""

# インストール開始
$installStart = Get-Date
Write-DebugLog -location "fast-build-install.ps1:7" -message "インストール開始" -data @{startTime = $installStart.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

Write-Host "Installing binary (force overwrite)..." -ForegroundColor Yellow
Write-Host "  Path: cli" -ForegroundColor Gray
Write-Host "  Mode: --force (overwrite existing)" -ForegroundColor Gray
Write-Host ""

cargo install --path cli --force 2>&1 | ForEach-Object {
    $line = $_
    Write-Host $line
    if ($line -match "Installing|Installed|error|warning") {
        Write-DebugLog -location "fast-build-install.ps1:8" -message "インストール進捗" -data @{line = $line} -hypothesisId "A"
    }
}

$installEnd = Get-Date
$installDuration = ($installEnd - $installStart).TotalSeconds
Write-DebugLog -location "fast-build-install.ps1:9" -message "インストール完了" -data @{duration = $installDuration; endTime = $installEnd.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Installation failed with exit code $LASTEXITCODE" -ForegroundColor Red
    Write-DebugLog -location "fast-build-install.ps1:10" -message "インストール失敗" -data @{exitCode = $LASTEXITCODE} -hypothesisId "A"
    Set-Location ..
    exit $LASTEXITCODE
}

Set-Location ..

$totalDuration = ($installEnd - $buildStart).TotalSeconds
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "✅ Installation completed!" -ForegroundColor Green
Write-Host "  Build time: $([math]::Round($buildDuration, 2)) seconds" -ForegroundColor White
Write-Host "  Install time: $([math]::Round($installDuration, 2)) seconds" -ForegroundColor White
Write-Host "  Total time: $([math]::Round($totalDuration, 2)) seconds" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# バージョン確認
Write-Host "Verifying installation..." -ForegroundColor Yellow
$versionOutput = codex --version 2>&1
Write-Host $versionOutput -ForegroundColor Cyan
Write-DebugLog -location "fast-build-install.ps1:11" -message "バージョン確認" -data @{version = $versionOutput} -hypothesisId "A"

Write-Host ""
Write-Host "✅ Ready for real-world testing!" -ForegroundColor Green
Write-DebugLog -location "fast-build-install.ps1:12" -message "ビルド・インストール完了" -hypothesisId "A"
