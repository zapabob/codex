#!/usr/bin/env python3
"""GUI TypeScript error auto-fixer for Next.js build.

Fixes:
- TS6133: Unused imports → remove from import statement
- TS2322: Button variant 'primary'/'ghost'/'secondary' → MUI compatible
- Preserves file encoding (UTF-8 without BOM)
"""

import re
import sys
import os
from collections import defaultdict
from pathlib import Path

GUI_ROOT = Path(r"C:\Users\downl\Desktop\codex-main\gui")
TSC_ERRORS_FILE = GUI_ROOT / "tsc_errors.txt"


def parse_errors(error_file: Path) -> dict:
    """Parse tsc errors into {filepath: [(line, col, code, message)]}"""
    errors = defaultdict(list)
    with open(error_file, "r", encoding="utf-8-sig", errors="replace") as f:
        for line in f:
            m = re.match(r"(src/.+?)\((\d+),(\d+)\): error (TS\d+): (.+)", line.strip())
            if m:
                filepath = m.group(1)
                lineno = int(m.group(2))
                col = int(m.group(3))
                code = m.group(4)
                msg = m.group(5)
                errors[filepath].append((lineno, col, code, msg))
    return errors


def remove_unused_import(content: str, identifier: str) -> str:
    """Remove a single unused identifier from TypeScript import statements."""
    # Pattern: identifier followed by , or just alone
    # Handle cases like:  Paper,\n  or  Paper,  or  , Paper  or  Paper
    patterns = [
        # identifier at start of line with trailing comma
        (rf"(\n\s+){re.escape(identifier)},", r"\1"),
        # identifier with trailing comma (inline)
        (rf",\s*{re.escape(identifier)}\s*,", ","),
        # identifier at end (last in list before closing brace)
        (rf",\s*\n(\s+){re.escape(identifier)}\s*\n(\s+)\}}", r"\n\2}}"),
        # standalone identifier with comma at end
        (rf"  {re.escape(identifier)},\n", ""),
    ]
    for pattern, replacement in patterns:
        new_content = re.sub(pattern, replacement, content)
        if new_content != content:
            return new_content

    # Fallback: simple line removal for import lines
    lines = content.split("\n")
    result = []
    for line in lines:
        stripped = line.strip().rstrip(",")
        if stripped == identifier or stripped == f"{identifier},":
            continue
        result.append(line)
    return "\n".join(result)


def fix_button_variants(content: str) -> str:
    """Fix MUI Button variant type mismatches."""
    # variant="primary" → variant="contained"
    content = re.sub(r'variant="primary"', 'variant="contained"', content)
    # variant="ghost" → variant="outlined"
    content = re.sub(r'variant="ghost"', 'variant="outlined"', content)
    # variant="secondary" (for Button) → variant="outlined"
    # Only for Button components
    content = re.sub(
        r'(<(?:Button|MuiButton)[^>]*?)variant="secondary"',
        r'\1variant="outlined"',
        content,
    )
    return content


def fix_unused_variable(content: str, identifier: str, lineno: int) -> str:
    """Fix unused variable by prefixing with underscore."""
    lines = content.split("\n")
    if lineno <= len(lines):
        line = lines[lineno - 1]
        # Replace identifier with _identifier in destructuring
        new_line = re.sub(
            rf"\b{re.escape(identifier)}\b",
            f"_{identifier}",
            line,
        )
        lines[lineno - 1] = new_line
    return "\n".join(lines)


def process_file(filepath_rel: str, errors: list) -> bool:
    """Process a single file and apply fixes. Returns True if modified."""
    full_path = GUI_ROOT / filepath_rel

    if not full_path.exists():
        print(f"  SKIP (not found): {filepath_rel}")
        return False

    with open(full_path, "r", encoding="utf-8", errors="replace") as f:
        content = f.read()

    original = content
    unused_imports = []

    for lineno, col, code, msg in errors:
        if code == "TS6133":
            # Extract identifier name from message
            m = re.match(r"'(.+?)' is declared but its value is never read", msg)
            if m:
                identifier = m.group(1)
                unused_imports.append(identifier)
        elif code == "TS2322":
            # Button variant mismatch
            content = fix_button_variants(content)

    # Remove unused imports
    for identifier in unused_imports:
        before = content
        content = remove_unused_import(content, identifier)
        if content != before:
            print(f"  Removed unused import: {identifier}")
        else:
            print(f"  WARNING: Could not remove: {identifier}")

    if content != original:
        with open(full_path, "w", encoding="utf-8", newline="\n") as f:
            f.write(content)
        print(f"  FIXED: {filepath_rel}")
        return True

    return False


def main():
    print("=== GUI TypeScript Error Fixer ===")
    print(f"Reading errors from: {TSC_ERRORS_FILE}")

    errors = parse_errors(TSC_ERRORS_FILE)
    print(f"Found errors in {len(errors)} files")

    fixed_count = 0
    for filepath, file_errors in sorted(errors.items()):
        print(f"\nProcessing: {filepath} ({len(file_errors)} errors)")
        codes = {e[2] for e in file_errors}
        fixable = {"TS6133", "TS2322"}
        if not (codes & fixable):
            print(f"  SKIP: no fixable errors (has {codes})")
            continue
        if process_file(filepath, file_errors):
            fixed_count += 1

    print(f"\n=== Done: Fixed {fixed_count} files ===")


if __name__ == "__main__":
    main()
