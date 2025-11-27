# Codex Fast Build & Install Script
# Enforces zero warnings, performs differential build, and overwrites installation.

$ErrorActionPreference = "Stop"
$ProgressPreference = 'SilentlyContinue'

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

# Ensure we are in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-ErrorMsg "Cargo.toml not found. Please run this script from the codex-rs directory."
    exit 1
}

$env:RUSTFLAGS = "-D warnings"

# Step 1: Type Check & Linting (Zero Warnings)
Write-Status "Step 1/4: Checking types and lints (Zero Warnings)..."
try {
    cargo check --workspace --all-targets --quiet
    if ($LASTEXITCODE -ne 0) { throw "Cargo check failed" }
    Write-Success "Type check passed with 0 warnings."
} catch {
    Write-ErrorMsg "Type check failed or warnings found."
    exit 1
}

# Step 2: Differential Build
Write-Status "Step 2/4: Building codex-cli (Release, Incremental)..."
try {
    cargo build --release --package codex-cli --quiet
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }
    Write-Success "Build completed."
} catch {
    Write-ErrorMsg "Build failed."
    exit 1
}

# Step 3: Kill Existing Processes
Write-Status "Step 3/4: Stopping running codex processes..."
$CodexProcesses = Get-Process codex -ErrorAction SilentlyContinue
if ($CodexProcesses) {
    $CodexProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 1
    Write-Success "Processes stopped."
} else {
    Write-Success "No running processes found."
}

# Step 4: Overwrite Install
Write-Status "Step 4/4: Installing binary..."
$SourcePath = ".\target\release\codex.exe"
$InstallPath = "$env:USERPROFILE\.cargo\bin\codex.exe"

if (-not (Test-Path $SourcePath)) {
    Write-ErrorMsg "Build artifact not found at $SourcePath"
    exit 1
}

try {
    Copy-Item $SourcePath $InstallPath -Force
    Write-Success "Installed to $InstallPath"
} catch {
    Write-ErrorMsg "Failed to copy binary. Ensure it is not in use."
    exit 1
}

# Verification
$Version = & codex --version
Write-Success "Installation verified: $Version"
