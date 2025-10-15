param([Parameter(ValueFromRemainingArguments=$true)][string[]]$args)
$wavPath = "C:\Users\downl\Desktop\新しいフォルダー (4)\marisa_owattaze.wav"
Write-Host "Cursor task completion notification" -ForegroundColor Magenta
if (Test-Path $wavPath) {
    try {
        $player = New-Object System.Media.SoundPlayer $wavPath
        $player.PlaySync()
        Write-Host "Sound played successfully (Marisa)" -ForegroundColor Green
    } catch {
        Write-Error "Error: $_"
        exit 1
    }
} else {
    Write-Warning "Sound file not found: $wavPath"
    Write-Host "Expected location: C:\Users\downl\Desktop\新しいフォルダー (4)\marisa_owattaze.wav"
    exit 1
}