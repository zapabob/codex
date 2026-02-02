# Binary Release Log - 2026-02-02

## Release Information

- **Version**: v2.13.0
- **Release URL**: [https://github.com/zapabob/codex/releases/tag/v2.13.0](https://github.com/zapabob/codex/releases/tag/v2.13.0)
- **Archive**: `codex-v2.13.0-windows-x64.tar.gz`

## Included Binaries

| Binary Name               | Description               |
| ------------------------- | ------------------------- |
| `codex.exe`               | Main application          |
| `codex-tui.exe`           | TUI application           |
| `codex-stdio-to-uds.exe`  | Communication utility     |
| `codex-linux-sandbox.exe` | Sandboxing utility        |
| `apply_patch.exe`         | Patch application utility |

## Release Process

1. Identified binaries in `target/release`.
2. Extracted version `2.13.0` from `Cargo.toml`.
3. Created a `dist` directory and collected binaries.
4. Archived into `tar.gz` using `tar`.
5. Created a GitHub Release and uploaded the archive using `gh release create`.
