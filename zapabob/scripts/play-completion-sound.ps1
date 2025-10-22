param([Parameter(ValueFromRemainingArguments=$true)][string[]]$args)
$wavPath = "C:\Users\downl\Desktop\codex-main\codex-main\.codex\marisa_owattaze.wav"
Write-Host "🎉 Agent/Plan completion notification (Marisa)" -ForegroundColor Magenta
if (Test-Path $wavPath) {
    try {
        $player = New-Object System.Media.SoundPlayer $wavPath
        $player.PlaySync()
        Write-Host "✅ Sound played: 終わったぜ！ (Marisa)" -ForegroundColor Green
    } catch {
        Write-Error "❌ Error playing sound: $_"
        exit 1
    }
} else {
    Write-Warning "⚠️  Sound file not found: $wavPath"
    Write-Host "Expected location: C:\Users\downl\Desktop\codex-main\codex-main\.codex\marisa_owattaze.wav"
    exit 1
}