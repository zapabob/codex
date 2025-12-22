# PowerShell Navigation Command

## Execute First:
```powershell
cd codex-rs
```

## Purpose:
Navigate to the Rust workspace directory where Cargo.toml and source code are located.

## Expected Result:
- PowerShell prompt changes to show `codex-rs` in the path
- No error messages if directory exists
- Ready for cargo check command

## If Error Occurs:
- "Cannot find path" error means wrong current directory
- Need to verify we're in the project root (codex-main)
- Use `pwd` to check current location

## Success Confirmation:
- Command completes without error
- Directory change is successful
- Ready to execute `cargo check --all-features`

Execute `cd codex-rs` now.