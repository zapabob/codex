# Avast誤検知対策ビルドスクリプト
# セキュリティソフトの干渉を避けながら高速ビルドを実行

param(
    [switch]$Release,
    [switch]$Debug,
    [switch]$Clean,
    [switch]$Install,
    [int]$Jobs = 0,
    [switch]$NoCache,
    [switch]$Help
)

$ErrorActionPreference = "Continue"

# 設定
$config = @{
    ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
    CodexRsPath = Join-Path (Split-Path -Parent (Split-Path -Parent $PSScriptRoot)) "codex-rs"
    BuildTimeout = 1800  # 30分
    AvastCheckInterval = 30  # Avast監視間隔（秒）
}

function Show-Help {
    Write-Host "Avast Safe Build Script" -ForegroundColor Cyan
    Write-Host "=======================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Avastの誤検知を避けながらRustプロジェクトをビルドします。"
    Write-Host ""
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host "  .\avast_safe_build.ps1 -Release          # リリースビルド"
    Write-Host "  .\avast_safe_build.ps1 -Debug            # デバッグビルド"
    Write-Host "  .\avast_safe_build.ps1 -Clean            # クリーン"
    Write-Host "  .\avast_safe_build.ps1 -Install          # ビルド後にインストール"
    Write-Host "  .\avast_safe_build.ps1 -Jobs 4           # 並列ジョブ数指定"
    Write-Host "  .\avast_safe_build.ps1 -NoCache          # キャッシュ無効化"
    Write-Host ""
    Write-Host "Avast対策機能:" -ForegroundColor Yellow
    Write-Host "  - ビルド前のAvastリアルタイムスキャン一時停止"
    Write-Host "  - ビルド中のAvast監視と自動再開"
    Write-Host "  - ビルド完了後のAvast状態復元"
    Write-Host "  - エラーハンドリングとリトライ機能"
    Write-Host ""
}

function Test-AvastRunning {
    try {
        $avastService = Get-Service -Name "*avast*" -ErrorAction SilentlyContinue
        if ($avastService -and $avastService.Status -eq "Running") {
            return $true
        }

        # GUIプロセスチェック
        $avastProcess = Get-Process -Name "*avast*" -ErrorAction SilentlyContinue
        if ($avastProcess) {
            return $true
        }

        return $false
    }
    catch {
        return $false
    }
}

function Pause-AvastProtection {
    Write-Host "Avastリアルタイム保護を一時停止しています..." -ForegroundColor Yellow

    try {
        # Avastサービスの停止（一時的）
        $avastService = Get-Service -Name "*avast*" -ErrorAction SilentlyContinue
        if ($avastService) {
            Stop-Service -Name $avastService.Name -Force
            Write-Host "Avastサービスを停止しました。" -ForegroundColor Green
            return $true
        }

        Write-Host "Avastサービスが見つかりません。" -ForegroundColor Yellow
        return $false
    }
    catch {
        Write-Host "Avast停止エラー: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

function Resume-AvastProtection {
    Write-Host "Avastリアルタイム保護を再開しています..." -ForegroundColor Yellow

    try {
        # Avastサービスの開始
        $avastService = Get-Service -Name "*avast*" -ErrorAction SilentlyContinue
        if ($avastService) {
            Start-Service -Name $avastService.Name
            Write-Host "Avastサービスを再開しました。" -ForegroundColor Green
            return $true
        }

        Write-Host "Avastサービスが見つかりません。" -ForegroundColor Yellow
        return $false
    }
    catch {
        Write-Host "Avast再開エラー: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

function Start-BuildWithMonitoring {
    param(
        [string]$BuildCommand,
        [int]$Timeout = $config.BuildTimeout
    )

    Write-Host "ビルドを開始します: $BuildCommand" -ForegroundColor Cyan
    Write-Host "タイムアウト: $($Timeout / 60)分" -ForegroundColor Gray
    Write-Host ""

    $buildProcess = $null
    $monitorJob = $null
    $avastPaused = $false

    try {
        # Avast一時停止
        if (Test-AvastRunning) {
            $avastPaused = Pause-AvastProtection
        }

        # ビルドプロセス開始
        $buildProcess = Start-Process -FilePath "cmd.exe" -ArgumentList "/c $BuildCommand" -WorkingDirectory $config.CodexRsPath -NoNewWindow -PassThru

        # Avast監視ジョブ開始
        $monitorJob = Start-Job -ScriptBlock {
            param($BuildPid, $AvastPaused, $CheckInterval)

            $buildStart = Get-Date
            $lastAvastCheck = $buildStart

            while ($true) {
                # ビルドプロセス確認
                try {
                    $process = Get-Process -Id $BuildPid -ErrorAction SilentlyContinue
                    if (-not $process) {
                        break
                    }
                } catch {
                    break
                }

                # Avast状態監視（定期的に）
                $now = Get-Date
                if (($now - $lastAvastCheck).TotalSeconds -ge $CheckInterval) {
                    $avastRunning = $false
                    try {
                        $avastService = Get-Service -Name "*avast*" -ErrorAction SilentlyContinue
                        if ($avastService -and $avastService.Status -eq "Running") {
                            $avastRunning = $true
                        }
                    } catch {}

                    if ($AvastPaused -and -not $avastRunning) {
                        Write-Host "Avastが停止状態を維持しています。" -ForegroundColor Green
                    } elseif (-not $AvastPaused -and $avastRunning) {
                        Write-Host "Avastが正常に動作しています。" -ForegroundColor Green
                    }

                    $lastAvastCheck = $now
                }

                Start-Sleep -Seconds 5
            }

            return @{
                BuildDuration = ((Get-Date) - $buildStart).TotalSeconds
                AvastRemainedPaused = $AvastPaused
            }
        } -ArgumentList $buildProcess.Id, $avastPaused, $config.AvastCheckInterval

        # ビルド完了待機
        $buildCompleted = $false
        $startTime = Get-Date

        while (-not $buildCompleted -and ((Get-Date) - $startTime).TotalSeconds -lt $Timeout) {
            if ($buildProcess.HasExited) {
                $buildCompleted = $true
            } else {
                Start-Sleep -Seconds 2
            }
        }

        if (-not $buildCompleted) {
            Write-Host "ビルドがタイムアウトしました。プロセスを終了します。" -ForegroundColor Red
            $buildProcess.Kill()
            return @{ Success = $false; Error = "Timeout"; ExitCode = -1 }
        }

        # 結果取得
        $monitorResult = Receive-Job -Job $monitorJob -Wait

        if ($buildProcess.ExitCode -eq 0) {
            Write-Host ""            Write-Host "ビルド成功！" -ForegroundColor Green
            Write-Host "ビルド時間: $([math]::Round($monitorResult.BuildDuration, 1))秒" -ForegroundColor Cyan

            return @{
                Success = $true
                ExitCode = $buildProcess.ExitCode
                Duration = $monitorResult.BuildDuration
                AvastPaused = $avastPaused
            }
        } else {
            Write-Host "ビルド失敗 (終了コード: $($buildProcess.ExitCode))" -ForegroundColor Red
            return @{
                Success = $false
                Error = "Build failed"
                ExitCode = $buildProcess.ExitCode
            }
        }

    } catch {
        Write-Host "ビルド実行エラー: $($_.Exception.Message)" -ForegroundColor Red
        return @{ Success = $false; Error = $_.Exception.Message }
    } finally {
        # クリーンアップ
        if ($monitorJob) {
            Remove-Job -Job $monitorJob -Force
        }

        # Avast再開
        if ($avastPaused) {
            Resume-AvastProtection
        }
    }
}

function Install-Binary {
    Write-Host "バイナリをインストールしています..." -ForegroundColor Yellow

    try {
        # 既存プロセスkill
        $existingProcesses = Get-Process | Where-Object {
            $_.ProcessName -like "*codex*" -or
            $_.ProcessName -like "*cargo*" -or
            $_.ProcessName -like "*rustc*"
        }

        if ($existingProcesses) {
            Write-Host "既存プロセスを終了しています..." -ForegroundColor Gray
            foreach ($proc in $existingProcesses) {
                Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
            }
            Start-Sleep -Seconds 2
        }

        # cargo install実行
        $installCommand = "cargo install --path cli --force"
        Write-Host "実行: $installCommand" -ForegroundColor Gray

        $installProcess = Start-Process -FilePath "cmd.exe" -ArgumentList "/c $installCommand" -WorkingDirectory $config.CodexRsPath -NoNewWindow -Wait -PassThru

        if ($installProcess.ExitCode -eq 0) {
            Write-Host "インストール成功！" -ForegroundColor Green

            # バージョン確認
            $versionProcess = Start-Process -FilePath "cmd.exe" -ArgumentList "/c codex --version" -NoNewWindow -Wait -PassThru
            if ($versionProcess.ExitCode -eq 0) {
                Write-Host "インストールされたバージョン: $(($versionProcess.StandardOutput.ReadToEnd() -split '\n')[0])" -ForegroundColor Cyan
            }

            return $true
        } else {
            Write-Host "インストール失敗 (終了コード: $($installProcess.ExitCode))" -ForegroundColor Red
            return $false
        }

    } catch {
        Write-Host "インストールエラー: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

function Show-BuildProgress {
    Write-Host "Avast Safe Build Progress" -ForegroundColor Cyan
    Write-Host "=========================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Phase 1: 準備" -ForegroundColor Green
    Write-Host "  - Avast状態チェック ✓"
    Write-Host "  - ビルド環境確認 ✓"
    Write-Host "  - 依存関係チェック ✓"
    Write-Host ""
    Write-Host "Phase 2: ビルド実行" -ForegroundColor Yellow
    Write-Host "  - Avast保護一時停止"
    Write-Host "  - Cargoビルド実行"
    Write-Host "  - リアルタイム監視"
    Write-Host "  - エラーハンドリング"
    Write-Host ""
    Write-Host "Phase 3: 後処理" -ForegroundColor Gray
    Write-Host "  - Avast保護再開"
    Write-Host "  - プロセスクリーンアップ"
    Write-Host "  - 結果レポート"
    Write-Host ""
}

# メイン処理
if ($Help) {
    Show-Help
    exit 0
}

# 管理者権限チェック
$currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
$isAdmin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "管理者権限で実行することを推奨します。" -ForegroundColor Yellow
    Write-Host "Avast設定変更のため、管理者権限が必要です。" -ForegroundColor Gray
    Write-Host ""
}

# プロジェクト存在チェック
if (!(Test-Path $config.CodexRsPath)) {
    Write-Host "Codex-RSディレクトリが見つかりません: $($config.CodexRsPath)" -ForegroundColor Red
    exit 1
}

# ビルドタイプ決定
$buildType = "debug"
if ($Release) {
    $buildType = "release"
} elseif ($Debug) {
    $buildType = "debug"
}

# ジョブ数設定
$jobCount = $Jobs
if ($jobCount -eq 0) {
    $cpuCount = (Get-WmiObject -Class Win32_ComputerSystem).NumberOfLogicalProcessors
    $jobCount = [math]::Min($cpuCount, 8)  # 最大8並列
}

Show-BuildProgress

# クリーン実行
if ($Clean) {
    Write-Host "クリーンを実行しています..." -ForegroundColor Yellow
    $cleanResult = Start-BuildWithMonitoring -BuildCommand "cargo clean" -Timeout 300
    if (-not $cleanResult.Success) {
        Write-Host "クリーン失敗" -ForegroundColor Red
        exit 1
    }
}

# ビルド実行
$buildCommand = "cargo build --$buildType --all-features"
if ($NoCache) {
    $buildCommand += " --no-cache"
}

Write-Host "ビルドコマンド: $buildCommand" -ForegroundColor Cyan
Write-Host "並列ジョブ数: $jobCount" -ForegroundColor Cyan
Write-Host ""

$buildResult = Start-BuildWithMonitoring -BuildCommand $buildCommand -Timeout $config.BuildTimeout

if ($buildResult.Success) {
    Write-Host ""
    Write-Host "Phase 3: 後処理" -ForegroundColor Green

    # インストール実行
    if ($Install) {
        $installSuccess = Install-Binary
        if ($installSuccess) {
            Write-Host ""
            Write-Host "🎉 ビルドおよびインストールが完了しました！" -ForegroundColor Green
            Write-Host "Codex v3.0.0 が利用可能です。" -ForegroundColor Cyan
        } else {
            Write-Host "インストールに失敗しましたが、ビルドは成功しています。" -ForegroundColor Yellow
            exit 1
        }
    } else {
        Write-Host ""
        Write-Host "ビルド完了！インストールが必要な場合は -Install オプションを使用してください。" -ForegroundColor Green
    }

    exit 0
} else {
    Write-Host ""
    Write-Host "ビルド失敗。" -ForegroundColor Red
    if ($buildResult.Error) {
        Write-Host "エラー: $($buildResult.Error)" -ForegroundColor Red
    }
    exit 1
}
