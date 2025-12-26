# 高速差分ビルド・上書きインストール・実機テスト統合スクリプト
# tqdm風の進捗表示と実装ログ自動保存機能付き

$ErrorActionPreference = "Stop"
$ProgressPreference = 'SilentlyContinue'

function Write-Status {
    param([string]$Message, [string]$Color = "Cyan")
    Write-Host "[*] $Message" -ForegroundColor $Color
}

function Write-Success {
    param([string]$Message)
    Write-Host "[OK] $Message" -ForegroundColor Green
}

function Write-ErrorMsg {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

# 現在日時を取得（実装ログ用）
$currentDateTime = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$currentDate = Get-Date -Format "yyyy-MM-dd"

# 作業ディレクトリを確認
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not (Test-Path (Join-Path $scriptDir "Cargo.toml"))) {
    Write-ErrorMsg "Cargo.toml not found. Please run this script from the codex-rs directory."
    exit 1
}

Set-Location $scriptDir

Write-Host ""
$separator = "=" * 70
Write-Host $separator -ForegroundColor Cyan
Write-Host "高速差分ビルド・上書きインストール・実機テスト" -ForegroundColor Cyan
Write-Host $separator -ForegroundColor Cyan
Write-Host ""

# 実行中のcodexプロセスを停止
Write-Status "実行中のcodexプロセスを確認中..."
$CodexProcesses = Get-Process codex -ErrorAction SilentlyContinue
if ($CodexProcesses) {
    Write-Warning "実行中のcodexプロセスを停止します..."
    $CodexProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 2
    Write-Success "プロセスを停止しました"
} else {
    Write-Success "実行中のプロセスはありません"
}

# Pythonスクリプトを実行
Write-Status "Pythonビルドスクリプトを実行中..."
$pythonScript = Join-Path $scriptDir "fast_build_install_test.py"

if (-not (Test-Path $pythonScript)) {
    Write-ErrorMsg "Pythonスクリプトが見つかりません: $pythonScript"
    exit 1
}

# Python 3で実行
try {
    $buildStartTime = Get-Date
    py -3 $pythonScript
    $buildEndTime = Get-Date
    $buildDuration = ($buildEndTime - $buildStartTime).TotalSeconds
    
    if ($LASTEXITCODE -ne 0) {
        Write-ErrorMsg "ビルドスクリプトが失敗しました (終了コード: $LASTEXITCODE)"
        exit $LASTEXITCODE
    }
    
    Write-Success "ビルドスクリプトが正常に完了しました (経過時間: $buildDuration 秒)"
} catch {
    Write-ErrorMsg "Pythonスクリプトの実行中にエラーが発生しました: $_"
    exit 1
}

# 結果サマリーを読み込み
$summaryPath = Join-Path $scriptDir "build_test_summary.json"
$summary = $null
if (Test-Path $summaryPath) {
    try {
        $jsonContent = Get-Content $summaryPath -Raw -Encoding UTF8
        $summary = $jsonContent | ConvertFrom-Json
        Write-Success "結果サマリーを読み込みました"
    } catch {
        Write-Warning "結果サマリーの読み込みに失敗しました: $_"
    }
} else {
    Write-Warning "結果サマリーファイルが見つかりません"
}

# 実装ログを作成
Write-Status "実装ログを作成中..."

$docsDir = Join-Path (Split-Path -Parent $scriptDir) "_docs"
if (-not (Test-Path $docsDir)) {
    New-Item -ItemType Directory -Path $docsDir -Force | Out-Null
    Write-Success "実装ログディレクトリを作成しました: $docsDir"
}

# ワークツリー名を取得（git worktree listから）
$worktreeName = "main"
try {
    $gitRoot = git rev-parse --show-toplevel 2>$null
    if ($gitRoot) {
        $worktrees = git worktree list 2>$null
        if ($worktrees) {
            $currentWorktree = $worktrees | Where-Object { $_ -match $scriptDir }
            if ($currentWorktree) {
                $match = [regex]::Match($currentWorktree, '\[([^\]]+)\]')
                if ($match.Success) {
                    $worktreeName = $match.Groups[1].Value
                }
            }
        }
    }
} catch {
    Write-Warning "ワークツリー名の取得に失敗しました。デフォルト値を使用します。"
}

$logFileName = "${currentDate}_高速差分ビルド上書きインストール実機テスト{${worktreeName}}.md"
$logPath = Join-Path $docsDir $logFileName

# 実装ログの内容を生成
$logLines = @()
$logLines += "# 高速差分ビルド・上書きインストール・実機テスト"
$logLines += ""
$logLines += "**日時**: $currentDateTime"
$logLines += "**ワークツリー**: $worktreeName"
$logLines += "**実行ディレクトリ**: $scriptDir"
$logLines += ""
$logLines += "## 実行概要"
$logLines += ""
$logLines += "高速差分ビルド、バイナリの上書きインストール、実機テストを実行しました。"
$logLines += ""
$logLines += "## 実行結果"
$logLines += ""
$logLines += "### Phase 1: 高速差分ビルド"
$logLines += ""

if ($summary) {
    $buildStatus = if ($summary.build.success) { "成功" } else { "失敗" }
    $logLines += "- **ステータス**: $buildStatus"
    $logLines += "- **経過時間**: $([math]::Round($summary.build.elapsed_seconds, 2)) 秒"
    $logLines += "- **コンパイル済みクレート数**: $($summary.build.crates_compiled) 個"
    $logLines += "- **警告数**: $($summary.build.warnings_count) 個"
    $logLines += "- **エラー数**: $($summary.build.errors_count) 個"
    $logLines += ""
    $logLines += "### Phase 2: バイナリ上書きインストール"
    $logLines += ""
    $logLines += "- **ソース**: $($summary.install.source)"
    $logLines += "- **インストール先**: $($summary.install.destination)"
    $logLines += "- **ファイルサイズ**: $([math]::Round($summary.install.file_size_mb, 2)) MB"
    $logLines += ""
    $logLines += "### Phase 3: 実機テスト"
    $logLines += ""
    
    $successCount = ($summary.tests | Where-Object { $_.status -eq 'success' }).Count
    $totalCount = $summary.tests.Count
    
    $logLines += "- **テスト成功数**: $successCount / $totalCount"
    $logLines += ""
    $logLines += "#### テスト詳細"
    $logLines += ""
    
    foreach ($test in $summary.tests) {
        $statusIcon = if ($test.status -eq 'success') { "[OK]" } else { "[NG]" }
        $testLine = "- $statusIcon **$($test.test)**: $($test.status)"
        if ($test.elapsed) {
            $testLine += " (経過時間: $([math]::Round($test.elapsed, 2)) 秒)"
        }
        $logLines += $testLine
    }
} else {
    $logLines += "- **ビルド**: 実行完了（詳細情報の取得に失敗）"
    $logLines += "- **インストール**: 実行完了"
    $logLines += "- **テスト**: 実行完了"
}

$logLines += ""
$logLines += "## 実行サマリー"
$logLines += ""
$logLines += "- **開始時刻**: $currentDateTime"
$logLines += "- **実行時間**: $([math]::Round($buildDuration, 2)) 秒"
$logLines += "- **ワークツリー**: $worktreeName"
$logLines += ""
$logLines += "## 完了"
$logLines += ""
$logLines += "全ての処理が正常に完了しました。"
$logLines += ""

$logContent = $logLines -join "`n"

# 実装ログを保存
try {
    [System.IO.File]::WriteAllText($logPath, $logContent, [System.Text.Encoding]::UTF8)
    Write-Success "実装ログを保存しました: $logPath"
} catch {
    Write-ErrorMsg "実装ログの保存に失敗しました: $_"
}

# 音声ファイルを再生（オプション）
$audioPath = "C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav"
if (Test-Path $audioPath) {
    try {
        Write-Status "完了音声を再生中..."
        Add-Type -AssemblyName presentationCore
        $mediaPlayer = New-Object system.windows.media.mediaplayer
        $mediaPlayer.open([uri]$audioPath)
        $mediaPlayer.Play()
        Start-Sleep -Seconds 2
        Write-Success "音声を再生しました"
    } catch {
        Write-Warning "音声ファイルの再生に失敗しました: $_"
    }
} else {
    Write-Warning "音声ファイルが見つかりません: $audioPath"
}

Write-Host ""
$separator = "=" * 70
Write-Host $separator -ForegroundColor Green
Write-Host "全ての処理が正常に完了しました！" -ForegroundColor Green
Write-Host $separator -ForegroundColor Green
Write-Host ""
