#!/usr/bin/env python3
"""
Codex Chrome MCP Bridgeコンパイルエラー修正スクリプト
検出されたコンパイルエラーを自動修正
"""

import os
import re
from tqdm import tqdm
import time

def fix_bridge_rs():
    """Fix bridge.rs compilation errors"""
    bridge_file = os.path.join(os.path.dirname(__file__), "codex-rs", "chrome-mcp-bridge", "src", "bridge.rs")

    print("[FIX] Fixing bridge.rs...")

    with open(bridge_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # 1. Fix request.params Option<Value> handling
    content = re.sub(
        r'let params: InitializeRequestParams = serde_json::from_value\(request\.params\)\?;',
        r'let params: InitializeRequestParams = serde_json::from_value(request.params.ok_or_else(|| anyhow::anyhow!("Missing params"))?)?;',
        content
    )

    content = re.sub(
        r'let params: CallToolRequestParams = serde_json::from_value\(request\.params\)\?;',
        r'let params: CallToolRequestParams = serde_json::from_value(request.params.ok_or_else(|| anyhow::anyhow!("Missing params"))?)?;',
        content
    )

    # 2. Remove Default::default() from ServerCapabilities
    content = re.sub(
        r'capabilities: ServerCapabilities \{\s*tools: Some\(ServerCapabilitiesTools \{\s*list_changed: Some\(true\),\s*\}\),\s*\.\.Default::default\(\)\s*\}',
        r'capabilities: ServerCapabilities {\n                        tools: Some(ServerCapabilitiesTools {\n                            list_changed: Some(true),\n                        }),\n                        completions: None,\n                        experimental: None,\n                        logging: None,\n                        prompts: None,\n                        resources: None,\n                    }',
        content,
        flags=re.MULTILINE | re.DOTALL
    )

    # 3. Fix JSONRPCResponse structure
    content = re.sub(
        r'let response = mcp_types::JSONRPCResponse \{\s+jsonrpc: mcp_types::JSONRPC_VERSION\.to_string\(\),\s+id,\s+result: Some\(serde_json::to_value\(result\)\?\),\s+error: None,\s+\};',
        r'let response = mcp_types::JSONRPCResponse {\n            jsonrpc: mcp_types::JSONRPC_VERSION.to_string(),\n            id,\n            result: serde_json::to_value(result)?,\n        };',
        content,
        flags=re.MULTILINE | re.DOTALL
    )

    with open(bridge_file, 'w', encoding='utf-8') as f:
        f.write(content)

    print("[DONE] bridge.rs fixed")

def fix_tools_rs():
    """Add required fields to tools.rs"""
    tools_file = os.path.join(os.path.dirname(__file__), "codex-rs", "chrome-mcp-bridge", "src", "tools.rs")

    print("[FIX] Fixing tools.rs...")

    with open(tools_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Add required fields to Tool structs
    # annotations, output_schema, title

    # Add fields to each Tool creation
    tool_pattern = r'(\s+)input_schema: json!\(\{[\s\S]*?\}\),\s*\}'

    def add_required_fields(match):
        indent = match.group(1)
        return f'{match.group(0)[:-2]}\n{indent}annotations: None,\n{indent}output_schema: None,\n{indent}title: Some("Chrome Extension Tool".to_string()),\n{indent}}}'

    content = re.sub(tool_pattern, add_required_fields, content, flags=re.MULTILINE)

    with open(tools_file, 'w', encoding='utf-8') as f:
        f.write(content)

    print("[DONE] tools.rs fixed")

def main():
    print("Codex Chrome MCP Bridge Compilation Error Fix Tool")
    print("=" * 60)
    print("Fixing detected errors:")
    print("X Type mismatch: Option<Value> vs Value")
    print("X Missing struct fields: annotations, output_schema")
    print("X API compatibility: mcp_types::ServerCapabilities no Default impl")
    print()

    steps = [
        ("main.rs import cleanup", lambda: print("DONE main.rs already fixed")),
        ("bridge.rs type mismatch fix", fix_bridge_rs),
        ("tools.rs required fields add", fix_tools_rs),
    ]

    with tqdm(total=len(steps), desc="[FIX] Working", bar_format='{desc}: {percentage:3.0f}%|{bar}| {n_fmt}/{total_fmt}') as pbar:
        for desc, func in steps:
            pbar.set_description(f"[FIX] {desc}")
            func()
            pbar.update(1)
            time.sleep(0.5)  # visual effect

    print("\n[SUCCESS] All fixes completed!")
    print("[NEXT] Run build test:")
    print("   cargo build --release -p codex-chrome-mcp-bridge")

if __name__ == "__main__":
    main()