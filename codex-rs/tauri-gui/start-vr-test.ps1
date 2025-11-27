# VRモード実機テスト起動スクリプト
# 使用方法: .\start-vr-test.ps1

Write-Host "🎮 Codex VRモード実機テスト起動スクリプト" -ForegroundColor Cyan
Write-Host ""

# PCのIPアドレス取得（@()で配列にラップして単一オブジェクトでも配列として扱う）
$ipAddresses = @(Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
    $_.InterfaceAlias -notlike '*Loopback*' -and 
    $_.IPAddress -notlike '169.254.*' -and
    $_.IPAddress -notlike '127.*'
} | Select-Object IPAddress, InterfaceAlias)

if ($null -eq $ipAddresses -or $ipAddresses.Count -eq 0) {
    Write-Host "❌ IPアドレスが見つかりません" -ForegroundColor Red
    exit 1
}

Write-Host "📡 検出されたネットワークインターフェース:" -ForegroundColor Yellow
$ipAddresses | ForEach-Object {
    Write-Host "  - $($_.IPAddress) ($($_.InterfaceAlias))" -ForegroundColor Green
}

$primaryIP = $ipAddresses[0].IPAddress
Write-Host ""
Write-Host "✅ 使用するIPアドレス: $primaryIP" -ForegroundColor Green
Write-Host ""

# 依存関係確認
Write-Host "🔍 依存関係確認中..." -ForegroundColor Yellow
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 node_modulesが見つかりません。npm installを実行します..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ npm installに失敗しました" -ForegroundColor Red
        exit 1
    }
}

# ポート1420が使用可能か確認
Write-Host "🔍 ポート1420の使用状況確認中..." -ForegroundColor Yellow
$portInUse = Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue
if ($portInUse) {
    Write-Host "⚠️  ポート1420は既に使用されています" -ForegroundColor Yellow
    Write-Host "   既存のプロセスを終了しますか？ (Y/N)" -ForegroundColor Yellow
    $response = Read-Host
    if ($response -eq 'Y' -or $response -eq 'y') {
        $process = Get-Process -Id $portInUse.OwningProcess -ErrorAction SilentlyContinue
        if ($process) {
            Stop-Process -Id $process.Id -Force
            Write-Host "✅ プロセスを終了しました" -ForegroundColor Green
        }
    }
}

# ファイアウォールルール確認
Write-Host "🔍 ファイアウォールルール確認中..." -ForegroundColor Yellow
$firewallRule = Get-NetFirewallRule -DisplayName "Codex VR Dev Server" -ErrorAction SilentlyContinue
if (-not $firewallRule) {
    Write-Host "🔒 ファイアウォールルールを作成します..." -ForegroundColor Yellow
    New-NetFirewallRule -DisplayName "Codex VR Dev Server" -Direction Inbound -LocalPort 1420 -Protocol TCP -Action Allow -ErrorAction SilentlyContinue | Out-Null
    Write-Host "✅ ファイアウォールルールを作成しました" -ForegroundColor Green
}

# 開発サーバー起動
Write-Host ""
Write-Host "🚀 開発サーバー起動中..." -ForegroundColor Cyan
Write-Host ""
Write-Host "📱 Questで以下のURLにアクセスしてください:" -ForegroundColor Yellow
Write-Host "   http://$primaryIP:1420/git-vr" -ForegroundColor Green
Write-Host ""
Write-Host "💡 ヒント:" -ForegroundColor Cyan
Write-Host "   - Quest内でブラウザを開く" -ForegroundColor White
Write-Host "   - 上記URLを入力" -ForegroundColor White
Write-Host "   - 「Enter VR」ボタンをクリック" -ForegroundColor White
Write-Host ""
Write-Host "⚠️  サーバーを停止するには Ctrl+C を押してください" -ForegroundColor Yellow
Write-Host ""

# Vite開発サーバー起動（ホスト0.0.0.0でバインド）
$env:VITE_HOST = "0.0.0.0"
npm run dev -- --host 0.0.0.0

