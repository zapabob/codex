# CursorブラウザでCodex GUIを直接操作
# 使用方法: .\launch-gui-cursor-playwright.ps1

param(
    [int]$Port = 3001,
    [switch]$Debug = $false
)

Write-Host "🚀 CursorブラウザでCodex GUI起動" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# GUIサーバー起動確認
$guiProcess = Get-Process | Where-Object { $_.ProcessName -like "*codex-gui*" -or $_.MainModule -like "*codex-gui*" } -ErrorAction SilentlyContinue

if (-not $guiProcess) {
    Write-Host "🔄 GUIサーバーを起動中..." -ForegroundColor Yellow

    # GUIサーバー起動
    Start-Process -FilePath "$env:USERPROFILE\.cargo\bin\codex-gui-new.exe" -ArgumentList "--port", $Port -NoNewWindow

    # 起動待機
    $maxRetries = 30
    $retryCount = 0

    while ($retryCount -lt $maxRetries) {
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:$Port" -TimeoutSec 2 -ErrorAction SilentlyContinue
            if ($response.StatusCode -eq 200) {
                Write-Host "✅ GUIサーバー起動完了 (ポート: $Port)" -ForegroundColor Green
                break
            }
        } catch {
            # 接続失敗は正常
        }

        $retryCount++
        Start-Sleep -Seconds 1
        Write-Host "  待機中... ($retryCount/$maxRetries)" -ForegroundColor Gray
    }

    if ($retryCount -ge $maxRetries) {
        Write-Host "❌ GUIサーバー起動タイムアウト" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✅ GUIサーバーは既に実行中" -ForegroundColor Green
}

# CursorブラウザでGUIを開く
Write-Host ""
Write-Host "🌐 CursorブラウザでGUIを開く..." -ForegroundColor Green

$cursorPath = "$env:LOCALAPPDATA\Programs\cursor\Cursor.exe"
if (-not (Test-Path $cursorPath)) {
    $cursorPath = "$env:ProgramFiles\Cursor\Cursor.exe"
}

if (Test-Path $cursorPath) {
    Write-Host "Cursor実行ファイル: $cursorPath" -ForegroundColor Gray

    # CursorでGUIを開く（新規ウィンドウ）
    $url = "http://localhost:$Port"
    Write-Host "URL: $url" -ForegroundColor Gray

    # CursorをGUI URLで起動
    Start-Process -FilePath $cursorPath -ArgumentList "--new-window", $url

    Write-Host "✅ CursorブラウザでGUIを開きました" -ForegroundColor Green

    if ($Debug) {
        Write-Host ""
        Write-Host "🔍 デバッグ情報:" -ForegroundColor Yellow
        Write-Host "  GUIプロセス: $(Get-Process | Where-Object { $_.ProcessName -like '*codex-gui*' } | Select-Object -First 1 | ForEach-Object { $_.Id })" -ForegroundColor Gray
        Write-Host "  ポート: $Port" -ForegroundColor Gray
        Write-Host "  URL: $url" -ForegroundColor Gray
    }

} else {
    Write-Host "❌ Cursorが見つかりません" -ForegroundColor Red
    Write-Host "Cursorがインストールされているか確認してください。" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎯 GUI操作開始" -ForegroundColor Green
Write-Host ""
Write-Host "Cursorブラウザで以下の機能をテストできます:" -ForegroundColor Cyan
Write-Host "  • Dashboard - メイン画面" -ForegroundColor White
Write-Host "  • Agents - AIエージェント管理" -ForegroundColor White
Write-Host "  • Tasks - タスク管理" -ForegroundColor White
Write-Host "  • QC - 品質管理 (ANOVAダッシュボード)" -ForegroundColor White
Write-Host "  • Security - セキュリティ管理" -ForegroundColor White
Write-Host "  • Virtual OS - 仮想環境" -ForegroundColor White
Write-Host "  • AI Tools - AIツールオーケストレーション" -ForegroundColor White
Write-Host "  • MCP - MCPサーバー管理" -ForegroundColor White
Write-Host "  • Code - コード実行 (Git4D可視化)" -ForegroundColor White
Write-Host ""
Write-Host "Playwrightテスト実行: .\test-gui-cursor-playwright.ps1" -ForegroundColor Yellow