# Codex Fast Build & Install Script (CLI & TUI)
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

function Create-Shortcut {
    param([string]$SourceExe, [string]$ShortcutPath)
    $WshShell = New-Object -comObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
    $Shortcut.TargetPath = $SourceExe
    $Shortcut.Save()
}

# Ensure we are in the right directory
$scriptPath = $PSScriptRoot
if (-not $scriptPath) {
    if ($MyInvocation.MyCommand.Path) {
        $scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
    }
    else {
        $scriptPath = Get-Location
    }
}

if (-not (Test-Path (Join-Path $scriptPath "Cargo.toml"))) {
    # Fallback search
    $possiblePaths = @(
        "c:\Users\downl\Desktop\codex-main\codex-rs",
        "$env:USERPROFILE\Desktop\codex-main\codex-rs"
    )
    foreach ($path in $possiblePaths) {
        if (Test-Path (Join-Path $path "Cargo.toml")) {
            $scriptPath = $path
            break
        }
    }
}

if (-not (Test-Path (Join-Path $scriptPath "Cargo.toml"))) {
    Write-ErrorMsg "Could not find Cargo.toml. Please run from project root."
    exit 1
}

Set-Location $scriptPath
Write-Status "Working directory: $(Get-Location)"

# Environment Checks
$env:RUSTC_WRAPPER = "" # Disable sccache if needed
$env:RUSTFLAGS = "-D warnings" # Enforce zero warnings

# Step 1: Kill Processes
Write-Status "Step 1: Stopping existing processes..."
$ProcessNames = @("codex", "codex-tui")
foreach ($name in $ProcessNames) {
    $procs = Get-Process $name -ErrorAction SilentlyContinue
    if ($procs) {
        $procs | Stop-Process -Force -ErrorAction SilentlyContinue
        Write-Success "Stopped $name."
    }
}
Start-Sleep -Seconds 5

# Step 2: Build CLI
Write-Status "Step 2: Building CLI (codex-cli)..."
try {
    cargo build --release --features custom-features -p codex-cli -j 4
    if ($LASTEXITCODE -ne 0) { throw "CLI Build failed" }
    Write-Success "CLI Build Complete"
}
catch {
    Write-ErrorMsg "CLI Build Failed: $_"
    exit 1
}

# Step 3: Build TUI
Write-Status "Step 3: Building TUI (codex-tui)..."
try {
    # Assuming 'codex-tui' is the package name. 
    # Try with default features first, or add specific features if known.
    # User mentioned 'backtrack' feature in previous convos, but let's stick to base for now unless error.
    # NOTE: Previous logs showed 'codex-tui' directory exists.
    cargo build --release -p codex-tui -j 4
    if ($LASTEXITCODE -ne 0) { throw "TUI Build failed" }
    Write-Success "TUI Build Complete"
}
catch {
    Write-ErrorMsg "TUI Build Failed: $_"
    exit 1
}

# Step 3.5: Build GUI (codex-tauri)
Write-Status "Step 3.5: Building GUI (codex-tauri)..."
try {
    Push-Location "tauri-gui"
    # Use 'cargo tauri build' or 'npm run tauri:build'. 
    # npm run tauri:build is reliable as it handles frontend-build + rust-build
    npm run tauri:build
    if ($LASTEXITCODE -ne 0) { throw "GUI Build failed" }
    Pop-Location
    Write-Success "GUI Build Complete"
}
catch {
    Write-ErrorMsg "GUI Build Failed: $_"
    Pop-Location
    exit 1
}

# Step 4: Install
Write-Status "Step 4: Installing Binaries..."
$InstallDir = "$env:USERPROFILE\.cargo\bin"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$Binaries = @(
    @{ Src = "target\release\codex.exe"; Dest = "codex.exe" },
    @{ Src = "target\release\codex-tui.exe"; Dest = "codex-tui.exe" },
    @{ Src = "target\release\codex-tauri-gui.exe"; Dest = "codex-gui.exe" }
)

foreach ($bin in $Binaries) {
    $src = Join-Path $scriptPath $bin.Src
    $dest = Join-Path $InstallDir $bin.Dest
    
    if (Test-Path $src) {
        try {
            Copy-Item $src $dest -Force
            Write-Success "Installed $($bin.Dest)"
        }
        catch {
            Write-ErrorMsg "Failed to install $($bin.Dest). File locked?"
            exit 1
        }
    }
    else {
        Write-ErrorMsg "Source binary not found: $src"
        exit 1
    }
}

# Verification
Write-Status "Verifying Installation..."
try {
    $CliVer = & "$InstallDir\codex.exe" --version 2>&1
    Write-Success "CLI Version: $CliVer"
}
catch {
    Write-ErrorMsg "CLI verification failed."
}

# Shortcut Creation (Optional convenience)
$DesktopDir = [Environment]::GetFolderPath("Desktop")
Create-Shortcut -SourceExe "$InstallDir\codex-tui.exe" -ShortcutPath "$DesktopDir\Codex TUI.lnk"
Write-Success "Created Desktop Shortcut for TUI"

Write-Success "All tasks complete!"
