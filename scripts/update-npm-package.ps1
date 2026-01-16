# npm @zapabob/codex パッケージ更新スクリプト
# ClaudeCode統合版 v2.11.0 公開用

param(
    [switch]$DryRun,
    [switch]$SkipLogin,
    [switch]$Force
)

Write-Host "📦 npm @zapabob/codex パッケージ更新" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 現在の状態確認
Write-Host "🔍 現在の状態確認..." -ForegroundColor Yellow
Write-Host "  • package.json version: $(Get-Content package.json | ConvertFrom-Json | Select-Object -ExpandProperty version)" -ForegroundColor Gray
Write-Host "  • Git status:" -ForegroundColor Gray
git status --porcelain | ForEach-Object { Write-Host "    $_" -ForegroundColor Gray }
Write-Host ""

# npmログイン確認
if (-not $SkipLogin) {
    Write-Host "🔐 npmログイン確認..." -ForegroundColor Yellow
    try {
        $npmUser = npm whoami 2>$null
        Write-Host "  ✅ Logged in as: $npmUser" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ Not logged in to npm" -ForegroundColor Red
        Write-Host "  💡 Please run: npm login" -ForegroundColor Yellow
        Write-Host "     Then visit: https://www.npmjs.com/login?next=/login/cli/[token]" -ForegroundColor Yellow
        exit 1
    }
    Write-Host ""
}

# バージョン確認
$currentVersion = (Get-Content package.json | ConvertFrom-Json).version
$expectedVersion = "2.11.0"

if ($currentVersion -ne $expectedVersion) {
    Write-Host "⚠️ Version mismatch!" -ForegroundColor Red
    Write-Host "  Expected: $expectedVersion" -ForegroundColor Gray
    Write-Host "  Current:  $currentVersion" -ForegroundColor Gray
    if (-not $Force) {
        Write-Host "  Use -Force to override" -ForegroundColor Yellow
        exit 1
    }
} else {
    Write-Host "✅ Version check passed: $currentVersion" -ForegroundColor Green
}

# Gitコミット確認
Write-Host "📋 Git状態確認..." -ForegroundColor Yellow
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Host "  ⚠️ Uncommitted changes detected:" -ForegroundColor Yellow
    $gitStatus | ForEach-Object { Write-Host "    $_" -ForegroundColor Gray }
    if (-not $Force) {
        Write-Host "  💡 Please commit changes first" -ForegroundColor Yellow
        exit 1
    }
} else {
    Write-Host "  ✅ Git repository is clean" -ForegroundColor Green
}

Write-Host ""

# Dry run mode
if ($DryRun) {
    Write-Host "🧪 Dry Run Mode - 実際の公開は行いません" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "📦 公開される内容:" -ForegroundColor White
    Write-Host "  • Package: @zapabob/codex" -ForegroundColor Gray
    Write-Host "  • Version: $currentVersion" -ForegroundColor Gray
    Write-Host "  • Description: ClaudeCode Integration" -ForegroundColor Gray
    Write-Host ""
    Write-Host "📁 含まれるファイル:" -ForegroundColor White
    npm pack --dry-run 2>$null | Select-Object -Skip 1 | ForEach-Object {
        Write-Host "  • $_" -ForegroundColor Gray
    }
    Write-Host ""
    Write-Host "✅ Dry run completed successfully" -ForegroundColor Green
    exit 0
}

# 公開確認
Write-Host "🚀 npmパッケージ公開の準備ができました！" -ForegroundColor Green
Write-Host ""
Write-Host "📋 公開情報:" -ForegroundColor White
Write-Host "  • Package: @zapabob/codex" -ForegroundColor Cyan
Write-Host "  • Version: $currentVersion" -ForegroundColor Cyan
Write-Host "  • Tag: latest" -ForegroundColor Cyan
Write-Host ""
Write-Host "🎯 新機能 (v2.11.0):" -ForegroundColor White
Write-Host "  • ClaudeCode完全統合" -ForegroundColor Gray
Write-Host "  • Cowork Productivity Suite" -ForegroundColor Gray
Write-Host "  • プロンプトインジェクション対策" -ForegroundColor Gray
Write-Host "  • マルチモデルインテリジェンス" -ForegroundColor Gray
Write-Host "  • コスト最適化 (70%削減)" -ForegroundColor Gray
Write-Host "  • プライバシー保護" -ForegroundColor Gray
Write-Host ""

$confirmation = Read-Host "npm publish を実行しますか？ (y/N)"
if ($confirmation -ne 'y' -and $confirmation -ne 'Y') {
    Write-Host "❌ キャンセルされました" -ForegroundColor Yellow
    exit 0
}

Write-Host ""
Write-Host "📦 npmパッケージを公開中..." -ForegroundColor Yellow

try {
    # npm publish 実行
    npm publish

    if ($LASTEXITCODE -eq 0) {
        Write-Host ""        Write-Host "🎉 npmパッケージ公開成功！" -ForegroundColor Green
        Write-Host ""
        Write-Host "📦 公開されたパッケージ:" -ForegroundColor White
        Write-Host "  • @zapabob/codex@$currentVersion" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "🔗 インストール方法:" -ForegroundColor White
        Write-Host "  npm install -g @zapabob/codex" -ForegroundColor Gray
        Write-Host "  # or" -ForegroundColor Gray
        Write-Host "  npm install -g @zapabob/codex@$currentVersion" -ForegroundColor Gray
        Write-Host ""
        Write-Host "📊 npmページ:" -ForegroundColor White
        Write-Host "  https://www.npmjs.com/package/@zapabob/codex" -ForegroundColor Gray
        Write-Host ""
        Write-Host "🎯 ClaudeCode統合版v2.11.0が利用可能になりました！" -ForegroundColor Green

    } else {
        Write-Host ""
        Write-Host "❌ npmパッケージ公開失敗" -ForegroundColor Red
        Write-Host "詳細は上記のエラーメッセージを確認してください。" -ForegroundColor Red
        exit 1
    }

} catch {
    Write-Host ""
    Write-Host "❌ 公開エラー: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "💡 次のステップ:" -ForegroundColor Cyan
Write-Host "  1. GitHub Releasesでv2.11.0タグを作成" -ForegroundColor White
Write-Host "  2. リリースノートを追加" -ForegroundColor White
Write-Host "  3. Discord/Slackで告知" -ForegroundColor White
Write-Host "  4. ブログ記事執筆" -ForegroundColor White