[CmdletBinding()]
param(
    [ValidateSet('fast-build', 'fast-build-install')]
    [string]$Task = 'fast-build-install',
    [ValidateSet('md5', 'mtime', 'cargo-metadata')]
    [string]$Method = $(if ($env:CODEX_FAST_BUILD_METHOD) { $env:CODEX_FAST_BUILD_METHOD } else { 'md5' }),
    [int]$Jobs = $(if ($env:CODEX_FAST_BUILD_JOBS) { [int]$env:CODEX_FAST_BUILD_JOBS } else { 6 }),
    [string[]]$Targets = @('codex-cli', 'codex-tui', 'codex-gui'),
    [string]$LogFile,
    [switch]$Force,
    [switch]$NoDenyWarnings
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$scriptPath = Join-Path $repoRoot 'scripts\fast_build.py'
$python = if (Get-Command py -ErrorAction SilentlyContinue) { 'py' } elseif (Get-Command python -ErrorAction SilentlyContinue) { 'python' } else { throw 'Python launcher not found (expected py or python).' }

$argsList = @()
if ($python -eq 'py') {
    $argsList += '-3'
}
$argsList += $scriptPath
$argsList += $Task
$argsList += '--changed-only'
$argsList += '--jobs'
$argsList += $Jobs
$argsList += '--method'
$argsList += $Method
if ($LogFile) {
    $argsList += '--log-file'
    $argsList += $LogFile
}
if ($Force) {
    $argsList += '--force'
}
if ($NoDenyWarnings) {
    $argsList += '--no-deny-warnings'
}
if ($Targets.Count -gt 0) {
    $argsList += $Targets
}

Write-Host "[fast_build.ps1] $python $($argsList -join ' ')" -ForegroundColor Cyan
& $python @argsList
exit $LASTEXITCODE
