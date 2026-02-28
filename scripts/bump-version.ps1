# Semantic version bump helper.
# Usage examples:
#   .\scripts\bump-version.ps1 patch
#   .\scripts\bump-version.ps1 minor -Apply
#   .\scripts\bump-version.ps1 major -Apply -IncludeCodexCli -IncludeTauri

param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("patch", "minor", "major")]
    [string]$Type,

    [switch]$Apply,
    [switch]$IncludeCodexCli,
    [switch]$IncludeTauri
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Read-FileRaw([string]$Path) {
    return [System.IO.File]::ReadAllText((Join-Path (Get-Location) $Path))
}

function Write-FileRaw([string]$Path, [string]$Content) {
    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText((Join-Path (Get-Location) $Path), $Content, $utf8NoBom)
}

function Get-WorkspaceVersion {
    $cargoPath = "codex-rs/Cargo.toml"
    if (-not (Test-Path $cargoPath)) {
        throw "Missing required file: $cargoPath"
    }

    $cargoRaw = Read-FileRaw $cargoPath
    $match = [Regex]::Match(
        $cargoRaw,
        '(?ms)\[workspace\.package\][^\[]*?^\s*version\s*=\s*"([^"]+)"'
    )
    if (-not $match.Success) {
        throw "Could not find [workspace.package].version in $cargoPath"
    }
    return $match.Groups[1].Value
}

function Get-CurrentVersion {
    if (Test-Path "VERSION") {
        $fromVersionFile = (Read-FileRaw "VERSION").Trim()
        if ($fromVersionFile) {
            return $fromVersionFile
        }
    }
    return Get-WorkspaceVersion
}

function Bump-Version([string]$CurrentVersion, [string]$BumpType) {
    $pattern = '^(\d+)\.(\d+)\.(\d+)(-[0-9A-Za-z\.-]+)?(\+[0-9A-Za-z\.-]+)?$'
    if ($CurrentVersion -notmatch $pattern) {
        throw "Invalid semver format: $CurrentVersion"
    }

    $major = [int]$Matches[1]
    $minor = [int]$Matches[2]
    $patch = [int]$Matches[3]
    $preRelease = $Matches[4]
    $buildMeta = $Matches[5]

    switch ($BumpType) {
        "patch" { $patch += 1 }
        "minor" {
            $minor += 1
            $patch = 0
        }
        "major" {
            $major += 1
            $minor = 0
            $patch = 0
        }
    }

    return "$major.$minor.$patch$preRelease$buildMeta"
}

function Replace-FirstMatch([string]$Path, [string]$Pattern, [string]$Replacement, [string]$ErrorLabel) {
    if (-not (Test-Path $Path)) {
        return $false
    }
    $raw = Read-FileRaw $Path
    $updated = [Regex]::Replace($raw, $Pattern, $Replacement, 1)
    if ($updated -eq $raw) {
        throw "Failed to update $ErrorLabel in $Path"
    }
    Write-FileRaw $Path $updated
    return $true
}

$currentVersion = Get-CurrentVersion
$newVersion = Bump-Version $currentVersion $Type

Write-Host "Current version: $currentVersion" -ForegroundColor Cyan
Write-Host "New version:     $newVersion" -ForegroundColor Green

$targets = @(
    @{ Path = "codex-rs/Cargo.toml"; Label = "workspace package version"; Pattern = '(?ms)(\[workspace\.package\][^\[]*?^\s*version\s*=\s*")[^"]+(")'; Replacement = "`$1$newVersion`$2" },
    @{ Path = "package.json"; Label = "root package.json version"; Pattern = '("version"\s*:\s*")[^"]+(")'; Replacement = "`$1$newVersion`$2" },
    @{ Path = "codex-gui-x/package.json"; Label = "codex-gui-x package.json version"; Pattern = '("version"\s*:\s*")[^"]+(")'; Replacement = "`$1$newVersion`$2" }
)

if ($IncludeCodexCli) {
    $targets += @{ Path = "codex-cli/package.json"; Label = "codex-cli package.json version"; Pattern = '("version"\s*:\s*")[^"]+(")'; Replacement = "`$1$newVersion`$2" }
}

if ($IncludeTauri) {
    $targets += @{ Path = "codex-rs/tauri-gui/package.json"; Label = "tauri GUI package.json version"; Pattern = '("version"\s*:\s*")[^"]+(")'; Replacement = "`$1$newVersion`$2" }
    $targets += @{ Path = "codex-rs/tauri-gui/src-tauri/Cargo.toml"; Label = "tauri src-tauri Cargo.toml version"; Pattern = '(?ms)(\[package\][^\[]*?^\s*version\s*=\s*")[^"]+(")'; Replacement = "`$1$newVersion`$2" }
}

Write-Host ""
Write-Host "Target files:" -ForegroundColor Yellow
foreach ($target in $targets) {
    Write-Host "  - $($target.Path)" -ForegroundColor Gray
}
Write-Host "  - VERSION" -ForegroundColor Gray

if (-not $Apply) {
    $confirm = Read-Host "Apply these changes? (y/n)"
    if ($confirm -ne "y") {
        Write-Host "Aborted." -ForegroundColor Yellow
        exit 0
    }
}

$updatedFiles = @()
foreach ($target in $targets) {
    $didUpdate = Replace-FirstMatch $target.Path $target.Pattern $target.Replacement $target.Label
    if ($didUpdate) {
        $updatedFiles += $target.Path
    } else {
        Write-Host "Skipped (not found): $($target.Path)" -ForegroundColor Yellow
    }
}

Write-FileRaw "VERSION" $newVersion
$updatedFiles += "VERSION"

Write-Host ""
Write-Host "Version bump complete." -ForegroundColor Green
Write-Host "Updated files:" -ForegroundColor Cyan
foreach ($path in $updatedFiles) {
    Write-Host "  - $path" -ForegroundColor Gray
}
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. git diff" -ForegroundColor Gray
Write-Host "  2. Run tests/build as needed" -ForegroundColor Gray
Write-Host "  3. git add <files> && git commit -m 'chore: bump version to $newVersion'" -ForegroundColor Gray
