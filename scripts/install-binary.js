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

const VERSION = '2.8.0';
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
    const request = https.get(url, { followAllRedirects: true }, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      
      if (response.statusCode === 404) {
        const error = new Error(`HTTP 404`);
        error.statusCode = 404;
        reject(error);
        return;
      }
      
      if (response.statusCode !== 200) {
        const error = new Error(`Failed to download: HTTP ${response.statusCode}`);
        error.statusCode = response.statusCode;
        reject(error);
        return;
      }
      
      const file = fs.createWriteStream(dest);
      pipeline(response, file)
        .then(() => resolve())
        .catch(reject);
    });
    
    request.on('error', reject);
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
  console.log('📦 Installing @zapabob/codex-cli v' + VERSION);
  
  try {
    const platformInfo = getPlatformInfo();
    console.log(`🖥️  Platform: ${platformInfo.os}-${platformInfo.archName}`);
    
    // Binary filename
    const binaryName = `codex-${platformInfo.os}-${platformInfo.archName}${platformInfo.ext}`;
    const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${binaryName}`;
    
    console.log(`⬇️  Downloading: ${downloadUrl}`);
    
    // Ensure bin directory exists
    const binDir = path.join(__dirname, '..', 'bin');
    if (!fs.existsSync(binDir)) {
      fs.mkdirSync(binDir, { recursive: true });
    }
    
    const binaryPath = path.join(binDir, 'codex' + platformInfo.ext);
    const tempPath = path.join(binDir, 'codex.tmp');
    
    // Download binary
    await downloadFile(downloadUrl, tempPath);
    console.log('✅ Download complete');
    
    // Move to final location
    fs.renameSync(tempPath, binaryPath);
    
    // Make executable (Unix)
    if (platformInfo.os !== 'windows') {
      fs.chmodSync(binaryPath, 0o755);
    }
    
    console.log(`✅ Installed: ${binaryPath}`);
    console.log('');
    console.log('🎉 Installation complete!');
    console.log('');
    console.log('Run: codex --version');
    console.log('Or:  npx @zapabob/codex-cli --version');
    
  } catch (error) {
    // If binary doesn't exist (404), warn but don't fail installation
    const is404 = error.statusCode === 404 || 
                  error.message.includes('404') || 
                  error.message.includes('Failed to download: HTTP 404');
    
    if (is404) {
      console.warn('⚠️  Binary not available for this version/platform');
      console.warn('   This is expected if the release hasn\'t been published yet.');
      console.warn('');
      console.warn('💡 To use codex, either:');
      console.warn('   1. Wait for the release to be published');
      console.warn('   2. Install from source:');
      console.warn('      git clone https://github.com/zapabob/codex.git');
      console.warn('      cd codex/codex-rs');
      console.warn('      cargo install --path cli');
      console.warn('');
      console.warn('   Installation will continue without the binary.');
      // Don't exit with error - allow installation to continue
      process.exit(0);
    }
    
    // For other errors, show full error message
    console.error('❌ Installation failed:', error.message);
    console.error('');
    console.error('💡 Alternative: Install from source');
    console.error('   git clone https://github.com/zapabob/codex.git');
    console.error('   cd codex/codex-rs');
    console.error('   cargo install --path cli');
    // Only exit with error for non-404 errors
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}



