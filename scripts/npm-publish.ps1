# @zapabob/codex npm公開スクリプト
# PowerShell版

Write-Host "📦 @zapabob/codex npm公開スクリプト" -ForegroundColor Cyan
Write-Host ""

# 1. npmログイン確認
Write-Host "1️⃣ npmログイン状態を確認..." -ForegroundColor Yellow
$npmUser = npm whoami 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ npmにログインしていません" -ForegroundColor Red
    Write-Host ""
    Write-Host "以下のコマンドでログインしてください:" -ForegroundColor Yellow
    Write-Host "  npm login" -ForegroundColor Green
    Write-Host ""
    Write-Host "または、npmアカウントを作成:" -ForegroundColor Yellow
    Write-Host "  npm adduser" -ForegroundColor Green
    exit 1
}

Write-Host "✅ npmログイン済み: $npmUser" -ForegroundColor Green
Write-Host ""

# 2. パッケージ情報確認
Write-Host "2️⃣ パッケージ情報を確認..." -ForegroundColor Yellow
$packageJson = Get-Content "package.json" | ConvertFrom-Json
Write-Host "  パッケージ名: $($packageJson.name)" -ForegroundColor Cyan
Write-Host "  バージョン: $($packageJson.version)" -ForegroundColor Cyan
Write-Host "  レジストリ: $($packageJson.publishConfig.registry)" -ForegroundColor Cyan
Write-Host "  公開範囲: $($packageJson.publishConfig.access)" -ForegroundColor Cyan
Write-Host ""

# 3. 公開前の確認
Write-Host "3️⃣ 公開前の確認..." -ForegroundColor Yellow
$confirm = Read-Host "npmに公開しますか? (y/N)"
if ($confirm -ne "y" -and $confirm -ne "Y") {
    Write-Host "❌ 公開をキャンセルしました" -ForegroundColor Red
    exit 0
}

# 4. ドライラン（公開内容の確認）
Write-Host ""
Write-Host "4️⃣ 公開内容を確認（ドライラン）..." -ForegroundColor Yellow
npm pack --dry-run
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ ドライランに失敗しました" -ForegroundColor Red
    exit 1
}

Write-Host ""
$confirm2 = Read-Host "この内容で公開しますか? (y/N)"
if ($confirm2 -ne "y" -and $confirm2 -ne "Y") {
    Write-Host "❌ 公開をキャンセルしました" -ForegroundColor Red
    exit 0
}

# 5. 公開実行
Write-Host ""
Write-Host "5️⃣ npmに公開中..." -ForegroundColor Yellow
npm publish
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 公開に失敗しました" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "✅ 公開成功!" -ForegroundColor Green
Write-Host ""
Write-Host "📦 パッケージURL: https://www.npmjs.com/package/$($packageJson.name)" -ForegroundColor Cyan
Write-Host ""
Write-Host "インストールテスト:" -ForegroundColor Yellow
Write-Host "  npm install -g $($packageJson.name)" -ForegroundColor Green
Write-Host "  codex --version" -ForegroundColor Green
