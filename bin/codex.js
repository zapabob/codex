#!/usr/bin/env node
// Codex AI-Native OS CLI Launcher
// Based on zapabob/codex v2.3.0

import { spawn } from "node:child_process";
import { existsSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const { platform, arch } = process;

// Platform detection for binary selection
let targetTriple = null;
switch (platform) {
  case "linux":
  case "android":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-unknown-linux-musl";
        break;
      case "arm64":
        targetTriple = "aarch64-unknown-linux-musl";
        break;
      default:
        break;
    }
    break;
  case "darwin":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-apple-darwin";
        break;
      case "arm64":
        targetTriple = "aarch64-apple-darwin";
        break;
      default:
        break;
    }
    break;
  case "win32":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-pc-windows-msvc";
        break;
      case "arm64":
        targetTriple = "aarch64-pc-windows-msvc";
        break;
      default:
        break;
    }
    break;
  default:
    break;
}

if (!targetTriple) {
  console.error("Unsupported platform: " + platform + "-" + arch);
  console.error("Supported platforms: Linux x64/arm64, macOS x64/arm64, Windows x64/arm64");
  process.exit(1);
}

// Find binary path
const binaryName = platform === "win32" ? "codex.exe" : "codex";
const binaryPaths = [
  // Local development
  join(__dirname, "..", "codex-rs", "target", "release", binaryName),
  join(__dirname, "..", "codex-rs", "target", "debug", binaryName),
  // Installed binary (from npm/pnpm postinstall)
  join(__dirname, binaryName),
  // Global installation
  process.env.CODEX_BINARY_PATH,
  // Cargo bin directory
  platform === "win32" 
    ? join(process.env.USERPROFILE || process.env.HOME || "", ".cargo", "bin", binaryName)
    : join(process.env.HOME || "", ".cargo", "bin", binaryName),
].filter(Boolean);

let binaryPath = null;
for (const path of binaryPaths) {
  if (existsSync(path)) {
    binaryPath = path;
    break;
  }
}

if (!binaryPath) {
  console.error("Codex binary not found. Please ensure:");
  console.error("1. Run 'npm run build' to build the binary, or");
  console.error("2. Install codex globally with 'npm install -g @zapabob/codex'");
  console.error("3. Or install via cargo: 'cargo install --path codex-rs/cli'");
  process.exit(1);
}

// Launch the Rust binary with all arguments
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  env: { ...process.env, CODEX_CLI_VERSION: "2.3.0" }
});

child.on("close", (code) => {
  process.exit(code || 0);
});

child.on("error", (error) => {
  console.error("Failed to launch Codex:", error.message);
  process.exit(1);
});

