# 開発サーバー起動 + Virtual Desktop VRモード実機テスト
# 使用方法: .\start-dev-vr-test.ps1
# 自動的にIPアドレスを取得して開発サーバーを起動します

Write-Host "🥽 Codex VRモード実機テスト（開発サーバー + Virtual Desktop）" -ForegroundColor Cyan
Write-Host ""

# Virtual Desktop Streamer確認
Write-Host "🔍 Virtual Desktop Streamer確認中..." -ForegroundColor Yellow
$streamerProcess = Get-Process -Name "VirtualDesktop.Streamer" -ErrorAction SilentlyContinue
if (-not $streamerProcess) {
    Write-Host "⚠️  Virtual Desktop Streamerが起動していません" -ForegroundColor Yellow
    Write-Host "    Streamerを起動してから再度実行してください" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "    Streamerダウンロード: https://www.vrdesktop.net/" -ForegroundColor Cyan
    $response = Read-Host "Streamerを今すぐ起動しますか？ (Y/N)"
    if ($response -eq 'Y' -or $response -eq 'y') {
        $streamerPath = "$env:LOCALAPPDATA\VirtualDesktop.Streamer\VirtualDesktop.Streamer.exe"
        if (Test-Path $streamerPath) {
            Start-Process $streamerPath
            Write-Host "✅ Streamerを起動しました" -ForegroundColor Green
            Start-Sleep -Seconds 3
        } else {
            Write-Host "❌ Streamerが見つかりません" -ForegroundColor Red
            Write-Host "   手動でStreamerをインストール・起動してください" -ForegroundColor Yellow
            exit 1
        }
    } else {
        exit 1
    }
} else {
    Write-Host "✅ Virtual Desktop Streamerが起動しています" -ForegroundColor Green
}

# IPアドレス自動取得
Write-Host ""
Write-Host "🔍 ネットワーク設定確認中..." -ForegroundColor Yellow
$ipAddresses = @(Get-NetIPAddress -AddressFamily IPv4 | Where-Object {
    $_.InterfaceAlias -notlike '*Loopback*' -and 
    $_.IPAddress -notlike '169.254.*' -and
    $_.IPAddress -notlike '127.*'
} | Select-Object IPAddress, InterfaceAlias)

if ($null -eq $ipAddresses -or $ipAddresses.Count -eq 0) {
    Write-Host "❌ IPアドレスが見つかりません" -ForegroundColor Red
    Write-Host "   ネットワーク接続を確認してください" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ 利用可能なIPアドレス:" -ForegroundColor Green
$ipAddresses | ForEach-Object {
    Write-Host "   $($_.IPAddress) ($($_.InterfaceAlias))" -ForegroundColor Cyan
}
$localIP = $ipAddresses[0].IPAddress
Write-Host ""
Write-Host "📌 使用するIPアドレス: $localIP" -ForegroundColor Green

# ポート3000確認と停止
Write-Host ""
Write-Host "🔍 ポート3000確認中..." -ForegroundColor Yellow
$port3000 = Get-NetTCPConnection -LocalPort 3000 -ErrorAction SilentlyContinue
if ($port3000) {
    Write-Host "⚠️  ポート3000が使用中です。既存のプロセスを停止します..." -ForegroundColor Yellow
    $processIds = $port3000 | Select-Object -ExpandProperty OwningProcess -Unique
    foreach ($pid in $processIds) {
        if ($pid -gt 0) {
            try {
                Stop-Process -Id $pid -Force -ErrorAction SilentlyContinue
                Write-Host "   ✅ プロセス $pid を停止しました" -ForegroundColor Green
            } catch {
                Write-Host "   ⚠️  プロセス $pid の停止に失敗しました" -ForegroundColor Yellow
            }
        }
    }
    Start-Sleep -Seconds 2
    Write-Host "✅ ポート3000を解放しました" -ForegroundColor Green
} else {
    Write-Host "✅ ポート3000は使用可能です" -ForegroundColor Green
}

# ファイアウォール設定
Write-Host ""
Write-Host "🔍 ファイアウォール設定確認中..." -ForegroundColor Yellow
$firewallRule = Get-NetFirewallRule -DisplayName "Next.js Dev Server (Port 3000)" -ErrorAction SilentlyContinue
if (-not $firewallRule) {
    Write-Host "📝 ファイアウォールルールを作成します..." -ForegroundColor Yellow
    try {
        New-NetFirewallRule -DisplayName "Next.js Dev Server (Port 3000)" -Direction Inbound -LocalPort 3000 -Protocol TCP -Action Allow -ErrorAction Stop | Out-Null
        Write-Host "✅ ファイアウォールルールを作成しました" -ForegroundColor Green
    } catch {
        Write-Host "⚠️  ファイアウォールルールの作成に失敗しました（管理者権限が必要な場合があります）" -ForegroundColor Yellow
    }
} else {
    Write-Host "✅ ファイアウォールルールが存在します" -ForegroundColor Green
}

# 依存関係確認
Write-Host ""
Write-Host "🔍 依存関係確認中..." -ForegroundColor Yellow
$guiPath = Join-Path $PSScriptRoot "..\..\gui"
if (-not (Test-Path $guiPath)) {
    Write-Host "❌ guiディレクトリが見つかりません: $guiPath" -ForegroundColor Red
    exit 1
}

Push-Location $guiPath

if (-not (Test-Path "node_modules")) {
    Write-Host "📦 node_modulesが見つかりません。npm installを実行します..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ npm installに失敗しました" -ForegroundColor Red
        Pop-Location
        exit 1
    }
    Write-Host "✅ 依存関係のインストールが完了しました" -ForegroundColor Green
} else {
    Write-Host "✅ 依存関係はインストール済みです" -ForegroundColor Green
}

# 開発サーバー自動起動
Write-Host ""
Write-Host "🚀 開発サーバーを自動起動します..." -ForegroundColor Cyan
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "📱 Quest 2でのアクセス方法" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Quest 2でVirtual Desktopアプリを起動" -ForegroundColor White
Write-Host "2. PCのデスクトップが表示されることを確認" -ForegroundColor White
Write-Host "3. Quest内のブラウザで以下のURLにアクセス:" -ForegroundColor White
Write-Host ""
Write-Host "   ⚠️  localhost:3000 は使用できません！" -ForegroundColor Red
Write-Host "   ✅ http://$localIP:3000" -ForegroundColor Green -BackgroundColor Black
Write-Host ""
Write-Host "4. 「🎮 Git VR/AR」ページに移動" -ForegroundColor White
Write-Host "5. リポジトリを選択して「Enter VR」ボタンをクリック" -ForegroundColor White
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "🌐 開発サーバーURL" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "   PCから:     http://localhost:3000" -ForegroundColor Cyan
Write-Host "   Quest 2から: http://$localIP:3000" -ForegroundColor Green
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "💡 最適化ヒント" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "   - Virtual Desktopの設定でVR Graphics QualityをHighに設定" -ForegroundColor White
Write-Host "   - VR Bitrateを100-150 Mbpsに設定" -ForegroundColor White
Write-Host "   - 5GHz Wi-Fiを使用（低レイテンシのため）" -ForegroundColor White
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "⚠️  サーバーを停止するには Ctrl+C を押してください" -ForegroundColor Yellow
Write-Host ""
Write-Host "開発サーバーを起動しています..." -ForegroundColor Cyan
Write-Host ""

# 開発サーバー自動起動（-H 0.0.0.0で外部アクセス許可）
npm run dev

Pop-Location
