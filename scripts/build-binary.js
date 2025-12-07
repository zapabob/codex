#!/usr/bin/env node
/**
 * Build binary script for @zapabob/codex
 * Cross-platform build script with process killing and incremental build support
 */

import { spawn } from 'child_process';
import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const args = process.argv.slice(2);
const skipClean = args.includes('--skip-clean');

// Platform detection
const isWindows = process.platform === 'win32';
const binaryName = isWindows ? 'codex.exe' : 'codex';
const codexRsPath = path.join(__dirname, '..', 'codex-rs');
const binPath = path.join(__dirname, '..', 'bin');
const targetReleasePath = path.join(codexRsPath, 'target', 'release', binaryName);
const targetDebugPath = path.join(codexRsPath, 'target', 'debug', binaryName);

// Color output functions
function log(message, color = 'reset') {
  const colors = {
    reset: '\x1b[0m',
    cyan: '\x1b[36m',
    green: '\x1b[32m',
    yellow: '\x1b[33m',
    red: '\x1b[31m',
    gray: '\x1b[90m',
  };
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function status(message) {
  log(`[*] ${message}`, 'cyan');
}

function success(message) {
  log(`[OK] ${message}`, 'green');
}

function warning(message) {
  log(`[WARN] ${message}`, 'yellow');
}

function error(message) {
  log(`[ERROR] ${message}`, 'red');
}

// Kill running processes
function killProcesses() {
  const processes = ['codex', 'codex-tui', 'codex-tauri-gui'];
  const killedProcesses = [];

  for (const procName of processes) {
    try {
      if (isWindows) {
        // Windows: use taskkill
        const result = execSync(`tasklist /FI "IMAGENAME eq ${procName}.exe" /NH`, { encoding: 'utf8', stdio: 'pipe' });
        if (result.includes(procName)) {
          status(`   Stopping ${procName} processes...`);
          execSync(`taskkill /F /IM ${procName}.exe /T`, { stdio: 'pipe' });
          killedProcesses.push(procName);
          success(`   Stopped ${procName}`);
        }
      } else {
        // Unix: use pkill or killall
        try {
          execSync(`pkill -9 ${procName}`, { stdio: 'pipe' });
          killedProcesses.push(procName);
          success(`   Stopped ${procName}`);
        } catch {
          try {
            execSync(`killall -9 ${procName}`, { stdio: 'pipe' });
            killedProcesses.push(procName);
            success(`   Stopped ${procName}`);
          } catch {
            // Process not running, ignore
          }
        }
      }
    } catch (err) {
      // Process not running, ignore
    }
  }

  if (killedProcesses.length > 0) {
    success(`Stopped ${killedProcesses.length} process type(s)`);
  } else {
    status('No running processes found');
  }
}

// Run command and return output
function runCommand(command, args, options = {}) {
  return new Promise((resolve, reject) => {
    const proc = spawn(command, args, {
      ...options,
      stdio: options.stdio || 'inherit',
      shell: isWindows,
    });

    let stdout = '';
    let stderr = '';

    if (options.capture) {
      proc.stdout.on('data', (data) => {
        stdout += data.toString();
      });
      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });
    }

    proc.on('close', (code) => {
      if (code === 0) {
        resolve({ code: 0, stdout, stderr });
      } else {
        reject({ code, stdout, stderr });
      }
    });

    proc.on('error', (err) => {
      reject({ code: -1, error: err.message, stdout, stderr });
    });
  });
}

// Clean build artifacts
async function cleanBuild() {
  status('Cleaning build artifacts (cargo clean)...');
  try {
    await runCommand('cargo', ['clean'], { cwd: codexRsPath });
    success('Clean completed');
  } catch (err) {
    warning('cargo clean failed, but continuing');
  }
}

// Format code
async function formatCode() {
  status('Formatting code (just fmt)...');
  try {
    await runCommand('just', ['fmt'], { cwd: codexRsPath });
    success('Format completed');
  } catch {
    try {
      await runCommand('cargo', ['fmt', '--all'], { cwd: codexRsPath });
      success('Format completed');
    } catch (err) {
      warning('Format failed, but continuing');
    }
  }
}

// Build binary
async function buildBinary() {
  status('Building release (codex-cli)...');
  if (skipClean) {
    log('   [INFO] Using incremental build (faster)...', 'cyan');
  } else {
    log('   [INFO] Full build (this may take several minutes)...', 'yellow');
  }

  const buildStart = Date.now();
  const buildArgs = ['build', '--release', '-p', 'codex-cli'];

  try {
    await runCommand('cargo', buildArgs, { cwd: codexRsPath });
    const buildDuration = Date.now() - buildStart;
    const buildMinutes = Math.round((buildDuration / 1000 / 60) * 10) / 10;
    const buildSeconds = Math.round(buildDuration / 1000);
    log(`   Build time: ${buildMinutes}m ${buildSeconds}s`, 'gray');
    success('Build succeeded!');
    return true;
  } catch (err) {
    error('Build failed');
    if (err.stdout) {
      console.error(err.stdout);
    }
    if (err.stderr) {
      console.error(err.stderr);
    }

    // Special handling for build errors - check for existing binary
    status('Build failed, checking for existing binary...');
    
    if (fs.existsSync(targetReleasePath)) {
      warning('Found existing binary. Using it');
      return true;
    } else if (fs.existsSync(targetDebugPath)) {
      warning('Found debug binary. Using it');
      return true;
    } else {
      error('Cannot continue build - no existing binary found');
      log('\nSuggested fixes:', 'yellow');
      log('  1. Install Visual Studio Build Tools', 'reset');
      log('  2. Run: cargo update -p aws-lc-sys', 'reset');
      log('  3. Or use existing binary: npm run build:binary (if binary exists)', 'reset');
      log('  4. Re-run this script', 'reset');
      process.exit(1);
    }
  }
}

// Copy binary to bin directory
function copyBinary() {
  status('Copying binary to bin directory...');

  // Ensure bin directory exists
  if (!fs.existsSync(binPath)) {
    fs.mkdirSync(binPath, { recursive: true });
  }

  const destPath = path.join(binPath, binaryName);

  // Try release first, then debug
  let sourcePath = null;
  if (fs.existsSync(targetReleasePath)) {
    sourcePath = targetReleasePath;
  } else if (fs.existsSync(targetDebugPath)) {
    sourcePath = targetDebugPath;
    warning('Using debug binary (release not found)');
  } else {
    error(`Binary not found at ${targetReleasePath} or ${targetDebugPath}`);
    process.exit(1);
  }

  try {
    fs.copyFileSync(sourcePath, destPath);
    
    // Make executable on Unix
    if (!isWindows) {
      fs.chmodSync(destPath, 0o755);
    }
    
    const stats = fs.statSync(destPath);
    const sizeMB = (stats.size / 1024 / 1024).toFixed(2);
    success(`Binary copied: ${destPath} (${sizeMB} MB)`);
  } catch (err) {
    error(`Failed to copy binary: ${err.message}`);
    process.exit(1);
  }
}

// Main function
async function main() {
  log('========================================', 'cyan');
  log(' Codex Binary Build', 'cyan');
  log('========================================', 'cyan');
  log('');

  // Step 1: Kill running processes
  status('Step 1/4: Stopping running processes...');
  killProcesses();
  log('');

  // Step 2: Clean (optional)
  if (!skipClean) {
    status('Step 2/4: Cleaning build artifacts...');
    await cleanBuild();
  } else {
    status('Step 2/4: Skipping clean step (using incremental build)');
  }
  log('');

  // Step 3: Format
  status('Step 3/4: Formatting code...');
  await formatCode();
  log('');

  // Step 4: Build
  status('Step 4/4: Building binary...');
  const buildSuccess = await buildBinary();
  if (!buildSuccess) {
    process.exit(1);
  }
  log('');

  // Copy binary
  copyBinary();
  log('');

  log('========================================', 'green');
  log('  Build Completed!', 'green');
  log('========================================', 'green');
  log('');
  log(`Binary: ${path.join(binPath, binaryName)}`, 'cyan');
  log('');
}

// Run main
main().catch((err) => {
  error(`Fatal error: ${err.message}`);
  console.error(err);
  process.exit(1);
});

