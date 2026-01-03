#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Codexのプロセスをキルして新しいバイナリで上書きインストール
.DESCRIPTION
    実行中のCodexプロセスを検出・終了させ、新しいバイナリをコピー
.PARAMETER SourcePath
    新しいバイナリのソースパス
.PARAMETER TargetPath
    インストール先のパス
.PARAMETER ProcessName
    終了させるプロセス名（デフォルト: codex）
.PARAMETER Force
    強制インストール（確認なし）
.EXAMPLE
    .\install_with_kill.ps1 -SourcePath "codex-rs\target\release\codex.exe" -TargetPath "C:\bin\codex.exe"
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$SourcePath,

    [Parameter(Mandatory=$true)]
    [string]$TargetPath,

    [string]$ProcessName = "codex",

    [switch]$Force
)

# 視覚化関数
function Write-ColoredMessage {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Show-Progress {
    param(
        [string]$Activity,
        [string]$Status,
        [int]$PercentComplete = -1
    )
    if ($PercentComplete -ge 0) {
        Write-Progress -Activity $Activity -Status $Status -PercentComplete $PercentComplete
    } else {
        Write-Progress -Activity $Activity -Status $Status
    }
}

# メイン処理開始
Write-ColoredMessage "[REBUILD] Codex 上書きインストールシステム開始" "Cyan"
Write-ColoredMessage "[DIR] ソース: $SourcePath" "Gray"
Write-ColoredMessage "[TARGET] ターゲット: $TargetPath" "Gray"

# ソースファイルの存在確認
if (!(Test-Path $SourcePath)) {
    Write-ColoredMessage "[ERROR] ソースファイルが見つかりません: $SourcePath" "Red"
    exit 1
}

# プロセス検出と終了
Write-ColoredMessage "[SEARCH] 実行中のプロセスを検索中..." "Yellow"
$runningProcesses = Get-Process -Name $ProcessName -ErrorAction SilentlyContinue

if ($runningProcesses) {
    Write-ColoredMessage "[WARN] 実行中のプロセスを検出: $($runningProcesses.Count) 個" "Yellow"

    foreach ($process in $runningProcesses) {
        Write-ColoredMessage "  [INFO] PID: $($process.Id), 開始時間: $($process.StartTime)" "Gray"
    }

    if (!$Force) {
        $confirm = Read-Host "プロセスを終了させてインストールしますか？ (y/N)"
        if ($confirm -notmatch "^[Yy]$") {
            Write-ColoredMessage "[ERROR] キャンセルされました" "Red"
            exit 0
        }
    }

    Show-Progress -Activity "プロセス終了" -Status "実行中のプロセスを終了中..." -PercentComplete 25

    foreach ($process in $runningProcesses) {
        try {
            Write-ColoredMessage "[STOP] プロセス終了: PID $($process.Id)" "Yellow"
            Stop-Process -Id $process.Id -Force -ErrorAction Stop
            Write-ColoredMessage "[OK] プロセス終了成功" "Green"
        }
        catch {
            Write-ColoredMessage "[WARN] プロセス終了失敗: $($_.Exception.Message)" "Red"
        }
    }

    # 少し待って完全に終了するのを待つ
    Start-Sleep -Seconds 2
} else {
    Write-ColoredMessage "[OK] 実行中のプロセスはありません" "Green"
}

Show-Progress -Activity "ファイルコピー" -Status "新しいバイナリをコピー中..." -PercentComplete 50

# ターゲットディレクトリの作成
$targetDir = Split-Path $TargetPath -Parent
if (!(Test-Path $targetDir)) {
    try {
        New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
        Write-ColoredMessage "[DIR] ディレクトリ作成: $targetDir" "Gray"
    }
    catch {
        Write-ColoredMessage "[ERROR] ディレクトリ作成失敗: $($_.Exception.Message)" "Red"
        exit 1
    }
}

# ファイルコピー
try {
    Copy-Item -Path $SourcePath -Destination $TargetPath -Force
    Write-ColoredMessage "[OK] ファイルコピー成功" "Green"
} catch {
    Write-ColoredMessage "[ERROR] ファイルコピー失敗: $($_.Exception.Message)" "Red"
    exit 1
}

Show-Progress -Activity "インストール完了" -Status "インストール完了を確認中..." -PercentComplete 75

# インストール確認
if (Test-Path $TargetPath) {
    $fileInfo = Get-Item $TargetPath
    Write-ColoredMessage "[OK] インストール成功!" "Green"
    Write-ColoredMessage "[STATS] ファイル情報:" "Gray"
    Write-ColoredMessage "  [DIR] パス: $($fileInfo.FullName)" "Gray"
    Write-ColoredMessage "  📏 サイズ: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" "Gray"
    Write-ColoredMessage "  [TIME] 更新日時: $($fileInfo.LastWriteTime)" "Gray"

    # バージョン情報取得（可能であれば）
    try {
        $version = & $TargetPath --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-ColoredMessage "  🏷️ バージョン: $version" "Gray"
        }
    } catch {
        # バージョン取得失敗は無視
    }
} else {
    Write-ColoredMessage "[ERROR] インストール確認失敗" "Red"
    exit 1
}

Show-Progress -Activity "完了" -Status "すべての処理が完了しました" -PercentComplete 100

Write-ColoredMessage "[SUCCESS] 上書きインストール完了!" "Green"
Write-ColoredMessage "[START] 新しいCodexを使用できます" "Cyan"

# 完了音を鳴らす
[console]::beep(800, 200)
[console]::beep(1000, 200)
[console]::beep(1200, 200)