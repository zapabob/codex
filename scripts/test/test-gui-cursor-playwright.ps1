# CursorブラウザでCodex GUIをPlaywrightテスト
# 使用方法: .\test-gui-cursor-playwright.ps1

param(
    [switch]$Headless = $false,
    [switch]$Debug = $false,
    [int]$Port = 3001
)

Write-Host "🎭 Cursorブラウザ + Playwright でCodex GUIテスト開始" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# 環境変数設定
$env:CURSOR_EXECUTABLE_PATH = "$env:LOCALAPPDATA\Programs\cursor\Cursor.exe"
if (-not (Test-Path $env:CURSOR_EXECUTABLE_PATH)) {
    $env:CURSOR_EXECUTABLE_PATH = "$env:ProgramFiles\Cursor\Cursor.exe"
}
$env:GUI_URL = "http://localhost:$Port"

# プロジェクトルートに移動
Set-Location $PSScriptRoot\..

# Playwrightインストール確認
if (-not (Test-Path "node_modules\.bin\playwright")) {
    Write-Host "📦 Playwrightをインストール中..." -ForegroundColor Yellow
    npm install
}

# テスト実行オプション
$testArgs = @()

if ($Headless) {
    $testArgs += "--headed=false"
}
else {
    $testArgs += "--headed"
}

if ($Debug) {
    $testArgs += "--debug"
}

# Cursorプロジェクトでテスト実行
Write-Host "🚀 CursorブラウザでGUIテスト実行中..." -ForegroundColor Green
Write-Host "プロジェクト: cursor" -ForegroundColor Gray
$HeadlessStr = if ($Headless) { 'ON' } else { 'OFF' }
Write-Host "ヘッドレスモード: $HeadlessStr" -ForegroundColor Gray
Write-Host ""

try {
    # Playwrightテスト実行
    $command = "npx playwright test --project=cursor $($testArgs -join ' ')"
    Write-Host "実行コマンド: $command" -ForegroundColor Gray
    Write-Host ""

    # テスト実行
    Invoke-Expression $command

    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ テスト成功！" -ForegroundColor Green
        Write-Host "CursorブラウザでのCodex GUIテストが正常に完了しました。" -ForegroundColor Green
    }
    else {
        Write-Host ""
        Write-Host "❌ テスト失敗" -ForegroundColor Red
        Write-Host "詳細は上記のエラーメッセージを確認してください。" -ForegroundColor Red
        exit 1
    }

}
catch {
    Write-Host ""
    Write-Host "❌ テスト実行エラー: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "📊 テストレポート" -ForegroundColor Cyan
Write-Host "HTMLレポート: npx playwright show-report" -ForegroundColor Gray
Write-Host "テストファイル: tests/playwright/gui-tests.spec.ts" -ForegroundColor Gray
Write-Host ""

Write-Host "🎯 テスト完了" -ForegroundColor Green