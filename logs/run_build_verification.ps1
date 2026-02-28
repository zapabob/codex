param(
    [string]$RunTag = (Get-Date -Format "yyyyMMdd_HHmmss"),
    [int]$CargoTimeoutSec = 600,
    [int]$GuiTimeoutSec = 600,
    [switch]$UseIsolatedCargoTarget
)

$ErrorActionPreference = "Continue"
Set-StrictMode -Version Latest

$logsRoot = $PSScriptRoot
$repoRoot = Split-Path -Parent $logsRoot
$outputDir = Join-Path $logsRoot ("build\verify_" + $RunTag)
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null

function Invoke-BuildStep {
    param(
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$WorkingDir,
        [Parameter(Mandatory = $true)][string]$FilePath,
        [Parameter(Mandatory = $true)][string[]]$Arguments,
        [Parameter(Mandatory = $true)][int]$TimeoutSec,
        [Parameter(Mandatory = $true)][string]$CombinedLogPath
    )

    $stdoutPath = $CombinedLogPath -replace "\.log$", ".stdout.log"
    $stderrPath = $CombinedLogPath -replace "\.log$", ".stderr.log"
    Remove-Item $stdoutPath, $stderrPath, $CombinedLogPath -ErrorAction SilentlyContinue

    $start = Get-Date
    $exitCode = 1
    $timedOut = $false
    $commandText = "$FilePath $($Arguments -join ' ')"

    try {
        $proc = Start-Process `
            -FilePath $FilePath `
            -ArgumentList $Arguments `
            -WorkingDirectory $WorkingDir `
            -RedirectStandardOutput $stdoutPath `
            -RedirectStandardError $stderrPath `
            -PassThru

        $timedOut = -not $proc.WaitForExit($TimeoutSec * 1000)
        if ($timedOut) {
            Stop-Process -Id $proc.Id -Force
            $exitCode = 124
        } else {
            $exitCode = [int]$proc.ExitCode
        }
    } catch {
        $_ | Out-String | Set-Content -Path $stderrPath
        $exitCode = 1
    }

    $end = Get-Date
    $header = @(
        "# $commandText",
        "",
        "- Working directory: $WorkingDir",
        "- Timeout: ${TimeoutSec}s",
        "",
        "## stdout"
    )
    $header | Set-Content -Path $CombinedLogPath -Encoding UTF8
    if (Test-Path $stdoutPath) {
        Get-Content $stdoutPath | Add-Content -Path $CombinedLogPath
    }
    "" | Add-Content -Path $CombinedLogPath
    "## stderr" | Add-Content -Path $CombinedLogPath
    if (Test-Path $stderrPath) {
        Get-Content $stderrPath | Add-Content -Path $CombinedLogPath
    }
    if ($timedOut) {
        "" | Add-Content -Path $CombinedLogPath
        "[QA note] Command timed out after ${TimeoutSec}s and was terminated." | Add-Content -Path $CombinedLogPath
    }

    return [PSCustomObject]@{
        Name            = $Name
        Command         = $commandText
        WorkingDir      = $WorkingDir
        TimeoutSec      = $TimeoutSec
        TimedOut        = $timedOut
        ExitCode        = $exitCode
        StartedAt       = $start.ToString("s")
        EndedAt         = $end.ToString("s")
        DurationSec     = [Math]::Round(($end - $start).TotalSeconds, 2)
        LogPath         = $CombinedLogPath
        StdoutLogPath   = $stdoutPath
        StderrLogPath   = $stderrPath
    }
}

$rustLog = Join-Path $outputDir "cargo_build_codex-cli.log"
$guiLog = Join-Path $outputDir "npm_build_codex-gui-x.log"
$summaryPath = Join-Path $outputDir "summary.md"
$summaryJsonPath = Join-Path $outputDir "summary.json"

$oldCargoTargetDir = $env:CARGO_TARGET_DIR
if ($UseIsolatedCargoTarget) {
    $isolatedCargoTarget = Join-Path $outputDir "cargo-target"
    New-Item -ItemType Directory -Force -Path $isolatedCargoTarget | Out-Null
    $env:CARGO_TARGET_DIR = $isolatedCargoTarget
}

$results = @()
$results += Invoke-BuildStep `
    -Name "Rust CLI build" `
    -WorkingDir (Join-Path $repoRoot "codex-rs") `
    -FilePath "cargo" `
    -Arguments @("build", "-p", "codex-cli", "--features", "custom-features", "-j", "6") `
    -TimeoutSec $CargoTimeoutSec `
    -CombinedLogPath $rustLog

$results += Invoke-BuildStep `
    -Name "GUI build" `
    -WorkingDir (Join-Path $repoRoot "codex-gui-x") `
    -FilePath "npm.cmd" `
    -Arguments @("run", "build") `
    -TimeoutSec $GuiTimeoutSec `
    -CombinedLogPath $guiLog

if ($UseIsolatedCargoTarget) {
    if ($null -eq $oldCargoTargetDir) {
        Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    } else {
        $env:CARGO_TARGET_DIR = $oldCargoTargetDir
    }
}

$overall = if (($results | Where-Object { $_.ExitCode -ne 0 }).Count -eq 0) { "PASS" } else { "FAIL" }
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss K"
$cargoVersion = try { cargo --version } catch { "unavailable" }
$npmVersion = try { npm --version } catch { "unavailable" }

$summaryLines = @(
    "# Build Verification Summary"
    ""
    "- Timestamp: $timestamp"
    "- Run tag: $RunTag"
    "- Overall result: **$overall**"
    "- Cargo version: $cargoVersion"
    "- npm version: $npmVersion"
    "- Isolated cargo target: $($UseIsolatedCargoTarget.IsPresent)"
    ""
    "## Steps"
)

foreach ($r in $results) {
    $status = if ($r.ExitCode -eq 0) { "PASS" } else { "FAIL" }
    $summaryLines += "- $($r.Name): $status (exit=$($r.ExitCode), duration=$($r.DurationSec)s, timeout=$($r.TimedOut))"
    $summaryLines += "  - Command: $($r.Command)"
    $summaryLines += "  - Working directory: $($r.WorkingDir)"
    $summaryLines += "  - Log file: $($r.LogPath)"
}

$summaryLines | Set-Content -Path $summaryPath -Encoding UTF8
$results | ConvertTo-Json -Depth 4 | Set-Content -Path $summaryJsonPath -Encoding UTF8

Write-Host "Verification run complete."
Write-Host "Overall: $overall"
Write-Host "Output directory: $outputDir"
Write-Host "Summary: $summaryPath"
