# Codex Clean Build & Install Script
# クリーン高速ビルドでバイナリをコピーアンドペーストで上書きインストール
# tqdm風の進捗表示付き（残り時間・経過時間表示）

param(
    [string]$InstallPath = "",  # インストール先パス（空の場合は選択プロンプト）
    [switch]$SkipClean = $false  # クリーンスキップ（差分ビルド）
)

$ErrorActionPreference = "Stop"
$ProgressPreference = 'SilentlyContinue'

# カラー出力関数
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

function Stop-BuildToolingProcesses {
    param([int]$WaitSeconds = 2)

    # cargo/rustc が残っていると target ディレクトリのロックで詰まることがあるので掃除
    $procs = Get-Process cargo,rustc -ErrorAction SilentlyContinue
    if ($procs) {
        try {
            $procs | Stop-Process -Force -ErrorAction SilentlyContinue
            Start-Sleep -Seconds $WaitSeconds
            Write-Success "Stopped build tooling processes (cargo/rustc)"
        } catch {
            Write-Warning "Failed to stop build tooling processes, continuing: $_"
        }
    }
}

# スクリプトディレクトリの検出
$scriptPath = $null
if ($PSScriptRoot) {
    $scriptPath = $PSScriptRoot
} elseif ($MyInvocation.MyCommand.Path) {
    $scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
} elseif ($MyInvocation.ScriptName) {
    $scriptPath = Split-Path -Parent $MyInvocation.ScriptName
} elseif ($PSCommandPath) {
    $scriptPath = Split-Path -Parent $PSCommandPath
} else {
    $currentDir = Get-Location
    if (Test-Path (Join-Path $currentDir "codex-rs\Cargo.toml")) {
        $scriptPath = Join-Path $currentDir "codex-rs"
    } elseif (Test-Path (Join-Path $currentDir "Cargo.toml")) {
        $scriptPath = $currentDir
    }
}

# フォールバック: 絶対パス
if (-not $scriptPath -or -not (Test-Path (Join-Path $scriptPath "Cargo.toml"))) {
    $possiblePaths = @(
        "c:\Users\downl\Desktop\codex-main\codex-rs",
        "$env:USERPROFILE\Desktop\codex-main\codex-rs",
        (Join-Path (Get-Location) "codex-rs"),
        (Join-Path (Split-Path (Get-Location) -Parent) "codex-rs")
    )
    
    foreach ($path in $possiblePaths) {
        if (Test-Path (Join-Path $path "Cargo.toml")) {
            $scriptPath = $path
            break
        }
    }
}

if (-not $scriptPath -or -not (Test-Path (Join-Path $scriptPath "Cargo.toml"))) {
    Write-ErrorMsg "Could not determine codex-rs directory"
    Write-Host "Current directory: $(Get-Location)" -ForegroundColor Yellow
    exit 1
}

$codexRsPath = $scriptPath
Set-Location $codexRsPath
Write-Status "Working directory: $(Get-Location)"

# tqdm風進捗表示関数
function Draw-ProgressBar {
    param(
        [int]$Current,
        [int]$Total,
        [double]$ElapsedSeconds,
        [string]$CurrentCrate = ""
    )
    
    $percent = if ($Total -gt 0) { [math]::Round(($Current / $Total) * 100, 1) } else { 0 }
    $barWidth = 40
    $filledWidth = [math]::Round(($Current / $Total) * $barWidth)
    
    # プログレスバー作成
    $bar = ""
    for ($i = 0; $i -lt $barWidth; $i++) {
        if ($i -lt $filledWidth) {
            $bar += "#"
        } else {
            $bar += "-"
        }
    }
    
    # 速度計算
    $rate = if ($ElapsedSeconds -gt 0) { [math]::Round($Current / $ElapsedSeconds, 2) } else { 0 }
    $eta = if ($rate -gt 0 -and $Current -lt $Total) { [math]::Round(($Total - $Current) / $rate, 0) } else { 0 }
    
    # 時間フォーマット
    $elapsedMin = [math]::Floor($ElapsedSeconds / 60)
    $elapsedSec = [math]::Floor($ElapsedSeconds % 60)
    $etaMin = [math]::Floor($eta / 60)
    $etaSec = [math]::Floor($eta % 60)
    
    # 進捗表示（tqdm風）
    $progressLine = "{0,5}% |{1}| {2}/{3} [{4}m{5}s<{6}m{7}s, {8} crates/s]" -f `
        $percent, $bar, $Current, $Total, $elapsedMin, $elapsedSec, $etaMin, $etaSec, $rate
    
    Write-Host "`r$progressLine" -NoNewline -ForegroundColor Cyan
    
    # 現在コンパイル中のクレート
    if ($CurrentCrate) {
        $crateDisplay = if ($CurrentCrate.Length -gt 30) { 
            $CurrentCrate.Substring(0, 27) + "..." 
        } else { 
            $CurrentCrate.PadRight(30) 
        }
        Write-Host " $crateDisplay" -NoNewline -ForegroundColor White
    }
}

# cargo build 出力をパースして進捗を取得
function Parse-CargoProgress {
    param([string]$OutputLine)
    
    # "Compiling crate-name v1.2.3" や "Checking crate-name v1.2.3" を検出
    if ($OutputLine -match '(?:Compiling|Checking)\s+([^\s]+)') {
        return @{
            Crate = $matches[1]
            Type = if ($OutputLine -match 'Compiling') { 'Compiling' } else { 'Checking' }
        }
    }
    # "Finished release [optimized] target(s)" を検出
    if ($OutputLine -match 'Finished\s+release') {
        return @{ Finished = $true }
    }
    return $null
}

# ビルド進捗モニタリング（cargo出力をリアルタイムでパース）
function Start-BuildProgressMonitor {
    param(
        [System.Management.Automation.Job]$BuildJob,
        [string]$BuildDir,
        [int]$TotalCrates = 4000
    )
    
    $startTime = Get-Date
    $compiledCrates = [System.Collections.Generic.HashSet[string]]::new()
    $currentCrate = ""
    $lastOutput = ""
    
    while ($true) {
        $elapsed = (Get-Date) - $startTime
        
        # Job の出力を取得
        $jobOutput = Receive-Job -Job $BuildJob -ErrorAction SilentlyContinue
        if ($jobOutput) {
            foreach ($line in $jobOutput) {
                $parsed = Parse-CargoProgress -OutputLine $line
                if ($parsed) {
                    if ($parsed.Finished) {
                        $currentCrate = ""
                    } elseif ($parsed.Crate) {
                        $compiledCrates.Add($parsed.Crate) | Out-Null
                        $currentCrate = "$($parsed.Type): $($parsed.Crate)"
                    }
                }
            }
        }
        
        $compiledCount = $compiledCrates.Count
        
        # 総クレート数の動的調整
        if ($compiledCount -gt $TotalCrates * 0.9) {
            $TotalCrates = [math]::Max($TotalCrates, $compiledCount + 100)
        }
        
        # 進捗表示
        Draw-ProgressBar -Current $compiledCount -Total $TotalCrates -ElapsedSeconds $elapsed.TotalSeconds -CurrentCrate $currentCrate
        
        # ビルド完了チェック
        $binaryPath = Join-Path $BuildDir "codex.exe"
        $jobState = (Get-Job -Id $BuildJob.Id -ErrorAction SilentlyContinue).State
        if ($jobState -eq 'Completed' -or (Test-Path $binaryPath)) {
            Write-Host ""
            Write-Host ""
            return $true
        }
        if ($jobState -eq 'Failed' -or $jobState -eq 'Stopped') {
            Write-Host ""
            Write-Host ""
            return $false
        }
        
        # 更新間隔
        Start-Sleep -Milliseconds 300
        
        # タイムアウト（60分）
        if ($elapsed.TotalMinutes -gt 60) {
            Write-Host ""
            Write-Host ""
            Write-Warning "Build timeout (60 minutes)"
            return $false
        }
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Codex Clean Build & Install" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Step 1: プロセスキル
Write-Status "Step 1/5: Stopping running codex processes..."
$CodexProcesses = Get-Process codex -ErrorAction SilentlyContinue
if ($CodexProcesses) {
    $CodexProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 1
    Write-Success "Stopped $($CodexProcesses.Count) process(es)"
} else {
    Write-Success "No running processes found"
}

# 追加: ビルドツールの残骸を掃除（ロック回避）
Stop-BuildToolingProcesses -WaitSeconds 2

# Step 2: クリーンビルド
if (-not $SkipClean) {
    Write-Status "Step 2/5: Cleaning build artifacts..."
    try {
        cargo clean
        Write-Success "Clean completed"
    } catch {
        Write-Warning "cargo clean failed, but continuing"
    }
} else {
    Write-Status "Step 2/5: Skipping clean (using incremental build)"
}

# Step 3: ビルド実行（進捗表示付き）
Write-Status "Step 3/5: Building codex-cli (Release)..."
Write-Host ""

$buildDir = Join-Path $codexRsPath "target\release"
$buildStart = Get-Date

    # ビルドをバックグラウンドで開始（ロック待ちが発生したら 1 回だけ掃除してリトライ）
    $buildJob = $null
    $buildOutput = $null
    try {
        $buildJob = Start-Job -ScriptBlock {
            param($Path)
            Set-Location $Path
            $env:CARGO_TERM_PROGRESS_WHEN = "always"
            $env:CARGO_TERM_PROGRESS_WIDTH = "80"
            cargo build --release -p codex-cli --features custom-features 2>&1
        } -ArgumentList $codexRsPath

        # 進捗モニタリング（cargo出力をリアルタイムでパース）
        $monitorResult = Start-BuildProgressMonitor -BuildJob $buildJob -BuildDir $buildDir

        # ビルド完了を待機
        $buildOutput = Wait-Job $buildJob | Receive-Job
} catch {
    Write-Warning "Build job failed or was interrupted: $_"
} finally {
    if ($buildJob) {
        try {
            if ((Get-Job -Id $buildJob.Id -ErrorAction SilentlyContinue).State -eq "Running") {
                Stop-Job -Id $buildJob.Id -Force -ErrorAction SilentlyContinue | Out-Null
            }
            Remove-Job -Id $buildJob.Id -Force -ErrorAction SilentlyContinue | Out-Null
        } catch {}
    }
}

$buildEnd = Get-Date
$buildTime = ($buildEnd - $buildStart).TotalSeconds

# ビルド結果確認
$binaryPath = Join-Path $buildDir "codex.exe"
if (-not (Test-Path $binaryPath)) {
    Write-ErrorMsg "Build failed - binary not found"
    Write-Host $buildOutput -ForegroundColor Red
    exit 1
}

Write-Success "Build completed in $([math]::Round($buildTime / 60, 2)) minutes"
$fileInfo = Get-Item $binaryPath
Write-Host "   Binary size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Gray

# Step 4: インストール先の選択
Write-Status "Step 4/5: Selecting install location..."

if ([string]::IsNullOrEmpty($InstallPath)) {
    Write-Host ""
    Write-Host "Select install location:" -ForegroundColor Yellow
    Write-Host "  1. $env:USERPROFILE\.cargo\bin\codex.exe (default)" -ForegroundColor Cyan
    Write-Host "  2. C:\bin\codex.exe" -ForegroundColor Cyan
    Write-Host ""
    $choice = Read-Host "Enter choice (1 or 2, default: 1)"
    
    if ($choice -eq "2") {
        $InstallPath = "C:\bin\codex.exe"
    } else {
        $InstallPath = "$env:USERPROFILE\.cargo\bin\codex.exe"
    }
}

$InstallDir = Split-Path $InstallPath -Parent
Write-Status "Install path: $InstallPath"

# Step 5: コピーアンドペーストで上書きインストール
Write-Status "Step 5/5: Installing binary (copy & paste overwrite)..."

# インストールディレクトリ作成
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Success "Created install directory: $InstallDir"
}

# 実行中のプロセスを再度確認・終了
Get-Process | Where-Object { $_.Path -eq $InstallPath } | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

# コピーアンドペーストで上書き
try {
    Copy-Item -Path $binaryPath -Destination $InstallPath -Force
    Write-Success "Binary installed successfully"
    
    $installedInfo = Get-Item $InstallPath
    Write-Host "   Installed size: $([math]::Round($installedInfo.Length / 1MB, 2)) MB" -ForegroundColor Gray
    Write-Host "   Install path: $InstallPath" -ForegroundColor Gray
} catch {
    Write-ErrorMsg "Failed to copy binary: $_"
    Write-Host "Binary may be in use. Please check processes." -ForegroundColor Yellow
    exit 1
}

# 検証
Write-Status "Verifying installation..."
try {
    $version = & $InstallPath --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Installation verified: $version"
    } else {
        Write-Warning "Version check failed, but binary exists"
    }
} catch {
    Write-Warning "Version check failed, but binary exists"
}

Write-Host ""
Write-Success "Installation complete!"
Write-Host "   Binary: $InstallPath" -ForegroundColor Cyan
Write-Host ""
