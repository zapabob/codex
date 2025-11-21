# Virtual Desktop経由VRモード実機テスト起動スクリプト
# 使用方法: .\start-vr-virtualdesktop.ps1

Write-Host "🥽 Codex VRモード実機テスト（Virtual Desktop経由）" -ForegroundColor Cyan
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

# Quest接続確認
Write-Host ""
Write-Host "🔍 Quest接続確認中..." -ForegroundColor Yellow
$streamerWindow = Get-Process -Name "VirtualDesktop.Streamer" -ErrorAction SilentlyContinue
if ($streamerWindow) {
    Write-Host "✅ Streamerプロセス確認済み" -ForegroundColor Green
    Write-Host "   QuestでVirtual Desktopアプリを起動して接続を確認してください" -ForegroundColor Yellow
} else {
    Write-Host "⚠️  Streamerウィンドウが見つかりません" -ForegroundColor Yellow
}

# Tauriアプリビルド確認
Write-Host ""
Write-Host "🔍 Tauriアプリビルド確認中..." -ForegroundColor Yellow
$appPath = "src-tauri\target\release\codex-tauri.exe"
if (-not (Test-Path $appPath)) {
    Write-Host "⚠️  ビルド済みアプリが見つかりません" -ForegroundColor Yellow
    Write-Host "   ビルドを実行しますか？ (Y/N)" -ForegroundColor Yellow
    $response = Read-Host
    if ($response -eq 'Y' -or $response -eq 'y') {
        Write-Host "📦 Tauriアプリビルド中..." -ForegroundColor Cyan
        npm run tauri:build
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ ビルドに失敗しました" -ForegroundColor Red
            exit 1
        }
        Write-Host "✅ ビルド完了" -ForegroundColor Green
    } else {
        Write-Host "❌ ビルドが必要です" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✅ ビルド済みアプリが見つかりました" -ForegroundColor Green
}

# 依存関係確認
Write-Host ""
Write-Host "🔍 依存関係確認中..." -ForegroundColor Yellow
if (-not (Test-Path "node_modules")) {
    Write-Host "📦 node_modulesが見つかりません。npm installを実行します..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ npm installに失敗しました" -ForegroundColor Red
        exit 1
    }
}

# アプリ起動
Write-Host ""
Write-Host "🚀 Codex Tauriアプリ起動中..." -ForegroundColor Cyan
Write-Host ""
Write-Host "📱 Questでの操作手順:" -ForegroundColor Yellow
Write-Host "   1. QuestでVirtual Desktopアプリを起動" -ForegroundColor White
Write-Host "   2. PCのデスクトップが表示されることを確認" -ForegroundColor White
Write-Host "   3. Codexアプリが起動したら、「🎮 Git VR/AR」ページに移動" -ForegroundColor White
Write-Host "   4. リポジトリを選択して「Enter VR」ボタンをクリック" -ForegroundColor White
Write-Host ""
Write-Host "💡 ヒント:" -ForegroundColor Cyan
Write-Host "   - Virtual Desktopの設定でVR Graphics QualityをHighに設定" -ForegroundColor White
Write-Host "   - VR Bitrateを100-150 Mbpsに設定" -ForegroundColor White
Write-Host "   - 5GHz Wi-Fiを使用（低レイテンシのため）" -ForegroundColor White
Write-Host ""
Write-Host "⚠️  アプリを停止するには Ctrl+C を押してください" -ForegroundColor Yellow
Write-Host ""

# アプリ起動
Start-Process -FilePath $appPath -WorkingDirectory (Get-Location)

Write-Host "✅ アプリを起動しました" -ForegroundColor Green
Write-Host ""
Write-Host "🎮 VRモードテストを開始してください！" -ForegroundColor Cyan

