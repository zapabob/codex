#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Codex Version Unity Check Script
プロジェクト全体のバージョン統一状況をチェック
"""

import os
import re
import glob
from pathlib import Path

def check_workspace_version():
    """ワークスペースのCargo.tomlのバージョンをチェック"""
    print("Checking workspace Cargo.toml...")
    workspace_toml = "codex-rs/Cargo.toml"

    try:
        with open(workspace_toml, 'r', encoding='utf-8') as f:
            content = f.read()

        version_match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
        if version_match:
            version = version_match.group(1)
            print(f"  Workspace version: {version}")
            return version
        else:
            print("  ERROR: Workspace version not found")
            return None
    except FileNotFoundError:
        print(f"  ERROR: {workspace_toml} not found")
        return None

def check_package_versions():
    """各パッケージのCargo.tomlのバージョンをチェック"""
    print("Checking package versions...")

    package_versions = {}
    cargo_files = glob.glob("codex-rs/*/Cargo.toml")

    for cargo_file in sorted(cargo_files):
        package_name = Path(cargo_file).parent.name

        try:
            with open(cargo_file, 'r', encoding='utf-8') as f:
                content = f.read()

            # パッケージ名取得
            name_match = re.search(r'^name\s*=\s*"([^"]+)"', content, re.MULTILINE)
            if name_match:
                actual_name = name_match.group(1)
            else:
                actual_name = package_name

            # バージョン取得
            version_match = re.search(r'^version\s*=\s*([^"\n]+)', content, re.MULTILINE)
            if version_match:
                version_line = version_match.group(1).strip()
                if version_line == '"workspace"':
                    version = "workspace"
                elif version_line.startswith('"') and version_line.endswith('"'):
                    version = version_line[1:-1]
                else:
                    version = version_line
            else:
                version = "NOT_FOUND"

            package_versions[actual_name] = {
                'version': version,
                'file': cargo_file
            }

            print(f"  {actual_name}: {version}")

        except Exception as e:
            print(f"  ERROR reading {cargo_file}: {e}")

    return package_versions

def check_readme_version():
    """README.mdのバージョンをチェック"""
    print("Checking README.md version...")

    try:
        with open("README.md", 'r', encoding='utf-8') as f:
            content = f.read()

        # バージョン表記の検索
        version_patterns = [
            r'version-([\d.]+)',
            r'v([\d.]+)',
            r'(\d+\.\d+\.\d+)'
        ]

        versions_found = []
        for pattern in version_patterns:
            matches = re.findall(pattern, content)
            versions_found.extend(matches)

        unique_versions = list(set(versions_found))
        if unique_versions:
            print(f"  README versions found: {unique_versions}")
            return unique_versions
        else:
            print("  No versions found in README")
            return []

    except FileNotFoundError:
        print("  README.md not found")
        return []

def analyze_version_consistency(workspace_version, package_versions, readme_versions):
    """バージョン統一性を分析"""
    print("Analyzing version consistency...")

    issues = []

    # ワークスペースバージョン vs パッケージバージョン
    for package_name, info in package_versions.items():
        version = info['version']
        if version == "workspace":
            continue  # workspace参照はOK
        elif version != workspace_version:
            issues.append(f"Package {package_name} has version {version}, expected {workspace_version}")

    # READMEバージョン vs ワークスペースバージョン
    readme_version_set = set()
    for rv in readme_versions:
        if rv.startswith('2.8'):  # 2.8.x系のバージョンだけチェック
            readme_version_set.add(rv)

    if workspace_version not in readme_version_set:
        issues.append(f"README mentions {readme_version_set}, but workspace is {workspace_version}")

    # 結果表示
        if issues:
            print("ISSUES FOUND:")
            for issue in issues:
                print(f"  [ERROR] {issue}")
            return False
    else:
        print("[OK] All versions are consistent!")
        return True

def main():
    print("Codex Version Unity Check")
    print("=" * 50)
    print("Checking version consistency across the project")
    print()

    # ワークスペースバージョン取得
    workspace_version = check_workspace_version()
    print()

    # パッケージバージョン取得
    package_versions = check_package_versions()
    print()

    # READMEバージョン取得
    readme_versions = check_readme_version()
    print()

    # 統一性分析
    if workspace_version:
        is_consistent = analyze_version_consistency(workspace_version, package_versions, readme_versions)
        print()

        if is_consistent:
            print("[SUCCESS] Version unity check PASSED!")
            print(f"   Unified version: {workspace_version}")
        else:
            print("[WARNING] Version unity check FAILED!")
            print("   Please fix the version inconsistencies above")
    else:
        print("❌ Cannot perform analysis: workspace version not found")

if __name__ == "__main__":
    main()