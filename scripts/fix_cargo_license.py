#!/usr/bin/env python3
"""Add license.workspace = true to all crate Cargo.toml files missing a license."""
from pathlib import Path
import re

RS = Path(r"C:\Users\downl\Desktop\codex-main\codex-rs")
WORKSPACE_CARGO = RS / "Cargo.toml"


def add_license_to_cargo_toml(path: Path) -> bool:
    """Add license = { workspace = true } to [package] section if missing."""
    content = path.read_text(encoding="utf-8", errors="replace")

    # Skip if already has license field
    if re.search(r"^license", content, re.MULTILINE | re.IGNORECASE):
        return False

    # Find [package] section and add license after version or name
    # Look for version = { workspace = true } and add license after it
    if "version = { workspace = true }" in content:
        new_content = content.replace(
            "version = { workspace = true }",
            'version = { workspace = true }\nlicense = { workspace = true }',
            1,
        )
    elif "[package]" in content:
        # Add after [package] line
        new_content = content.replace(
            "[package]",
            "[package]",
            1,
        )
        # Find the package section and add license field
        lines = content.splitlines(keepends=True)
        new_lines = []
        in_package = False
        license_added = False
        for i, line in enumerate(lines):
            new_lines.append(line)
            if line.strip() == "[package]":
                in_package = True
            elif in_package and not license_added:
                # Add after name or version line in package section
                if re.match(r"^(name|version|edition)\s*=", line):
                    # Check if next line is also a field
                    if i + 1 < len(lines) and not re.match(r"^(name|version|edition)\s*=", lines[i + 1]):
                        new_lines.append('license = { workspace = true }\n')
                        license_added = True
                elif line.strip().startswith("[") and line.strip() != "[package]":
                    # Reached next section without adding
                    new_lines.insert(-1, 'license = { workspace = true }\n')
                    license_added = True
                    in_package = False
        new_content = "".join(new_lines)
    else:
        return False

    if new_content != content:
        path.write_text(new_content, encoding="utf-8")
        return True
    return False


def main():
    cargo_tomls = [
        f for f in RS.rglob("Cargo.toml")
        if "target" not in str(f) and f.parent != RS
    ]

    updated = []
    skipped = []

    for f in sorted(cargo_tomls):
        content = f.read_text(encoding="utf-8", errors="replace")
        if re.search(r"^license", content, re.MULTILINE | re.IGNORECASE):
            skipped.append(f)
            continue

        if add_license_to_cargo_toml(f):
            rel = str(f).replace(str(RS) + "\\", "")
            updated.append(rel)
            print(f"  UPDATED: {rel}")
        else:
            skipped.append(f)

    print(f"\nUpdated: {len(updated)}, Skipped: {len(skipped)}")


if __name__ == "__main__":
    main()
