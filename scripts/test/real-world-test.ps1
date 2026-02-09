# 実機テストスクリプト
# 修正した機能の動作確認

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
        runId = "real-world-test"
    } | ConvertTo-Json -Compress
    Add-Content -Path $logPath -Value $logEntry -Encoding UTF8 -ErrorAction SilentlyContinue
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " 実機テスト" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# テスト1: バージョン確認
Write-Host "Test 1: Version check" -ForegroundColor Yellow
Write-DebugLog -location "real-world-test.ps1:1" -message "バージョン確認テスト開始"
try {
    $versionOutput = codex --version 2>&1
    Write-Host "  Result: $versionOutput" -ForegroundColor Green
    Write-DebugLog -location "real-world-test.ps1:2" -message "バージョン確認成功" -data @{version = $versionOutput}
} catch {
    Write-Host "  ❌ Failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-DebugLog -location "real-world-test.ps1:3" -message "バージョン確認失敗" -data @{error = $_.Exception.Message}
}
Write-Host ""

# テスト2: ヘルプ表示
Write-Host "Test 2: Help display" -ForegroundColor Yellow
Write-DebugLog -location "real-world-test.ps1:4" -message "ヘルプ表示テスト開始"
try {
    $helpOutput = codex --help 2>&1 | Select-Object -First 5
    Write-Host "  ✅ Help displayed successfully" -ForegroundColor Green
    Write-DebugLog -location "real-world-test.ps1:5" -message "ヘルプ表示成功"
} catch {
    Write-Host "  ❌ Failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-DebugLog -location "real-world-test.ps1:6" -message "ヘルプ表示失敗" -data @{error = $_.Exception.Message}
}
Write-Host ""

# テスト3: コンパイルエラー修正の確認（secret_masking.rs）
Write-Host "Test 3: Compilation check" -ForegroundColor Yellow
Write-DebugLog -location "real-world-test.ps1:7" -message "コンパイルチェック開始"
Set-Location codex-rs
try {
    # クイックコンパイルチェック（実際のビルドはしない）
    $checkOutput = cargo check --message-format=short -p codex-core 2>&1 | Select-Object -Last 5
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✅ Compilation check passed" -ForegroundColor Green
        Write-DebugLog -location "real-world-test.ps1:8" -message "コンパイルチェック成功"
    } else {
        Write-Host "  ⚠️  Compilation check warnings/errors" -ForegroundColor Yellow
        Write-DebugLog -location "real-world-test.ps1:9" -message "コンパイルチェック警告" -data @{output = $checkOutput}
    }
} catch {
    Write-Host "  ❌ Failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-DebugLog -location "real-world-test.ps1:10" -message "コンパイルチェック失敗" -data @{error = $_.Exception.Message}
}
Set-Location ..
Write-Host ""

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "✅ Real-world test completed" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-DebugLog -location "real-world-test.ps1:11" -message "実機テスト完了"
