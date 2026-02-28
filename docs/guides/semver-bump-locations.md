# SemVer Bump Locations

This repository currently treats these files as the primary product-version touch points:

1. `codex-rs/Cargo.toml`
2. `package.json`
3. `codex-gui-x/package.json`
4. `VERSION`

Optional (only when intentionally versioning these surfaces together):

1. `codex-cli/package.json`
2. `codex-rs/tauri-gui/package.json`
3. `codex-rs/tauri-gui/src-tauri/Cargo.toml`

## Bump Script

Use:

```powershell
.\scripts\bump-version.ps1 patch
.\scripts\bump-version.ps1 minor -Apply
.\scripts\bump-version.ps1 major -Apply -IncludeCodexCli -IncludeTauri
```

By default, the script updates the primary touch points only.  
Use the optional switches to include additional package surfaces.
