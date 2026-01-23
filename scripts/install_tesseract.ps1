# Tesseract OCR インストールスクリプト (Windows)
# ClaudeCowork統合機能のOCR機能に必要

param(
    [switch]$CheckOnly,
    [switch]$Help
)

$ErrorActionPreference = "Continue"

function Show-Help {
    Write-Host "Tesseract OCR インストールスクリプト" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "使用方法:" -ForegroundColor Yellow
    Write-Host "  .\install_tesseract.ps1              # Tesseractをインストール"
    Write-Host "  .\install_tesseract.ps1 -CheckOnly   # インストール状態を確認"
    Write-Host ""
    Write-Host "インストール方法:" -ForegroundColor Yellow
    Write-Host "  1. Chocolatey (推奨): choco install tesseract"
    Write-Host "  2. winget: winget install UB-Mannheim.TesseractOCR"
    Write-Host "  3. 手動: https://github.com/UB-Mannheim/tesseract/wiki からダウンロード"
    Write-Host ""
}

function Test-TesseractInstalled {
    """Tesseractがインストールされているか確認"""
    try {
        $tesseractPath = Get-Command tesseract -ErrorAction SilentlyContinue
        if ($tesseractPath) {
            $version = & tesseract --version 2>&1 | Select-Object -First 1
            Write-Host "[OK] Tesseractがインストールされています: $version" -ForegroundColor Green
            return $true
        }
        return $false
    } catch {
        return $false
    }
}

function Install-TesseractWithChocolatey {
    """ChocolateyでTesseractをインストール"""
    Write-Host "[INSTALL] ChocolateyでTesseractをインストール中..." -ForegroundColor Yellow
    
    # Chocolateyがインストールされているか確認
    $choco = Get-Command choco -ErrorAction SilentlyContinue
    if (-not $choco) {
        Write-Host "[WARN] Chocolateyがインストールされていません" -ForegroundColor Yellow
        Write-Host "   インストール: Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"
        return $false
    }
    
    try {
        & choco install tesseract -y
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[OK] Tesseractインストール完了" -ForegroundColor Green
            return $true
        } else {
            Write-Host "[ERROR] Chocolateyインストール失敗" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "[ERROR] Chocolateyインストールエラー: $_" -ForegroundColor Red
        return $false
    }
}

function Install-TesseractWithWinget {
    """wingetでTesseractをインストール"""
    Write-Host "[INSTALL] wingetでTesseractをインストール中..." -ForegroundColor Yellow
    
    # wingetがインストールされているか確認
    $winget = Get-Command winget -ErrorAction SilentlyContinue
    if (-not $winget) {
        Write-Host "[WARN] wingetがインストールされていません" -ForegroundColor Yellow
        Write-Host "   Windows 10/11のMicrosoft StoreからApp Installerをインストールしてください"
        return $false
    }
    
    try {
        & winget install UB-Mannheim.TesseractOCR --accept-package-agreements --accept-source-agreements
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[OK] Tesseractインストール完了" -ForegroundColor Green
            return $true
        } else {
            Write-Host "[ERROR] wingetインストール失敗" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "[ERROR] wingetインストールエラー: $_" -ForegroundColor Red
        return $false
    }
}

function Show-ManualInstallInstructions {
    """手動インストール手順を表示"""
    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host "手動インストール手順" -ForegroundColor Cyan
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host ""
    Write-Host "1. 以下のURLからTesseractをダウンロード:" -ForegroundColor Yellow
    Write-Host "   https://github.com/UB-Mannheim/tesseract/wiki" -ForegroundColor White
    Write-Host ""
    Write-Host "2. インストーラーを実行してインストール" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "3. 環境変数PATHにTesseractのインストールパスを追加:" -ForegroundColor Yellow
    Write-Host "   通常: C:\Program Files\Tesseract-OCR" -ForegroundColor White
    Write-Host ""
    Write-Host "4. インストール後、PowerShellを再起動して確認:" -ForegroundColor Yellow
    Write-Host "   tesseract --version" -ForegroundColor White
    Write-Host ""
}

function Main {
    if ($Help) {
        Show-Help
        return 0
    }
    
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host "Tesseract OCR インストールチェック" -ForegroundColor Cyan
    Write-Host "=" * 60 -ForegroundColor Cyan
    Write-Host ""
    
    # インストール状態を確認
    if (Test-TesseractInstalled) {
        if ($CheckOnly) {
            Write-Host "[OK] Tesseractは既にインストールされています" -ForegroundColor Green
            return 0
        } else {
            Write-Host "[INFO] Tesseractは既にインストールされています" -ForegroundColor Cyan
            return 0
        }
    }
    
    if ($CheckOnly) {
        Write-Host "[WARN] Tesseractがインストールされていません" -ForegroundColor Yellow
        Show-ManualInstallInstructions
        return 1
    }
    
    Write-Host "[WARN] Tesseractがインストールされていません" -ForegroundColor Yellow
    Write-Host "[INFO] 自動インストールを試みます..." -ForegroundColor Cyan
    Write-Host ""
    
    # インストール方法を試行
    $installed = $false
    
    # 1. wingetを試行
    if (-not $installed) {
        Write-Host "[INFO] wingetでインストールを試みます..." -ForegroundColor Cyan
        $installed = Install-TesseractWithWinget
    }
    
    # 2. Chocolateyを試行
    if (-not $installed) {
        Write-Host "[INFO] Chocolateyでインストールを試みます..." -ForegroundColor Cyan
        $installed = Install-TesseractWithChocolatey
    }
    
    # 3. 手動インストール案内
    if (-not $installed) {
        Write-Host ""
        Write-Host "[WARN] 自動インストールに失敗しました" -ForegroundColor Yellow
        Show-ManualInstallInstructions
        return 1
    }
    
    # インストール確認
    Write-Host ""
    Write-Host "[INFO] インストール状態を確認中..." -ForegroundColor Cyan
    Start-Sleep -Seconds 2
    
    if (Test-TesseractInstalled) {
        Write-Host ""
        Write-Host "[OK] Tesseractのインストールが完了しました！" -ForegroundColor Green
        Write-Host "[INFO] PowerShellを再起動するか、環境変数を再読み込みしてください" -ForegroundColor Cyan
        return 0
    } else {
        Write-Host ""
        Write-Host "[WARN] インストールは完了しましたが、PATHが更新されていない可能性があります" -ForegroundColor Yellow
        Write-Host "[INFO] PowerShellを再起動してから確認してください" -ForegroundColor Cyan
        Show-ManualInstallInstructions
        return 1
    }
}

# スクリプト実行
if ($MyInvocation.InvocationName -ne '.') {
    exit Main
}
