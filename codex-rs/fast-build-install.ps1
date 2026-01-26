# Codex Fast Build & Install Script (Version 2.11.1)
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
# Try multiple methods to find the script directory
$scriptPath = $null

# Method 1: Use PSScriptRoot (most reliable for direct script execution)
if ($PSScriptRoot) {
    $scriptPath = $PSScriptRoot
}
# Method 2: Use MyInvocation (works when script is invoked directly)
elseif ($MyInvocation.MyCommand.Path) {
    $scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
}
# Method 3: Use MyInvocation.ScriptName (works with Code Runner)
elseif ($MyInvocation.ScriptName) {
    $scriptPath = Split-Path -Parent $MyInvocation.ScriptName
}
# Method 4: Use $PSCommandPath (PowerShell 3.0+)
elseif ($PSCommandPath) {
    $scriptPath = Split-Path -Parent $PSCommandPath
}
# Method 5: Fallback - try to find codex-rs directory from current location
else {
    $currentDir = Get-Location
    if (Test-Path (Join-Path $currentDir "Cargo.toml")) {
        $scriptPath = $currentDir
    } elseif (Test-Path (Join-Path $currentDir "codex-rs\Cargo.toml")) {
        $scriptPath = Join-Path $currentDir "codex-rs"
    } elseif (Test-Path "Cargo.toml") {
        $scriptPath = Get-Location
    }
}

# If still not found, try absolute path based on common structure
if (-not $scriptPath -or -not (Test-Path (Join-Path $scriptPath "Cargo.toml"))) {
    $possiblePaths = @(
        "c:\Users\downl\Desktop\codex-main\codex-rs",
        "$env:USERPROFILE\Desktop\codex-main\codex-rs",
        (Join-Path (Get-Location) "codex-rs"),
        (Join-Path (Split-Path (Get-Location) -Parent) "codex-rs")
    )
    
    foreach ($path in $possiblePaths) {
        if (Test-Path (Join-Path $path "Cargo.toml")) {
            $scriptPath = $path
            break
        }
    }
}

if (-not $scriptPath) {
    Write-ErrorMsg "Could not determine script directory"
    Write-Host "Current directory: $(Get-Location)" -ForegroundColor Yellow
    Write-Host "PSScriptRoot: $PSScriptRoot" -ForegroundColor Yellow
    Write-Host "MyInvocation.MyCommand.Path: $($MyInvocation.MyCommand.Path)" -ForegroundColor Yellow
    exit 1
}

$cargoTomlPath = Join-Path $scriptPath "Cargo.toml"
if (-not (Test-Path $cargoTomlPath)) {
    Write-ErrorMsg "Cargo.toml not found at $cargoTomlPath"
    Write-Host "Current directory: $(Get-Location)" -ForegroundColor Yellow
    Write-Host "Script path: $scriptPath" -ForegroundColor Yellow
    Write-Host "Looking for: $cargoTomlPath" -ForegroundColor Yellow
    exit 1
}

# Change to the script directory
Set-Location $scriptPath
Write-Status "Working directory: $(Get-Location)"

# Disable sccache if causing issues
$env:RUSTC_WRAPPER = ""
$env:RUSTFLAGS = "-D warnings"

# Step 1: Kill Existing Processes
Write-Status "Step 1/4: Stopping running codex processes..."
$CodexProcesses = Get-Process codex -ErrorAction SilentlyContinue
if ($CodexProcesses) {
    $CodexProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 1
    Write-Success "Processes stopped."
} else {
    Write-Success "No running processes found."
}

# Step 2: Differential Build with custom-features
Write-Status "Step 2/4: Building codex-cli (Release, Incremental, custom-features)..."
Write-Host "Version: 2.11.1" -ForegroundColor Yellow
Write-Host "Starting incremental build..." -ForegroundColor Cyan

try {
    # Check for existing build processes
    Write-Status "Checking for existing build processes..."
    $cargoProcesses = Get-Process cargo -ErrorAction SilentlyContinue
    $rustcProcesses = Get-Process rustc -ErrorAction SilentlyContinue
    
    if ($cargoProcesses -or $rustcProcesses) {
        Write-Host "Found existing build processes, waiting for completion..." -ForegroundColor Yellow
        $waitTime = 0
        $maxWait = 300  # 5 minutes max wait
        while (($cargoProcesses -or $rustcProcesses) -and $waitTime -lt $maxWait) {
            Start-Sleep -Seconds 5
            $waitTime += 5
            $cargoProcesses = Get-Process cargo -ErrorAction SilentlyContinue
            $rustcProcesses = Get-Process rustc -ErrorAction SilentlyContinue
            if ($waitTime % 30 -eq 0) {
                Write-Host "Still waiting... ($waitTime/$maxWait seconds)" -ForegroundColor Yellow
            }
        }
        if ($waitTime -ge $maxWait) {
            Write-Host "Timeout waiting for build processes. Proceeding anyway..." -ForegroundColor Yellow
        } else {
            Write-Success "Build processes completed."
        }
    }
    
    # Wait for build lock to be released (max 60 seconds)
    Write-Status "Checking for build lock..."
    $lockWaitTime = 0
    $maxLockWait = 60
    while ($lockWaitTime -lt $maxLockWait) {
        try {
            # Try to create a test file in target directory to check lock
            $testLock = Join-Path $scriptPath "target\.build-lock-test"
            $null = New-Item -ItemType File -Path $testLock -Force -ErrorAction Stop
            Remove-Item $testLock -Force -ErrorAction SilentlyContinue
            break
        } catch {
            $lockWaitTime += 2
            if ($lockWaitTime -lt $maxLockWait) {
                Write-Host "Build directory locked, waiting... ($lockWaitTime/$maxLockWait seconds)" -ForegroundColor Yellow
                Start-Sleep -Seconds 2
            }
        }
    }
    
    $buildStart = Get-Date
    Write-Status "Starting build..."
    cargo build --release --features custom-features -p codex-cli
    $buildEnd = Get-Date
    $buildTime = ($buildEnd - $buildStart).TotalSeconds
    
    if ($LASTEXITCODE -ne 0) { 
        Write-ErrorMsg "Build failed."
        exit 1
    }
    
    Write-Success "Build completed in $([math]::Round($buildTime, 2)) seconds."
} catch {
    Write-ErrorMsg "Build failed: $_"
    exit 1
}

# Step 3: Verify Binary
Write-Status "Step 3/4: Verifying binary..."
$SourcePath = ".\target\release\codex.exe"

if (-not (Test-Path $SourcePath)) {
    Write-ErrorMsg "Build artifact not found at $SourcePath"
    exit 1
}

$fileInfo = Get-Item $SourcePath
Write-Success "Binary found: $([math]::Round($fileInfo.Length / 1MB, 2)) MB"

# Step 4: Overwrite Install
Write-Status "Step 4/4: Installing binary..."
$InstallPath = "$env:USERPROFILE\.cargo\bin\codex.exe"
$InstallDir = Split-Path $InstallPath

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Success "Created install directory: $InstallDir"
}

try {
    # Kill any processes using the binary
    Get-Process | Where-Object { $_.Path -eq $InstallPath } | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 1
    
    Copy-Item $SourcePath $InstallPath -Force
    Write-Success "Installed to $InstallPath"
} catch {
    Write-ErrorMsg "Failed to copy binary: $_"
    Write-Host "Binary may be in use. Please check processes." -ForegroundColor Yellow
    exit 1
}

# Verification
Write-Status "Verifying installation..."
try {
    $Version = & codex --version 2>&1
    Write-Success "Installation verified: $Version"
} catch {
    Write-ErrorMsg "Version check failed. Binary may not be in PATH."
}

Write-Host ""
Write-Success "Installation complete! Version 2.11.1"
