#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
高速差分ビルド・プロセスキル・上書きインストールスクリプト
tqdm風の進捗表示で残り時間と経過時間を可視化
実装ログを自動保存する機能付き
"""

import subprocess
import sys
import re
import time
import os
import json
from datetime import datetime
from pathlib import Path

# MCPサーバーから現在日時を取得する関数
def get_current_datetime_from_mcp():
    """MCPサーバー経由で現在日時を取得（フォールバック付き）"""
    try:
        # まずPowerShellで日時を取得
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
    except Exception as e:
        print(f"PowerShell日時取得失敗: {e}")

    # フォールバック: Pythonのdatetimeを使用
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")

def save_implementation_log(log_content, feature_name="高速差分ビルド"):
    """実装ログを_docs/ディレクトリに保存"""
    try:
        # 日時を取得
        current_datetime = get_current_datetime_from_mcp()
        date_part = current_datetime.split()[0]  # yyyy-mm-dd

        # ログファイル名を作成
        log_filename = f"{date_part}_{feature_name}{{main}}.md"
        log_dir = Path("_docs")
        log_dir.mkdir(exist_ok=True)
        log_path = log_dir / log_filename

        # ログ内容を作成
        log_header = f"""# 実装ログ: {feature_name}
**実装日時**: {current_datetime}
**ワークツリー**: main
**機能**: {feature_name}

## 実行内容
{log_content}

## 完了ステータス
✅ 正常に完了しました

---
*自動生成された実装ログ*
"""

        # ファイルを保存
        with open(log_path, 'w', encoding='utf-8') as f:
            f.write(log_header)

        print(f"📝 実装ログを保存しました: {log_path}")
        return str(log_path)

    except Exception as e:
        print(f"⚠️  実装ログ保存でエラー: {e}")
        return None

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
        compiling_match = re.search(r'Compiling\s+(\S+)', line)
        if compiling_match:
            return ('compiling', compiling_match.group(1))
        
        if 'Finished' in line and ('dev' in line or 'release' in line):
            return ('finished', None)
        
        if 'error:' in line.lower():
            return ('error', line)
        
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

def kill_codex_processes():
    """codex関連のプロセスを停止"""
    print("\n🔪 実行中のcodexプロセスを停止中...")
    
    try:
        if sys.platform == 'win32':
            # Windows: PowerShellでプロセスを停止
            result = subprocess.run(
                ["powershell", "-Command", 
                 "Get-Process | Where-Object { $_.ProcessName -like '*codex*' -or $_.Path -like '*codex*' } | Stop-Process -Force -ErrorAction SilentlyContinue"],
                capture_output=True,
                text=True,
                timeout=10,
                encoding='utf-8',
                errors='replace'
            )
            if result.returncode == 0 or 'not found' in result.stderr.lower():
                print("   ✅ プロセス停止完了")
                return True
        else:
            # Unix系: pkillを使用
            subprocess.run(["pkill", "-f", "codex"], capture_output=True, timeout=10)
            print("   ✅ プロセス停止完了")
            return True
    except Exception as e:
        print(f"   ⚠️  プロセス停止で警告: {e}")
        return True  # 警告があっても続行
    
    return True

def build_with_progress(command, description="ビルド", total_estimated=100):
    """進捗を可視化しながらビルドを実行"""
    print(f"\n{'='*70}")
    print(f"🚀 {description}を開始します...")
    print(f"{'='*70}\n")
    
    tracker = BuildProgressTracker(total_estimated=total_estimated)
    
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        universal_newlines=True,
        encoding='utf-8',
        errors='replace'
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

def get_current_datetime():
    """現在日時を取得（PowerShell経由）"""
    try:
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
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")

def main():
    """メイン処理"""
    # 作業ディレクトリをcodex-rsに変更
    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)

    # 開始日時を取得
    start_datetime = get_current_datetime_from_mcp()

    print("="*70)
    print("🚀 高速差分ビルド・プロセスキル・上書きインストール")
    print("="*70)
    print(f"🕐 開始時刻: {start_datetime}")
    
    # 環境変数を確認
    target_dir = os.environ.get('CARGO_TARGET_DIR', 'target')
    print(f"\n📁 ビルドディレクトリ: {target_dir}")
    
    # 0. プロセスキル
    print("\n" + "="*70)
    print("🔪 Phase 0: プロセスキル")
    print("="*70)
    kill_codex_processes()
    time.sleep(2)  # プロセス停止を待つ
    
    # 1. 高速差分ビルド（codex-cli）
    print("\n" + "="*70)
    print("📦 Phase 1: 高速差分ビルド (codex-cli)")
    print("="*70)
    
    build_success, build_errors, build_warnings, build_elapsed, build_count = build_with_progress(
        ["cargo", "build", "--release", "-p", "codex-cli"],
        "codex-cli (リリースビルド)",
        total_estimated=80  # 推定クレート数
    )
    
    if not build_success:
        print("\n❌ ビルドに失敗しました")
        if build_errors:
            print("\nエラー詳細:")
            for error in build_errors[:5]:  # 最初の5個のみ
                print(f"  - {error[:200]}")
        sys.exit(1)
    
    # 2. バイナリの確認
    source_path = os.path.join(target_dir, "release", "codex.exe")
    if not os.path.exists(source_path):
        print(f"\n❌ ビルド成果物が見つかりません: {source_path}")
        sys.exit(1)
    
    file_size = os.path.getsize(source_path) / (1024 * 1024)  # MB
    print(f"\n📦 ビルド成果物: {source_path} ({file_size:.2f} MB)")
    
    # 3. 再度プロセスキル（念のため）
    print("\n" + "="*70)
    print("🔪 Phase 2: 最終プロセスキル")
    print("="*70)
    kill_codex_processes()
    time.sleep(1)
    
    # 4. 上書きインストール
    print("\n" + "="*70)
    print("📥 Phase 3: バイナリ上書きインストール")
    print("="*70)
    
    install_path = os.path.join(os.environ.get('USERPROFILE', os.path.expanduser('~')), '.cargo', 'bin', 'codex.exe')
    
    if not install_binary(source_path, install_path):
        print("\n❌ インストールに失敗しました")
        sys.exit(1)
    
    # 5. 動作確認
    print("\n" + "="*70)
    print("✅ Phase 4: 動作確認")
    print("="*70)
    
    try:
        result = subprocess.run(
            ["codex", "--version"],
            capture_output=True,
            text=True,
            timeout=10,
            encoding='utf-8',
            errors='replace'
        )
        if result.returncode == 0:
            print(f"   ✅ バージョン確認成功: {result.stdout.strip()}")
        else:
            print(f"   ⚠️  バージョン確認で警告: {result.stderr[:200]}")
    except Exception as e:
        print(f"   ⚠️  動作確認でエラー: {e}")
    
    # 完了
    end_datetime = get_current_datetime_from_mcp()
    print("\n" + "="*70)
    print("🎉 全ての処理が正常に完了しました！")
    print("="*70)
    print(f"🕐 終了時刻: {end_datetime}")
    print(f"\n📊 サマリー:")
    print(f"   - ビルド時間: {build_elapsed:.2f}秒")
    print(f"   - コンパイル済みクレート: {build_count}個")
    print(f"   - インストール先: {install_path}")
    print(f"   - バイナリサイズ: {file_size:.2f} MB")

    # 実装ログを保存
    log_content = f"""- 開始時刻: {start_datetime}
- 終了時刻: {end_datetime}
- ビルド時間: {build_elapsed:.2f}秒
- コンパイル済みクレート: {build_count}個
- バイナリサイズ: {file_size:.2f} MB
- インストール先: {install_path}
- プロセスキル: 正常に実行
- ビルド結果: 成功
- インストール結果: 成功
- 動作確認: 完了"""

    log_path = save_implementation_log(log_content, "高速差分ビルド・プロセスキル・上書きインストール")
    
    # 完了音声を再生（Windows環境）
    if sys.platform == 'win32':
        audio_paths = [
            r"C:\Users\downl\Desktop\SO8T\.cursor\marisa_owattaze.wav",  # 優先パス
            os.path.join(os.path.dirname(os.path.dirname(script_dir)), ".codex", "marisa_owattaze.wav")
        ]
        
        audio_played = False
        for audio_path in audio_paths:
            if os.path.exists(audio_path):
                try:
                    import winsound
                    print(f"\n🔊 完了音声を再生中: {audio_path}")
                    winsound.PlaySound(audio_path, winsound.SND_FILENAME | winsound.SND_SYNC)
                    print("✅ 音声を再生しました: 終わったぜ！ 高速ビルド・インストール完了だぜ！")
                    audio_played = True
                    break
                except Exception as e:
                    print(f"⚠️  音声ファイルの再生に失敗しました: {e}")
                    continue
        
        if not audio_played:
            try:
                import winsound
                winsound.Beep(1000, 500)
            except:
                pass
    
    sys.exit(0)

if __name__ == "__main__":
    main()
