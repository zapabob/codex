#!/usr/bin/env python3
"""
Unicode文字をASCIIに置き換えるスクリプト
"""

import re

# 置き換えマップ
replacements = {
    '📋': '[INFO]',
    '⚠️': '[WARN]',
    '💾': '[SAVE]',
    '🔍': '[SEARCH]',
    '🏗️': '[BUILD]',
    '🚀': '[FAST]',
    '✅': '[OK]',
    '❌': '[ERROR]',
    '📊': '[STATS]',
    '✏️': '[MODIFIED]',
    '➕': '[NEW]',
    '➖': '[DELETED]',
    '🎯': '[TARGET]',
    '🔨': '[FULL]',
    '⚡': '[DIFF]',
    '⏰': '[TIMEOUT]',
    '💥': '[CRASH]',
    '🔄': '[REBUILD]',
    '📁': '[DIR]',
    '📦': '[PKG]',
    '🕒': '[TIME]',
    '⏱️': '[CLOCK]',
    '🚀': '[START]',
    '🎉': '[SUCCESS]',
    '📈': '[CHART]',
    '🛑': '[STOP]',
    '💡': '[TIP]',
    '🎊': '[CELEBRATE]'
}

def fix_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    for emoji, replacement in replacements.items():
        content = content.replace(emoji, replacement)

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

    print(f"Fixed: {filepath}")

if __name__ == "__main__":
    import sys
    if len(sys.argv) < 2:
        print("Usage: python fix_unicode.py <file>")
        sys.exit(1)

    fix_file(sys.argv[1])