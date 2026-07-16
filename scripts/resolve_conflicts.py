#!/usr/bin/env python3
"""
Upstream Sync Conflict Resolver v2.17.0
zapabob/codex ← openai/codex 163コミット分のコンフリクトを自動解決する

解決戦略:
  - zapabob独自ドキュメント/設定 → ours (HEAD) を保持
  - セキュリティ・CI関連 → theirs (upstream) を採用
  - Rustソース: upstream変更をベースにzapabob拡張を保持
  - README.md → ours を保持（zapabob customized）
  - Cargo.lock → theirs を採用し後でcargo updateで更新
"""

import os
import sys
import io
import re
import shutil
from pathlib import Path
from tqdm import tqdm

# Windows cp932対策: UTF-8でstdoutを強制
if sys.platform == "win32":
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

REPO_ROOT = Path(__file__).parent.parent

# zapabob独自ディレクトリ → ours を保持
ZAPABOB_DIRS = [
    "codex-gui-x",
    "prism-mcp-server",
    "prism-web",
    "zapabob",
    "_docs",
    "kernel-extensions",
    "docs/zapabob",
    "social_announcements",
    "tools",
]

# upstream優先ファイル（セキュリティ・CI関連）
UPSTREAM_PREFER = [
    ".codespellignore",
    ".github/workflows/issue-labeler.yml",
    ".github/workflows/rust-ci.yml",
    "codex-rs/Cargo.lock",
    "codex-rs/app-server-protocol/src/protocol/thread_history.rs",
    "codex-rs/app-server/tests/suite/codex_message_processor_flow.rs",
    "codex-rs/core/src/mcp_connection_manager.rs",
    "codex-rs/core/src/models_manager/model_info.rs",
    "codex-rs/core/src/unified_exec/async_watcher.rs",
    "codex-rs/mcp-server/src/codex_tool_runner.rs",
    "codex-rs/tui/src/bottom_pane/feedback/view.rs",
    "codex-rs/tui/src/chatwidget.rs",
]

# ours優先ファイル（zapabob特有）
OURS_PREFER = [
    "README.md",
]

# 特殊処理が必要なファイル
SPECIAL_FILES = {
    "codex-rs/core/Cargo.toml": "merge_cargo_toml",
    "docs/zapabob/AGENTS.md": "merge_agents_md",
    "codex-rs/core/src/agent/control.rs": "take_upstream",
    "codex-rs/core/src/codex.rs": "take_upstream",
    "codex-rs/core/src/tools/orchestrator.rs": "take_upstream",
    "codex-rs/tui/src/history_cell.rs": "take_upstream",
}


def parse_conflicts(content: str) -> list[dict]:
    """コンフリクトマーカーを解析して各セクションを抽出"""
    conflicts = []
    lines = content.split('\n')
    i = 0
    segments = []  # (type, content) type: 'normal', 'ours', 'theirs'
    
    while i < len(lines):
        if lines[i].startswith('<<<<<<< '):
            # コンフリクト開始
            ours_lines = []
            theirs_lines = []
            i += 1
            # ours部分を収集
            while i < len(lines) and not lines[i].startswith('======='):
                ours_lines.append(lines[i])
                i += 1
            i += 1  # ======= をスキップ
            # theirs部分を収集
            while i < len(lines) and not lines[i].startswith('>>>>>>> '):
                theirs_lines.append(lines[i])
                i += 1
            i += 1  # >>>>>>> をスキップ
            segments.append(('conflict', ours_lines, theirs_lines))
        else:
            segments.append(('normal', lines[i], None))
            i += 1
    
    return segments


def resolve_take_theirs(content: str) -> str:
    """upstream（theirs）の変更を採用"""
    segments = parse_conflicts(content)
    result_lines = []
    for seg in segments:
        if seg[0] == 'normal':
            result_lines.append(seg[1])
        else:
            # theirs を採用
            result_lines.extend(seg[2])
    return '\n'.join(result_lines)


def resolve_take_ours(content: str) -> str:
    """ours（HEAD）の変更を保持"""
    segments = parse_conflicts(content)
    result_lines = []
    for seg in segments:
        if seg[0] == 'normal':
            result_lines.append(seg[1])
        else:
            # ours を採用
            result_lines.extend(seg[1])
    return '\n'.join(result_lines)


def resolve_merge_cargo_toml(content: str) -> str:
    """
    Cargo.toml の競合解決:
    - dependency版本はupstreamを優先
    - zapabob独自依存関係（askama, dashmap等）は保持
    - [features]セクションはmerge（両方の機能フラグを保持）
    """
    segments = parse_conflicts(content)
    result_lines = []
    
    for seg in segments:
        if seg[0] == 'normal':
            result_lines.append(seg[1])
        else:
            ours = seg[1]
            theirs = seg[2]
            
            # 両方が空の場合はスキップ
            if not any(l.strip() for l in ours) and not any(l.strip() for l in theirs):
                continue
            
            # oursにしかない行（zapabob独自依存関係）を抽出
            ours_unique = set(l.strip() for l in ours if l.strip())
            theirs_set = set(l.strip() for l in theirs if l.strip())
            
            # theirs（upstream）を採用
            result_lines.extend(theirs)
            
            # oursにしか無くてzapabob固有の依存関係を追加
            zapabob_only = ours_unique - theirs_set
            for line in ours:
                stripped = line.strip()
                if stripped in zapabob_only and stripped:
                    # zapabob独自の依存関係（askama, dashmap等）を保持
                    if any(dep in stripped for dep in [
                        'askama', 'dashmap', 'codex-deep-research', 
                        'codex-supervisor', 'nucleo'
                    ]):
                        result_lines.append(line)
    
    return '\n'.join(result_lines)


def resolve_merge_agents_md(content: str) -> str:
    """
    docs/zapabob/AGENTS.md の競合解決:
    - zapabobセクション（ours）を保持
    - upstream追加のルール（ConfigToml, Bazel lockなど）も取り込む
    """
    segments = parse_conflicts(content)
    result_lines = []
    
    for seg in segments:
        if seg[0] == 'normal':
            result_lines.append(seg[1])
        else:
            ours = seg[1]
            theirs = seg[2]
            
            # oursを基本として採用し、theirs固有の有用なルールを末尾に追加
            result_lines.extend(ours)
            
            # theirs（upstream）のルールで ours にないものを追加
            ours_text = '\n'.join(ours)
            for line in theirs:
                if line.strip() and line.strip() not in ours_text:
                    result_lines.append(line)
    
    return '\n'.join(result_lines)


def get_conflict_files() -> list[str]:
    """gitコンフリクト中のファイルを取得"""
    import subprocess
    result = subprocess.run(
        ['git', 'diff', '--name-only', '--diff-filter=U'],
        capture_output=True, text=True, cwd=REPO_ROOT
    )
    files = [f.strip() for f in result.stdout.strip().split('\n') if f.strip()]
    return files


def resolve_file(rel_path: str) -> tuple[bool, str]:
    """
    ファイルを解決する
    Returns: (success, message)
    """
    abs_path = REPO_ROOT / rel_path
    
    if not abs_path.exists():
        # modify/delete conflict でファイルが削除されている場合
        # upstream版を保持（ファイルはすでにworktreeにある）
        return True, f"[SKIP] ファイルが存在しない（modify/delete handled）: {rel_path}"
    
    content = abs_path.read_text(encoding='utf-8', errors='replace')
    
    # コンフリクトマーカーがない場合はスキップ
    if '<<<<<<< ' not in content:
        # modify/delete conflictなど特殊ケース
        return True, f"[OK] コンフリクトマーカーなし（自動解決済み）: {rel_path}"
    
    # 特殊処理ファイル
    if rel_path in SPECIAL_FILES:
        strategy = SPECIAL_FILES[rel_path]
        if strategy == "merge_cargo_toml":
            resolved = resolve_merge_cargo_toml(content)
        elif strategy == "merge_agents_md":
            resolved = resolve_merge_agents_md(content)
        elif strategy == "take_upstream":
            resolved = resolve_take_theirs(content)
        elif strategy == "take_ours":
            resolved = resolve_take_ours(content)
        else:
            return False, f"[ERR] 未知の戦略: {strategy}"
        abs_path.write_text(resolved, encoding='utf-8')
        return True, f"[SPECIAL:{strategy}] {rel_path}"
    
    # zapabob独自ディレクトリ → ours を保持
    for zapabob_dir in ZAPABOB_DIRS:
        if rel_path.startswith(zapabob_dir):
            resolved = resolve_take_ours(content)
            abs_path.write_text(resolved, encoding='utf-8')
            return True, f"[OURS] zapabob独自: {rel_path}"
    
    # ours優先ファイル
    if rel_path in OURS_PREFER:
        resolved = resolve_take_ours(content)
        abs_path.write_text(resolved, encoding='utf-8')
        return True, f"[OURS] {rel_path}"
    
    # upstream優先ファイル
    if rel_path in UPSTREAM_PREFER:
        resolved = resolve_take_theirs(content)
        abs_path.write_text(resolved, encoding='utf-8')
        return True, f"[UPSTREAM] {rel_path}"
    
    # デフォルト: upstream採用（安全側）
    resolved = resolve_take_theirs(content)
    abs_path.write_text(resolved, encoding='utf-8')
    return True, f"[UPSTREAM/DEFAULT] {rel_path}"


def git_add_file(rel_path: str):
    """解決済みファイルをgit addする"""
    import subprocess
    subprocess.run(
        ['git', 'add', rel_path],
        cwd=REPO_ROOT,
        capture_output=True
    )


def main():
    print("=" * 70)
    print("  Upstream Sync Conflict Resolver v2.17.0")
    print("  zapabob/codex ← openai/codex")
    print("=" * 70)
    
    conflict_files = get_conflict_files()
    
    if not conflict_files:
        print("コンフリクトファイルが見つからへんで！終了するわ。")
        return 0
    
    print(f"\nコンフリクトファイル数: {len(conflict_files)}")
    print()
    
    success_count = 0
    fail_count = 0
    failed_files = []
    
    for rel_path in tqdm(conflict_files, desc="コンフリクト解決中", unit="file"):
        try:
            ok, msg = resolve_file(rel_path)
            if ok:
                git_add_file(rel_path)
                success_count += 1
                print(f"  ✓ {msg}")
            else:
                fail_count += 1
                failed_files.append(rel_path)
                print(f"  ✗ {msg}")
        except Exception as e:
            fail_count += 1
            failed_files.append(rel_path)
            print(f"  ✗ [ERR] {rel_path}: {e}")
    
    print()
    print("=" * 70)
    print(f"  解決完了: {success_count}/{len(conflict_files)} ファイル")
    if failed_files:
        print(f"  手動解決が必要: {fail_count} ファイル")
        for f in failed_files:
            print(f"    - {f}")
    print("=" * 70)
    
    return 0 if fail_count == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
