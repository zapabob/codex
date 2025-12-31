# ビルド進捗確認スクリプト
# tqdm風の進捗表示で残り時間と経過時間を可視化

$ErrorActionPreference = "Continue"

function Show-Progress {
    param(
        [string]$Message,
        [int]$Progress = 0,
        [int]$Total = 100,
        [double]$ElapsedSeconds = 0,
        [double]$EstimatedRemaining = 0
    )
    
    $barLength = 50
    $filled = [math]::Floor($barLength * $Progress / [math]::Max($Total, 1))
    $bar = "█" * $filled + "░" * ($barLength - $filled)
    $percentage = [math]::Min(100, [math]::Floor($Progress * 100 / [math]::Max($Total, 1)))
    
    $elapsedStr = Format-Time $ElapsedSeconds
    $remainingStr = if ($EstimatedRemaining -gt 0) { " | 残り: $(Format-Time $EstimatedRemaining)" } else { "" }
    
    Write-Host "`r[$bar] $percentage% | $Message | 経過: $elapsedStr$remainingStr" -NoNewline
}

function Format-Time {
    param([double]$Seconds)
    
    if ($Seconds -lt 60) {
        return "{0:F1}秒" -f $Seconds
    } elseif ($Seconds -lt 3600) {
        $minutes = [math]::Floor($Seconds / 60)
        $secs = [math]::Floor($Seconds % 60)
        return "$minutes分$secs秒"
    } else {
        $hours = [math]::Floor($Seconds / 3600)
        $minutes = [math]::Floor(($Seconds % 3600) / 60)
        return "$hours時間$minutes分"
    }
}

function Get-BuildProgress {
    $startTime = Get-Date
    
    Write-Host "`n" + "="*70
    Write-Host "  📊 Codex ビルド進捗モニター"
    Write-Host "="*70 + "`n"
    
    $cargoProcesses = @()
    $buildFileExists = $false
    $checkCount = 0
    
    while ($true) {
        $checkCount++
        $elapsed = (Get-Date) - $startTime
        $elapsedSeconds = $elapsed.TotalSeconds
        
        # Cargoプロセス確認
        $cargoProcesses = Get-Process cargo -ErrorAction SilentlyContinue
        $cargoRunning = $cargoProcesses.Count -gt 0
        
        # ビルドファイル確認
        $buildFileExists = Test-Path "target\release\codex.exe"
        
        if ($buildFileExists) {
            $file = Get-Item "target\release\codex.exe"
            $fileSizeMB = [math]::Round($file.Length / 1MB, 2)
            $fileTime = $file.LastWriteTime.ToString("yyyy-MM-dd HH:mm:ss")
            
            Write-Host "`n`n" + "="*70
            Write-Host "  ✅ ビルド完了！"
            Write-Host "="*70
            Write-Host "  ファイル: target\release\codex.exe"
            Write-Host "  サイズ: $fileSizeMB MB"
            Write-Host "  更新日時: $fileTime"
            Write-Host "  経過時間: $(Format-Time $elapsedSeconds)"
            Write-Host "="*70 + "`n"
            
            return $true
        }
        
        if ($cargoRunning) {
            $cpuUsage = ($cargoProcesses | Measure-Object -Property CPU -Sum).Sum
            $memoryMB = [math]::Round(($cargoProcesses | Measure-Object -Property WorkingSet -Sum).Sum / 1MB, 1)
            
            # 簡易的な進捗推定（実際の進捗は取得できないため、経過時間ベース）
            $estimatedTotal = 300 # 5分を想定
            $progress = [math]::Min(95, [math]::Floor($elapsedSeconds / $estimatedTotal * 100))
            $estimatedRemaining = [math]::Max(0, $estimatedTotal - $elapsedSeconds)
            
            $message = "ビルド中... (CPU: $([math]::Round($cpuUsage, 1))s, メモリ: ${memoryMB}MB)"
            Show-Progress -Message $message -Progress $progress -Total 100 -ElapsedSeconds $elapsedSeconds -EstimatedRemaining $estimatedRemaining
        } else {
            if ($elapsedSeconds -gt 5) {
                Write-Host "`n`n⚠️  Cargoプロセスが停止しましたが、ビルドファイルが見つかりません"
                Write-Host "   ビルドエラーの可能性があります。ログを確認してください。`n"
                return $false
            }
        }
        
        Start-Sleep -Seconds 2
    }
}

# メイン処理
Push-Location $PSScriptRoot
try {
    $buildComplete = Get-BuildProgress
    
    if ($buildComplete) {
        Write-Host "🚀 インストール準備完了！"
        Write-Host "   以下のコマンドでインストールできます：`n"
        Write-Host "   Get-Process codex -ErrorAction SilentlyContinue | Stop-Process -Force"
        Write-Host "   Copy-Item `"target\release\codex.exe`" `"`$env:USERPROFILE\.cargo\bin\codex.exe`" -Force"
        Write-Host "   codex --version`n"
    }
} finally {
    Pop-Location
}
