# Ultra Fast Build & Install Script for Codex + OpenCode
# Features: Parallel builds, differential detection, robust process kill, atomic installation
# Author: AI-Assisted Implementation
# Version: 1.0.0

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
            foreach ($pid in $pids) {
                $proc = Get-Process -Id $pid -ErrorAction SilentlyContinue
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
                Id = 0
            }
        }
    }
    return $processes
}

function Stop-ProcessRobust {
    param(
        [string]$ProcessName,
        [int]$MaxAttempts = 3,
        [int]$WaitSeconds = 5
    )
    
    for ($attempt = 1; $attempt -le $MaxAttempts; $attempt++) {
        $procs = Get-Process $ProcessName -ErrorAction SilentlyContinue
        if (-not $procs) {
            return $true
        }
        
        Write-Status "Attempting to stop $ProcessName (attempt $attempt/$MaxAttempts)..."
        
        foreach ($proc in $procs) {
            try {
                Stop-Process -Id $proc.Id -Force -ErrorAction Stop
                Write-Success "Stopped process: $($proc.ProcessName) (PID: $($proc.Id))"
            }
            catch {
                Write-WarningMsg "Failed to stop process $($proc.ProcessName) (PID: $($proc.Id)): $_"
                
                # Try taskkill as fallback
                try {
                    $null = & taskkill /F /IM $ProcessName /T 2>&1
                    Write-Success "Used taskkill for $ProcessName"
                }
                catch {
                    Write-WarningMsg "taskkill also failed for $ProcessName"
                }
            }
        }
        
        Start-Sleep -Seconds $WaitSeconds
    }
    
    # Final check
    $remaining = Get-Process $ProcessName -ErrorAction SilentlyContinue
    if ($remaining) {
        Write-ErrorMsg "Failed to kill all $ProcessName processes after $MaxAttempts attempts"
        return $false
    }
    return $true
}

function Install-BinaryAtomic {
    param(
        [string]$Source,
        [string]$Destination,
        [int]$MaxRetries = 5
    )
    
    if (-not (Test-Path $Source)) {
        throw "Source file not found: $Source"
    }
    
    $retryCount = 0
    $backoffSeconds = 1
    
    while ($retryCount -lt $MaxRetries) {
        try {
            # Check if destination is locked
            if (Test-Path $Destination) {
                if (Test-FileLocked $Destination) {
                    Write-WarningMsg "$Destination is locked. Finding locking processes..."
                    $lockingProcs = Get-LockingProcesses $Destination
                    foreach ($proc in $lockingProcs) {
                        if ($proc.Id -ne 0) {
                            Write-Status "Killing locking process: $($proc.ProcessName) (PID: $($proc.Id))"
                            Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
                        }
                    }
                    Start-Sleep -Seconds $backoffSeconds
                    $retryCount++
                    $backoffSeconds *= 2
                    continue
                }
            }
            
            # Atomic copy: write to temp then rename
            $tempDest = "$Destination.tmp.$PID"
            Copy-Item $Source $tempDest -Force -ErrorAction Stop
            
            # If destination exists, backup first
            if (Test-Path $Destination) {
                $backup = "$Destination.backup.$PID"
                Move-Item $Destination $backup -Force -ErrorAction Stop
            }
            
            # Atomic rename
            Move-Item $tempDest $Destination -Force -ErrorAction Stop
            
            # Clean up backup
            if (Test-Path "$Destination.backup.$PID") {
                Remove-Item "$Destination.backup.$PID" -Force -ErrorAction SilentlyContinue
            }
            
            Write-Success "Installed: $(Split-Path $Destination -Leaf)"
            return $true
        }
        catch {
            Write-WarningMsg "Install attempt $retryCount failed: $_"
            $retryCount++
            if ($retryCount -lt $MaxRetries) {
                Write-Status "Retrying in $backoffSeconds seconds..."
                Start-Sleep -Seconds $backoffSeconds
                $backoffSeconds *= 2
            }
        }
    }
    
    throw "Failed to install $(Split-Path $Destination -Leaf) after $MaxRetries attempts"
}

#endregion

#region Build Cache Management

function Get-BuildCache {
    param([string]$CacheFile)
    
    if (Test-Path $CacheFile) {
        try {
            $content = Get-Content $CacheFile -Raw -ErrorAction Stop
            $cache = $content | ConvertFrom-Json -ErrorAction Stop
            return $cache
        }
        catch {
            Write-WarningMsg "Failed to parse build cache, starting fresh"
        }
    }
    return @{}
}

function Save-BuildCache {
    param(
        [string]$CacheFile,
        [hashtable]$Cache
    )
    
    try {
        $Cache | ConvertTo-Json -Depth 10 | Set-Content $CacheFile -Force
    }
    catch {
        Write-WarningMsg "Failed to save build cache: $_"
    }
}

function Get-SourceHash {
    param(
        [string]$PackagePath,
        [string[]]$Extensions = @("*.rs", "*.toml", "*.lock")
    )
    
    $hashes = @()
    foreach ($ext in $Extensions) {
        $files = Get-ChildItem -Path $PackagePath -Recurse -Filter $ext -File -ErrorAction SilentlyContinue
        foreach ($file in $files) {
            $hash = Get-FileHash256 $file.FullName
            if ($hash) {
                $hashes += "$($file.FullName):$hash"
            }
        }
    }
    
    # Sort for consistent hashing
    $sortedHashes = $hashes | Sort-Object
    $combined = $sortedHashes -join "|"
    
    if ($combined) {
        $sha256 = [System.Security.Cryptography.SHA256]::Create()
        $bytes = [System.Text.Encoding]::UTF8.GetBytes($combined)
        $hash = $sha256.ComputeHash($bytes)
        return [BitConverter]::ToString($hash).Replace("-", "").ToLower()
    }
    return $null
}

function Test-DifferentialBuild {
    param(
        [string]$PackageName,
        [string]$PackagePath,
        [hashtable]$Cache
    )
    
    $currentHash = Get-SourceHash $PackagePath
    $cachedHash = $Cache[$PackageName]
    
    if (-not $cachedHash) {
        Write-Status "No cache for $PackageName, full build required"
        return $true, $currentHash
    }
    
    if ($currentHash -ne $cachedHash) {
        Write-Status "Changes detected in $PackageName, rebuild required"
        return $true, $currentHash
    }
    
    Write-Success "No changes in $PackageName, skipping build"
    return $false, $currentHash
}

#endregion

#region Parallel Build Functions

function Invoke-ParallelBuild {
    param(
        [Parameter(Mandatory)]
        [hashtable]$BuildJobs,
        [int]$MaxParallel = 4
    )
    
    $runspacePool = [runspacefactory]::CreateRunspacePool(1, $MaxParallel)
    $runspacePool.Open()
    
    $runspaces = @()
    $results = @{}
    
    foreach ($jobName in $BuildJobs.Keys) {
        $job = $BuildJobs[$jobName]
        
        $powershell = [powershell]::Create().AddScript({
            param($WorkingDirectory, $Command, $PackageName)
            
            Set-Location $WorkingDirectory
            
            $output = @{
                PackageName = $PackageName
                Success = $false
                Output = ""
                Error = ""
                Duration = 0
            }
            
            $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
            
            try {
                $process = Start-Process -FilePath "cargo" -ArgumentList $Command -NoNewWindow -Wait -PassThru -RedirectStandardOutput "output.txt" -RedirectStandardError "error.txt"
                
                $output.Output = Get-Content "output.txt" -Raw -ErrorAction SilentlyContinue
                $output.Error = Get-Content "error.txt" -Raw -ErrorAction SilentlyContinue
                
                $output.Success = ($process.ExitCode -eq 0)
            }
            catch {
                $output.Error = $_.Exception.Message
            }
            finally {
                $stopwatch.Stop()
                $output.Duration = $stopwatch.Elapsed.TotalSeconds
                Remove-Item "output.txt", "error.txt" -Force -ErrorAction SilentlyContinue
            }
            
            return $output
        }).AddArgument($job.WorkingDirectory).AddArgument($job.Command).AddArgument($job.PackageName)
        
        $powershell.RunspacePool = $runspacePool
        
        $runspaces += [PSCustomObject]@{
            Pipe = $powershell
            Status = $powershell.BeginInvoke()
            JobName = $jobName
        }
    }
    
    # Wait for completion and collect results
    foreach ($rs in $runspaces) {
        $result = $rs.Pipe.EndInvoke($rs.Status)
        $results[$rs.JobName] = $result[0]
        $rs.Pipe.Dispose()
    }
    
    $runspacePool.Close()
    $runspacePool.Dispose()
    
    return $results
}

#endregion

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
$env:RUSTC_WRAPPER = ""
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
Write-Status "Step 2: Stopping existing processes..."
$processesToKill = @("codex", "codex-tui", "codex-gui", "opencode")
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
    @{ Name = "codex-cli"; Path = "cli"; Features = "--features custom-features" },
    @{ Name = "codex-tui"; Path = "tui"; Features = "" },
    @{ Name = "codex-tauri-gui"; Path = "tauri-gui/src-tauri"; Features = ""; IsTauri = $true }
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

# Step 4: Parallel Build Phase 1 - CLI and TUI
$phase1Jobs = @{}
if ($buildsRequired.ContainsKey("codex-cli")) {
    $phase1Jobs["codex-cli"] = @{
        WorkingDirectory = $scriptPath
        Command = "build --release $($buildsRequired["codex-cli"].Features) -p codex-cli -j $MaxParallelJobs"
        PackageName = "codex-cli"
    }
}

if ($buildsRequired.ContainsKey("codex-tui")) {
    $phase1Jobs["codex-tui"] = @{
        WorkingDirectory = $scriptPath
        Command = "build --release -p codex-tui -j $MaxParallelJobs"
        PackageName = "codex-tui"
    }
}

if ($phase1Jobs.Count -gt 0) {
    Write-Status "Step 4: Building CLI and TUI in parallel ($($phase1Jobs.Count) jobs)..."
    $phase1Results = Invoke-ParallelBuild -BuildJobs $phase1Jobs -MaxParallel $MaxParallelJobs
    
    foreach ($jobName in $phase1Results.Keys) {
        $result = $phase1Results[$jobName]
        if ($result.Success) {
            Write-Success "$jobName built successfully in $($result.Duration.ToString('F1'))s"
        }
        else {
            Write-ErrorMsg "$jobName build failed!"
            if ($result.Error) {
                Write-Host $result.Error -ForegroundColor Red
            }
            exit 1
        }
    }
}
else {
    Write-Success "Step 4: CLI and TUI are up to date, skipping build"
}

# Step 5: Build GUI (depends on core crates)
if ($buildsRequired.ContainsKey("codex-tauri-gui")) {
    Write-Status "Step 5: Building GUI..."
    Push-Location "tauri-gui"
    
    try {
        # Check if npm dependencies are installed
        if (-not (Test-Path "node_modules")) {
            Write-Status "Installing npm dependencies..."
            & npm ci
            if ($LASTEXITCODE -ne 0) {
                throw "npm install failed"
            }
        }
        
        # Build Tauri
        $guiStopwatch = [System.Diagnostics.Stopwatch]::StartNew()
        & npm run tauri:build
        $guiStopwatch.Stop()
        
        if ($LASTEXITCODE -ne 0) {
            throw "GUI build failed"
        }
        
        Write-Success "GUI built successfully in $($guiStopwatch.Elapsed.TotalSeconds.ToString('F1'))s"
    }
    catch {
        Write-ErrorMsg "GUI build failed: $_"
        Pop-Location
        exit 1
    }
    finally {
        Pop-Location
    }
}
else {
    Write-Success "Step 5: GUI is up to date, skipping build"
}

# Step 6: Update build cache
foreach ($pkgName in $sourceHashes.Keys) {
    $buildCache[$pkgName] = $sourceHashes[$pkgName]
}
Save-BuildCache -CacheFile $BuildCacheFile -Cache $buildCache

# Step 7: Install binaries
Write-Status "Step 6: Installing binaries..."

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$binaries = @(
    @{ Source = "target\release\codex.exe"; Dest = "codex.exe" },
    @{ Source = "target\release\codex-tui.exe"; Dest = "codex-tui.exe" },
    @{ Source = "target\release\codex-tauri-gui.exe"; Dest = "codex-gui.exe" }
)

$installSuccess = $true
foreach ($bin in $binaries) {
    $source = Join-Path $scriptPath $bin.Source
    $dest = Join-Path $InstallDir $bin.Dest
    
    try {
        Install-BinaryAtomic -Source $source -Destination $dest -MaxRetries 5
    }
    catch {
        Write-ErrorMsg "Failed to install $($bin.Dest): $_"
        $installSuccess = $false
    }
}

if (-not $installSuccess) {
    exit 1
}

# Step 8: Verification
Write-Status "Step 7: Verifying installation..."
try {
    $cliVer = & "$InstallDir\codex.exe" --version 2>&1
    Write-Success "CLI Version: $cliVer"
}
catch {
    Write-WarningMsg "CLI verification failed: $_"
}

# Create desktop shortcuts
$desktopDir = [Environment]::GetFolderPath("Desktop")
$WshShell = New-Object -ComObject WScript.Shell

$shortcuts = @(
    @{ Source = "$InstallDir\codex-tui.exe"; Name = "Codex TUI.lnk" },
    @{ Source = "$InstallDir\codex-gui.exe"; Name = "Codex GUI.lnk" }
)

foreach ($shortcut in $shortcuts) {
    if (Test-Path $shortcut.Source) {
        $shortcutPath = Join-Path $desktopDir $shortcut.Name
        $sc = $WshShell.CreateShortcut($shortcutPath)
        $sc.TargetPath = $shortcut.Source
        $sc.Save()
        Write-Success "Created shortcut: $($shortcut.Name)"
    }
}

# Summary
$scriptEndTime = Get-Date
$totalDuration = ($scriptEndTime - $scriptStartTime).TotalSeconds

Write-Host ""
Write-Success "=== Build Complete ==="
Write-Status "Total duration: $($totalDuration.ToString('F1')) seconds"
Write-Status "Install location: $InstallDir"
Write-Status "Binaries: codex.exe, codex-tui.exe, codex-gui.exe"

#endregion
