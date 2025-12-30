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
// GitHub's certificate fingerprint for pinning (SHA256 of github.com's certificate)
// This should be updated if GitHub changes their certificate
// To get the actual fingerprint, run:
// openssl s_client -connect github.com:443 -showcerts | openssl x509 -fingerprint -sha256 -noout
// Or use: echo | openssl s_client -servername github.com -connect github.com:443 2>/dev/null | openssl x509 -fingerprint -sha256 -noout
const GITHUB_CERT_PIN = 'sha256//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='; // Placeholder - update with actual cert pin
const GITHUB_CERT_PINS = [
  // Add multiple pins for certificate chain rotation
  'sha256//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=', // Placeholder
];
const MAX_REDIRECTS = 5; // Limit redirects to prevent redirect loops

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

// Download file from URL with redirect limit and host validation
async function downloadFile(url, dest, redirectCount = 0) {
  return new Promise((resolve, reject) => {
    // Validate URL to prevent SSRF attacks
    const urlObj = new URL(url);
    if (!urlObj.hostname.endsWith('github.com') && !urlObj.hostname.endsWith('githubusercontent.com')) {
      reject(new Error(`Invalid hostname: ${urlObj.hostname}. Only github.com and githubusercontent.com are allowed.`));
      return;
    }
    
    // Limit redirects to prevent redirect loops
    if (redirectCount >= MAX_REDIRECTS) {
      reject(new Error(`Too many redirects (max ${MAX_REDIRECTS})`));
      return;
    }
    
    const options = {
      hostname: urlObj.hostname,
      path: urlObj.pathname + urlObj.search,
      method: 'GET',
      // Reject self-signed certificates
      rejectUnauthorized: true,
      // Note: Certificate pinning verification should be done after connection
      // For full certificate pinning, use tls.connect with secureContext
      // or verify the certificate chain after the connection is established
      // TODO: Implement full certificate pinning with tls.connect
      // This requires fetching the certificate chain and verifying fingerprints
    };
    
    const request = https.get(options, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        const location = response.headers.location;
        if (!location) {
          reject(new Error('Redirect without Location header'));
          return;
        }
        // Resolve relative redirects
        const redirectUrl = new URL(location, url);
        downloadFile(redirectUrl.toString(), dest, redirectCount + 1).then(resolve).catch(reject);
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

// Validate and sanitize path to prevent command injection
function validatePath(filePath) {
  // Reject paths with null bytes, command separators, or other dangerous characters
  if (filePath.includes('\0') || 
      filePath.includes(';') || 
      filePath.includes('&') || 
      filePath.includes('|') || 
      filePath.includes('`') ||
      filePath.includes('$') ||
      filePath.includes('(') ||
      filePath.includes(')')) {
    throw new Error(`Invalid path: contains dangerous characters`);
  }
  
  // Ensure path is absolute and within expected directory
  const normalized = path.normalize(filePath);
  if (path.isAbsolute(normalized)) {
    return normalized;
  }
  throw new Error(`Invalid path: must be absolute`);
}

// Extract archive using parameterized commands to prevent injection
async function extractArchive(archivePath, destDir) {
  const { promisify } = require('util');
  const { spawn } = require('child_process');
  
  // Validate paths to prevent command injection
  const safeArchivePath = validatePath(path.resolve(archivePath));
  const safeDestDir = validatePath(path.resolve(destDir));
  
  const ext = path.extname(safeArchivePath);
  
  return new Promise((resolve, reject) => {
    let child;
    
    if (ext === '.zip') {
      // Windows: use PowerShell with parameterized arguments
      if (process.platform === 'win32') {
        // Use -File with a script block to avoid command injection
        // Escape single quotes by doubling them for PowerShell
        const escapedArchivePath = safeArchivePath.replace(/'/g, "''");
        const escapedDestDir = safeDestDir.replace(/'/g, "''");
        // Use -Command with properly escaped arguments
        const psCommand = `$archivePath = [System.IO.Path]::GetFullPath('${escapedArchivePath}'); $destPath = [System.IO.Path]::GetFullPath('${escapedDestDir}'); Expand-Archive -Path $archivePath -DestinationPath $destPath -Force`;
        child = spawn('powershell', [
          '-NoProfile',
          '-NonInteractive',
          '-Command',
          psCommand
        ]);
      } else {
        // Unix: try unzip first, fallback to 7z
        child = spawn('unzip', ['-q', '-o', safeArchivePath, '-d', safeDestDir]);
        child.on('error', () => {
          // Fallback to 7z if unzip not available
          // Use separate arguments to prevent command injection
          const child7z = spawn('7z', ['x', safeArchivePath, '-o' + safeDestDir, '-y']);
          child7z.on('close', (code) => {
            if (code === 0) resolve();
            else reject(new Error(`7z extraction failed with code ${code}`));
          });
          child7z.on('error', reject);
        });
      }
    } else if (safeArchivePath.endsWith('.tar.gz')) {
      // Unix: use tar with parameterized arguments
      child = spawn('tar', ['-xzf', safeArchivePath, '-C', safeDestDir]);
    } else {
      reject(new Error(`Unsupported archive format: ${ext}`));
      return;
    }
    
    if (child) {
      child.on('close', (code) => {
        if (code === 0) resolve();
        else reject(new Error(`Extraction failed with code ${code}`));
      });
      child.on('error', reject);
    }
  });
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
    const checksumUrl = `${downloadUrl}.sha256`;
    const checksumPath = path.join(binDir, 'codex.sha256.tmp');
    
    // Download checksum file if available
    let expectedHash = null;
    try {
      await downloadFile(checksumUrl, checksumPath);
      const checksumContent = fs.readFileSync(checksumPath, 'utf8').trim();
      // SHA256 checksum format: "hash  filename" or just "hash"
      expectedHash = checksumContent.split(/\s+/)[0];
      fs.unlinkSync(checksumPath);
      console.log('✅ Checksum file downloaded');
    } catch (error) {
      if (error.statusCode !== 404) {
        console.warn(`⚠️  Failed to download checksum: ${error.message}`);
      }
      // Continue without checksum if not available
    }
    
    // Download binary
    await downloadFile(downloadUrl, tempPath);
    console.log('✅ Download complete');
    
    // Verify SHA256 checksum if available
    if (expectedHash) {
      const isValid = await verifySHA256(tempPath, expectedHash);
      if (!isValid) {
        fs.unlinkSync(tempPath);
        throw new Error(`SHA256 checksum verification failed. Expected: ${expectedHash}`);
      }
      console.log('✅ SHA256 checksum verified');
    } else {
      console.warn('⚠️  No checksum file available - skipping verification');
    }
    
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



