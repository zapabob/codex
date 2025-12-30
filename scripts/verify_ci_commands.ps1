# CI/CDコマンド検証スクリプト
# CI/CDで実行されるコマンドをローカルで実行して、問題を特定

$ErrorActionPreference = "Stop"
$logPath = ".cursor\debug.log"

# ログファイルをクリア
if (Test-Path $logPath) {
    Remove-Item $logPath -Force
}

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
        runId = "run1"
        hypothesisId = $hypothesisId
    } | ConvertTo-Json -Compress
    Add-Content -Path $logPath -Value $logEntry -Encoding UTF8
}

Write-DebugLog -location "verify_ci_commands.ps1:1" -message "CI/CD検証スクリプト開始" -hypothesisId "A"

# 仮説A: コンパイルエラー
Write-DebugLog -location "verify_ci_commands.ps1:2" -message "仮説A検証開始: コンパイルエラー" -hypothesisId "A"
try {
    Set-Location codex-rs
    Write-DebugLog -location "verify_ci_commands.ps1:3" -message "cargo check実行前" -data @{cwd = (Get-Location).Path} -hypothesisId "A"
    
    $checkOutput = cargo check --message-format=json 2>&1 | Out-String
    Write-DebugLog -location "verify_ci_commands.ps1:4" -message "cargo check実行後" -data @{exitCode = $LASTEXITCODE; outputLength = $checkOutput.Length} -hypothesisId "A"
    
    if ($LASTEXITCODE -ne 0) {
        Write-DebugLog -location "verify_ci_commands.ps1:5" -message "コンパイルエラー検出" -data @{output = $checkOutput} -hypothesisId "A"
        Write-Host "❌ コンパイルエラーが検出されました"
        Write-Host $checkOutput
    } else {
        Write-DebugLog -location "verify_ci_commands.ps1:6" -message "コンパイル成功" -hypothesisId "A"
        Write-Host "✅ コンパイル成功"
    }
} catch {
    Write-DebugLog -location "verify_ci_commands.ps1:7" -message "コンパイルエラー例外" -data @{error = $_.Exception.Message} -hypothesisId "A"
    Write-Host "❌ コンパイルエラー: $($_.Exception.Message)"
}

Set-Location ..

# 仮説B: フォーマットエラー
Write-DebugLog -location "verify_ci_commands.ps1:8" -message "仮説B検証開始: フォーマットエラー" -hypothesisId "B"
try {
    Set-Location codex-rs
    Write-DebugLog -location "verify_ci_commands.ps1:9" -message "cargo fmt --check実行前" -hypothesisId "B"
    
    $fmtOutput = cargo fmt -- --check 2>&1 | Out-String
    Write-DebugLog -location "verify_ci_commands.ps1:10" -message "cargo fmt --check実行後" -data @{exitCode = $LASTEXITCODE; outputLength = $fmtOutput.Length} -hypothesisId "B"
    
    if ($LASTEXITCODE -ne 0) {
        Write-DebugLog -location "verify_ci_commands.ps1:11" -message "フォーマットエラー検出" -data @{output = $fmtOutput} -hypothesisId "B"
        Write-Host "❌ フォーマットエラーが検出されました"
        Write-Host $fmtOutput
    } else {
        Write-DebugLog -location "verify_ci_commands.ps1:12" -message "フォーマットチェック成功" -hypothesisId "B"
        Write-Host "✅ フォーマットチェック成功"
    }
} catch {
    Write-DebugLog -location "verify_ci_commands.ps1:13" -message "フォーマットエラー例外" -data @{error = $_.Exception.Message} -hypothesisId "B"
    Write-Host "❌ フォーマットエラー: $($_.Exception.Message)"
}

Set-Location ..

# 仮説C: Lintエラー
Write-DebugLog -location "verify_ci_commands.ps1:14" -message "仮説C検証開始: Lintエラー" -hypothesisId "C"
try {
    Set-Location codex-rs
    Write-DebugLog -location "verify_ci_commands.ps1:15" -message "cargo clippy実行前" -hypothesisId "C"
    
    $clippyOutput = cargo clippy --all-features --tests -- -D warnings 2>&1 | Out-String
    Write-DebugLog -location "verify_ci_commands.ps1:16" -message "cargo clippy実行後" -data @{exitCode = $LASTEXITCODE; outputLength = $clippyOutput.Length} -hypothesisId "C"
    
    if ($LASTEXITCODE -ne 0) {
        Write-DebugLog -location "verify_ci_commands.ps1:17" -message "Lintエラー検出" -data @{output = $clippyOutput} -hypothesisId "C"
        Write-Host "❌ Lintエラーが検出されました"
        Write-Host $clippyOutput
    } else {
        Write-DebugLog -location "verify_ci_commands.ps1:18" -message "Lintチェック成功" -hypothesisId "C"
        Write-Host "✅ Lintチェック成功"
    }
} catch {
    Write-DebugLog -location "verify_ci_commands.ps1:19" -message "Lintエラー例外" -data @{error = $_.Exception.Message} -hypothesisId "C"
    Write-Host "❌ Lintエラー: $($_.Exception.Message)"
}

Set-Location ..

Write-DebugLog -location "verify_ci_commands.ps1:20" -message "CI/CD検証スクリプト完了" -hypothesisId "A"
