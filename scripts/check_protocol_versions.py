#!/usr/bin/env python3

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GENERATOR = ROOT / "codex-rs" / "mcp-types" / "generate_mcp_types.py"
LIB_RS = ROOT / "codex-rs" / "mcp-types" / "src" / "lib.rs"
STRING_VERSION_FILES = [
    ROOT / "extensions" / "chrome-codex" / "background" / "background.js",
    ROOT / "extensions" / "chrome-codex" / "background" / "mcp_client.js",
]
RUST_ENUM_FILES = [
    ROOT
    / "codex-rs"
    / "app-server"
    / "tests"
    / "suite"
    / "v2"
    / "mcp_server_elicitation.rs",
    ROOT / "codex-rs" / "rmcp-client" / "tests" / "resources.rs",
    ROOT / "codex-rs" / "cli" / "src" / "chrome_cmd.rs",
    ROOT / "codex-rs" / "core" / "src" / "mcp_connection_manager.rs",
    ROOT / "codex-rs" / "core" / "src" / "agents" / "runtime.rs",
]


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def extract_regex(path: Path, pattern: str) -> str:
    match = re.search(pattern, read_text(path), re.MULTILINE)
    if not match:
        raise SystemExit(f"Could not find pattern in {path}")
    return match.group(1)


canonical = extract_regex(GENERATOR, r'^SCHEMA_VERSION = "([0-9\-]+)"$')
lib_version = extract_regex(
    LIB_RS, r'^pub const MCP_SCHEMA_VERSION: &str = "([0-9\-]+)";$'
)
if lib_version != canonical:
    raise SystemExit(
        f"codex-rs/mcp-types/src/lib.rs has MCP_SCHEMA_VERSION={lib_version}, expected {canonical}"
    )

for file_path in STRING_VERSION_FILES:
    actual = extract_regex(file_path, r'protocol_version:\s*"([0-9\-]+)"')
    if actual != canonical:
        raise SystemExit(
            f"{file_path.relative_to(ROOT)} has protocol_version={actual}, expected {canonical}"
        )

expected_enum = f"V_{canonical.replace('-', '_')}"
for file_path in RUST_ENUM_FILES:
    contents = read_text(file_path)
    if expected_enum not in contents:
        raise SystemExit(
            f"{file_path.relative_to(ROOT)} does not reference ProtocolVersion::{expected_enum}"
        )

print(f"Protocol version checks passed for {canonical}.")
