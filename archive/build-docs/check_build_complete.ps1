# Codex ������Ɍ����դ���빯���

# ��Ɍ����
while (!(Test-Path 'codex-rs/target/x86_64-pc-windows-msvc/release/codex.exe')) {
    Write-Host '��Ʌ_-... (30�Thk��)'
    Start-Sleep -Seconds 30
}

Write-Host '��Ɍ�! Ф��L�dK�~W_'
ls codex-rs/target/x86_64-pc-windows-msvc/release/codex.exe

# ����������뒟L
py scripts/install_with_kill.ps1 -SourcePath 'codex-rs/target/x86_64-pc-windows-msvc/release/codex.exe' -TargetPath 'C:\bin\codex.exe' -Force

# ����뺍
codex --version