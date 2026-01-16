# Avast除外設定 - シンプル版
# Rust開発環境の除外設定を追加

param(
    [switch]$Help
)

function Show-Help {
    Write-Host "Avast Exclusion Setup (Simple)" -ForegroundColor Cyan
    Write-Host "==============================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Rust開発時のAvast誤検知を防ぐため、基本的な除外設定を追加します。"
    Write-Host ""
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host "  .\setup_avast_exclusions_simple.ps1"
    Write-Host ""
    Write-Host "Features:" -ForegroundColor Yellow
    Write-Host "  - Windows Defenderに開発関連パスを除外"
    Write-Host "  - 管理者権限不要（ベストエフォート）"
    Write-Host "  - シンプルで確実な設定"
    Write-Host ""
}

if ($Help) {
    Show-Help
    exit 0
}

# 除外対象のパス
$exclusions = @(
    "$env:USERPROFILE\.cargo",
    "$env:USERPROFILE\.rustup",
    "$env:USERPROFILE\Desktop\codex-main",
    "$env:USERPROFILE\Desktop\codex-main\codex-rs",
    "$env:USERPROFILE\Desktop\codex-main\codex-rs\target"
)

Write-Host "Avast/Windows Defender Exclusion Setup" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 管理者権限チェック
$currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
$isAdmin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "Note: Administrator privileges recommended for full exclusion setup." -ForegroundColor Yellow
    Write-Host ""
}

Write-Host "Adding exclusions for Rust development..." -ForegroundColor Green
Write-Host ""

$successCount = 0
$totalCount = $exclusions.Length

foreach ($path in $exclusions) {
    Write-Host "Processing: $path" -ForegroundColor Gray

    # パスが存在するかチェック
    if (Test-Path $path) {
        try {
            # Windows Defenderに除外設定を追加
            Add-MpPreference -ExclusionPath $path -ErrorAction Stop
            Write-Host "  ✓ Added to Windows Defender exclusions" -ForegroundColor Green
            $successCount++
        }
        catch {
            Write-Host "  ⚠ Could not add to Windows Defender: $($_.Exception.Message)" -ForegroundColor Yellow
        }

        # Avastがインストールされている場合の追加設定
        try {
            $avastPaths = @(
                "HKLM:\SOFTWARE\AVAST Software\Avast\properties",
                "HKLM:\SOFTWARE\WOW6432Node\AVAST Software\Avast\properties"
            )

            $avastConfigured = $false
            foreach ($regPath in $avastPaths) {
                if (Test-Path $regPath) {
                    try {
                        $current = (Get-ItemProperty -Path $regPath -Name "Exclusions" -ErrorAction SilentlyContinue).Exclusions
                        if ($current) {
                            if ($current -notlike "*$path*") {
                                $newValue = "$current;$path"
                                Set-ItemProperty -Path $regPath -Name "Exclusions" -Value $newValue -Type String
                                Write-Host "  ✓ Added to Avast exclusions" -ForegroundColor Green
                                $avastConfigured = $true
                            } else {
                                Write-Host "  - Already in Avast exclusions" -ForegroundColor Gray
                                $avastConfigured = $true
                            }
                        } else {
                            Set-ItemProperty -Path $regPath -Name "Exclusions" -Value $path -Type String
                            Write-Host "  ✓ Created Avast exclusions entry" -ForegroundColor Green
                            $avastConfigured = $true
                        }
                    }
                    catch {
                        # レジストリアクセスが失敗しても続行
                    }
                }
            }

            if (-not $avastConfigured) {
                Write-Host "  - Avast not detected or not configured" -ForegroundColor Gray
            }
        }
        catch {
            Write-Host "  - Avast configuration skipped" -ForegroundColor Gray
        }
    } else {
        Write-Host "  - Path does not exist" -ForegroundColor Yellow
    }

    Write-Host ""
}

Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "  Total paths processed: $totalCount" -ForegroundColor White
Write-Host "  Successfully configured: $successCount" -ForegroundColor Green
Write-Host ""

if ($successCount -gt 0) {
    Write-Host "Exclusion setup completed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Restart your antivirus software if needed"
    Write-Host "2. Try building your Rust project"
    Write-Host "3. If issues persist, check antivirus logs"
    Write-Host ""
    Write-Host "For manual configuration:" -ForegroundColor Yellow
    Write-Host "- Windows Defender: Settings > Update & Security > Windows Security > Virus & threat protection > Manage settings > Exclusions"
    Write-Host "- Avast: Settings > General > Exceptions"
} else {
    Write-Host "No exclusions were successfully configured." -ForegroundColor Red
    Write-Host "Try running as Administrator or check permissions." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Done." -ForegroundColor Green