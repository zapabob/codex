#!/usr/bin/env python3
"""
Rustビルド進捗可視化モニター
"""

import time
import os
import sys
from pathlib import Path
import subprocess
import threading
import queue

class BuildProgressMonitor:
    def __init__(self, project_root):
        self.project_root = Path(project_root)
        self.codex_rs_path = self.project_root / "codex-rs"
        self.target_dir = self.codex_rs_path / "target" / "release"
        self.start_time = time.time()
        self.last_update = self.start_time
        self.total_compiled = 0
        self.current_package = ""
        self.build_log = []

    def get_build_progress(self):
        """ビルド進捗を取得"""
        progress = {
            'elapsed_time': time.time() - self.start_time,
            'total_compiled': self.total_compiled,
            'current_package': self.current_package,
            'target_files': self.count_target_files(),
            'build_rate': self.calculate_build_rate(),
            'estimated_completion': self.estimate_completion_time()
        }
        return progress

    def count_target_files(self):
        """targetディレクトリのファイル数をカウント"""
        try:
            if self.target_dir.exists():
                return sum(1 for _ in self.target_dir.rglob('*') if _.is_file())
            return 0
        except:
            return 0

    def calculate_build_rate(self):
        """ビルドレートを計算（ファイル/分）"""
        elapsed = time.time() - self.start_time
        if elapsed > 0:
            return self.count_target_files() / (elapsed / 60)
        return 0

    def estimate_completion_time(self):
        """完了予定時間を推定"""
        # 典型的なRustビルド時間に基づく推定
        # Codexプロジェクトの平均ビルド時間: 約25-40分
        elapsed = time.time() - self.start_time
        if elapsed < 300:  # 5分未満
            return "推定中..."
        elif elapsed < 1200:  # 20分未満
            return "~30分"
        elif elapsed < 1800:  # 30分未満
            return "~15-20分"
        else:
            return "間もなく完了"

    def monitor_build_process(self):
        """ビルドプロセスを監視"""
        try:
            # cargo buildプロセスを実行
            cmd = ["cargo", "build", "--release", "--all-features"]
            process = subprocess.Popen(
                cmd,
                cwd=str(self.codex_rs_path),
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                universal_newlines=True,
                bufsize=1
            )

            print("Starting Rust incremental build...")
            print("=" * 60)

            while True:
                output = process.stdout.readline()
                if output == '' and process.poll() is not None:
                    break
                if output:
                    self.parse_build_output(output.strip())
                    self.display_progress()

            return_code = process.poll()
            if return_code == 0:
                print("\n" + "=" * 60)
                print("Build successful!")
                print(".2f")
                print("Installing binary with overwrite...")
                return True
            else:
                print(f"\nBuild failed (exit code: {return_code})")
                return False

        except Exception as e:
            print(f"\nBuild monitor error: {e}")
            return False

    def parse_build_output(self, line):
        """ビルド出力を解析"""
        self.build_log.append(line)
        self.last_update = time.time()

        if "Compiling" in line:
            parts = line.split()
            if len(parts) >= 2:
                self.current_package = parts[1]
                self.total_compiled += 1

    def display_progress(self):
        """進捗を表示"""
        progress = self.get_build_progress()

        # プログレスバーのような表示
        elapsed_str = ".0f"
        compiled = progress['total_compiled']
        current = progress['current_package']
        files = progress['target_files']
        rate = ".1f"
        estimate = progress['estimated_completion']

        # ANSIエスケープシーケンスでプログレス表示
        progress_line = f"\rCompiling: {compiled} packages | Current: {current} | Files: {files} | Rate: {rate}/min | ETA: {estimate} | Elapsed: {elapsed_str}"

        # 行をクリアして再表示
        sys.stdout.write('\r' + ' ' * 120 + '\r')
        sys.stdout.write(progress_line)
        sys.stdout.flush()

def main():
    """メイン関数"""
    if len(sys.argv) > 1:
        project_root = sys.argv[1]
    else:
        project_root = Path(__file__).parent

    monitor = BuildProgressMonitor(project_root)

    # 初期状態表示
    print("Build preparation complete")
    print(f"Project: {project_root}")
    print(f"Target: {monitor.codex_rs_path}")
    print(f"Output: {monitor.target_dir}")

    # ビルド実行
    success = monitor.monitor_build_process()

    if success:
        # バイナリインストール
        print("\nInstalling binary...")

        try:
            # cargo install実行
            result = subprocess.run(
                ["cargo", "install", "--path", "cli", "--force"],
                cwd=str(monitor.codex_rs_path),
                capture_output=True,
                text=True
            )

            if result.returncode == 0:
                print("Binary installation completed!")
                print("Codex v2.10.1 is now available")

                # バージョン確認
                version_result = subprocess.run(
                    ["codex", "--version"],
                    capture_output=True,
                    text=True
                )

                if version_result.returncode == 0:
                    print(f"Version: {version_result.stdout.strip()}")
                else:
                    print("Version check failed, but installation succeeded")

            else:
                print("Installation failed:")
                print(result.stderr)

        except Exception as e:
            print(f"Installation error: {e}")

    print("\n" + "=" * 60)
    print("Build process completed")

if __name__ == "__main__":
    main()