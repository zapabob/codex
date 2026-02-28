# Legacy wrapper for version updates.
# Prefer using: .\scripts\bump-version.ps1

param(
    [ValidateSet("patch", "minor", "major")]
    [string]$Type = "patch",
    [switch]$Apply,
    [switch]$IncludeCodexCli,
    [switch]$IncludeTauri
)

$ErrorActionPreference = "Stop"

Write-Host "Delegating to scripts/bump-version.ps1..." -ForegroundColor Cyan

$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$canonical = Join-Path $root "scripts\bump-version.ps1"

if (-not (Test-Path $canonical)) {
    throw "Canonical version script not found: $canonical"
}

$args = @($Type)
if ($Apply) { $args += "-Apply" }
if ($IncludeCodexCli) { $args += "-IncludeCodexCli" }
if ($IncludeTauri) { $args += "-IncludeTauri" }

& $canonical @args
