# Avast誤検知対策 - 除外設定スクリプト
# Rust開発環境とビルドプロセスをAvastから除外する

param(
    [switch]$AddExclusions,
    [switch]$RemoveExclusions,
    [switch]$CheckExclusions,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

# 除外対象のパス
$exclusions = @(
    # Rust関連
    "$env:USERPROFILE\.cargo",
    "$env:USERPROFILE\.rustup",
    "$env:USERPROFILE\.multirust",

    # プロジェクト関連
    "$env:USERPROFILE\Desktop\codex-main",
    "$env:USERPROFILE\Desktop\codex-main\codex-rs",
    "$env:USERPROFILE\Desktop\codex-main\codex-rs\target",

    # ビルドツール
    "C:\Program Files\Microsoft Visual Studio",
    "C:\Program Files (x86)\Microsoft Visual Studio",

    # Git関連
    "$env:USERPROFILE\.git",

    # Node.js/npm関連
    "$env:APPDATA\npm",
    "$env:APPDATA\npm-cache",
    "$env:LOCALAPPDATA\Yarn",

    # Python関連
    "C:\Python*",
    "$env:USERPROFILE\AppData\Local\Programs\Python",

    # 開発ツール
    "C:\Program Files\Git",
    "C:\Program Files\CMake",
    "C:\Program Files\LLVM",

    # CI/CD関連
    "$env:USERPROFILE\.github",
    "$env:USERPROFILE\.cursor"
)

function Show-Help {
    Write-Host "Avast Exclusion Setup Script" -ForegroundColor Cyan
    Write-Host "============================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host "  .\setup_avast_exclusions.ps1 -AddExclusions    # 除外設定を追加"
    Write-Host "  .\setup_avast_exclusions.ps1 -RemoveExclusions # 除外設定を削除"
    Write-Host "  .\setup_avast_exclusions.ps1 -CheckExclusions  # 除外設定を確認"
    Write-Host "  .\setup_avast_exclusions.ps1 -Help            # このヘルプを表示"
    Write-Host ""
    Write-Host "Description:" -ForegroundColor Yellow
    Write-Host "  Rust開発時のAvast誤検知を防ぐため、ビルド関連ディレクトリを"
    Write-Host "  Avastのリアルタイムスキャン除外設定に追加します。"
    Write-Host ""
    Write-Host "Requirements:" -ForegroundColor Yellow
    Write-Host "  - Administrator privileges (管理者権限)"
    Write-Host "  - Avast Antivirus installed"
    Write-Host ""
}

function Test-AdminPrivileges {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Check-AvastInstalled {
    try {
        $avastPaths = @(
            "HKLM:\SOFTWARE\AVAST Software\Avast",
            "HKLM:\SOFTWARE\WOW6432Node\AVAST Software\Avast",
            "${env:ProgramFiles}\Avast",
            "${env:ProgramFiles(x86)}\Avast"
        )

        foreach ($path in $avastPaths) {
            if (Test-Path $path) {
                return $true
            }
        }

        # サービスチェック
        $avastService = Get-Service -Name "*avast*" -ErrorAction SilentlyContinue
        if ($avastService) {
            return $true
        }

        return $false
    }
    catch {
        return $false
    }
}

function Add-AvastExclusions {
    Write-Host "Avast除外設定を追加しています..." -ForegroundColor Yellow

    $avastExe = "${env:ProgramFiles}\Avast\AvastUI.exe"
    if (!(Test-Path $avastExe)) {
        $avastExe = "${env:ProgramFiles(x86)}\Avast\AvastUI.exe"
    }

    if (!(Test-Path $avastExe)) {
        Write-Host "Avastが見つかりません。Avastがインストールされているか確認してください。" -ForegroundColor Red
        return
    }

    Write-Host "除外設定を追加するパス:" -ForegroundColor Cyan
    foreach ($exclusion in $exclusions) {
        Write-Host "  - $exclusion" -ForegroundColor Gray

        # Avastのコマンドラインで除外設定を追加
        # 注意: Avastのコマンドラインインターフェースはバージョンによって異なる
        try {
            # レジストリ経由で除外設定を追加（Avastの設定方法）
            $regPath = "HKLM:\SOFTWARE\AVAST Software\Avast\properties"
            if (!(Test-Path $regPath)) {
                $regPath = "HKLM:\SOFTWARE\WOW6432Node\AVAST Software\Avast\properties"
            }

            if (Test-Path $regPath) {
                # 除外リストに追加（実際のレジストリパスはAvastバージョンによる）
                $currentExclusions = (Get-ItemProperty -Path $regPath -Name "Exclusions" -ErrorAction SilentlyContinue).Exclusions
                if ($currentExclusions) {
                    $currentExclusions += ";$exclusion"
                } else {
                    $currentExclusions = $exclusion
                }

                Set-ItemProperty -Path $regPath -Name 'Exclusions' -Value $currentExclusions -Type String
                Write-Host "    ✓ 追加完了: $exclusion" -ForegroundColor Green
            } else {
                Write-Host "    ⚠ レジストリパスが見つかりません: $regPath" -ForegroundColor Yellow
            }
        }
        catch {
            Write-Host "    ✗ 追加失敗: $exclusion - $($_.Exception.Message)" -ForegroundColor Red
        }
    }

    Write-Host ""
    Write-Host "Avast GUIでの除外設定確認をおすすめします:" -ForegroundColor Cyan
    Write-Host "1. Avast UIを開く"
    Write-Host "2. メニュー → 設定 → 一般 → 除外"
    Write-Host "3. 上記のパスを追加"
    Write-Host ""

    # Avastサービスの再起動を促す
    Write-Host "設定変更を反映するため、Avastサービスの再起動をおすすめします。" -ForegroundColor Yellow
}

function Remove-AvastExclusions {
    Write-Host "Avast除外設定を削除しています..." -ForegroundColor Yellow

    # レジストリから除外設定を削除
    try {
        $regPath = "HKLM:\SOFTWARE\AVAST Software\Avast\properties"
        if (!(Test-Path $regPath)) {
            $regPath = "HKLM:\SOFTWARE\WOW6432Node\AVAST Software\Avast\properties"
        }

        if (Test-Path $regPath) {
            $currentExclusions = (Get-ItemProperty -Path $regPath -Name "Exclusions" -ErrorAction SilentlyContinue).Exclusions
            if ($currentExclusions) {
                $exclusionList = $currentExclusions -split ";"
                $filteredList = @()

                foreach ($existingExclusion in $exclusionList) {
                    $shouldKeep = $true
                    foreach ($targetExclusion in $exclusions) {
                        if ($existingExclusion -eq $targetExclusion) {
                            $shouldKeep = $false
                            Write-Host "  - 削除: $targetExclusion" -ForegroundColor Green
                            break
                        }
                    }
                    if ($shouldKeep) {
                        $filteredList += $existingExclusion
                    }
                }

                $newExclusions = $filteredList -join ";"
                Set-ItemProperty -Path $regPath -Name "Exclusions" -Value $newExclusions -Type String
            }
        }
    }
    catch {
        Write-Host "除外設定削除エラー: $($_.Exception.Message)" -ForegroundColor Red
    }
}

function Check-AvastExclusions {
    Write-Host "現在のAvast除外設定を確認しています..." -ForegroundColor Yellow

    try {
        $regPath = "HKLM:\SOFTWARE\AVAST Software\Avast\properties"
        if (!(Test-Path $regPath)) {
            $regPath = "HKLM:\SOFTWARE\WOW6432Node\AVAST Software\Avast\properties"
        }

        if (Test-Path $regPath) {
            $currentExclusions = (Get-ItemProperty -Path $regPath -Name "Exclusions" -ErrorAction SilentlyContinue).Exclusions

            if ($currentExclusions) {
                $exclusionList = $currentExclusions -split ";"
                Write-Host "現在の除外設定:" -ForegroundColor Cyan

                foreach ($exclusion in $exclusionList) {
                    if ($exclusion) {
                        $isTarget = $false
                        foreach ($targetExclusion in $exclusions) {
                            if ($exclusion -eq $targetExclusion) {
                                $isTarget = $true
                                break
                            }
                        }

                        if ($isTarget) {
                            Write-Host "  ✓ $exclusion" -ForegroundColor Green
                        } else {
                            Write-Host "  - $exclusion" -ForegroundColor Gray
                        }
                    }
                }
            } else {
                Write-Host "除外設定が見つかりません。" -ForegroundColor Yellow
            }
        } else {
            Write-Host "Avastレジストリ設定が見つかりません。" -ForegroundColor Red
        }
    }
    catch {
        Write-Host "除外設定確認エラー: $($_.Exception.Message)" -ForegroundColor Red
    }

    Write-Host ""
    Write-Host "推奨される除外設定:" -ForegroundColor Cyan
    foreach ($exclusion in $exclusions) {
        Write-Host "  - $exclusion" -ForegroundColor Gray
    }
}

# メイン処理
if ($Help) {
    Show-Help
    exit 0
}

# 管理者権限チェック
if (!(Test-AdminPrivileges)) {
    Write-Host "このスクリプトは管理者権限で実行してください。" -ForegroundColor Red
    Write-Host "PowerShellを管理者として開き直して実行してください。" -ForegroundColor Yellow
    exit 1
}

# Avastインストールチェック
if (!(Check-AvastInstalled)) {
    Write-Host "Avastがインストールされていないか、検出できません。" -ForegroundColor Red
    Write-Host "Avastが正しくインストールされているか確認してください。" -ForegroundColor Yellow
    exit 1
}

Write-Host "Avast Exclusion Setup" -ForegroundColor Cyan
Write-Host "====================" -ForegroundColor Cyan
Write-Host ""

if ($AddExclusions) {
    Add-AvastExclusions
} elseif ($RemoveExclusions) {
    Remove-AvastExclusions
} elseif ($CheckExclusions) {
    Check-AvastExclusions
} else {
    Write-Host "オプションを指定してください。-Helpで詳細を確認できます。" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "完了しました。" -ForegroundColor Green