#!/usr/bin/env python3
"""
unstable let chain構文を安定版に変換するスクリプト
`&& let` パターンを nested if let に変換
"""

import re
import sys
from pathlib import Path

def fix_let_chains_in_file(file_path):
    """ファイル内のlet chainパターンを修正"""
    try:
        content = file_path.read_text(encoding='utf-8')
        original_content = content
        
        # パターン1: if let ... && let ...
        # これは複雑なので、手動で確認が必要
        # とりあえずレポートだけする
        if_let_chain_pattern = r'if\s+let\s+.*?&&\s+let\s+'
        matches = list(re.finditer(if_let_chain_pattern, content))
        
        if matches:
            print(f"Found {len(matches)} if-let chain(s) in {file_path}")
            return False  # 修正が必要
        
        return True  # OK
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """メイン処理"""
    codex_rs_dir = Path("codex-rs")
    
    if not codex_rs_dir.exists():
        print("Error: codex-rs directory not found")
        sys.exit(1)
    
    # .rsファイルを全て検索
    rs_files = list(codex_rs_dir.rglob("*.rs"))
    
    print(f"Checking {len(rs_files)} Rust files...")
    
    files_need_fix = []
    for rs_file in rs_files:
        if not fix_let_chains_in_file(rs_file):
            files_need_fix.append(rs_file)
    
    if files_need_fix:
        print(f"\n{len(files_need_fix)} files need manual fixing:")
        for f in files_need_fix:
            print(f"  - {f}")
        return 1
    else:
        print("\nOK: All files are good!")
        return 0

if __name__ == "__main__":
    sys.exit(main())

