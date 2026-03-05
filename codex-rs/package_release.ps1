$ReleaseVer = "v3.0.0"
$SourceDir = "target\release"
$OutputDir = "codex-release-win64"
$ArchiveName = "codex-$ReleaseVer-windows-x86_64.tar.gz"

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

Write-Host "Done. Archive created at $ArchiveName"
Get-Item $ArchiveName | Select-Object Name, Length
