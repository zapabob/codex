$ReleaseVer = "v2.12.1"
$SourceDir = "target\release"
$OutputDir = "codex-release-win64"
$ArchiveName = "codex-$ReleaseVer-windows-x86_64.tar.gz"

Write-Host "Checking for release binaries..."

# Wait loop check (optional, but good for manual run)
if (-not (Test-Path "$SourceDir\codex-cli.exe") -and -not (Test-Path "$SourceDir\codex.exe")) {
    Write-Warning "Main binaries (codex.exe / codex-cli.exe) not found in $SourceDir."
    Write-Warning "Please ensure 'cargo build --release' completes successfully before running this script."
    # We don't exit here to allow partial packaging if user really wants, or just to show what's there.
}

Write-Host "Creating release package directory: $OutputDir"
if (Test-Path $OutputDir) { Remove-Item $OutputDir -Recurse -Force }
New-Item -ItemType Directory -Path $OutputDir | Out-Null

# Copy all .exe files from release folder
$Executables = Get-ChildItem -Path $SourceDir -Filter "*.exe"
foreach ($exe in $Executables) {
    Write-Host "Copying $($exe.Name)..."
    Copy-Item $exe.FullName -Destination $OutputDir
}

# Add README if exists
if (Test-Path "README.md") {
    Copy-Item "README.md" -Destination $OutputDir
}

Write-Host "Creating archive: $ArchiveName"
tar -czf $ArchiveName -C . $OutputDir

Write-Host "Archive created at $ArchiveName"
Get-Item $ArchiveName | Select-Object Name, Length

Write-Host "Uploading to GitHub Release $ReleaseVer..."
gh release upload $ReleaseVer $ArchiveName

if ($?) {
    Write-Host "Upload successful!" -ForegroundColor Green
}
else {
    Write-Error "Upload failed. Please check gh cli authentication or network."
}
