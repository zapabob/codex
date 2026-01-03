#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
高速差分ビルドシステム for Codex
Cargoのincremental compilationを活用した高速ビルド
tqdm風の視覚化で進捗を表示
"""

import os
import sys
import subprocess
import time
import json
from pathlib import Path
from typing import List, Dict, Optional
import hashlib
import pickle
from datetime import datetime

# WindowsコンソールでUTF-8を強制
if os.name == 'nt':
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')

try:
    from tqdm import tqdm
    import psutil
except ImportError:
    print("必要なパッケージをインストール中...")
    subprocess.run([sys.executable, "-m", "pip", "install", "tqdm", "psutil"], check=True)
    from tqdm import tqdm
    import psutil

class FastBuildSystem:
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.codex_rs_dir = project_root / "codex-rs"
        self.cache_file = project_root / ".build_cache.pkl"
        self.build_times = {}
        self.file_hashes = {}
        self.last_build_time = None

        # キャッシュディレクトリの初期化
        self.cache_dir = project_root / ".codex_build_cache"
        self.cache_dir.mkdir(exist_ok=True)

    def load_cache(self):
        """ビルドキャッシュを読み込む"""
        if self.cache_file.exists():
            try:
                with open(self.cache_file, 'rb') as f:
                    cache = pickle.load(f)
                    self.file_hashes = cache.get('hashes', {})
                    self.build_times = cache.get('times', {})
                    self.last_build_time = cache.get('last_build', None)
                print("[INFO] ビルドキャッシュを読み込みました")
            except Exception as e:
                print(f"[WARN] キャッシュ読み込みエラー: {e}")
                self.file_hashes = {}
                self.build_times = {}

    def save_cache(self):
        """ビルドキャッシュを保存"""
        cache = {
            'hashes': self.file_hashes,
            'times': self.build_times,
            'last_build': datetime.now()
        }
        try:
            with open(self.cache_file, 'wb') as f:
                pickle.dump(cache, f)
            print("[SAVE] ビルドキャッシュを保存しました")
        except Exception as e:
            print(f"[WARN] キャッシュ保存エラー: {e}")

    def get_rust_files(self) -> List[Path]:
        """Rustソースファイルを収集"""
        rust_files = []
        for ext in ['.rs', 'Cargo.toml', 'Cargo.lock']:
            rust_files.extend(self.codex_rs_dir.rglob(f'*{ext}'))
        return sorted(rust_files)

    def calculate_file_hash(self, file_path: Path) -> str:
        """ファイルのハッシュを計算"""
        try:
            with open(file_path, 'rb') as f:
                return hashlib.md5(f.read()).hexdigest()
        except Exception:
            return ""

    def detect_changes(self) -> Dict[str, List[Path]]:
        """変更されたファイルを検出"""
        changed_files = {'modified': [], 'new': [], 'deleted': []}

        current_files = set(self.get_rust_files())
        previous_files = set(Path(p) for p in self.file_hashes.keys())

        # 新しいファイルと変更されたファイルを検出
        with tqdm(current_files, desc="[SEARCH] ファイル変更検出中", unit="files") as pbar:
            for file_path in pbar:
                file_str = str(file_path)
                current_hash = self.calculate_file_hash(file_path)

                if file_str not in self.file_hashes:
                    changed_files['new'].append(file_path)
                elif self.file_hashes[file_str] != current_hash:
                    changed_files['modified'].append(file_path)

                self.file_hashes[file_str] = current_hash

        # 削除されたファイルを検出
        for file_str in previous_files - current_files:
            file_path = Path(file_str)
            changed_files['deleted'].append(file_path)
            if file_str in self.file_hashes:
                del self.file_hashes[file_str]

        return changed_files

    def should_full_build(self, changes: Dict[str, List[Path]]) -> bool:
        """フルビルドが必要かどうかを判定"""
        # Cargo.toml または Cargo.lock が変更された場合はフルビルド
        critical_files = ['Cargo.toml', 'Cargo.lock']
        for file_path in changes['modified'] + changes['new']:
            if any(file_path.name == cf for cf in critical_files):
                return True

        # 削除されたファイルがある場合はフルビルド
        if changes['deleted']:
            return True

        return False

    def run_build(self, target: str = "debug", packages: Optional[List[str]] = None) -> bool:
        """Cargoビルドを実行"""
        os.chdir(self.codex_rs_dir)

        cmd = ["cargo", "build"]
        if target == "release":
            cmd.append("--release")

        if packages:
            for pkg in packages:
                cmd.extend(["-p", pkg])

        # 高速ビルドのための環境変数
        env = os.environ.copy()
        env['CARGO_INCREMENTAL'] = '1'
        env['CARGO_BUILD_JOBS'] = str(max(1, psutil.cpu_count() - 1))  # CPUコア数-1
        # sccacheがincremental compilationと競合するので無効化
        env['SCCACHE_DISABLE'] = '1'
        env['RUSTC_WRAPPER'] = ''  # sccacheラッパーを無効化

        print(f"[BUILD] ビルドコマンド: {' '.join(cmd)}")
        print(f"[START] 使用CPUコア数: {env['CARGO_BUILD_JOBS']}")

        start_time = time.time()
        try:
            result = subprocess.run(
                cmd,
                env=env,
                capture_output=True,
                text=True,
                timeout=1800  # 30分タイムアウト
            )
            build_time = time.time() - start_time

            if result.returncode == 0:
                print(f"[OK] ビルド成功 ({build_time:.1f}s)")
                self.build_times[target] = build_time
                return True
            else:
                print(f"[ERROR] ビルド失敗 ({build_time:.1f}s)")
                print("エラー出力:")
                print(result.stderr)
                return False

        except subprocess.TimeoutExpired:
            print("[TIMEOUT] ビルドがタイムアウトしました (30分)")
            return False
        except Exception as e:
            print(f"[CRASH] ビルドエラー: {e}")
            return False

    def incremental_build(self, target: str = "debug") -> bool:
        """差分ビルドを実行"""
        print("[REBUILD] 高速差分ビルドを開始します...")

        # キャッシュ読み込み
        self.load_cache()

        # 変更検出
        changes = self.detect_changes()

        total_changes = len(changes['modified']) + len(changes['new']) + len(changes['deleted'])
        print(f"[STATS] 検出された変更: {total_changes} files")

        if total_changes > 0:
            print(f"  [MODIFIED]  変更: {len(changes['modified'])} files")
            print(f"  [NEW] 新規: {len(changes['new'])} files")
            print(f"  [DELETED] 削除: {len(changes['deleted'])} files")

        # フルビルド判定
        if self.should_full_build(changes) or total_changes == 0:
            if total_changes == 0:
                print("[TARGET] 前回ビルドから変更なし - スキップします")
                return True
            else:
                print("[FULL] フルビルドが必要です")
                return self.run_build(target)
        else:
            print("[DIFF] 差分ビルドを実行します")
            return self.run_build(target)

    def get_build_stats(self) -> Dict:
        """ビルド統計を取得"""
        return {
            'last_build_time': self.last_build_time.isoformat() if self.last_build_time else None,
            'build_times': self.build_times,
            'cached_files': len(self.file_hashes)
        }

def main():
    if len(sys.argv) > 1 and sys.argv[1] in ['--help', '-h', 'help']:
        print("使用法: python fast_build.py [debug|release] [package1 package2 ...]")
        print("  debug|release: ビルドターゲット（デフォルト: debug）")
        print("  package: ビルドする特定のパッケージ（オプション）")
        sys.exit(0)

    if len(sys.argv) < 2:
        print("使用法: python fast_build.py [debug|release] [package1 package2 ...]")
        sys.exit(1)

    target = sys.argv[1] if len(sys.argv) > 1 else "debug"
    packages = sys.argv[2:] if len(sys.argv) > 2 else None

    project_root = Path(__file__).parent.parent
    builder = FastBuildSystem(project_root)

    print("[START] Codex 高速ビルドシステム")
    print(f"[DIR] プロジェクト: {project_root}")
    print(f"[TARGET] ターゲット: {target}")
    if packages:
        print(f"[PKG] パッケージ: {', '.join(packages)}")

    success = builder.incremental_build(target)

    if success:
        print("\n[OK] ビルド成功！")
        # キャッシュ保存
        builder.save_cache()

        # 統計表示
        stats = builder.get_build_stats()
        print("\n[CHART] ビルド統計:")
        if stats['last_build_time']:
            print(f"  [TIME] 最終ビルド: {stats['last_build_time']}")
        for build_type, time_taken in stats['build_times'].items():
            print(f"  [CLOCK]  {build_type}ビルド: {time_taken:.1f}s")
        print(f"  [DIR] キャッシュファイル: {stats['cached_files']}")

    else:
        print("\n[ERROR] ビルド失敗")
        sys.exit(1)

if __name__ == "__main__":
    main()