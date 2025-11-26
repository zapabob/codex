#!/usr/bin/env node
/**
 * Post-install script for @zapabob/codex-cli
 * Downloads pre-built binaries from GitHub Releases
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { promisify } = require('util');
const pipeline = promisify(require('stream').pipeline);

// Get version from package.json
const packageJson = require('../package.json');
const VERSION = packageJson.version || '2.3.0';
const GITHUB_REPO = 'zapabob/codex';

// Platform detection
function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;
  
  const platformMap = {
    win32: { os: 'windows', ext: '.exe', archive: 'zip' },
    darwin: { os: 'macos', ext: '', archive: 'tar.gz' },
    linux: { os: 'linux', ext: '', archive: 'tar.gz' },
  };
  
  const archMap = {
    x64: 'x64',
    arm64: 'arm64',
  };
  
  if (!platformMap[platform]) {
    throw new Error(`Unsupported platform: ${platform}`);
  }
  
  if (!archMap[arch]) {
    throw new Error(`Unsupported architecture: ${arch}`);
  }
  
  return {
    ...platformMap[platform],
    archName: archMap[arch],
  };
}

// Download file from URL
async function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    https.get(url, { followAllRedirects: true }, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: HTTP ${response.statusCode}`));
        return;
      }
      
      const file = fs.createWriteStream(dest);
      pipeline(response, file)
        .then(() => resolve())
        .catch(reject);
    }).on('error', reject);
  });
}

// Verify SHA256 checksum
async function verifySHA256(filePath, expectedHash) {
  const hash = crypto.createHash('sha256');
  const stream = fs.createReadStream(filePath);
  
  return new Promise((resolve, reject) => {
    stream.on('data', (data) => hash.update(data));
    stream.on('end', () => {
      const computed = hash.digest('hex');
      resolve(computed === expectedHash);
    });
    stream.on('error', reject);
  });
}

// Extract archive
async function extractArchive(archivePath, destDir) {
  const { promisify } = require('util');
  const exec = promisify(require('child_process').exec);
  
  const ext = path.extname(archivePath);
  
  if (ext === '.zip') {
    // Windows: use unzip or 7z
    try {
      await exec(`powershell -command "Expand-Archive -Path '${archivePath}' -DestinationPath '${destDir}' -Force"`);
    } catch {
      await exec(`7z x "${archivePath}" -o"${destDir}" -y`);
    }
  } else if (archivePath.endsWith('.tar.gz')) {
    // Unix: use tar
    await exec(`tar -xzf "${archivePath}" -C "${destDir}"`);
  }
}

async function main() {
  console.log('📦 Installing @zapabob/codex v' + VERSION);
  
  try {
    const platformInfo = getPlatformInfo();
    console.log(`🖥️  Platform: ${platformInfo.os}-${platformInfo.archName}`);
    
    // Ensure bin directory exists
    const binDir = path.join(__dirname, '..', 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }
    
    const binaryPath = path.join(binDir, 'codex' + platformInfo.ext);
    
    // Try to find local binary first (for development)
    const localBinaryPaths = [
      path.join(__dirname, '..', 'codex-rs', 'target', 'release', 'codex' + platformInfo.ext),
      path.join(__dirname, '..', 'codex-rs', 'target', 'debug', 'codex' + platformInfo.ext),
    ];
    
    let foundLocal = false;
    for (const localPath of localBinaryPaths) {
      if (fs.existsSync(localPath)) {
        console.log(`📋 Found local binary: ${localPath}`);
        fs.copyFileSync(localPath, binaryPath);
        foundLocal = true;
        break;
      }
    }
    
    if (!foundLocal) {
      // Try to find in cargo bin directory
      const homeDir = process.env.USERPROFILE || process.env.HOME || '';
      const cargoBinPath = platformInfo.os === 'windows'
        ? path.join(homeDir, '.cargo', 'bin', 'codex' + platformInfo.ext)
        : path.join(homeDir, '.cargo', 'bin', 'codex' + platformInfo.ext);
      
      if (fs.existsSync(cargoBinPath)) {
        console.log(`📋 Found cargo-installed binary: ${cargoBinPath}`);
        fs.copyFileSync(cargoBinPath, binaryPath);
        foundLocal = true;
      }
    }
    
    if (!foundLocal) {
      // Try to build from source if CODEX_BUILD_ON_INSTALL is set
      if (process.env.CODEX_BUILD_ON_INSTALL === 'true' || process.argv.includes('--build-on-install')) {
        console.log('🔨 Building binary from source...');
        try {
          const { execSync } = require('child_process');
          execSync('npm run build:binary:fast', { 
            stdio: 'inherit',
            cwd: path.join(__dirname, '..')
          });
          
          // Check if binary was built
          const builtBinaryPath = path.join(__dirname, '..', 'codex-rs', 'target', 'release', 'codex' + platformInfo.ext);
          if (fs.existsSync(builtBinaryPath)) {
            console.log(`📋 Found built binary: ${builtBinaryPath}`);
            fs.copyFileSync(builtBinaryPath, binaryPath);
            foundLocal = true;
          }
        } catch (err) {
          console.warn('⚠️  Build failed, falling back to download:', err.message);
        }
      }
      
      if (!foundLocal) {
      // Download from GitHub Releases
      const binaryName = `codex-${platformInfo.os}-${platformInfo.archName}${platformInfo.ext}`;
      const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${binaryName}`;
      
      console.log(`⬇️  Downloading: ${downloadUrl}`);
      
      const tempPath = path.join(binDir, 'codex.tmp');
      
      // Download binary
      await downloadFile(downloadUrl, tempPath);
      console.log('✅ Download complete');
      
      // Move to final location
      fs.renameSync(tempPath, binaryPath);
      }
    }
    
    // Make executable (Unix)
    if (platformInfo.os !== 'windows') {
      fs.chmodSync(binaryPath, 0o755);
    }
    
    console.log(`✅ Installed: ${binaryPath}`);
    console.log('');
    console.log('🎉 Installation complete!');
    console.log('');
    console.log('Run: codex --version');
    console.log('Or:  npx @zapabob/codex --version');
    console.log('Or:  pnpm exec codex --version');
    
  } catch (error) {
    console.error('❌ Installation failed:', error.message);
    console.error('');
    console.error('💡 Alternative: Install from source');
    console.error('   git clone https://github.com/zapabob/codex.git');
    console.error('   cd codex/codex-rs');
    console.error('   cargo install --path cli');
    // Don't exit with error - allow fallback to cargo install
    process.exit(0);
  }
}

if (require.main === module) {
  main();
}



