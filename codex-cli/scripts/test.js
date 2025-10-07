#!/usr/bin/env node
/**
 * Test script for the npm package.
 * Verifies that the binary can be executed.
 */

import { spawn } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const codexScript = path.join(__dirname, '..', 'bin', 'codex.js');

console.log('🧪 Testing codex binary...\n');

const child = spawn('node', [codexScript, '--version'], {
  stdio: 'inherit',
});

child.on('error', (err) => {
  console.error('❌ Test failed:', err.message);
  process.exit(1);
});

child.on('exit', (code) => {
  if (code === 0) {
    console.log('\n✅ Test passed!');
  } else {
    console.error(`\n❌ Test failed with exit code ${code}`);
    process.exit(code || 1);
  }
});

