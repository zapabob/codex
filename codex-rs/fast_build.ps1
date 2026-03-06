# Fast Build & Install Script (Simplified)
# Requirements: 6-core, sccache, kill processes, overwrite install

$ErrorActionPreference = "Stop"

function Write-Status($msg) { Write-Host "[$(Get-Date -Format 'HH:mm:ss')] $msg" -ForegroundColor Cyan }
function Write-Success($msg) { Write-Host "[$(Get-Date -Format 'HH:mm:ss')] $msg" -ForegroundColor Green }

# 1. Setup Environment
if ($env:USE_SCCACHE -eq "1" -and (Get-Command sccache -ErrorAction SilentlyContinue)) {
    $env:RUSTC_WRAPPER = "sccache"
    $cacheMode = "sccache"
} else {
    Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue
    $env:SCCACHE_DISABLE = "1"
    $cacheMode = "rustc"
}
$env:RUSTFLAGS = "-D warnings"
Write-Status "Environment configured: compiler=$cacheMode, Jobs=6"

# 2. Kill Processes
Write-Status "Killing existing processes..."
Stop-Process -Name "codex" -Force -ErrorAction SilentlyContinue
Stop-Process -Name "codex-tui" -Force -ErrorAction SilentlyContinue
Stop-Process -Name "codex-gui" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

# 3. Build (Direct execution for visibility)
Write-Status "Starting Cargo Build (CLI + TUI)..."
# We build both. If CLI is already built, sccache/cargo will skip it quickly.
cargo build --release -p codex-cli -p codex-tui -j 6

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed with code $LASTEXITCODE"
    exit $LASTEXITCODE
}

Write-Success "Build Complete."

# 4. Install (Overwrite)
Write-Status "Installing binaries..."
$installDir = "$env:USERPROFILE\.cargo\bin"
if (-not (Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir -Force }

$binaries = @("codex.exe", "codex-tui.exe")
foreach ($bin in $binaries) {
    $src = "target\release\$bin"
    $dest = "$installDir\$bin"
    
    if (Test-Path $src) {
        Write-Host "Copying $bin to $dest..."
        Copy-Item -Path $src -Destination $dest -Force
        Write-Success "Installed $bin"
    }
    else {
        Write-Warning "$bin not found in target/release!"
    }
}

# 5. Verify
Write-Status "Verifying installation..."
& "$installDir\codex.exe" --version
