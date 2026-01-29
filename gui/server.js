const express = require('express');
const cors = require('cors');
const os = require('os');
const si = require('systeminformation');
const { spawn } = require('child_process');
const http = require('http');
const { Server } = require('ws');

const app = express();
app.use(cors());
app.use(express.json());

// Function to get real system metrics
async function getRealSystemMetrics() {
  try {
    // Get CPU usage
    const cpuLoad = await si.currentLoad();
    const cpuUsage = Math.round(cpuLoad.currentLoad);

    // Get memory usage
    const memInfo = await si.mem();
    const totalMem = memInfo.total;
    const usedMem = memInfo.used;
    const memoryUsage = Math.round((usedMem / totalMem) * 100);

    // Get disk usage (first drive)
    const diskInfo = await si.fsSize();
    const diskUsage = diskInfo.length > 0 ? Math.round(diskInfo[0].use) : 50;

    // Get GPU information with detailed metrics
    const gpuInfo = await si.graphics();
    let gpuUsage = 0;
    let gpuMemoryUsed = 0;
    let gpuMemoryTotal = 0;
    let gpuMemoryUsage = 0;
    let gpuTemperature = null;
    let gpuName = null;
    let gpuVendor = 'unknown';

    if (gpuInfo.controllers && gpuInfo.controllers.length > 0) {
      const gpu = gpuInfo.controllers[0];
      gpuUsage = gpu.utilizationGpu || 0;
      gpuMemoryUsed = gpu.memoryUsed || 0;
      gpuMemoryTotal = gpu.memoryTotal || 0;
      gpuMemoryUsage = gpuMemoryTotal > 0 
        ? Math.round((gpuMemoryUsed / gpuMemoryTotal) * 100) 
        : 0;
      gpuTemperature = gpu.temperatureGpu || null;
      gpuName = gpu.name || gpu.model || null;
      
      // Determine vendor from name/model
      if (gpuName) {
        const nameLower = gpuName.toLowerCase();
        if (nameLower.includes('nvidia') || nameLower.includes('geforce') || nameLower.includes('rtx') || nameLower.includes('gtx')) {
          gpuVendor = 'nvidia';
        } else if (nameLower.includes('amd') || nameLower.includes('radeon') || nameLower.includes('rx')) {
          gpuVendor = 'amd';
        } else if (nameLower.includes('intel') || nameLower.includes('iris') || nameLower.includes('uhd')) {
          gpuVendor = 'intel';
        }
      }
    }

    // Get active processes
    const processes = await si.processes();
    const activeProcesses = processes.list ? processes.list.length : os.cpus().length;

    // Get uptime
    const uptime = Math.floor(os.uptime());

    return {
      cpu_usage: cpuUsage,
      memory_usage: memoryUsage,
      disk_usage: diskUsage,
      gpu_usage: gpuUsage,
      gpu_memory_used: gpuMemoryUsed,
      gpu_memory_total: gpuMemoryTotal,
      gpu_memory_usage: gpuMemoryUsage,
      gpu_temperature: gpuTemperature,
      gpu_name: gpuName,
      gpu_vendor: gpuVendor,
      active_processes: activeProcesses,
      uptime: uptime
    };
  } catch (error) {
    console.error('Error getting system metrics:', error);
    // Fallback to minimal data if system information fails
    return {
      cpu_usage: 0,
      memory_usage: 0,
      disk_usage: 0,
      gpu_usage: 0,
      active_processes: 0,
      uptime: Math.floor(os.uptime())
    };
  }
}

// API Routes
app.get('/api/system/metrics', async (req, res) => {
  try {
    const metrics = await getRealSystemMetrics();
    res.json(metrics);
  } catch (error) {
    console.error('Error in /api/system/metrics:', error);
    res.status(500).json({ error: 'Failed to get system metrics' });
  }
});

// Mock actions endpoint (to keep GUI components happy)
app.get('/api/actions', (req, res) => {
  // Return the same structure as mock-server
  res.json([
    {
      id: 'ask',
      label: 'Ask an Agent',
      description: 'Send a quick question or mention a specialized agent',
      category: 'Collaboration',
      ctaLabel: 'Send request',
      fields: [{ id: 'prompt', label: 'Task or question', kind: 'textArea', placeholder: '@code-reviewer Review the changes', required: true }]
    },
    // ... we can add more if needed
  ]);
});

// CLI Execution Endpoint
app.post('/api/cli/execute', (req, res) => {
  const { command, args } = req.body;
  
  if (command !== 'codex') {
    return res.status(400).json({ error: 'Only codex command is allowed' });
  }

  // Security note: In a real production app, ensure strict validation of args
  // For now, we allow execution to demonstrate functionality
  
  const child = spawn('codex', args || [], { shell: true });
  
  let stdout = '';
  let stderr = '';

  child.stdout.on('data', (data) => {
    stdout += data.toString();
  });

  child.stderr.on('data', (data) => {
    stderr += data.toString();
  });

  child.on('close', (code) => {
    res.json({
      id: 'exec-' + Date.now(),
      command: ['codex', ...(args || [])],
      executed_at: new Date().toISOString(),
      duration_ms: 0, // Simplified
      status: code === 0 ? 'completed' : 'failed',
      exit_code: code,
      stdout: stdout,
      stderr: stderr
    });
  });
});

// WebSocket Server
const server = http.createServer(app);
const wss = new Server({ server });

wss.on('connection', (ws) => {
  console.log('WebSocket client connected');

  // Send periodic real system metrics
  const interval = setInterval(async () => {
    try {
      const metrics = await getRealSystemMetrics();
      if (ws.readyState === ws.OPEN) {
        ws.send(JSON.stringify({
          type: 'system_metrics',
          data: metrics
        }));
      }
    } catch (error) {
      console.error('Error sending WebSocket metrics:', error);
    }
  }, 2000);

  ws.on('close', () => {
    clearInterval(interval);
    console.log('WebSocket client disconnected');
  });
});

const PORT = 8787;
server.listen(PORT, () => {
  console.log(`Production API server running on port ${PORT}`);
  console.log(`- Metrics: http://localhost:${PORT}/api/system/metrics`);
  console.log(`- CLI: POST http://localhost:${PORT}/api/cli/execute`);
});
