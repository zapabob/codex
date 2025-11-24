#!/usr/bin/env node
/**
 * Build and install script for @zapabob/codex
 * Integrated script for fast incremental build, process killing, and installation
 */

const { spawn, execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const args = process.argv.slice(2);
const skipClean = args.includes('--skip-clean');
const globalInstall = args.includes('--global');

// Platform detection
const isWindows = process.platform === 'win32';
const binaryName = isWindows ? 'codex.exe' : 'codex';
const codexRsPath = path.join(__dirname, '..', 'codex-rs');
const binPath = path.join(__dirname, '..', 'bin');
const targetReleasePath = path.join(codexRsPath, 'target', 'release', binaryName);

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
        const result = execSync(`tasklist /FI "IMAGENAME eq ${procName}.exe" /NH`, { encoding: 'utf8', stdio: 'pipe' });
        if (result.includes(procName)) {
          status(`   Stopping ${procName} processes...`);
          execSync(`taskkill /F /IM ${procName}.exe /T`, { stdio: 'pipe' });
          killedProcesses.push(procName);
          success(`   Stopped ${procName}`);
        }
      } else {
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

// Run command
function runCommand(command, args, options = {}) {
  return new Promise((resolve, reject) => {
    const proc = spawn(command, args, {
      ...options,
      stdio: options.stdio || 'inherit',
      shell: isWindows,
    });

    proc.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with exit code ${code}`));
      }
    });

    proc.on('error', (err) => {
      reject(err);
    });
  });
}

// Build binary using build-binary.js
async function buildBinary() {
  status('Building binary...');
  const buildScript = path.join(__dirname, 'build-binary.js');
  const buildArgs = skipClean ? ['--skip-clean'] : [];
  
  try {
    await runCommand('node', [buildScript, ...buildArgs]);
    return true;
  } catch (err) {
    error(`Build failed: ${err.message}`);
    return false;
  }
}

// Install globally (optional)
async function installGlobally() {
  if (!globalInstall) {
    return;
  }

  status('Installing globally...');
  
  const homeDir = process.env.USERPROFILE || process.env.HOME || '';
  const cargoBinPath = path.join(homeDir, '.cargo', 'bin');
  const globalBinaryPath = path.join(cargoBinPath, binaryName);

  // Ensure cargo bin directory exists
  if (!fs.existsSync(cargoBinPath)) {
    fs.mkdirSync(cargoBinPath, { recursive: true });
  }

  // Copy binary
  try {
    fs.copyFileSync(targetReleasePath, globalBinaryPath);
    
    // Make executable on Unix
    if (!isWindows) {
      fs.chmodSync(globalBinaryPath, 0o755);
    }
    
    success(`Installed globally: ${globalBinaryPath}`);
    
    // Verify installation
    status('Verifying installation...');
    try {
      const versionOutput = execSync(`"${globalBinaryPath}" --version`, { encoding: 'utf8', stdio: 'pipe' });
      success(`Installation verified: ${versionOutput.trim()}`);
    } catch (err) {
      warning('Version check failed, but binary is installed');
    }
  } catch (err) {
    error(`Failed to install globally: ${err.message}`);
    process.exit(1);
  }
}

// Main function
async function main() {
  log('========================================', 'cyan');
  log(' Codex Build & Install', 'cyan');
  log('========================================', 'cyan');
  log('');

  // Step 1: Kill processes
  status('Step 1/3: Stopping running processes...');
  killProcesses();
  log('');

  // Step 2: Build
  status('Step 2/3: Building binary...');
  const buildSuccess = await buildBinary();
  if (!buildSuccess) {
    process.exit(1);
  }
  log('');

  // Step 3: Install globally (optional)
  if (globalInstall) {
    status('Step 3/3: Installing globally...');
    await installGlobally();
  } else {
    status('Step 3/3: Skipping global install (use --global to install)');
  }
  log('');

  log('========================================', 'green');
  log('  Build & Install Completed!', 'green');
  log('========================================', 'green');
  log('');
  log(`Binary: ${path.join(binPath, binaryName)}`, 'cyan');
  if (globalInstall) {
    const homeDir = process.env.USERPROFILE || process.env.HOME || '';
    log(`Global: ${path.join(homeDir, '.cargo', 'bin', binaryName)}`, 'cyan');
  }
  log('');
  log('Next steps:', 'yellow');
  log('  codex --version', 'reset');
  log('  npx @zapabob/codex --version', 'reset');
  log('  pnpm exec codex --version', 'reset');
  log('');
}

// Run main
main().catch((err) => {
  error(`Fatal error: ${err.message}`);
  console.error(err);
  process.exit(1);
});

