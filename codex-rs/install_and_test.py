#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
ビルド済みバイナリのインストールとテストスクリプト
"""

import subprocess
import sys
import os
import time
from datetime import datetime
from pathlib import Path

# Windows環境での文字エンコーディング対策
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

def install_binary(source_path, install_path):
    """バイナリを上書きインストール"""
    print(f"\n📦 バイナリをインストール中...")
    print(f"   ソース: {source_path}")
    print(f"   インストール先: {install_path}")
    
    try:
        # インストール先ディレクトリが存在するか確認
        install_dir = os.path.dirname(install_path)
        if not os.path.exists(install_dir):
            os.makedirs(install_dir, exist_ok=True)
            print(f"   ✅ ディレクトリ作成: {install_dir}")
        
        # 既存のバイナリをバックアップ（オプション）
        if os.path.exists(install_path):
            backup_path = f"{install_path}.backup-{datetime.now().strftime('%Y%m%d-%H%M%S')}"
            import shutil
            shutil.copy2(install_path, backup_path)
            print(f"   📋 バックアップ作成: {backup_path}")
        
        # バイナリをコピー
        import shutil
        shutil.copy2(source_path, install_path)
        file_size = os.path.getsize(install_path) / (1024 * 1024)  # MB
        print(f"   ✅ インストール完了 ({file_size:.2f} MB)")
        
        return True, file_size
    except Exception as e:
        print(f"   ❌ インストール失敗: {e}")
        return False, 0

def test_binary(binary_path="codex"):
    """実機テストを実行"""
    print(f"\n🧪 実機テストを実行中...")
    
    tests = [
        ("バージョン確認", [binary_path, "--version"]),
        ("ヘルプ表示", [binary_path, "--help"]),
    ]
    
    results = []
    
    for test_name, command in tests:
        print(f"\n   テスト: {test_name}")
        print(f"   コマンド: {' '.join(command)}")
        
        try:
            start_time = time.time()
            result = subprocess.run(
                command,
                capture_output=True,
                text=True,
                timeout=30,
                encoding='utf-8',
                errors='replace'
            )
            elapsed = time.time() - start_time
            
            if result.returncode == 0:
                print(f"   ✅ 成功 (経過: {elapsed:.2f}秒)")
                results.append({
                    "test": test_name,
                    "status": "success",
                    "elapsed": elapsed,
                    "output": result.stdout[:200]  # 最初の200文字のみ
                })
            else:
                print(f"   ❌ 失敗 (終了コード: {result.returncode})")
                print(f"   エラー出力: {result.stderr[:200]}")
                results.append({
                    "test": test_name,
                    "status": "failed",
                    "elapsed": elapsed,
                    "error": result.stderr[:200]
                })
        except subprocess.TimeoutExpired:
            print(f"   ⏱️  タイムアウト")
            results.append({
                "test": test_name,
                "status": "timeout",
                "elapsed": 30.0
            })
        except Exception as e:
            print(f"   ❌ エラー: {e}")
            results.append({
                "test": test_name,
                "status": "error",
                "error": str(e)
            })
    
    return results

def run_gui_playwright_tests(script_dir):
    """GUIのPlaywrightテストを実行（Cursorブラウザで）"""
    print(f"\n🎭 GUI Playwrightテストを実行中...")
    
    # gui-testsディレクトリのパス
    # script_dirはcodex-rsなので、その親ディレクトリがプロジェクトルート
    project_root = os.path.dirname(script_dir)
    gui_tests_dir = os.path.join(project_root, "gui-tests")
    
    if not os.path.exists(gui_tests_dir):
        print(f"   ⚠️  gui-testsディレクトリが見つかりません: {gui_tests_dir}")
        return {
            "test": "GUI Playwrightテスト",
            "status": "skipped",
            "elapsed": 0,
            "error": "gui-testsディレクトリが見つかりません"
        }
    
    # package.jsonの確認
    package_json = os.path.join(gui_tests_dir, "package.json")
    if not os.path.exists(package_json):
        print(f"   ⚠️  package.jsonが見つかりません: {package_json}")
        return {
            "test": "GUI Playwrightテスト",
            "status": "skipped",
            "elapsed": 0,
            "error": "package.jsonが見つかりません"
        }
    
    # npm installが必要か確認
    node_modules = os.path.join(gui_tests_dir, "node_modules")
    if not os.path.exists(node_modules):
        print(f"   📦 node_modulesが見つかりません。npm installを実行します...")
        try:
            install_result = subprocess.run(
                ["npm", "install"],
                cwd=gui_tests_dir,
                capture_output=True,
                text=True,
                timeout=300,
                encoding='utf-8',
                errors='replace'
            )
            if install_result.returncode != 0:
                print(f"   ❌ npm installに失敗しました")
                print(f"   エラー: {install_result.stderr[:500]}")
                return {
                    "test": "GUI Playwrightテスト",
                    "status": "failed",
                    "elapsed": 0,
                    "error": f"npm install失敗: {install_result.stderr[:200]}"
                }
            print(f"   ✅ npm install完了")
        except Exception as e:
            print(f"   ❌ npm install中にエラー: {e}")
            return {
                "test": "GUI Playwrightテスト",
                "status": "error",
                "elapsed": 0,
                "error": str(e)
            }
    
    # Playwrightテストを実行
    print(f"   🎭 Playwrightテストを実行中（Cursorブラウザ）...")
    try:
        start_time = time.time()
        
        # Playwrightテストを実行（headedモードでCursorブラウザを使用）
        test_result = subprocess.run(
            ["npx", "playwright", "test", "--project=chromium-cursor"],
            cwd=gui_tests_dir,
            capture_output=True,
            text=True,
            timeout=600,  # 10分のタイムアウト
            encoding='utf-8',
            errors='replace'
        )
        
        elapsed = time.time() - start_time
        
        if test_result.returncode == 0:
            print(f"   ✅ GUIテスト成功 (経過: {elapsed:.2f}秒)")
            
            # テスト結果のサマリーを抽出
            output_lines = test_result.stdout.split('\n')
            passed_count = 0
            failed_count = 0
            
            for line in output_lines:
                if 'passed' in line.lower() or '✓' in line:
                    passed_count += line.count('passed') + line.count('✓')
                if 'failed' in line.lower() or '✘' in line:
                    failed_count += line.count('failed') + line.count('✘')
            
            return {
                "test": "GUI Playwrightテスト",
                "status": "success",
                "elapsed": elapsed,
                "passed": passed_count,
                "failed": failed_count,
                "output": test_result.stdout[-500:] if len(test_result.stdout) > 500 else test_result.stdout
            }
        else:
            print(f"   ❌ GUIテスト失敗 (終了コード: {test_result.returncode}, 経過: {elapsed:.2f}秒)")
            print(f"   エラー出力: {test_result.stderr[:500]}")
            
            return {
                "test": "GUI Playwrightテスト",
                "status": "failed",
                "elapsed": elapsed,
                "error": test_result.stderr[:500] if test_result.stderr else "テスト失敗",
                "output": test_result.stdout[-500:] if len(test_result.stdout) > 500 else test_result.stdout
            }
            
    except subprocess.TimeoutExpired:
        print(f"   ⏱️  GUIテストタイムアウト（10分）")
        return {
            "test": "GUI Playwrightテスト",
            "status": "timeout",
            "elapsed": 600.0,
            "error": "テストが10分でタイムアウトしました"
        }
    except Exception as e:
        print(f"   ❌ GUIテスト実行中にエラー: {e}")
        return {
            "test": "GUI Playwrightテスト",
            "status": "error",
            "elapsed": 0,
            "error": str(e)
        }

def main():
    """メイン処理"""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    print("="*70)
    print("📥 バイナリインストール・実機テスト")
    print("="*70)
    
    # ビルド成果物の確認
    target_dir = os.environ.get('CARGO_TARGET_DIR', 'target')
    source_path = os.path.join(script_dir, target_dir, "release", "codex.exe")
    
    if not os.path.exists(source_path):
        print(f"\n❌ ビルド成果物が見つかりません: {source_path}")
        print("   先にビルドを完了してください。")
        sys.exit(1)
    
    file_size = os.path.getsize(source_path) / (1024 * 1024)  # MB
    print(f"\n📦 ビルド成果物: {source_path} ({file_size:.2f} MB)")
    
    # インストール
    print("\n" + "="*70)
    print("📥 Phase 1: バイナリ上書きインストール")
    print("="*70)
    
    install_path = os.path.join(os.environ.get('USERPROFILE', os.path.expanduser('~')), '.cargo', 'bin', 'codex.exe')
    
    install_success, installed_size = install_binary(source_path, install_path)
    
    if not install_success:
        print("\n❌ インストールに失敗しました")
        sys.exit(1)
    
    # 実機テスト（CLI）
    print("\n" + "="*70)
    print("🧪 Phase 2: 実機テスト (CLI)")
    print("="*70)
    
    test_results = test_binary("codex")
    
    # GUI Playwrightテスト
    print("\n" + "="*70)
    print("🎭 Phase 3: GUI Playwrightテスト (Cursorブラウザ)")
    print("="*70)
    
    gui_test_result = run_gui_playwright_tests(script_dir)
    test_results.append(gui_test_result)
    
    # 結果サマリー
    print("\n" + "="*70)
    print("📊 実行結果サマリー")
    print("="*70)
    
    print(f"\n✅ インストール: 成功")
    print(f"   - インストール先: {install_path}")
    print(f"   - ファイルサイズ: {installed_size:.2f} MB")
    
    print(f"\n🧪 テスト結果:")
    success_count = sum(1 for r in test_results if r['status'] == 'success')
    print(f"   - 成功: {success_count}/{len(test_results)}")
    for result in test_results:
        status_icon = "✅" if result['status'] == 'success' else "❌"
        print(f"   {status_icon} {result['test']}: {result['status']}")
    
    # 完了音声を再生（Windows環境）
    if sys.platform == 'win32':
        audio_paths = [
            r"C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav",
            r"C:\Users\downl\Desktop\新しいフォルダー (4)\marisa_owattaze.wav",
            os.path.join(os.path.dirname(os.path.dirname(script_dir)), ".codex", "marisa_owattaze.wav")
        ]
        
        audio_played = False
        for audio_path in audio_paths:
            if os.path.exists(audio_path):
                try:
                    import winsound
                    print(f"\n🔊 完了音声を再生中: {audio_path}")
                    winsound.PlaySound(audio_path, winsound.SND_FILENAME | winsound.SND_SYNC)
                    print("✅ 音声を再生しました: 終わったぜ！")
                    audio_played = True
                    break
                except Exception as e:
                    print(f"⚠️  音声ファイルの再生に失敗しました: {e}")
                    continue
        
        if not audio_played:
            # フォールバック: ピープ音を再生
            try:
                import winsound
                print(f"\n🔊 ピープ音を再生中...")
                winsound.Beep(1000, 500)
                print("✅ ピープ音を再生しました")
            except Exception as e:
                print(f"⚠️  音声の再生に失敗しました: {e}")
    
    # 全て成功した場合のみ終了コード0
    if install_success and success_count == len(test_results):
        print("\n🎉 全ての処理が正常に完了しました！")
        sys.exit(0)
    else:
        print("\n⚠️  一部の処理で問題が発生しました")
        sys.exit(1)

if __name__ == "__main__":
    main()
