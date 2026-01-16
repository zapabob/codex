#!/usr/bin/env python3
"""
MCPサーバービルドスクリプト
MCP関連のエラーを根本的に解決する
"""

import subprocess
import sys
import os
from pathlib import Path

def run_command(cmd, cwd=None, check=True):
    """コマンドを実行して結果を表示"""
    print(f"🔧 Executing: {' '.join(cmd)}")
    if cwd:
        print(f"📁 In directory: {cwd}")

    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            capture_output=True,
            text=True,
            check=check
        )
        if result.stdout:
            print(f"✅ Output: {result.stdout[:200]}...")
        return result
    except subprocess.CalledProcessError as e:
        print(f"❌ Error: {e}")
        if e.stdout:
            print(f"📄 Stdout: {e.stdout}")
        if e.stderr:
            print(f"📄 Stderr: {e.stderr}")
        if not check:
            return e
        raise

def main():
    """メイン処理"""
    print("🚀 Starting MCP Server Build Process")

    # プロジェクトルートを確認
    root_dir = Path(__file__).parent
    codex_rs_dir = root_dir / "codex-rs"

    if not codex_rs_dir.exists():
        print(f"❌ codex-rs directory not found: {codex_rs_dir}")
        sys.exit(1)

    print(f"📁 Working in: {codex_rs_dir}")

    # 1. Rustプロジェクトのビルド
    print("\n📦 Building Rust MCP servers...")

    # codex-gemini-cli-mcp-serverのビルド
    print("\n🔨 Building codex-gemini-cli-mcp-server...")
    try:
        run_command(
            ["cargo", "build", "--release", "-p", "codex-gemini-cli-mcp-server"],
            cwd=codex_rs_dir
        )
        print("✅ codex-gemini-cli-mcp-server built successfully")
    except Exception as e:
        print(f"❌ Failed to build codex-gemini-cli-mcp-server: {e}")

    # mcp-serverのビルド
    print("\n🔨 Building mcp-server...")
    try:
        run_command(
            ["cargo", "build", "--release", "-p", "codex-mcp-server"],
            cwd=codex_rs_dir
        )
        print("✅ mcp-server built successfully")
    except Exception as e:
        print(f"❌ Failed to build mcp-server: {e}")

    # 2. Node.js MCPサーバーのビルド
    print("\n📦 Building Node.js MCP servers...")

    # prism-mcp-server
    prism_dir = root_dir / "prism-mcp-server"
    if prism_dir.exists():
        print("\n🔨 Building prism-mcp-server...")
        try:
            run_command(["npm", "install"], cwd=prism_dir)
            run_command(["npm", "run", "build"], cwd=prism_dir)
            print("✅ prism-mcp-server built successfully")
        except Exception as e:
            print(f"❌ Failed to build prism-mcp-server: {e}")
    else:
        print("⚠️ prism-mcp-server directory not found")

    # shell-tool-mcp
    shell_dir = root_dir / "shell-tool-mcp"
    if shell_dir.exists():
        print("\n🔨 Building shell-tool-mcp...")
        try:
            run_command(["npm", "install"], cwd=shell_dir)
            run_command(["npm", "run", "build"], cwd=shell_dir)
            print("✅ shell-tool-mcp built successfully")
        except Exception as e:
            print(f"❌ Failed to build shell-tool-mcp: {e}")
    else:
        print("⚠️ shell-tool-mcp directory not found")

    # 3. ビルド結果の確認
    print("\n🔍 Checking build results...")

    # 実行ファイルの確認
    target_dir = codex_rs_dir / "target" / "release"
    if target_dir.exists():
        mcp_executables = [
            "codex-gemini-cli-mcp-server",
            "codex-gemini-cli-mcp-server.exe",
            "codex-mcp-server",
            "codex-mcp-server.exe"
        ]

        for exe in mcp_executables:
            exe_path = target_dir / exe
            if exe_path.exists():
                print(f"✅ Found: {exe_path}")
            else:
                print(f"❌ Missing: {exe_path}")

    # 4. MCP設定の更新
    print("\n⚙️ Updating MCP configuration...")

    config_path = root_dir / "config.toml"
    if config_path.exists():
        print("📄 MCP configuration exists")

        # codex-gemini-mcpを有効化する提案
        print("⚠️ Please manually enable MCP servers in config.toml:")
        print("   - Change codex-gemini-mcp: enabled = false → enabled = true")
        print("   - Consider enabling codex-research, codex-agent, codex-supervisor if needed")

    print("\n🎉 MCP Server Build Process Complete!")

if __name__ == "__main__":
    main()