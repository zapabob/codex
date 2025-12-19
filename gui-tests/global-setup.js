const { spawn } = require('child_process');

module.exports = async function globalSetup() {
  console.log('Starting mock API server...');

  // Start the mock server
  const serverProcess = spawn('node', ['mock-server.js'], {
    cwd: __dirname,
    detached: false,
    stdio: ['ignore', 'pipe', 'pipe']
  });

  // Wait for server to start
  await new Promise((resolve, reject) => {
    let started = false;
    const timeout = setTimeout(() => {
      if (!started) {
        reject(new Error('Mock server failed to start within 10 seconds'));
      }
    }, 10000);

    serverProcess.stdout.on('data', (data) => {
      const output = data.toString();
      console.log('Mock server output:', output);
      if (output.includes('Mock API server running on port 8787')) {
        started = true;
        clearTimeout(timeout);
        resolve();
      }
    });

    serverProcess.stderr.on('data', (data) => {
      console.error('Mock server error:', data.toString());
    });

    serverProcess.on('close', (code) => {
      if (!started) {
        reject(new Error(`Mock server exited with code ${code}`));
      }
    });

    // Store process ID for cleanup
    global.mockServerProcess = serverProcess;
  });

  console.log('Mock API server started successfully');
};
