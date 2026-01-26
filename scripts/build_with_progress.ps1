# Rustビルド進捗表示スクリプト（tqdm風）
param(
    [string]$Command = "cargo build --workspace --features custom-features"
)

$ErrorActionPreference = "Continue"
$startTime = Get-Date

Write-Host "🚀 Rustビルド開始: $Command" -ForegroundColor Cyan
Write-Host "開始時刻: $($startTime.ToString('yyyy-MM-dd HH:mm:ss'))" -ForegroundColor Gray
Write-Host ""

# バックグラウンドジョブでビルドを実行
$job = Start-Job -ScriptBlock {
    param($cmd)
    Set-Location $using:PWD
    & cmd /c $cmd 2>&1
} -ArgumentList $Command

# 進捗表示ループ
$lastOutput = ""
while ($job.State -eq "Running") {
    $currentTime = Get-Date
    $elapsed = $currentTime - $startTime
    $elapsedStr = "{0:D2}:{1:D2}:{2:D2}" -f $elapsed.Hours, $elapsed.Minutes, $elapsed.Seconds
    
    # ジョブの出力を取得
    $output = Receive-Job -Job $job
    if ($output -and $output -ne $lastOutput) {
        # コンパイル中のメッセージを抽出
        if ($output -match "Compiling|Finished|error|warning") {
            Write-Host "[経過: $elapsedStr] $output" -ForegroundColor Yellow
            $lastOutput = $output
        }
    }
    
    Start-Sleep -Milliseconds 500
}

# 最終結果を取得
$result = Receive-Job -Job $job
Remove-Job -Job $job

$endTime = Get-Date
$totalTime = $endTime - $startTime
$totalTimeStr = "{0:D2}:{1:D2}:{2:D2}" -f $totalTime.Hours, $totalTime.Minutes, $totalTime.Seconds

Write-Host ""
Write-Host "⏱️  総経過時間: $totalTimeStr" -ForegroundColor Cyan
Write-Host ""

# 結果を表示
$result | ForEach-Object { Write-Host $_ }

# 終了コードを確認
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ ビルド成功！" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ ビルド失敗 (終了コード: $LASTEXITCODE)" -ForegroundColor Red
    exit $LASTEXITCODE
}
