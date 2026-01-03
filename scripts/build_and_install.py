#!/usr/bin/env python3
"""
Codex 高速ビルド＆インストールシステム
ビルドからインストールまでを完全自動化
"""

import os
import sys
import subprocess
import time
import platform
from pathlib import Path
from typing import Optional, Dict, Any
import json

try:
    from tqdm import tqdm
    import psutil
except ImportError:
    print("必要なパッケージをインストール中...")
    subprocess.run([sys.executable, "-m", "pip", "install", "tqdm", "psutil"], check=True)
    from tqdm import tqdm
    import psutil

class BuildAndInstallSystem:
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.codex_rs_dir = project_root / "codex-rs"
        self.scripts_dir = project_root / "scripts"
        self.is_windows = platform.system() == "Windows"

        # デフォルトインストールパス
        if self.is_windows:
            self.default_install_path = Path("C:/bin/codex.exe")
        else:
            self.default_install_path = Path("/usr/local/bin/codex")

    def run_command_with_progress(self, cmd: list, description: str, timeout: int = 300) -> bool:
        """コマンドを実行し、進捗を表示"""
        print(f"🔧 {description}")
        print(f"💻 コマンド: {' '.join(cmd)}")

        start_time = time.time()
        try:
            # プログレスバー付きで実行
            with tqdm(total=100, desc=f"⚙️  {description}", unit="%") as pbar:
                process = subprocess.Popen(
                    cmd,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    cwd=self.codex_rs_dir
                )

                # 進捗をシミュレートしながら待機
                while process.poll() is None:
                    time.sleep(0.1)
                    # 進捗を少しずつ進める（実際のビルド進捗はわからないので）
                    current_progress = min(95, pbar.n + 1)
                    pbar.update(current_progress - pbar.n)

                # 完了
                pbar.update(100 - pbar.n)

                stdout, stderr = process.communicate(timeout=timeout)

                if process.returncode == 0:
                    elapsed = time.time() - start_time
                    print(".1f"                    return True
                else:
                    print(f"[ERROR] コマンド失敗 ({time.time() - start_time:.1f}s)")
                    if stderr:
                        print("エラー出力:")
                        print(stderr)
                    return False

        except subprocess.TimeoutExpired:
            print(f"[TIMEOUT] コマンドがタイムアウトしました ({timeout}s)")
            return False
        except Exception as e:
            print(f"[CRASH] コマンド実行エラー: {e}")
            return False

    def build_release(self) -> bool:
        """リリースビルドを実行"""
        cmd = [
            "cargo", "build", "--release",
            "--target", self.get_target_triple(),
            "-p", "codex-cli"
        ]

        return self.run_command_with_progress(
            cmd,
            "リリースビルド実行中",
            timeout=1800  # 30分
        )

    def get_target_triple(self) -> str:
        """現在のプラットフォームのターゲットトリプルを取得"""
        system = platform.system().lower()
        machine = platform.machine().lower()

        if system == "windows":
            return "x86_64-pc-windows-msvc"
        elif system == "linux":
            return f"{machine}-unknown-linux-gnu"
        elif system == "darwin":
            return f"{machine}-apple-darwin"
        else:
            return f"{machine}-unknown-{system}"

    def get_binary_path(self) -> Path:
        """ビルドされたバイナリのパスを取得"""
        target_dir = self.codex_rs_dir / "target" / self.get_target_triple() / "release"
        binary_name = "codex.exe" if self.is_windows else "codex"
        return target_dir / binary_name

    def kill_existing_processes(self) -> bool:
        """既存のCodexプロセスを終了"""
        print("[SEARCH] 既存プロセスを検索中...")

        killed_processes = []

        try:
            for proc in psutil.process_iter(['pid', 'name', 'cmdline']):
                try:
                    # codexプロセスを検出
                    if 'codex' in proc.info['name'].lower():
                        # 自分自身は除外
                        if proc.pid != os.getpid():
                            print(f"[STOP] プロセス終了: PID {proc.pid} ({proc.info['name']})")
                            proc.kill()
                            killed_processes.append(proc.pid)
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    continue

            if killed_processes:
                print(f"[OK] {len(killed_processes)} 個のプロセスを終了しました")
                # 完全に終了するのを待つ
                time.sleep(2)
                return True
            else:
                print("[OK] 実行中のプロセスはありません")
                return True

        except Exception as e:
            print(f"[WARN] プロセス終了エラー: {e}")
            return False

    def install_binary(self, install_path: Optional[Path] = None) -> bool:
        """バイナリをインストール"""
        if install_path is None:
            install_path = self.default_install_path

        binary_path = self.get_binary_path()

        if not binary_path.exists():
            print(f"[ERROR] バイナリが見つかりません: {binary_path}")
            return False

        print(f"[PKG] バイナリをインストール: {binary_path} -> {install_path}")

        try:
            # インストールディレクトリ作成
            install_path.parent.mkdir(parents=True, exist_ok=True)

            # Windows PowerShellスクリプト使用
            if self.is_windows:
                script_path = self.scripts_dir / "install_with_kill.ps1"
                if script_path.exists():
                    cmd = [
                        "powershell", "-ExecutionPolicy", "Bypass",
                        "-File", str(script_path),
                        "-SourcePath", str(binary_path),
                        "-TargetPath", str(install_path),
                        "-Force"
                    ]
                    return self.run_command_with_progress(cmd, "PowerShellインストール実行中")
                else:
                    # 直接コピー
                    import shutil
                    shutil.copy2(binary_path, install_path)
                    print(f"[OK] 直接コピー完了: {install_path}")
                    return True
            else:
                # Unix系システム
                import shutil
                shutil.copy2(binary_path, install_path)
                # 実行権限付与
                os.chmod(install_path, 0o755)
                print(f"[OK] インストール完了: {install_path}")
                return True

        except Exception as e:
            print(f"[ERROR] インストールエラー: {e}")
            return False

    def verify_installation(self, install_path: Optional[Path] = None) -> bool:
        """インストールを検証"""
        if install_path is None:
            install_path = self.default_install_path

        print(f"[SEARCH] インストール検証: {install_path}")

        if not install_path.exists():
            print("[ERROR] バイナリファイルが見つかりません")
            return False

        try:
            # バージョン確認
            result = subprocess.run(
                [str(install_path), "--version"],
                capture_output=True,
                text=True,
                timeout=10
            )

            if result.returncode == 0:
                version = result.stdout.strip()
                print(f"[OK] インストール成功! バージョン: {version}")
                return True
            else:
                print("[ERROR] バージョン確認失敗")
                if result.stderr:
                    print(f"エラー: {result.stderr}")
                return False

        except Exception as e:
            print(f"[ERROR] 検証エラー: {e}")
            return False

    def full_build_and_install(self, install_path: Optional[Path] = None) -> bool:
        """完全なビルド＆インストールを実行"""
        print("[START] Codex 高速ビルド＆インストールシステム開始")
        print(f"[DIR] プロジェクト: {self.project_root}")
        print(f"[TARGET] プラットフォーム: {platform.system()} {platform.machine()}")

        steps = [
            ("ビルド準備", lambda: True),
            ("高速リリースビルド", self.build_release),
            ("既存プロセス終了", self.kill_existing_processes),
            ("バイナリインストール", lambda: self.install_binary(install_path)),
            ("インストール検証", lambda: self.verify_installation(install_path))
        ]

        total_steps = len(steps)

        for i, (step_name, step_func) in enumerate(steps, 1):
            print(f"\n{'='*50}")
            print(f"[INFO] ステップ {i}/{total_steps}: {step_name}")
            print(f"{'='*50}")

            with tqdm(total=1, desc=f"[REBUILD] {step_name}", unit="step") as pbar:
                start_time = time.time()
                success = step_func()
                elapsed = time.time() - start_time
                pbar.update(1)

            if not success:
                print(f"[ERROR] ステップ {i} 失敗: {step_name}")
                return False

            print(".1f"
        print(f"\n[SUCCESS] すべてのステップが完了しました!")
        return True

def main():
    import argparse

    parser = argparse.ArgumentParser(description="Codex 高速ビルド＆インストールシステム")
    parser.add_argument(
        "--install-path",
        type=str,
        help="インストール先パス",
        default=None
    )
    parser.add_argument(
        "--skip-kill",
        action="store_true",
        help="プロセス終了をスキップ"
    )

    args = parser.parse_args()

    project_root = Path(__file__).parent.parent
    system = BuildAndInstallSystem(project_root)

    # インストールパス設定
    install_path = None
    if args.install_path:
        install_path = Path(args.install_path)

    print("[TARGET] 設定:")
    print(f"  [DIR] プロジェクトルート: {project_root}")
    print(f"  [TARGET] インストール先: {install_path or system.default_install_path}")
    print(f"  🚫 プロセス終了: {'スキップ' if args.skip_kill else '実行'}")

    # プロセス終了スキップ設定
    if args.skip_kill:
        original_kill = system.kill_existing_processes
        system.kill_existing_processes = lambda: (print("⏭️ プロセス終了をスキップ"), True)[1]

    success = system.full_build_and_install(install_path)

    if success:
        print("\n[CELEBRATE] ビルド＆インストール成功!")
        print("[START] 新しいCodexを使用できます")

        # 完了音（Windowsのみ）
        if system.is_windows:
            try:
                import winsound
                winsound.Beep(800, 200)
                winsound.Beep(1000, 200)
                winsound.Beep(1200, 200)
            except ImportError:
                pass
    else:
        print("\n[CRASH] ビルド＆インストール失敗")
        sys.exit(1)

if __name__ == "__main__":
    main()