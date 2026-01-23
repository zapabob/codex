#!/usr/bin/env python3
"""
ClaudeCowork統合機能に必要なライブラリのインストールスクリプト
"""

import subprocess
import sys
from pathlib import Path

def install_package(package: str):
    """パッケージをインストール"""
    try:
        print(f"[INSTALL] {package} をインストール中...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", package, "--quiet"])
        print(f"[OK] {package} インストール完了")
        return True
    except subprocess.CalledProcessError as e:
        print(f"[ERROR] {package} インストール失敗: {e}")
        return False

def main():
    """メイン実行"""
    print("=" * 60)
    print("ClaudeCowork統合機能 - 依存ライブラリインストール")
    print("=" * 60)
    
    # 必要なパッケージリスト
    packages = [
        # ブラウザ自動化
        "playwright",
        "pytesseract",
        "Pillow",
        
        # ドキュメント生成
        "openpyxl",
        "python-docx",
        "python-pptx",
        
        # データ分析
        "pandas",
        "numpy",
        "matplotlib",
        "seaborn",
        
        # 外部サービス統合
        "aiohttp",
        "requests",
        
        # その他
        "pystray",
    ]
    
    failed_packages = []
    
    for package in packages:
        if not install_package(package):
            failed_packages.append(package)
    
    # Playwrightブラウザのインストール
    if "playwright" not in failed_packages:
        print("\n[INSTALL] Playwrightブラウザをインストール中...")
        try:
            subprocess.check_call([sys.executable, "-m", "playwright", "install", "chromium", "--quiet"])
            print("[OK] Playwrightブラウザインストール完了")
        except subprocess.CalledProcessError as e:
            print(f"[WARN] Playwrightブラウザインストール失敗: {e}")
            print("   手動で実行: python -m playwright install chromium")
    
    # Tesseract OCRのインストール確認
    print("\n[INSTALL] Tesseract OCRのインストール状態を確認中...")
    try:
        # Tesseractがインストールされているか確認
        result = subprocess.run(
            ["tesseract", "--version"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            version_line = result.stdout.split('\n')[0] if result.stdout else "不明"
            print(f"[OK] Tesseract OCRがインストールされています: {version_line}")
        else:
            print("[WARN] Tesseract OCRがインストールされていません")
            print("   OCR機能を使用するにはTesseractのインストールが必要です")
            print("   インストール方法:")
            print("   1. PowerShellで実行: .\\scripts\\install_tesseract.ps1")
            print("   2. Chocolatey: choco install tesseract")
            print("   3. winget: winget install UB-Mannheim.TesseractOCR")
            print("   4. 手動: https://github.com/UB-Mannheim/tesseract/wiki")
    except (subprocess.CalledProcessError, FileNotFoundError, subprocess.TimeoutExpired):
        print("[WARN] Tesseract OCRがインストールされていません")
        print("   OCR機能を使用するにはTesseractのインストールが必要です")
        print("   インストール方法:")
        print("   1. PowerShellで実行: .\\scripts\\install_tesseract.ps1")
        print("   2. Chocolatey: choco install tesseract")
        print("   3. winget: winget install UB-Mannheim.TesseractOCR")
        print("   4. 手動: https://github.com/UB-Mannheim/tesseract/wiki")
    except Exception as e:
        print(f"[WARN] Tesseract確認中にエラー: {e}")
        print("   OCR機能を使用するにはTesseractのインストールが必要です")
    
    print("\n" + "=" * 60)
    if failed_packages:
        print(f"[WARN] 失敗したパッケージ: {', '.join(failed_packages)}")
        print("   手動でインストールしてください")
        return 1
    else:
        print("[OK] すべてのパッケージのインストールが完了しました")
        return 0

if __name__ == "__main__":
    sys.exit(main())
