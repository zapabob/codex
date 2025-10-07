#!/usr/bin/env node
/**
 * Post-install script for @openai/codex npm package.
 * 
 * This script:
 * 1. Detects the current platform and architecture
 * 2. Verifies the Rust binary exists for this platform
 * 3. Makes the binary executable (Unix-like systems)
 * 4. Displays installation success message
 */

import { existsSync, chmodSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const { platform, arch } = process;

// Determine target triple
let targetTriple = null;
switch (platform) {
  case 'linux':
  case 'android':
    switch (arch) {
      case 'x64':
        targetTriple = 'x86_64-unknown-linux-musl';
        break;
      case 'arm64':
        targetTriple = 'aarch64-unknown-linux-musl';
        break;
    }
    break;
  case 'darwin':
    switch (arch) {
      case 'x64':
        targetTriple = 'x86_64-apple-darwin';
        break;
      case 'arm64':
        targetTriple = 'aarch64-apple-darwin';
        break;
    }
    break;
  case 'win32':
    switch (arch) {
      case 'x64':
        targetTriple = 'x86_64-pc-windows-msvc';
        break;
      case 'arm64':
        targetTriple = 'aarch64-pc-windows-msvc';
        break;
    }
    break;
}

if (!targetTriple) {
  console.error(`❌ Unsupported platform: ${platform} (${arch})`);
  console.error('Supported platforms:');
  console.error('  - Linux (x64, arm64)');
  console.error('  - macOS (x64, arm64)');
  console.error('  - Windows (x64, arm64)');
  process.exit(1);
}

const vendorRoot = path.join(__dirname, '..', 'vendor');
const archRoot = path.join(vendorRoot, targetTriple);
const codexBinaryName = platform === 'win32' ? 'codex.exe' : 'codex';
const binaryPath = path.join(archRoot, 'codex', codexBinaryName);

// Check if binary exists
if (!existsSync(binaryPath)) {
  console.error(`❌ Binary not found for ${platform}-${arch}`);
  console.error(`Expected location: ${binaryPath}`);
  console.error('\nThis might be a packaging issue. Please report this at:');
  console.error('https://github.com/openai/codex/issues');
  process.exit(1);
}

// Make binary executable on Unix-like systems
if (platform !== 'win32') {
  try {
    chmodSync(binaryPath, 0o755);
  } catch (err) {
    console.warn(`⚠️  Could not make binary executable: ${err.message}`);
  }
}

// Display success message
console.log('✅ OpenAI Codex installed successfully!');
console.log('');
console.log(`Platform: ${platform}-${arch} (${targetTriple})`);
console.log(`Binary: ${binaryPath}`);
console.log('');
console.log('🚀 Get started:');
console.log('  $ codex --help');
console.log('  $ codex --profile workspace');
console.log('');
console.log('📚 Documentation:');
console.log('  https://github.com/openai/codex#readme');
console.log('');
console.log('🔒 Security Features:');
console.log('  - Sandboxed execution');
console.log('  - Audit logging');
console.log('  - Multiple security profiles');
console.log('');

