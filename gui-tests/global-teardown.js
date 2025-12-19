module.exports = async function globalTeardown() {
  console.log('Stopping mock API server...');

  if (global.mockServerProcess) {
    global.mockServerProcess.kill('SIGTERM');

    // Wait for process to exit
    await new Promise((resolve) => {
      global.mockServerProcess.on('close', () => {
        console.log('Mock API server stopped');
        resolve();
      });

      // Force kill after 5 seconds
      setTimeout(() => {
        global.mockServerProcess.kill('SIGKILL');
        console.log('Mock API server force stopped');
        resolve();
      }, 5000);
    });
  }
};
