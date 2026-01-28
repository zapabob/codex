# Competition Build & Install Script
# コンペで採用された実装をクリーンビルドして上書きインストール
#
# 既存の `scripts/clean-build-install.ps1` をラップして使う。

param(
    [string]$InstallPath = "",
    [switch]$SkipClean = $false
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

Write-Host "[*] Competition build/install starting..." -ForegroundColor Cyan

$script = Join-Path $repoRoot "scripts\clean-build-install.ps1"
if (-not (Test-Path $script)) {
    Write-Host "[ERROR] Missing script: $script" -ForegroundColor Red
    exit 1
}

if ($InstallPath) {
    if ($SkipClean) {
        pwsh -File $script -InstallPath $InstallPath -SkipClean
    } else {
        pwsh -File $script -InstallPath $InstallPath
    }
} else {
    if ($SkipClean) {
        pwsh -File $script -SkipClean
    } else {
        pwsh -File $script
    }
}

