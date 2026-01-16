# Rust高速差分ビルドスクリプト（インクリメンタル + キャッシュ最適化）
# CI/CD高速化とプロセス管理を統合

param(
    [switch]$Release = $false,
    [switch]$Clean = $false,
    [switch]$NoCache = $false,
    [int]$Jobs = 0,
    [switch]$KillProcesses = $true
)

$ErrorActionPreference = "Continue"

# 設定
$config = @{
    ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    CodexRsPath = Join-Path (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)) "codex-rs"
    LogPath = ".cursor\debug.log"
    TempDir = [System.IO.Path]::GetTempPath()
    IncrementalEnabled = $true
    UseSccache = $true
    KillTimeout = 30
}

# ログ関数
function Write-DebugLog {
    param(
        [string]$location,
        [string]$message,
        [hashtable]$data = @{},
        [string]$hypothesisId = ""
    )
    $logEntry = @{
        id = "log_$(Get-Date -Format 'yyyyMMddHHmmssfff')_$(New-Guid)"
        timestamp = [DateTimeOffset]::Now.ToUnixTimeMilliseconds()
        location = $location
        message = $message
        data = $data
        sessionId = "rust-build-session"
        runId = "incremental-build"
        hypothesisId = $hypothesisId
    } | ConvertTo-Json -Compress
    Add-Content -Path $config.LogPath -Value $logEntry -Encoding UTF8 -ErrorAction SilentlyContinue
}

Write-DebugLog -location "rust_incremental_build.ps1:1" -message "高速差分ビルド開始" -data @{release = $Release; clean = $Clean; noCache = $NoCache; jobs = $Jobs} -hypothesisId "A"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Rust 高速差分ビルドシステム" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# システム情報収集
$cpuInfo = Get-WmiObject Win32_Processor
$cpuCores = $cpuInfo.NumberOfLogicalProcessors
$cpuName = $cpuInfo.Name

$memoryInfo = Get-WmiObject Win32_OperatingSystem
$totalMemoryGB = [math]::Round($memoryInfo.TotalVisibleMemorySize / 1MB, 2)

Write-Host "システム情報:" -ForegroundColor White
Write-Host "  CPU: $cpuName ($cpuCores cores)" -ForegroundColor Gray
Write-Host "  Memory: $totalMemoryGB GB" -ForegroundColor Gray
Write-Host "  Build Mode: $(if ($Release) { 'Release' } else { 'Debug' })" -ForegroundColor Gray
Write-Host "  Incremental: $(if ($config.IncrementalEnabled) { 'Enabled' } else { 'Disabled' })" -ForegroundColor Gray
Write-Host "  Sccache: $(if ($config.UseSccache) { 'Enabled' } else { 'Disabled' })" -ForegroundColor Gray
Write-Host ""

# プロセス管理（既存プロセス終了）
if ($KillProcesses) {
    Write-Host "プロセス管理: 実行中のCodexプロセスを終了..." -ForegroundColor Yellow

    # 実行中のcodex関連プロセスを検知・終了
    $codexProcesses = Get-Process | Where-Object {
        $_.ProcessName -like "*codex*" -or
        $_.MainModule.FileName -like "*codex*"
    }

    if ($codexProcesses) {
        Write-Host "検知されたプロセス:" -ForegroundColor Gray
        foreach ($proc in $codexProcesses) {
            Write-Host "  - $($proc.ProcessName) (PID: $($proc.Id))" -ForegroundColor Gray
        }

        # プロセス終了
        foreach ($proc in $codexProcesses) {
            try {
                Stop-Process -Id $proc.Id -Force -ErrorAction Stop
                Write-Host "  ✓ プロセス終了: $($proc.ProcessName) ($($proc.Id))" -ForegroundColor Green
                Write-DebugLog -location "rust_incremental_build.ps1:2" -message "プロセス終了" -data @{processName = $proc.ProcessName; pid = $proc.Id} -hypothesisId "A"
            } catch {
                Write-Host "  ⚠ プロセス終了失敗: $($proc.ProcessName) ($($proc.Id)) - $($_.Exception.Message)" -ForegroundColor Yellow
            }
        }

        # プロセスが完全に終了するまで待機
        Write-Host "プロセス完全終了待機..." -ForegroundColor Gray
        $timeout = $config.KillTimeout
        $startTime = Get-Date

        while ((Get-Process | Where-Object { $_.ProcessName -like "*codex*" }).Count -gt 0 -and ((Get-Date) - $startTime).TotalSeconds -lt $timeout) {
            Start-Sleep -Milliseconds 500
        }

        $remainingProcesses = Get-Process | Where-Object { $_.ProcessName -like "*codex*" }
        if ($remainingProcesses) {
            Write-Host "⚠ 以下のプロセスが残存: $($remainingProcesses | ForEach-Object { "$($_.ProcessName)($($_.Id))" } -join ', ')" -ForegroundColor Yellow
        } else {
            Write-Host "✅ 全プロセス正常終了" -ForegroundColor Green
        }
    } else {
        Write-Host "ℹ 実行中のCodexプロセスなし" -ForegroundColor Gray
    }
    Write-Host ""
}

# プロジェクトディレクトリ移動
Set-Location $config.CodexRsPath
Write-Host "作業ディレクトリ: $(Get-Location)" -ForegroundColor White
Write-Host ""

# キャッシュクリーンアップ（オプション）
if ($Clean) {
    Write-Host "キャッシュクリーンアップ..." -ForegroundColor Yellow
    Write-DebugLog -location "rust_incremental_build.ps1:3" -message "キャッシュクリーンアップ開始" -hypothesisId "A"

    # Cargoキャッシュクリーンアップ
    cargo clean
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Cargoキャッシュクリーンアップ完了" -ForegroundColor Green
    } else {
        Write-Host "⚠ Cargoキャッシュクリーンアップ失敗" -ForegroundColor Yellow
    }

    # sccacheクリア（使用する場合）
    if ($config.UseSccache -and (Get-Command sccache -ErrorAction SilentlyContinue)) {
        sccache --stop-server 2>$null
        sccache --start-server
        Write-Host "✅ Sccacheキャッシュクリア完了" -ForegroundColor Green
    }

    Write-DebugLog -location "rust_incremental_build.ps1:4" -message "キャッシュクリーンアップ完了" -hypothesisId "A"
    Write-Host ""
}

# 環境変数設定（高速ビルド最適化）
$env:CARGO_INCREMENTAL = if ($config.IncrementalEnabled) { "1" } else { "0" }
$env:RUSTC_WRAPPER = if ($config.UseSccache -and (Get-Command sccache -ErrorAction SilentlyContinue)) { "sccache" } else { "" }

# 並列ビルドジョブ数設定
if ($Jobs -gt 0) {
    $env:CARGO_BUILD_JOBS = $Jobs
} elseif ($cpuCores -le 4) {
    $env:CARGO_BUILD_JOBS = [math]::Max(1, $cpuCores - 1)
} else {
    $env:CARGO_BUILD_JOBS = [math]::Min($cpuCores, 8)  # 最大8並列
}

# メモリ最適化設定
$env:RUST_BACKTRACE = "0"  # デバッグ時は1に設定可能
$env:RUSTFLAGS = "-C target-cpu=native -C opt-level=2"

Write-Host "ビルド環境設定:" -ForegroundColor White
Write-Host "  CARGO_INCREMENTAL: $env:CARGO_INCREMENTAL" -ForegroundColor Gray
Write-Host "  RUSTC_WRAPPER: $($env:RUSTC_WRAPPER ? $env:RUSTC_WRAPPER : 'None')" -ForegroundColor Gray
Write-Host "  CARGO_BUILD_JOBS: $env:CARGO_BUILD_JOBS" -ForegroundColor Gray
Write-Host "  RUSTFLAGS: $env:RUSTFLAGS" -ForegroundColor Gray
Write-Host ""

# 差分ビルドチェック（変更ファイル分析）
Write-Host "差分ビルド分析..." -ForegroundColor Yellow
Write-DebugLog -location "rust_incremental_build.ps1:5" -message "差分ビルド分析開始" -hypothesisId "A"

$needsFullBuild = $true

if (-not $Clean -and -not $NoCache) {
    try {
        # Git変更ファイル分析
        $changedFiles = git diff --name-only --cached
        $stagedFiles = git diff --name-only

        $allChangedFiles = ($changedFiles + $stagedFiles) | Where-Object { $_ -ne "" }

        # Rustファイルの変更をチェック
        $rustChanges = $allChangedFiles | Where-Object { $_ -like "*.rs" -or $_ -like "Cargo.*" }
        $coreChanges = $allChangedFiles | Where-Object { $_ -like "core/*" -or $_ -like "core/src/*" }
        $cliChanges = $allChangedFiles | Where-Object { $_ -like "cli/*" -or $_ -like "cli/src/*" }

        if ($rustChanges.Count -eq 0 -and -not $Release) {
            Write-Host "ℹ Rustコード変更なし - 差分ビルドをスキップ" -ForegroundColor Cyan
            $needsFullBuild = $false

            # ターゲットディレクトリの存在確認
            if (Test-Path "target/debug/codex-cli.exe" -or Test-Path "target/debug/codex-cli") {
                Write-Host "✅ 既存バイナリを使用" -ForegroundColor Green
                Write-DebugLog -location "rust_incremental_build.ps1:6" -message "差分ビルドスキップ - 変更なし" -hypothesisId "A"
            } else {
                Write-Host "⚠ バイナリが見つからないためフルビルド実行" -ForegroundColor Yellow
                $needsFullBuild = $true
            }
        } else {
            Write-Host "📝 変更検知: $($rustChanges.Count)個のRustファイル" -ForegroundColor White
            if ($coreChanges) { Write-Host "  Core: $($coreChanges.Count)ファイル" -ForegroundColor Gray }
            if ($cliChanges) { Write-Host "  CLI: $($cliChanges.Count)ファイル" -ForegroundColor Gray }
            $needsFullBuild = $true
        }
    } catch {
        Write-Host "⚠ Git変更分析失敗 - フルビルド実行: $($_.Exception.Message)" -ForegroundColor Yellow
        $needsFullBuild = $true
    }
} else {
    Write-Host "ℹ キャッシュ無効またはクリーン指定 - フルビルド実行" -ForegroundColor Cyan
    $needsFullBuild = $true
}

Write-Host ""

# ビルド実行
$buildStart = Get-Date
Write-DebugLog -location "rust_incremental_build.ps1:7" -message "ビルド開始" -data @{needsFullBuild = $needsFullBuild; startTime = $buildStart.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

if ($needsFullBuild) {
    Write-Host "ビルド実行中..." -ForegroundColor Cyan

    # sccache起動（使用する場合）
    if ($config.UseSccache -and (Get-Command sccache -ErrorAction SilentlyContinue)) {
        Write-Host "Sccache起動..." -ForegroundColor Gray
        sccache --start-server 2>$null
    }

    # ビルドコマンド実行
    $buildArgs = @("build", "-p", "codex-cli")
    if ($Release) {
        $buildArgs = @("build", "--release", "-p", "codex-cli")
    }

    Write-Host "実行コマンド: cargo $($buildArgs -join ' ')" -ForegroundColor Gray

    # リアルタイム進捗表示付きビルド
    $job = Start-Job -ScriptBlock {
        param($args, $workingDir)
        Set-Location $workingDir
        & cargo $args 2>&1
    } -ArgumentList $buildArgs, $config.CodexRsPath

    # 進捗監視
    $lastProgressTime = Get-Date
    while ($job.State -eq "Running") {
        Start-Sleep -Milliseconds 1000

        # 5秒ごとに進捗確認
        if (((Get-Date) - $lastProgressTime).TotalSeconds -ge 5) {
            $output = Receive-Job -Job $job -Keep
            if ($output) {
                $progressLines = $output | Where-Object { $_ -match "Compiling|Finished|Checking|error|warning" }
                if ($progressLines) {
                    Write-Host "  [$((Get-Date).ToString('HH:mm:ss'))] $($progressLines[-1])" -ForegroundColor Gray
                }
            }
            $lastProgressTime = Get-Date
        }
    }

    # 最終出力取得
    $buildOutput = Receive-Job -Job $job
    $buildExitCode = $job.State -eq "Completed" ? $job.ChildJobs[0].Output[-1] : $LASTEXITCODE

    Remove-Job -Job $job

    # ビルド結果表示
    $buildOutput | ForEach-Object {
        $line = $_
        if ($line -match "error") {
            Write-Host $line -ForegroundColor Red
        } elseif ($line -match "warning") {
            Write-Host $line -ForegroundColor Yellow
        } else {
            Write-Host $line -ForegroundColor White
        }
    }

} else {
    Write-Host "差分ビルドスキップ - 既存バイナリ使用" -ForegroundColor Cyan
    $buildExitCode = 0
}

$buildEnd = Get-Date
$buildDuration = ($buildEnd - $buildStart).TotalSeconds
Write-DebugLog -location "rust_incremental_build.ps1:8" -message "ビルド完了" -data @{duration = $buildDuration; exitCode = $buildExitCode; endTime = $buildEnd.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

if ($buildExitCode -ne 0) {
    Write-Host ""
    Write-Host "❌ Build failed with exit code $buildExitCode" -ForegroundColor Red
    Write-DebugLog -location "rust_incremental_build.ps1:9" -message "ビルド失敗" -data @{exitCode = $buildExitCode} -hypothesisId "A"
    Set-Location $config.ProjectRoot
    exit $buildExitCode
}

Write-Host ""
Write-Host "✅ Build completed in $([math]::Round($buildDuration, 2)) seconds" -ForegroundColor Green

# インストール実行（プロセスキル付き）
$installStart = Get-Date
Write-DebugLog -location "rust_incremental_build.ps1:10" -message "インストール開始" -data @{startTime = $installStart.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

Write-Host ""
Write-Host "インストール実行中（プロセスキル付き）..." -ForegroundColor Cyan

# インストールコマンド実行
$installArgs = @("install", "--path", "cli", "--force", "--root", "~/.cargo")
if ($Release) {
    $installArgs = @("install", "--path", "cli", "--force", "--release", "--root", "~/.cargo")
}

Write-Host "実行コマンド: cargo $($installArgs -join ' ')" -ForegroundColor Gray

# インストール実行
cargo $installArgs 2>&1 | ForEach-Object {
    $line = $_
    Write-Host $line
    if ($line -match "Installing|Installed|error|warning") {
        Write-DebugLog -location "rust_incremental_build.ps1:11" -message "インストール進捗" -data @{line = $line} -hypothesisId "A"
    }
}

$installEnd = Get-Date
$installDuration = ($installEnd - $installStart).TotalSeconds
Write-DebugLog -location "rust_incremental_build.ps1:12" -message "インストール完了" -data @{duration = $installDuration; endTime = $installEnd.ToString("yyyy-MM-dd HH:mm:ss")} -hypothesisId "A"

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Installation failed with exit code $LASTEXITCODE" -ForegroundColor Red
    Write-DebugLog -location "rust_incremental_build.ps1:13" -message "インストール失敗" -data @{exitCode = $LASTEXITCODE} -hypothesisId "A"
    Set-Location $config.ProjectRoot
    exit $LASTEXITCODE
}

Set-Location $config.ProjectRoot

$totalDuration = ($installEnd - $buildStart).TotalSeconds
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "✅ 高速差分ビルド & インストール完了!" -ForegroundColor Green
Write-Host "  Build time: $([math]::Round($buildDuration, 2)) seconds" -ForegroundColor White
Write-Host "  Install time: $([math]::Round($installDuration, 2)) seconds" -ForegroundColor White
Write-Host "  Total time: $([math]::Round($totalDuration, 2)) seconds" -ForegroundColor White
if ($needsFullBuild) {
    Write-Host "  Build type: Full build" -ForegroundColor White
} else {
    Write-Host "  Build type: Incremental (cached)" -ForegroundColor White
}
Write-Host "========================================" -ForegroundColor Cyan

# インストール検証
Write-Host ""
Write-Host "インストール検証..." -ForegroundColor Yellow
try {
    $versionOutput = codex --version 2>&1
    Write-Host "Codex version: $versionOutput" -ForegroundColor Green
    Write-DebugLog -location "rust_incremental_build.ps1:14" -message "バージョン確認成功" -data @{version = $versionOutput} -hypothesisId "A"
} catch {
    Write-Host "⚠ バージョン確認失敗: $($_.Exception.Message)" -ForegroundColor Yellow
    Write-DebugLog -location "rust_incremental_build.ps1:15" -message "バージョン確認失敗" -data @{error = $_.Exception.Message} -hypothesisId "A"
}

Write-Host ""
Write-Host "🎯 高速差分ビルドシステム稼働中!" -ForegroundColor Green
Write-Host "   - インクリメンタルビルド: $(if ($config.IncrementalEnabled) { '有効' } else { '無効' })" -ForegroundColor Gray
Write-Host "   - Sccache: $(if ($config.UseSccache) { '有効' } else { '無効' })" -ForegroundColor Gray
Write-Host "   - プロセス管理: $(if ($KillProcesses) { '有効' } else { '無効' })" -ForegroundColor Gray
Write-DebugLog -location "rust_incremental_build.ps1:16" -message "高速差分ビルド完了" -data @{totalDuration = $totalDuration; needsFullBuild = $needsFullBuild} -hypothesisId "A"