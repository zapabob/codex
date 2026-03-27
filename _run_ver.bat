@echo off
set CARGO_TARGET_DIR=C:\Users\downl\codex-target-offload\codex-main-upstream-sync
cd /d C:\Users\downl\Desktop\codex-main-upstream-sync\codex-rs
cargo run --bin codex -- --version > C:\Users\downl\Desktop\codex-main-upstream-sync\_ver_out.txt 2>&1
echo EXITCODE=%ERRORLEVEL% >> C:\Users\downl\Desktop\codex-main-upstream-sync\_ver_out.txt
