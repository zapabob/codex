#!/usr/bin/env python3
import re

# Read the file
with open('scripts/wait_build_complete.py', 'r', encoding='utf-8') as f:
    content = f.read()

# Replace Unicode characters
replacements = {
    '🔄': '[REBUILD]',
    '⏳': '[WAIT]',
    '✅': '[OK]',
    '📁': '[DIR]',
    '📏': '[INFO]',
    '🔧': '[TOOL]',
    '❌': '[ERROR]',
    '🔍': '[SEARCH]',
    '📋': '[INFO]',
    '🎉': '[SUCCESS]',
    '🚀': '[START]',
    '💥': '[CRASH]'
}

for emoji, replacement in replacements.items():
    content = content.replace(emoji, replacement)

# Write back
with open('scripts/wait_build_complete.py', 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed Unicode characters in wait_build_complete.py")