# Ultra Fast Build & Install Script for Codex + OpenCode
# Features: Parallel builds, differential detection, robust process kill, atomic installation
# Author: AI-Assisted Implementation
# Version: 1.4.0

#requires -Version 5.1

[CmdletBinding()]
param(
    [switch]$Clean,
    [switch]$SkipOpenCode,
    [int]$MaxParallelJobs = 0,  # 0 = auto (CPU cores / 2)
    [string]$InstallDir = "$env:USERPROFILE\.cargo\bin",
    [string]$BuildCacheFile = ".buildcache.json"
)

$ErrorActionPreference = "Stop"
$ProgressPreference = 'SilentlyContinue'

#region Utility Functions

function Write-Status {
    param([string]$Message, [string]$Color = "Cyan")
    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] [*] $Message" -ForegroundColor $Color
}

function Write-Success {
    param([string]$Message)
    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] [OK] $Message" -ForegroundColor Green
}

function Write-WarningMsg {
    param([string]$Message)
    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] [WARN] $Message" -ForegroundColor Yellow
}

function Write-ErrorMsg {
    param([string]$Message)
    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] [ERROR] $Message" -ForegroundColor Red
}

function Get-FileHash256 {
    param([string]$Path)
    if (-not (Test-Path $Path)) { return $null }
    $hash = Get-FileHash -Path $Path -Algorithm SHA256
    return $hash.Hash
}

function Test-FileLocked {
    param([string]$Path)
    try {
        $file = [System.IO.File]::Open($Path, [System.IO.FileMode]::Open, [System.IO.FileAccess]::ReadWrite, [System.IO.FileShare]::None)
        $file.Close()
        return $false
    }
    catch {
        return $true
    }
}

function Get-LockingProcesses {
    param([string]$Path)
    $processes = @()
    try {
        # Try using handle.exe if available
        $handlePath = Get-Command "handle.exe" -ErrorAction SilentlyContinue
        if ($handlePath) {
            $output = & $handlePath.Source $Path 2>&1
            $pids = $output | Select-String "pid:\s+(\d+)" | ForEach-Object { $_.Matches.Groups[1].Value }
            foreach ($lockingPid in $pids) {
                $proc = Get-Process -Id $lockingPid -ErrorAction SilentlyContinue
                if ($proc) { $processes += $proc }
            }
        }
    }
    catch {
        # Fallback: check if file is locked by trying to rename
        if (Test-FileLocked $Path) {
            # Return generic process info
            $processes += [PSCustomObject]@{
                ProcessName = "Unknown"
                Id          = 0
            }
        }
    }
    return $processes
}

function Stop-ProcessRobust {
    param(
        [string]$ProcessName,
        [int]$MaxAttempts = 2,
        [int]$WaitSeconds = 2
    )
    
    for ($attempt = 1; $attempt -le $MaxAttempts; $attempt++) {
        $procs = Get-Process $ProcessName -ErrorAction SilentlyContinue
        if (-not $procs) {
            return $true
        }
        
        Write-Status "Aggressively stopping $ProcessName (attempt $attempt/$MaxAttempts)..."
        
        # Use taskkill /F /T immediately for high speed
        try {
            $null = & taskkill /F /IM "$ProcessName.exe" /T 2>&1
            Start-Sleep -Seconds $WaitSeconds
        }
        catch {
            Write-WarningMsg "taskkill failed for $ProcessName"
        }
        
        $procs = Get-Process $ProcessName -ErrorAction SilentlyContinue
        if (-not $procs) { return $true }
    }
    
    # Final check
    $remaining = Get-Process $ProcessName -ErrorAction SilentlyContinue
    if ($remaining) {
        Write-ErrorMsg "Failed to kill all $ProcessName processes. It might be stuck."
        return $false
    }
    return $true
}

function Install-BinaryAtomic {
    param(
        [string]$Source,
        [string]$Destination,
        [int]$MaxRetries = 3
    )
    
    if (-not (Test-Path $Source)) {
        throw "Source file not found: $Source"
    }
    
    for ($i = 0; $i -lt $MaxRetries; $i++) {
        try {
            # Skip if destination is already newer or equal to source
            if (Test-Path $Destination) {
                $srcTime = (Get-Item $Source).LastWriteTime
                $destTime = (Get-Item $Destination).LastWriteTime
                if ($destTime -ge $srcTime) {
                    Write-Success "Skipping installation: $(Split-Path $Destination -Leaf) is already up-to-date"
                    return $true
                }
            }
            
            # Direct copy-paste (overwrite) as requested for ultra-fast performance
            Copy-Item $Source $Destination -Force -ErrorAction Stop
            Write-Success "Installed: $(Split-Path $Destination -Leaf)"
            return $true
        }
        catch {
            Write-WarningMsg "Failed to overwrite $Destination. Retrying after process check..."
            $binName = [System.IO.Path]::GetFileNameWithoutExtension($Destination)
            Stop-ProcessRobust -ProcessName $binName
            Start-Sleep -Milliseconds 500
        }
    }
    
    throw "Failed to install $(Split-Path $Destination -Leaf) after $MaxRetries attempts"
}

#endregion

#region Build Cache Management

function Get-SourceTimestamp {
    param(
        [string]$PackagePath,
        [string[]]$Extensions = @("*.rs", "*.toml", "*.lock")
    )
    
    $maxTime = [DateTime]::MinValue
    foreach ($ext in $Extensions) {
        $files = Get-ChildItem -Path $PackagePath -Recurse -Filter $ext -File -ErrorAction SilentlyContinue
        foreach ($file in $files) {
            if ($file.LastWriteTime -gt $maxTime) {
                $maxTime = $file.LastWriteTime
            }
        }
    }
    
    return $maxTime.Ticks # Using Ticks as a simple numeric representation
}

function Test-DifferentialBuild {
    param(
        [string]$PackageName,
        [string]$PackagePath,
        [hashtable]$Cache
    )
    
    $currentTime = Get-SourceTimestamp $PackagePath
    $cachedTime = $Cache[$PackageName]
    
    if (-not $cachedTime) {
        Write-Status "No cache for $PackageName, full build required"
        return $true, $currentTime
    }
    
    if ($currentTime -gt $cachedTime) {
        Write-Status "Changes detected in $PackageName, rebuild required"
        return $true, $currentTime
    }
    
    Write-Success "No changes in $PackageName, skipping build"
    return $false, $currentTime
}

#endregion

# Note: Invoke-ParallelBuild is deprecated for recursive cargo builds in the same workspace 
# to avoid "Waiting for file lock on package cache". 
# Combined cargo builds are handled in the main script now.

# region Build Cache Management (Added back)
function Get-BuildCache {
    param([string]$CacheFile)
    if (Test-Path $CacheFile) {
        try {
            $content = Get-Content $CacheFile -Raw -ErrorAction Stop
            return $content | ConvertFrom-Json -ErrorAction Stop
        }
        catch { Write-WarningMsg "Failed to parse build cache, starting fresh" }
    }
    return @{}
}

function Save-BuildCache {
    param([string]$CacheFile, [hashtable]$Cache)
    try { $Cache | ConvertTo-Json -Depth 10 | Set-Content $CacheFile -Force }
    catch { Write-WarningMsg "Failed to save build cache: $_" }
}
# endregion

#region Main Script

# Initialize
$scriptStartTime = Get-Date
$scriptPath = $PSScriptRoot
if (-not $scriptPath) {
    if ($MyInvocation.MyCommand.Path) {
        $scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
    }
    else {
        $scriptPath = Get-Location
    }
}

# Auto-detect parallel jobs
if ($MaxParallelJobs -eq 0) {
    $cpuCores = (Get-CimInstance Win32_Processor).NumberOfLogicalProcessors
    $MaxParallelJobs = [math]::Max(1, [math]::Floor($cpuCores / 2))
    Write-Status "Auto-detected parallel jobs: $MaxParallelJobs (CPU cores: $cpuCores)"
}

Set-Location $scriptPath
Write-Status "Working directory: $(Get-Location)"
Write-Status "Install directory: $InstallDir"

# Environment setup
# Environment setup
if ($env:USE_SCCACHE -eq "1" -and (Get-Command sccache -ErrorAction SilentlyContinue)) {
    $env:RUSTC_WRAPPER = "sccache"
    Write-Status "sccache enabled via RUSTC_WRAPPER"
}
else {
    Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue
    $env:SCCACHE_DISABLE = "1"
    Write-Status "sccache disabled for this build"
}
$env:RUSTFLAGS = "-D warnings"
$env:CARGO_TERM_COLOR = "always"

# Step 1: Clean if requested
if ($Clean) {
    Write-Status "Step 1: Clean build requested, running cargo clean..."
    & cargo clean --release
    if (Test-Path $BuildCacheFile) {
        Remove-Item $BuildCacheFile -Force
    }
    Write-Success "Clean complete"
}

# Step 2: Kill processes
Write-Status "Step 2: Stopping non-CodexApp helper processes..."
$processesToKill = @("opencode")
$killSuccess = $true

foreach ($proc in $processesToKill) {
    if (-not (Stop-ProcessRobust -ProcessName $proc -MaxAttempts 3 -WaitSeconds 5)) {
        $killSuccess = $false
    }
}

if (-not $killSuccess) {
    Write-WarningMsg "Some processes could not be terminated, proceeding anyway..."
}

# Step 3: Load build cache and determine what needs building
Write-Status "Step 3: Analyzing build requirements..."
$buildCache = Get-BuildCache $BuildCacheFile
$buildsRequired = @{}
$sourceHashes = @{}

$packages = @(
    @{ Name = "codex-cli"; Path = "cli" }
)

if (-not $SkipOpenCode) {
    # Check if opencode exists in PATH
    $opencodePath = Get-Command "opencode" -ErrorAction SilentlyContinue
    if ($opencodePath) {
        Write-Status "OpenCode detected: $($opencodePath.Source)"
    }
}

foreach ($pkg in $packages) {
    $needsBuild, $hash = Test-DifferentialBuild -PackageName $pkg.Name -PackagePath $pkg.Path -Cache $buildCache
    $sourceHashes[$pkg.Name] = $hash
    
    if ($needsBuild -or $Clean) {
        $buildsRequired[$pkg.Name] = $pkg
    }
}

# Step 4: Build codex-cli
$pkgsToBuild = @()
if ($buildsRequired.ContainsKey("codex-cli")) { $pkgsToBuild += "codex-cli" }

if ($pkgsToBuild.Count -gt 0) {
    Write-Status "Step 4: Building $($pkgsToBuild -join ', ') in combined cargo process..."
    
    $pkgArgs = $pkgsToBuild | ForEach-Object { "-p $_" }
    $cargoCmd = "build --release $($pkgArgs -join ' ') -j $MaxParallelJobs"
    $buildStopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    
    # Combined build with corruption auto-recovery
    $attempts = 0
    $maxBuildAttempts = 2
    
    while ($attempts -lt $maxBuildAttempts) {
        $attempts++
        Write-Status "Build attempt $attempts..."
        
        # Help with LNK1207: delete problematic PDBs before build
        $pdbs = Get-ChildItem -Path "target\release" -Filter "*.pdb" -Recurse -ErrorAction SilentlyContinue
        if ($pdbs) { $pdbs | Remove-Item -Force -ErrorAction SilentlyContinue }

        # Capture output to detect corruption
        $outputFile = "build_output_$PID.txt"
        $process = Start-Process -FilePath "cargo" -ArgumentList $cargoCmd -NoNewWindow -Wait -PassThru -RedirectStandardError $outputFile
        
        $buildExitCode = $process.ExitCode
        $buildOutput = Get-Content $outputFile -Raw -ErrorAction SilentlyContinue
        Remove-Item $outputFile -Force -ErrorAction SilentlyContinue
        
        if ($buildExitCode -eq 0) {
            $buildStopwatch.Stop()
            Write-Success "Build successful in $($buildStopwatch.Elapsed.TotalSeconds.ToString('F1'))s"
            break
        }
        
        # Check for metadata corruption
        if ($buildOutput -match "corrupt metadata|invalid metadata|E0786|Unsupported archive identifier") {
            Write-WarningMsg "Metadata corruption detected (E0786). Attempting auto-fix by clearing deps..."
            Remove-Item "target\release\deps" -Force -Recurse -ErrorAction SilentlyContinue
            continue # Retry build
        }
        else {
            $buildStopwatch.Stop()
            Write-ErrorMsg "Build failed with exit code $buildExitCode"
            exit 1
        }
    }
}
else {
    Write-Success "Step 4: codex-cli is up to date, skipping build"
}

# Step 5: Legacy GUI install is retired in favor of Codex App and plugins
Write-Success "Step 5: Skipping legacy GUI build and install"

# Step 6: Update build cache
foreach ($pkgName in $sourceHashes.Keys) {
    $buildCache[$pkgName] = $sourceHashes[$pkgName]
}
Save-BuildCache -CacheFile $BuildCacheFile -Cache $buildCache

# Step 7: Install codex.exe
Write-Status "Step 6: Installing codex.exe..."

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$installHelper = Join-Path $scriptPath "..\scripts\install_with_kill.ps1"
$resolvedInstallHelper = [System.IO.Path]::GetFullPath($installHelper)
$sourceBinary = Join-Path $scriptPath "target\release\codex.exe"
$destinationBinary = Join-Path $InstallDir "codex.exe"

& $resolvedInstallHelper `
    -SourcePath $sourceBinary `
    -TargetPath $destinationBinary `
    -ProcessNames @("codex", "codex-tui", "codex-gui", "opencode") `
    -ExcludePathPrefixes @("C:\Program Files\WindowsApps\OpenAI.Codex_") `
    -Force `
    -MaxRetries 5

# Step 8: Verification
Write-Status "Step 7: Verifying installation..."
try {
    $cliVer = & "$InstallDir\codex.exe" --version 2>&1
    Write-Success "CLI Version: $cliVer"
}
catch {
    Write-WarningMsg "CLI verification failed: $_"
}

# Summary
$scriptEndTime = Get-Date
$totalDuration = ($scriptEndTime - $scriptStartTime).TotalSeconds

Write-Host ""
Write-Success "=== Build Complete ==="
Write-Status "Total duration: $($totalDuration.ToString('F1')) seconds"
Write-Status "Install location: $InstallDir"
Write-Status "Installed binary: codex.exe"

#endregion
