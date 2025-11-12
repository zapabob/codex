# 高速差分ビルド & 強制インストールスクリプト（tqdm風可視化）
param(
    [switch]$Clean = $false,
    [switch]$All = $false
)

$ErrorActionPreference = "Continue"

function Write-ProgressBar {
    param(
        [int]$Percent,
        [string]$Activity,
        [string]$Status,
        [int]$ElapsedSeconds,
        [int]$RemainingSeconds = 0
    )
    
    $barLength = 50
    $filled = [math]::Floor($Percent / 100 * $barLength)
    $empty = $barLength - $filled
    $bar = "█" * $filled + "░" * $empty
    
    $elapsedStr = "{0:D2}:{1:D2}" -f ([math]::Floor($ElapsedSeconds / 60)), ($ElapsedSeconds % 60)
    $remainingStr = if ($RemainingSeconds -gt 0) { "残り: {0:D2}:{1:D2}" -f ([math]::Floor($RemainingSeconds / 60)), ($RemainingSeconds % 60) } else { "" }
    
    Write-Host -NoNewline "`r[$bar] $Percent% | 経過: $elapsedStr $remainingStr | $Status" -ForegroundColor Cyan
}

function Stop-RunningProcesses {
    Write-Host "🛑 起動中のプロセスを確認中..." -ForegroundColor Yellow
    
    # codex関連プロセスを停止
    $codexProcesses = Get-Process | Where-Object {
        $_.ProcessName -like "*codex*"
    }
    
    if ($codexProcesses) {
        Write-Host "  → codexプロセスを停止中..." -ForegroundColor Cyan
        $codexProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 2
        Write-Host "  ✅ codexプロセス停止完了" -ForegroundColor Green
    } else {
        Write-Host "  ✅ 起動中のcodexプロセスなし" -ForegroundColor Green
    }
    
    # cargoビルドプロセスを停止（オプション）
    $cargoProcesses = Get-Process | Where-Object {
        $_.ProcessName -eq "cargo" -and $_.MainWindowTitle -eq ""
    }
    
    if ($cargoProcesses) {
        Write-Host "  → cargoビルドプロセスを停止中..." -ForegroundColor Cyan
        $cargoProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
        Start-Sleep -Seconds 1
        Write-Host "  ✅ cargoプロセス停止完了" -ForegroundColor Green
    }
}

function Invoke-CargoBuild {
    param(
        [string[]]$Packages,
        [string]$Profile = "release"
    )
    
    $startTime = Get-Date
    $buildArgs = @("build", "--$Profile")
    foreach ($pkg in $Packages) {
        $buildArgs += "-p"
        $buildArgs += $pkg
    }
    
    # ビルド出力をリアルタイムでキャプチャ
    $buildOutput = ""
    $buildError = ""
    $process = Start-Process -FilePath "cargo" -ArgumentList $buildArgs -NoNewWindow -PassThru -RedirectStandardOutput "build_output.txt" -RedirectStandardError "build_error.txt"
    
    $lastPercent = 0
    $lastOutput = ""
    $compilingCount = 0
    $finishedCount = 0
    
    while (-not $process.HasExited) {
        Start-Sleep -Milliseconds 500
        
        $elapsed = [math]::Floor((Get-Date) - $startTime).TotalSeconds
        
        # 出力を読み取って進捗を推定
        if (Test-Path "build_output.txt") {
            $output = Get-Content "build_output.txt" -Tail 10 -ErrorAction SilentlyContinue
            if ($output -ne $null -and $output.Count -gt 0) {
                $lastOutput = $output[-1]
                
                # Compiling メッセージから進捗を推定
                $newCompiling = ($output | Select-String -Pattern "Compiling" -AllMatches).Matches.Count
                $newFinished = ($output | Select-String -Pattern "Finished" -AllMatches).Matches.Count
                
                if ($newCompiling -gt $compilingCount) {
                    $compilingCount = $newCompiling
                    $lastPercent = [math]::Min(90, $lastPercent + 3)
                }
                if ($newFinished -gt $finishedCount) {
                    $finishedCount = $newFinished
                    $lastPercent = [math]::Min(95, $lastPercent + 2)
                }
                
                # エラー検出
                if ($lastOutput -match "error\[E") {
                    $lastPercent = [math]::Max(0, $lastPercent - 5)
                }
            }
        }
        
        Write-ProgressBar -Percent $lastPercent -Activity "ビルド中" -Status $lastOutput -ElapsedSeconds $elapsed
    }
    
    Write-Host "" # 改行
    
    # 出力を読み取る
    if (Test-Path "build_output.txt") {
        $buildOutput = Get-Content "build_output.txt" -Raw -ErrorAction SilentlyContinue
    }
    if (Test-Path "build_error.txt") {
        $buildError = Get-Content "build_error.txt" -Raw -ErrorAction SilentlyContinue
    }
    
    $exitCode = $process.ExitCode
    Remove-Item "build_output.txt", "build_error.txt" -ErrorAction SilentlyContinue
    
    return @{
        ExitCode = $exitCode
        Output = $buildOutput
        Error = $buildError
    }
}

function Get-BuildStats {
    param([string]$Output)
    
    $warningCount = ([regex]::Matches($Output, 'warning:')).Count
    $errorCount = ([regex]::Matches($Output, 'error\[E')).Count
    
    return @{
        Warnings = $warningCount
        Errors = $errorCount
    }
}

Write-Host "=== 高速差分ビルド & 強制インストール ===" -ForegroundColor Cyan
Write-Host ""

# 0. 起動中プロセス停止
Stop-RunningProcesses
Write-Host ""

# 1. クリーンビルド（オプション）
if ($Clean) {
    Write-Host "🧹 クリーンビルド実行中..." -ForegroundColor Yellow
    cargo clean 2>&1 | Out-Null
    Write-Host "✅ クリーンビルド完了" -ForegroundColor Green
    Write-Host ""
}

$buildStart = Get-Date

# 2. CLI/TUI ビルド
Write-Host "📦 CLI/TUI ビルド中..." -ForegroundColor Yellow
$cliBuildStart = Get-Date
$cliResult = Invoke-CargoBuild -Packages @("codex-cli", "codex-tui") -Profile "release"
$cliBuildEnd = Get-Date
$cliDuration = ($cliBuildEnd - $cliBuildStart).TotalMinutes

if ($cliResult.ExitCode -ne 0) {
    Write-Host "❌ CLI/TUI ビルド失敗" -ForegroundColor Red
    if ($cliResult.Error) {
        Write-Host $cliResult.Error -ForegroundColor Red
    }
    exit 1
}

$cliStats = Get-BuildStats -Output ($cliResult.Output + $cliResult.Error)
Write-Host "✅ CLI/TUI ビルド成功 ($([math]::Round($cliDuration, 1)) 分)" -ForegroundColor Green
Write-Host "  エラー数: $($cliStats.Errors)" -ForegroundColor $(if ($cliStats.Errors -eq 0) { "Green" } else { "Red" })
Write-Host "  警告数: $($cliStats.Warnings)" -ForegroundColor $(if ($cliStats.Warnings -eq 0) { "Green" } else { "Yellow" })

if ($cliStats.Errors -gt 0) {
    Write-Host "❌ エラーが検出されました。ビルドを中止します。" -ForegroundColor Red
    exit 1
}

# 3. 強制インストール
Write-Host ""
Write-Host "🔧 強制インストール中..." -ForegroundColor Yellow

$installStart = Get-Date

# 再度プロセス確認
Stop-RunningProcesses

# CLI インストール
Write-Host "  → codex-cli インストール中..." -ForegroundColor Cyan
$installOutput = cargo install --path cli --force 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ codex-cli インストール失敗" -ForegroundColor Red
    Write-Host $installOutput -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ codex-cli インストール完了" -ForegroundColor Green

# TUI インストール（必要に応じて）
Write-Host "  → codex-tui インストール確認中..." -ForegroundColor Cyan
$tuiInstallOutput = cargo install --path tui --force 2>&1 | Out-String
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ codex-tui インストール完了" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  codex-tui インストールスキップ（TUIは通常CLIに含まれる）" -ForegroundColor Yellow
}

$installEnd = Get-Date
$installDuration = ($installEnd - $installStart).TotalSeconds

Write-Host "✅ インストール完了 ($([math]::Round($installDuration, 1)) 秒)" -ForegroundColor Green

# 4. GUI ビルド（オプション）
if ($All) {
    Write-Host ""
    Write-Host "🖥️  GUI ビルド中..." -ForegroundColor Yellow
    $guiBuildStart = Get-Date
    
    $tauriGuiPath = Join-Path $PSScriptRoot "tauri-gui"
    if (Test-Path $tauriGuiPath) {
        Push-Location $tauriGuiPath
        
        Write-Host "  → GUI ディレクトリに移動: $tauriGuiPath" -ForegroundColor Cyan
        
        # build.ps1があるか確認
        $buildScript = Join-Path $tauriGuiPath "build.ps1"
        if (Test-Path $buildScript) {
            Write-Host "  → build.ps1 を実行中..." -ForegroundColor Cyan
            & $buildScript
            if ($LASTEXITCODE -ne 0) {
                Write-Host "  ⚠️  GUI ビルド失敗（スキップ）" -ForegroundColor Yellow
            } else {
                Write-Host "  ✅ GUI ビルド成功" -ForegroundColor Green
                
                # MSIインストーラーを探す
                $msiPath = Get-ChildItem -Path (Join-Path $tauriGuiPath "src-tauri\target\release\bundle\msi\*.msi") -ErrorAction SilentlyContinue | Select-Object -First 1
                if ($msiPath) {
                    Write-Host "  → MSIインストーラーを実行中: $($msiPath.Name)" -ForegroundColor Cyan
                    Start-Process -FilePath $msiPath.FullName -Wait -NoNewWindow
                    Write-Host "  ✅ GUI インストール完了" -ForegroundColor Green
                } else {
                    Write-Host "  ⚠️  MSIインストーラーが見つかりませんでした" -ForegroundColor Yellow
                }
            }
        } else {
            # npm run tauri build を試す
            if (Test-Path "package.json") {
                Write-Host "  → npm run tauri build を実行中..." -ForegroundColor Cyan
                npm run tauri build 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "  ✅ GUI ビルド成功" -ForegroundColor Green
                } else {
                    Write-Host "  ⚠️  GUI ビルド失敗（スキップ）" -ForegroundColor Yellow
                }
            } else {
                Write-Host "  ⚠️  GUI ビルドスクリプトが見つかりませんでした（スキップ）" -ForegroundColor Yellow
            }
        }
        
        Pop-Location
        
        $guiBuildEnd = Get-Date
        $guiDuration = ($guiBuildEnd - $guiBuildStart).TotalMinutes
        Write-Host "  GUI ビルド時間: $([math]::Round($guiDuration, 1)) 分" -ForegroundColor Cyan
    } else {
        Write-Host "  ⚠️  tauri-gui ディレクトリが見つかりませんでした（スキップ）" -ForegroundColor Yellow
    }
}

$buildEnd = Get-Date
$totalDuration = ($buildEnd - $buildStart).TotalMinutes

Write-Host ""
Write-Host "=== 完了 ===" -ForegroundColor Green
Write-Host "総時間: $([math]::Round($totalDuration, 1)) 分" -ForegroundColor Green
Write-Host ""

# 5. 動作確認
Write-Host "📋 インストール確認:" -ForegroundColor Cyan
$versionOutput = codex --version 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ $versionOutput" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  バージョン確認失敗: $versionOutput" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📊 ビルド統計:" -ForegroundColor Cyan
Write-Host "  エラー数: $($cliStats.Errors)" -ForegroundColor $(if ($cliStats.Errors -eq 0) { "Green" } else { "Red" })
Write-Host "  警告数: $($cliStats.Warnings)" -ForegroundColor $(if ($cliStats.Warnings -eq 0) { "Green" } else { "Yellow" })

if ($cliStats.Warnings -eq 0 -and $cliStats.Errors -eq 0) {
    Write-Host ""
    Write-Host "🎉 警告0・エラー0でビルド成功！" -ForegroundColor Green
}
