# CursorIDE - Codex自動統合スクリプト
# 実行: .\install_to_cursor.ps1

param(
    [switch]$Backup = $true,
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Continue"

Write-Host "🚀 CursorIDE - Codex統合スクリプト" -ForegroundColor Cyan
Write-Host "=" * 70
Write-Host ""

# ステップ1: Codexインストール確認
Write-Host "📦 Step 1: Codexインストール確認..." -ForegroundColor Yellow

$codexPath = "$env:USERPROFILE\.cargo\bin\codex.exe"
if (Test-Path $codexPath) {
    Write-Host "  ✅ Codex発見: $codexPath" -ForegroundColor Green
    $version = & codex --version 2>&1 | Out-String
    Write-Host "  バージョン: $($version.Trim())" -ForegroundColor Cyan
} else {
    Write-Host "  ❌ Codexが見つかりません" -ForegroundColor Red
    Write-Host "  先にCodexをインストールしてください:" -ForegroundColor Yellow
    Write-Host "    cd codex-rs" -ForegroundColor White
    Write-Host "    cargo install --path cli --force" -ForegroundColor White
    exit 1
}

Write-Host ""

# ステップ2: Cursor設定ファイル確認
Write-Host "📝 Step 2: Cursor設定ファイル確認..." -ForegroundColor Yellow

$cursorSettings = "$env:APPDATA\Cursor\User\settings.json"
$cursorDir = Split-Path $cursorSettings -Parent

if (-not (Test-Path $cursorDir)) {
    Write-Host "  ⚠️ Cursorが見つかりません" -ForegroundColor Yellow
    Write-Host "  Cursor設定ディレクトリを作成します: $cursorDir" -ForegroundColor Cyan
    New-Item -ItemType Directory -Path $cursorDir -Force | Out-Null
}

Write-Host "  ✅ Cursor設定: $cursorSettings" -ForegroundColor Green
Write-Host ""

# ステップ3: バックアップ作成
if ($Backup -and (Test-Path $cursorSettings)) {
    Write-Host "💾 Step 3: バックアップ作成..." -ForegroundColor Yellow
    
    $backupPath = "$cursorSettings.backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    Copy-Item $cursorSettings $backupPath
    Write-Host "  ✅ バックアップ作成: $backupPath" -ForegroundColor Green
    Write-Host ""
}

# ステップ4: 設定の読み込み・マージ
Write-Host "⚙️ Step 4: Codex設定を追加..." -ForegroundColor Yellow

# 既存設定を読み込み
if (Test-Path $cursorSettings) {
    try {
        $settingsContent = Get-Content $cursorSettings -Raw
        $settings = $settingsContent | ConvertFrom-Json -AsHashtable -ErrorAction Stop
        Write-Host "  ✅ 既存設定を読み込みました" -ForegroundColor Green
    } catch {
        Write-Host "  ⚠️ 既存設定の読み込みに失敗。新規作成します" -ForegroundColor Yellow
        $settings = @{}
    }
} else {
    Write-Host "  ℹ️ 設定ファイルが存在しません。新規作成します" -ForegroundColor Cyan
    $settings = @{}
}

# Codex設定を追加
$codexConfig = @{
    "codex.executablePath" = "C:\Users\downl\.cargo\bin\codex.exe"
    "codex.enableDeepResearch" = $true
    "codex.enableSubAgents" = $true
    "codex.supervisorEnabled" = $true
    "codex.maxConcurrentAgents" = 8
    "codex.deepResearch.maxSources" = 20
    "codex.deepResearch.maxDepth" = 3
    "codex.deepResearch.includeAcademic" = $true
    "codex.sandbox.enabled" = $true
    "codex.sandbox.mode" = "workspace-write"
}

foreach ($key in $codexConfig.Keys) {
    $settings[$key] = $codexConfig[$key]
    if ($Verbose) {
        Write-Host "    設定: $key = $($codexConfig[$key])" -ForegroundColor Gray
    }
}

Write-Host "  ✅ Codex設定を追加しました" -ForegroundColor Green
Write-Host ""

# ステップ5: 設定を保存
Write-Host "💾 Step 5: 設定を保存..." -ForegroundColor Yellow

try {
    $settings | ConvertTo-Json -Depth 10 | Set-Content $cursorSettings -Encoding UTF8
    Write-Host "  ✅ 設定ファイルを更新しました" -ForegroundColor Green
} catch {
    Write-Host "  ❌ 設定ファイルの保存に失敗しました: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""

# ステップ6: 確認
Write-Host "🔍 Step 6: 設定確認..." -ForegroundColor Yellow

Write-Host "  追加された設定:" -ForegroundColor Cyan
Write-Host "    codex.executablePath: C:\Users\downl\.cargo\bin\codex.exe" -ForegroundColor White
Write-Host "    codex.enableDeepResearch: true" -ForegroundColor White
Write-Host "    codex.enableSubAgents: true" -ForegroundColor White
Write-Host "    codex.supervisorEnabled: true" -ForegroundColor White
Write-Host "    codex.maxConcurrentAgents: 8" -ForegroundColor White
Write-Host ""

# 完了メッセージ
Write-Host "=" * 70
Write-Host ""
Write-Host "🎉 CursorIDE統合が完了しました！" -ForegroundColor Green
Write-Host ""
Write-Host "次のステップ:" -ForegroundColor Yellow
Write-Host "  1. CursorIDEを再起動してください" -ForegroundColor White
Write-Host "  2. Ctrl + ` でターミナルを開く" -ForegroundColor White
Write-Host "  3. 'codex --version' を実行" -ForegroundColor White
Write-Host "  4. Ctrl + K でCodexを使用開始！" -ForegroundColor White
Write-Host ""
Write-Host "テストコマンド:" -ForegroundColor Yellow
Write-Host "  codex exec `"Create a hello world function in Rust`"" -ForegroundColor White
Write-Host ""
Write-Host "ドキュメント:" -ForegroundColor Yellow
Write-Host "  CURSOR_INSTALL.md - インストールガイド" -ForegroundColor White
Write-Host "  cursor-integration/README.md - 詳細ガイド" -ForegroundColor White
Write-Host "  START_HERE.md - クイックスタート" -ForegroundColor White
Write-Host ""
Write-Host "=" * 70
Write-Host ""
Write-Host "🎊 最高の開発体験を楽しんでください！" -ForegroundColor Green

