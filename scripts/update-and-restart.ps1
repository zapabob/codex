# Codex Update and Restart Script
# Performs differential build, kills processes, and overwrites installation

param(
    [switch]$Force,
    [switch]$SkipTests,
    [switch]$Verbose
)

# Configuration
$CodexProcesses = @("codex", "codex-tui", "codex-gui", "codex-cli")
$BuildTarget = "release"
$BackupSuffix = ".backup"

function Write-Step {
    param([string]$Message)
    Write-Host "🔧 $Message" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    if ($Verbose) {
        Write-Host "ℹ️  $Message" -ForegroundColor Blue
    }
}

# Function to kill Codex processes
function Stop-CodexProcesses {
    Write-Step "Stopping Codex processes..."

    $killedProcesses = @()

    foreach ($processName in $CodexProcesses) {
        $processes = Get-Process -Name $processName -ErrorAction SilentlyContinue
        if ($processes) {
            foreach ($process in $processes) {
                try {
                    Stop-Process -Id $process.Id -Force
                    $killedProcesses += "$processName (PID: $($process.Id))"
                    Write-Info "Killed process: $processName (PID: $($process.Id))"
                } catch {
                    Write-Error "Failed to kill process $processName (PID: $($process.Id)): $_"
                }
            }
        }
    }

    if ($killedProcesses.Count -gt 0) {
        Write-Success "Stopped processes: $($killedProcesses -join ', ')"
    } else {
        Write-Info "No Codex processes were running"
    }

    # Wait a moment for processes to fully terminate
    Start-Sleep -Seconds 2
}

# Function to backup current installation
function Backup-CurrentInstallation {
    Write-Step "Creating backup of current installation..."

    $installPaths = @(
        "$env:USERPROFILE\.cargo\bin\codex.exe",
        "$env:USERPROFILE\.cargo\bin\codex-cli.exe",
        "$env:USERPROFILE\.cargo\bin\codex-tui.exe"
    )

    foreach ($path in $installPaths) {
        if (Test-Path $path) {
            $backupPath = $path + $BackupSuffix
            try {
                Copy-Item -Path $path -Destination $backupPath -Force
                Write-Info "Backed up: $path -> $backupPath"
            } catch {
                Write-Error "Failed to backup $path : $_"
            }
        }
    }

    Write-Success "Backup completed"
}

# Function to perform differential build
function Invoke-DifferentialBuild {
    Write-Step "Performing differential build..."

    $startTime = Get-Date

    # Check if we need a clean build
    $cargoLockPath = "codex-rs\Cargo.lock"
    $lastBuildTime = Get-ItemProperty -Path $cargoLockPath -Name LastWriteTime -ErrorAction SilentlyContinue

    if ($Force -or -not $lastBuildTime) {
        Write-Info "Performing clean build (forced or first time)"
        & cargo clean
    } else {
        Write-Info "Performing incremental build"
    }

    # Build CLI
    Write-Info "Building codex-cli..."
    $buildResult = & cargo build --release -p codex-cli
    if ($LASTEXITCODE -ne 0) {
        Write-Error "CLI build failed"
        return $false
    }

    # Build TUI
    Write-Info "Building codex-tui..."
    $buildResult = & cargo build --release -p codex-tui
    if ($LASTEXITCODE -ne 0) {
        Write-Error "TUI build failed"
        return $false
    }

    $buildTime = (Get-Date) - $startTime
    Write-Success "Build completed in $($buildTime.TotalSeconds.ToString("F2")) seconds"
    return $true
}

# Function to run tests
function Invoke-Tests {
    if ($SkipTests) {
        Write-Info "Skipping tests as requested"
        return $true
    }

    Write-Step "Running tests..."

    $testResult = & cargo test --release --workspace --exclude codex-tui 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Tests failed:`n$testResult"
        return $false
    }

    Write-Success "All tests passed"
    return $true
}

# Function to install binaries
function Install-Binaries {
    Write-Step "Installing binaries..."

    # Install CLI
    $installResult = & cargo install --path codex-rs/cli --force
    if ($LASTEXITCODE -ne 0) {
        Write-Error "CLI installation failed"
        return $false
    }

    # Install TUI (if available)
    if (Test-Path "codex-rs\tui") {
        try {
            Copy-Item -Path "codex-rs\target\release\codex-tui.exe" -Destination "$env:USERPROFILE\.cargo\bin\" -Force
            Write-Info "Installed TUI binary"
        } catch {
            Write-Error "Failed to install TUI binary: $_"
        }
    }

    Write-Success "Installation completed"
    return $true
}

# Function to verify installation
function Test-Installation {
    Write-Step "Verifying installation..."

    $versionCheck = & codex --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Codex version: $versionCheck"
        return $true
    } else {
        Write-Error "Version check failed: $versionCheck"
        return $false
    }
}

# Function to rollback on failure
function Invoke-Rollback {
    Write-Step "Rolling back to previous version..."

    $backupPaths = @(
        "$env:USERPROFILE\.cargo\bin\codex.exe$BackupSuffix",
        "$env:USERPROFILE\.cargo\bin\codex-cli.exe$BackupSuffix",
        "$env:USERPROFILE\.cargo\bin\codex-tui.exe$BackupSuffix"
    )

    foreach ($backupPath in $backupPaths) {
        $originalPath = $backupPath -replace $BackupSuffix, ""
        if (Test-Path $backupPath) {
            try {
                Move-Item -Path $backupPath -Destination $originalPath -Force
                Write-Info "Restored: $originalPath"
            } catch {
                Write-Error "Failed to restore $originalPath : $_"
            }
        }
    }

    Write-Success "Rollback completed"
}

# Function to cleanup backups
function Remove-Backups {
    Write-Step "Cleaning up backups..."

    $backupFiles = Get-ChildItem -Path "$env:USERPROFILE\.cargo\bin\" -Filter "*$BackupSuffix" -File
    foreach ($file in $backupFiles) {
        try {
            Remove-Item -Path $file.FullName -Force
            Write-Info "Removed backup: $($file.Name)"
        } catch {
            Write-Error "Failed to remove backup $($file.Name): $_"
        }
    }

    Write-Success "Cleanup completed"
}

# Main execution
function Main {
    Write-Host "🚀 Codex Update and Restart Script v2.0" -ForegroundColor Magenta
    Write-Host "=====================================" -ForegroundColor Magenta

    $ErrorActionPreference = "Stop"

    try {
        # Change to project root
        $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
        $projectRoot = Split-Path -Parent $scriptDir
        Set-Location $projectRoot

        Write-Info "Working directory: $(Get-Location)"

        # Step 1: Stop processes
        Stop-CodexProcesses

        # Step 2: Backup current installation
        Backup-CurrentInstallation

        # Step 3: Differential build
        $buildSuccess = Invoke-DifferentialBuild
        if (-not $buildSuccess) {
            Write-Error "Build failed, aborting update"
            Invoke-Rollback
            exit 1
        }

        # Step 4: Run tests
        $testsSuccess = Invoke-Tests
        if (-not $testsSuccess) {
            Write-Error "Tests failed, aborting update"
            Invoke-Rollback
            exit 1
        }

        # Step 5: Install binaries
        $installSuccess = Install-Binaries
        if (-not $installSuccess) {
            Write-Error "Installation failed, aborting update"
            Invoke-Rollback
            exit 1
        }

        # Step 6: Verify installation
        $verifySuccess = Test-Installation
        if (-not $verifySuccess) {
            Write-Error "Verification failed, rolling back"
            Invoke-Rollback
            exit 1
        }

        # Step 7: Cleanup
        Remove-Backups

        Write-Success "🎉 Update completed successfully!"
        Write-Host ""
        Write-Host "Next steps:" -ForegroundColor Yellow
        Write-Host "1. Start Codex: codex" -ForegroundColor White
        Write-Host "2. Start TUI: codex-tui" -ForegroundColor White
        Write-Host "3. Check version: codex --version" -ForegroundColor White

    } catch {
        Write-Error "Update failed with error: $_"
        Write-Error "Attempting rollback..."
        Invoke-Rollback
        exit 1
    }
}

# Run main function
Main
