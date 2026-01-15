#!/usr/bin/env python3
# Replace Unicode characters in migrate_to_skills.py

replacements = {
    '❌': '[ERROR]',
    '🔍': '[SEARCH]',
    '✅': '[OK]',
    '📁': '[DIR]',
    '📋': '[INFO]'
}

with open('scripts/migrate_to_skills.py', 'r', encoding='utf-8') as f:
    content = f.read()

for old, new in replacements.items():
    content = content.replace(old, new)

with open('scripts/migrate_to_skills.py', 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed Unicode characters in migrate_to_skills.py")