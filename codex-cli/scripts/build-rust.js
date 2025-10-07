#!/usr/bin/env node
/**
 * Build script for Rust binaries.
 * 
 * This script builds the Codex Rust binary for multiple platforms.
 * Used during npm package preparation.
 */

import { execSync } from 'child_process';
import { existsSync, mkdirSync, copyFileSync, readdirSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const TARGETS = [
  'x86_64-unknown-linux-musl',
  'aarch64-unknown-linux-musl',
  'x86_64-apple-darwin',
  'aarch64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'aarch64-pc-windows-msvc',
];

const CODEX_RS_ROOT = path.join(__dirname, '..', '..', 'codex-rs');
const VENDOR_ROOT = path.join(__dirname, '..', 'vendor');

console.log('🔨 Building Codex Rust binaries...\n');

// Ensure vendor directory exists
if (!existsSync(VENDOR_ROOT)) {
  mkdirSync(VENDOR_ROOT, { recursive: true });
}

// Determine which targets to build
const buildTargets = process.env.BUILD_TARGETS 
  ? process.env.BUILD_TARGETS.split(',')
  : [getCurrentPlatformTarget()]; // Default: build only for current platform

console.log(`Targets: ${buildTargets.join(', ')}\n`);

for (const target of buildTargets) {
  if (!TARGETS.includes(target)) {
    console.warn(`⚠️  Unknown target: ${target}, skipping...`);
    continue;
  }

  console.log(`Building for ${target}...`);

  try {
    // Add target if not already installed
    execSync(`rustup target add ${target}`, {
      cwd: CODEX_RS_ROOT,
      stdio: 'inherit',
    });

    // Build release binary
    execSync(`cargo build --release --target ${target} --bin codex-tui`, {
      cwd: CODEX_RS_ROOT,
      stdio: 'inherit',
    });

    // Copy binary to vendor directory
    const targetDir = path.join(VENDOR_ROOT, target, 'codex');
    mkdirSync(targetDir, { recursive: true });

    const binaryName = target.includes('windows') ? 'codex.exe' : 'codex';
    const sourcePath = path.join(
      CODEX_RS_ROOT,
      'target',
      target,
      'release',
      binaryName
    );
    const destPath = path.join(targetDir, binaryName);

    if (existsSync(sourcePath)) {
      copyFileSync(sourcePath, destPath);
      console.log(`✅ Copied ${binaryName} to ${targetDir}`);
    } else {
      console.error(`❌ Binary not found: ${sourcePath}`);
      process.exit(1);
    }
  } catch (err) {
    console.error(`❌ Failed to build for ${target}:`);
    console.error(err.message);
    process.exit(1);
  }

  console.log('');
}

console.log('🎉 All binaries built successfully!');
console.log(`\nVendor directory: ${VENDOR_ROOT}`);
console.log('Contents:');
readdirSync(VENDOR_ROOT).forEach((dir) => {
  console.log(`  - ${dir}/`);
});

function getCurrentPlatformTarget() {
  const { platform, arch } = process;

  if (platform === 'linux' && arch === 'x64') return 'x86_64-unknown-linux-musl';
  if (platform === 'linux' && arch === 'arm64') return 'aarch64-unknown-linux-musl';
  if (platform === 'darwin' && arch === 'x64') return 'x86_64-apple-darwin';
  if (platform === 'darwin' && arch === 'arm64') return 'aarch64-apple-darwin';
  if (platform === 'win32' && arch === 'x64') return 'x86_64-pc-windows-msvc';
  if (platform === 'win32' && arch === 'arm64') return 'aarch64-pc-windows-msvc';

  throw new Error(`Unsupported platform: ${platform}-${arch}`);
}

