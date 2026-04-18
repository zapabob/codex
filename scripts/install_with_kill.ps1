#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Stop eligible standalone Codex processes and overwrite-install a new binary.
.DESCRIPTION
    Stops only the processes whose executable paths are not excluded, then copies the
    source binary to the target path with retries. Windows Store CodexApp paths can be
    excluded so the app keeps running while standalone CLI/TUI binaries are replaced.
#>

param(
    [Parameter(Mandatory = $true)]
    [string]$SourcePath,

    [Parameter(Mandatory = $true)]
    [string]$TargetPath,

    [string[]]$ProcessNames = @("codex"),

    [string[]]$ExcludePathPrefixes = @(),

    [switch]$Force,

    [int]$MaxRetries = 3
)

$ErrorActionPreference = "Stop"

function Write-Status {
    param([string]$Message, [string]$Color = "Cyan")
    Write-Host "[*] $Message" -ForegroundColor $Color
}

function Write-Success {
    param([string]$Message)
    Write-Host "[OK] $Message" -ForegroundColor Green
}

function Write-WarningMsg {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-ErrorMsg {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Resolve-ProcessPath {
    param([System.Diagnostics.Process]$Process)

    if ($Process.Path) {
        return $Process.Path
    }

    try {
        return $Process.MainModule.FileName
    }
    catch {
        return $null
    }
}

function Test-ExcludedProcessPath {
    param(
        [string]$ProcessPath,
        [string[]]$Prefixes
    )

    if ([string]::IsNullOrWhiteSpace($ProcessPath)) {
        return $false
    }

    foreach ($prefix in $Prefixes) {
        if ([string]::IsNullOrWhiteSpace($prefix)) {
            continue
        }
        if ($ProcessPath.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
            return $true
        }
    }

    return $false
}

function Get-ManagedProcesses {
    param(
        [string[]]$Names,
        [string[]]$Prefixes
    )

    $items = @()
    foreach ($name in $Names) {
        $processes = Get-Process -Name $name -ErrorAction SilentlyContinue
        foreach ($process in $processes) {
            $processPath = Resolve-ProcessPath -Process $process
            $excluded = Test-ExcludedProcessPath -ProcessPath $processPath -Prefixes $Prefixes
            $items += [PSCustomObject]@{
                ProcessName = $process.ProcessName
                Id          = $process.Id
                Path        = $processPath
                Excluded    = $excluded
            }
        }
    }
    return $items
}

function Stop-EligibleProcesses {
    param([object[]]$ProcessItems)

    $eligible = @($ProcessItems | Where-Object { -not $_.Excluded })
    $excluded = @($ProcessItems | Where-Object { $_.Excluded })

    foreach ($item in $excluded) {
        Write-Status "Preserving excluded process $($item.ProcessName) [$($item.Id)] at $($item.Path)"
    }

    foreach ($item in $eligible) {
        try {
            Write-Status "Stopping $($item.ProcessName) [$($item.Id)]"
            Stop-Process -Id $item.Id -Force -ErrorAction Stop
        }
        catch {
            Write-WarningMsg "Failed to stop PID $($item.Id): $($_.Exception.Message)"
        }
    }

    if ($eligible.Count -gt 0) {
        Start-Sleep -Seconds 1
    }
}

Write-Status "Source: $SourcePath"
Write-Status "Target: $TargetPath"

if (-not (Test-Path -LiteralPath $SourcePath)) {
    Write-ErrorMsg "Source file not found: $SourcePath"
    exit 1
}

$targetDir = Split-Path -Parent $TargetPath
if (-not [string]::IsNullOrWhiteSpace($targetDir) -and -not (Test-Path -LiteralPath $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir -Force | Out-Null
    Write-Success "Created install directory: $targetDir"
}

$managedProcesses = Get-ManagedProcesses -Names $ProcessNames -Prefixes $ExcludePathPrefixes
if ($managedProcesses.Count -eq 0) {
    Write-Status "No matching processes found."
}
else {
    foreach ($item in $managedProcesses) {
        $label = if ($item.Excluded) { "excluded" } else { "eligible" }
        Write-Status "Found $label process $($item.ProcessName) [$($item.Id)] path=$($item.Path)"
    }

    if (-not $Force) {
        $confirm = Read-Host "Stop eligible processes and install the new binary? (y/N)"
        if ($confirm -notmatch "^[Yy]$") {
            Write-WarningMsg "Installation cancelled."
            exit 0
        }
    }

    Stop-EligibleProcesses -ProcessItems $managedProcesses
}

$copied = $false
for ($attempt = 1; $attempt -le $MaxRetries; $attempt++) {
    try {
        Copy-Item -LiteralPath $SourcePath -Destination $TargetPath -Force -ErrorAction Stop
        $copied = $true
        Write-Success "Installed binary on attempt $attempt"
        break
    }
    catch {
        Write-WarningMsg "Copy attempt $attempt failed: $($_.Exception.Message)"
        if ($attempt -lt $MaxRetries) {
            $retryProcesses = Get-ManagedProcesses -Names $ProcessNames -Prefixes $ExcludePathPrefixes
            Stop-EligibleProcesses -ProcessItems $retryProcesses
            Start-Sleep -Seconds 1
        }
    }
}

if (-not $copied) {
    Write-ErrorMsg "Failed to install $TargetPath after $MaxRetries attempts"
    exit 1
}

if (-not (Test-Path -LiteralPath $TargetPath)) {
    Write-ErrorMsg "Target binary missing after install: $TargetPath"
    exit 1
}

$installedInfo = Get-Item -LiteralPath $TargetPath
Write-Success "Install complete: $($installedInfo.FullName)"
Write-Status "Size: $([math]::Round($installedInfo.Length / 1MB, 2)) MB"
Write-Status "Updated: $($installedInfo.LastWriteTime)"

try {
    $version = & $TargetPath --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Status "Version: $version"
    }
}
catch {
    Write-WarningMsg "Version check skipped: $($_.Exception.Message)"
}
