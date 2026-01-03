# Differential Build Script for Codex
# Builds only changed crates for faster compilation

Write-Host ""
Write-Host "=== CODEX DIFFERENTIAL BUILD ===" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location (Join-Path $repoRoot "codex-rs")

$startTime = Get-Date

Write-Host "[1/4] Detecting changes..." -ForegroundColor Yellow

# Always build core and CLI for now (safest approach)
$cratesToBuild = @("codex-core", "codex-cli")

Write-Host "  Will build: $($cratesToBuild -join ', ')" -ForegroundColor Green
Write-Host ""

Write-Host "[2/4] Building Rust crates..." -ForegroundColor Yellow
Write-Host ""

$buildFailed = $false

foreach ($crate in $cratesToBuild) {
    Write-Host "  Building $crate..." -ForegroundColor Cyan

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $buildOutput = & cargo build --release -p $crate 2>&1 | Out-String
    $buildExitCode = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorActionPreference

    if ($buildExitCode -eq 0) {
        Write-Host "    [OK] $crate" -ForegroundColor Green
    } else {
        Write-Host "    [FAILED] $crate" -ForegroundColor Red
        Write-Host $buildOutput -ForegroundColor Red
        $buildFailed = $true
        break
    }
}

if ($buildFailed) {
    Write-Host ""
    Write-Host "[ERROR] Build failed!" -ForegroundColor Red
    exit 1
}

$buildTime = (Get-Date) - $startTime

Write-Host ""
Write-Host "[3/4] Installing binary (overwrite)..." -ForegroundColor Yellow
Write-Host ""

if ($IsWindows) {
    $binaryName = "codex.exe"
    $installDir = Join-Path $env:USERPROFILE ".cargo\bin"
} else {
    $binaryName = "codex"
    $installDir = Join-Path $env:HOME ".cargo/bin"
}

$sourceDir = Join-Path (Join-Path $repoRoot "codex-rs") (Join-Path "target" "release")
$sourcePath = Join-Path $sourceDir $binaryName
$installPath = Join-Path $installDir $binaryName

if (-not (Test-Path $sourcePath)) {
    Write-Host "  [FAILED] Build artifact not found: $sourcePath" -ForegroundColor Red
    exit 1
}

try {
    if ($IsWindows) {
        taskkill /F /IM $binaryName /T 2>$null | Out-Null
    } else {
        pkill -f $binaryName 2>$null
    }
} catch {
    # Ignore if process not running
}

New-Item -ItemType Directory -Force -Path $installDir | Out-Null
Copy-Item -Path $sourcePath -Destination $installPath -Force
Write-Host "  [OK] Installed: $installPath" -ForegroundColor Green

Write-Host ""
Write-Host "[4/4] Done!" -ForegroundColor Green
Write-Host ""
Write-Host "  Crates built: $($cratesToBuild.Count)" -ForegroundColor Cyan
Write-Host "  Build time: $([math]::Round($buildTime.TotalSeconds, 2))s" -ForegroundColor Cyan
Write-Host ""
Write-Host "[VERIFY] Version:" -ForegroundColor Yellow
try {
    codex --version
} catch {
    Write-Host "  [WARN] codex --version failed; check PATH or install location." -ForegroundColor Yellow
}
Write-Host ""
