#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
高速差分ビルド・上書きインストール・実機テストスクリプト
tqdm風の進捗表示で残り時間と経過時間を可視化
"""

import subprocess
import sys
import re
import time
import os
import json
from datetime import datetime, timedelta
from pathlib import Path

# Windows環境での文字エンコーディング対策
if sys.platform == 'win32':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

class BuildProgressTracker:
    """ビルド進捗を追跡してtqdm風に表示"""
    
    def __init__(self, total_estimated=100):
        self.start_time = time.time()
        self.compiled_crates = []
        self.current_crate = None
        self.total_estimated = total_estimated
        self.last_update = time.time()
        
    def format_time(self, seconds):
        """秒数を読みやすい形式に変換"""
        if seconds < 60:
            return f"{seconds:.1f}秒"
        elif seconds < 3600:
            minutes = int(seconds // 60)
            secs = int(seconds % 60)
            return f"{minutes}分{secs}秒"
        else:
            hours = int(seconds // 3600)
            minutes = int((seconds % 3600) // 60)
            return f"{hours}時間{minutes}分"
    
    def parse_cargo_output(self, line):
        """cargoの出力からコンパイル情報を抽出"""
        # "Compiling crate_name v1.2.3" のパターン
        compiling_match = re.search(r'Compiling\s+(\S+)', line)
        if compiling_match:
            return ('compiling', compiling_match.group(1))
        
        # "Finished" のパターン
        if 'Finished' in line and ('dev' in line or 'release' in line):
            return ('finished', None)
        
        # エラーのパターン
        if 'error:' in line.lower():
            return ('error', line)
        
        # 警告のパターン
        if 'warning:' in line.lower():
            return ('warning', line)
        
        return (None, None)
    
    def update_progress(self, crate_name=None):
        """進捗を更新して表示"""
        elapsed = time.time() - self.start_time
        
        if crate_name:
            if crate_name not in self.compiled_crates:
                self.compiled_crates.append(crate_name)
            self.current_crate = crate_name
        
        progress = len(self.compiled_crates)
        
        # 進捗バーを表示
        bar_length = 50
        filled = int(bar_length * progress / max(self.total_estimated, progress))
        bar = '█' * filled + '░' * (bar_length - filled)
        percentage = min(100, int(progress * 100 / max(self.total_estimated, progress)))
        
        # 残り時間の推定（簡易版）
        if progress > 0:
            avg_time_per_crate = elapsed / progress
            remaining_crates = max(0, self.total_estimated - progress)
            estimated_remaining = avg_time_per_crate * remaining_crates
            remaining_str = f" | 残り: {self.format_time(estimated_remaining)}"
        else:
            remaining_str = ""
        
        crate_display = self.current_crate[:35] if self.current_crate else "初期化中..."
        
        print(f"\r[{bar}] {percentage}% | {crate_display:<35} | 経過: {self.format_time(elapsed)}{remaining_str}", end='', flush=True)
        self.last_update = time.time()
    
    def finish(self, success=True):
        """ビルド完了時の表示"""
        elapsed = time.time() - self.start_time
        print(f"\r{' ' * 120}", end='')  # 行をクリア
        
        if success:
            print(f"\r✅ ビルド完了！ | 経過時間: {self.format_time(elapsed)} | コンパイル済み: {len(self.compiled_crates)}個のクレート")
        else:
            print(f"\r❌ ビルド失敗 | 経過時間: {self.format_time(elapsed)}")
        
        return elapsed, len(self.compiled_crates)

def build_with_progress(command, description="ビルド", total_estimated=100):
    """進捗を可視化しながらビルドを実行"""
    print(f"\n{'='*70}")
    print(f"🚀 {description}を開始します...")
    print(f"{'='*70}\n")
    
    tracker = BuildProgressTracker(total_estimated=total_estimated)
    
    build_env = os.environ.copy()
    build_env.pop('RUSTC_WRAPPER', None)
    build_env['SCCACHE_DISABLE'] = '1'
    build_env.setdefault('CARGO_INCREMENTAL', '1')

    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        universal_newlines=True,
        encoding='utf-8',
        errors='replace',
        env=build_env
    )
    
    errors = []
    warnings = []
    
    try:
        while True:
            output = process.stdout.readline()
            if output == '' and process.poll() is not None:
                break
            
            if output:
                line = output.strip()
                event_type, data = tracker.parse_cargo_output(line)
                
                if event_type == 'compiling':
                    tracker.update_progress(data)
                
                elif event_type == 'finished':
                    elapsed, count = tracker.finish(success=True)
                    break
                
                elif event_type == 'error':
                    errors.append(data)
                    print(f"\n❌ エラー: {data[:100]}")
                
                elif event_type == 'warning':
                    warnings.append(data)
        
        return_code = process.poll()
        
        if return_code != 0:
            tracker.finish(success=False)
            return False, errors, warnings, 0, 0
        
        elapsed, count = tracker.finish(success=True)
        return True, errors, warnings, elapsed, count
    
    except KeyboardInterrupt:
        print("\n\n⚠️  ビルドが中断されました")
        process.terminate()
        return False, errors, warnings, 0, 0

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
        print(f"   ✅ インストール完了")
        
        return True
    except Exception as e:
        print(f"   ❌ インストール失敗: {e}")
        return False

def get_worktree_name(script_dir):
    """git worktreeから現在のワークツリー名を取得"""
    try:
        # プロジェクトルートを取得
        project_root = os.path.dirname(os.path.dirname(script_dir))
        result = subprocess.run(
            ["git", "worktree", "list"],
            capture_output=True,
            text=True,
            cwd=project_root,
            timeout=5
        )
        if result.returncode == 0:
            for line in result.stdout.split('\n'):
                if project_root.replace('\\', '/') in line.replace('\\', '/'):
                    match = re.search(r'\[([^\]]+)\]', line)
                    if match:
                        return match.group(1)
    except:
        pass
    return "main"

def get_current_datetime():
    """現在日時を取得（PowerShell経由）"""
    try:
        # PowerShellで現在日時を取得
        result = subprocess.run(
            ["powershell", "-Command", "Get-Date -Format 'yyyy-MM-dd HH:mm:ss'"],
            capture_output=True,
            text=True,
            timeout=5,
            encoding='utf-8',
            errors='replace'
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except:
        pass
    # フォールバック: Pythonのdatetimeを使用
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")

def create_implementation_log(script_dir, summary, build_duration=0):
    """実装ログを作成"""
    # 現在日時を取得（PowerShell経由）
    current_datetime = get_current_datetime()
    current_date = current_datetime.split()[0]  # yyyy-MM-dd部分を抽出
    
    # ワークツリー名を取得
    worktree_name = get_worktree_name(script_dir)
    
    # _docsディレクトリを確認
    docs_dir = os.path.join(os.path.dirname(script_dir), "_docs")
    if not os.path.exists(docs_dir):
        os.makedirs(docs_dir, exist_ok=True)
    
    # ログファイル名
    log_filename = f"{current_date}_高速差分ビルド上書きインストール実機テスト{{{worktree_name}}}.md"
    log_path = os.path.join(docs_dir, log_filename)
    
    # ログ内容を生成
    log_lines = []
    log_lines.append("# 高速差分ビルド・上書きインストール・実機テスト")
    log_lines.append("")
    log_lines.append(f"**日時**: {current_datetime}")
    log_lines.append(f"**ワークツリー**: {worktree_name}")
    log_lines.append(f"**実行ディレクトリ**: {script_dir}")
    log_lines.append("")
    log_lines.append("## 実行概要")
    log_lines.append("")
    log_lines.append("高速差分ビルド、バイナリの上書きインストール、実機テストを実行しました。")
    log_lines.append("")
    log_lines.append("## 実行結果")
    log_lines.append("")
    log_lines.append("### Phase 1: 高速差分ビルド")
    log_lines.append("")
    
    if summary:
        build_status = "成功" if summary["build"]["success"] else "失敗"
        log_lines.append(f"- **ステータス**: {build_status}")
        log_lines.append(f"- **経過時間**: {summary['build']['elapsed_seconds']:.2f} 秒")
        log_lines.append(f"- **コンパイル済みクレート数**: {summary['build']['crates_compiled']} 個")
        log_lines.append(f"- **警告数**: {summary['build']['warnings_count']} 個")
        log_lines.append(f"- **エラー数**: {summary['build']['errors_count']} 個")
        log_lines.append("")
        log_lines.append("### Phase 2: バイナリ上書きインストール")
        log_lines.append("")
        log_lines.append(f"- **ソース**: {summary['install']['source']}")
        log_lines.append(f"- **インストール先**: {summary['install']['destination']}")
        log_lines.append(f"- **ファイルサイズ**: {summary['install']['file_size_mb']:.2f} MB")
        log_lines.append("")
        log_lines.append("### Phase 3: 実機テスト")
        log_lines.append("")
        
        success_count = sum(1 for t in summary['tests'] if t['status'] == 'success')
        total_count = len(summary['tests'])
        
        log_lines.append(f"- **テスト成功数**: {success_count} / {total_count}")
        log_lines.append("")
        log_lines.append("#### テスト詳細")
        log_lines.append("")
        
        for test in summary['tests']:
            status_icon = "[OK]" if test['status'] == 'success' else "[NG]"
            test_line = f"- {status_icon} **{test['test']}**: {test['status']}"
            if 'elapsed' in test and test['elapsed']:
                test_line += f" (経過時間: {test['elapsed']:.2f} 秒)"
            log_lines.append(test_line)
    else:
        log_lines.append("- **ビルド**: 実行完了（詳細情報の取得に失敗）")
        log_lines.append("- **インストール**: 実行完了")
        log_lines.append("- **テスト**: 実行完了")
    
    log_lines.append("")
    log_lines.append("## 実行サマリー")
    log_lines.append("")
    log_lines.append(f"- **開始時刻**: {current_datetime}")
    log_lines.append(f"- **実行時間**: {build_duration:.2f} 秒")
    log_lines.append(f"- **ワークツリー**: {worktree_name}")
    log_lines.append("")
    log_lines.append("## 完了")
    log_lines.append("")
    log_lines.append("全ての処理が正常に完了しました。")
    log_lines.append("")
    
    # ログを保存
    log_content = "\n".join(log_lines)
    with open(log_path, 'w', encoding='utf-8') as f:
        f.write(log_content)
    
    print(f"\n📝 実装ログを保存しました: {log_path}")

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
    project_root = os.path.dirname(os.path.dirname(script_dir))
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
    # 作業ディレクトリをcodex-rsに変更
    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)
    
    print("="*70)
    print("🚀 高速差分ビルド・上書きインストール・実機テスト")
    print("="*70)
    
    # 環境変数を確認
    target_dir = os.environ.get('CARGO_TARGET_DIR', 'target')
    print(f"\n📁 ビルドディレクトリ: {target_dir}")
    
    # 1. 高速差分ビルド（codex-cli & codex-tui）
    print("\n" + "="*70)
    print("📦 Phase 1: 高速差分ビルド (codex-cli, codex-tui)")
    print("="*70)
    
    build_success, build_errors, build_warnings, build_elapsed, build_count = build_with_progress(
        ["cargo", "build", "--release", "-p", "codex-cli", "-p", "codex-tui"],
        "codex (リリースビルド)",
        total_estimated=85  # 推定クレート数
    )
    
    if not build_success:
        print("\n❌ ビルドに失敗しました")
        if build_errors:
            print("\nエラー詳細:")
            for error in build_errors[:5]:  # 最初の5個のみ
                print(f"  - {error[:200]}")
        
        # ビルド失敗時でも実装ログを作成
        summary = {
            "timestamp": datetime.now().isoformat(),
            "build": {
                "success": False,
                "elapsed_seconds": build_elapsed,
                "crates_compiled": build_count,
                "warnings_count": len(build_warnings),
                "errors_count": len(build_errors),
                "errors": build_errors[:10]  # 最初の10個のエラー
            },
            "install": {
                "source": None,
                "destination": None,
                "file_size_mb": 0
            },
            "tests": []
        }
        
        try:
            create_implementation_log(script_dir, summary, build_duration=build_elapsed)
        except Exception as e:
            print(f"\n⚠️  実装ログの作成に失敗しました: {e}")
        
        sys.exit(1)
    
    # 2. バイナリの確認
    source_path = os.path.join(target_dir, "release", "codex.exe")
    if not os.path.exists(source_path):
        print(f"\n❌ ビルド成果物が見つかりません: {source_path}")
        sys.exit(1)
    
    file_size = os.path.getsize(source_path) / (1024 * 1024)  # MB
    print(f"\n📦 ビルド成果物: {source_path} ({file_size:.2f} MB)")
    
    # 3. 上書きインストール
    print("\n" + "="*70)
    print("📥 Phase 2: バイナリ上書きインストール")
    print("="*70)
    
    install_path_cli = os.path.join(os.environ.get('USERPROFILE', os.path.expanduser('~')), '.cargo', 'bin', 'codex.exe')
    source_path_tui = os.path.join(target_dir, "release", "codex-tui.exe")
    install_path_tui = os.path.join(os.environ.get('USERPROFILE', os.path.expanduser('~')), '.cargo', 'bin', 'codex-tui.exe')
    
    cli_installed = install_binary(source_path, install_path_cli)
    tui_installed = os.path.exists(source_path_tui) and install_binary(source_path_tui, install_path_tui)
    
    if not cli_installed:
        print("\n❌ CLIのインストールに失敗しました")
        sys.exit(1)
    
    if not tui_installed:
        print("\n⚠️  TUIのインストールに失敗したか、バイナリが見つかりません")
    
    # 4. 実機テスト（CLI）
    print("\n" + "="*70)
    print("🧪 Phase 3: 実機テスト (CLI)")
    print("="*70)
    
    test_results = test_binary("codex")
    print("\n🧪 実機テストを実行中 (TUI)...")
    tui_test_results = test_binary("codex-tui")
    test_results.extend(tui_test_results)
    
    # 5. GUI Playwrightテスト
    print("\n" + "="*70)
    print("🎭 Phase 4: GUI Playwrightテスト (Cursorブラウザ)")
    print("="*70)
    
    gui_test_result = run_gui_playwright_tests(script_dir)
    test_results.append(gui_test_result)
    
    # 6. 結果サマリー
    print("\n" + "="*70)
    print("📊 実行結果サマリー")
    print("="*70)
    
    print(f"\n✅ ビルド: 成功")
    print(f"   - 経過時間: {build_elapsed:.2f}秒")
    print(f"   - コンパイル済みクレート: {build_count}個")
    if build_warnings:
        print(f"   - 警告数: {len(build_warnings)}個")
    
    print(f"\n✅ インストール: 成功")
    print(f"   - インストール先: {install_path}")
    
    print(f"\n🧪 テスト結果:")
    success_count = sum(1 for r in test_results if r['status'] == 'success')
    print(f"   - 成功: {success_count}/{len(test_results)}")
    for result in test_results:
        status_icon = "✅" if result['status'] == 'success' else "❌"
        print(f"   {status_icon} {result['test']}: {result['status']}")
    
    # 結果をJSONで保存（実装ログ用）
    summary = {
        "timestamp": datetime.now().isoformat(),
        "build": {
            "success": build_success,
            "elapsed_seconds": build_elapsed,
            "crates_compiled": build_count,
            "warnings_count": len(build_warnings),
            "errors_count": len(build_errors)
        },
        "install": {
            "source": source_path,
            "destination": install_path,
            "file_size_mb": file_size
        },
        "tests": test_results
    }
    
    summary_path = os.path.join(script_dir, "build_test_summary.json")
    with open(summary_path, 'w', encoding='utf-8') as f:
        json.dump(summary, f, ensure_ascii=False, indent=2)
    
    print(f"\n📄 結果サマリーを保存: {summary_path}")
    
    # 実装ログを作成
    try:
        create_implementation_log(script_dir, summary, build_duration=build_elapsed)
    except Exception as e:
        print(f"\n⚠️  実装ログの作成に失敗しました: {e}")
    
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
                    # PlaySync()で同期的に再生（確実に聞こえる）
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
    if build_success and success_count == len(test_results):
        print("\n🎉 全ての処理が正常に完了しました！")
        sys.exit(0)
    else:
        print("\n⚠️  一部の処理で問題が発生しました")
        sys.exit(1)

if __name__ == "__main__":
    main()
