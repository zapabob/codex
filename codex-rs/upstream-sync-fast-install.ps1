#requires -Version 5.1

[CmdletBinding()]
param(
    [int]$Jobs = 6,
    [ValidateSet('md5', 'mtime', 'cargo-metadata')]
    [string]$Method = 'md5',
    [string]$Profile = 'release',
    [switch]$Force,
    [switch]$NoDenyWarnings,
    [string]$LogFile = ''
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

function Write-Status {
    param(
        [string]$Message,
        [string]$Color = 'Cyan'
    )

    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] $Message" -ForegroundColor $Color
}

function Resolve-PythonCommand {
    if (Get-Command py -ErrorAction SilentlyContinue) {
        return @('py', '-3')
    }
    if (Get-Command python -ErrorAction SilentlyContinue) {
        return @('python')
    }

    throw 'Python launcher not found. Install py or python first.'
}

function Stop-CodexProcesses {
    $processNames = @('codex', 'codex-cli', 'codex-tui', 'codex-tui-app-server')
    foreach ($name in $processNames) {
        Get-Process -Name $name -ErrorAction SilentlyContinue | ForEach-Object {
            Write-Status "Stopping $($_.ProcessName) (PID $($_.Id))" 'Yellow'
            Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
        }
    }

    & taskkill /F /IM codex.exe /T *> $null
}

$workspaceRoot = $PSScriptRoot
$repoRoot = Split-Path -Parent $workspaceRoot
$fastBuildScript = Join-Path $repoRoot 'scripts\fast_build.py'
$python = Resolve-PythonCommand
$cargoBinDir = Join-Path $env:USERPROFILE '.cargo\bin'
$targetBinary = Join-Path $workspaceRoot "target\$Profile\codex.exe"
$installedBinary = Join-Path $cargoBinDir 'codex.exe'
$backupBinary = Join-Path $cargoBinDir 'codex.exe.bak'

if (-not (Test-Path $fastBuildScript)) {
    throw "Missing fast build script: $fastBuildScript"
}

New-Item -ItemType Directory -Force -Path $cargoBinDir | Out-Null

$buildArgs = @($fastBuildScript, 'fast-build', '--changed-only', '--jobs', $Jobs, '--method', $Method, '--profile', $Profile)
if ($LogFile) {
    $buildArgs += @('--log-file', $LogFile)
}
if ($Force) {
    $buildArgs += '--force'
}
if ($NoDenyWarnings) {
    $buildArgs += '--no-deny-warnings'
}
$buildArgs += 'codex-cli'

Write-Status "Building codex-cli with $Jobs jobs via $($python -join ' ')" 'Cyan'
$command = @($python + $buildArgs)
Push-Location $repoRoot
try {
    & $command[0] $command[1..($command.Length - 1)]
    if ($LASTEXITCODE -ne 0) {
        throw "fast-build failed with exit code $LASTEXITCODE"
    }
}
finally {
    Pop-Location
}

if (-not (Test-Path $targetBinary)) {
    throw "Built binary not found: $targetBinary"
}

Stop-CodexProcesses

if (Test-Path $installedBinary) {
    Copy-Item $installedBinary $backupBinary -Force
    Write-Status "Backed up existing binary to $backupBinary" 'DarkGray'
}

Copy-Item $targetBinary $installedBinary -Force
Write-Status "Installed $targetBinary -> $installedBinary" 'Green'

$versionOutput = & $installedBinary --version 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "Installed codex.exe failed verification.`n$versionOutput"
}

Write-Status "Verification passed: $versionOutput" 'Green'
