const express = require('express');
const cors = require('cors');
const os = require('os');
const si = require('systeminformation');

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
      gpuUsage = gpu.utilizationGpu || gpu.utilizationGpu || 0;
      gpuMemoryUsed = gpu.memoryUsed || gpu.memoryUsed || 0;
      gpuMemoryTotal = gpu.memoryTotal || gpu.memoryTotal || 0;
      gpuMemoryUsage = gpuMemoryTotal > 0 
        ? Math.round((gpuMemoryUsed / gpuMemoryTotal) * 100) 
        : 0;
      gpuTemperature = gpu.temperatureGpu || gpu.temperatureGpu || null;
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
    // Fallback to mock data if system information is not available
    return {
      cpu_usage: Math.floor(Math.random() * 100),
      memory_usage: Math.floor(Math.random() * 100),
      disk_usage: Math.floor(Math.random() * 100),
      gpu_usage: Math.floor(Math.random() * 100),
      active_processes: Math.floor(Math.random() * 500) + 50,
      uptime: Math.floor(Math.random() * 86400) // Random uptime up to 1 day
    };
  }
}

// Mock API responses (will be replaced with real data)
const mockResponses = {
  '/api/system/metrics': async () => await getRealSystemMetrics(),
  '/api/actions': [
    {
      id: 'ask',
      label: 'Ask an Agent',
      description: 'Send a quick question or mention a specialized agent',
      category: 'Collaboration',
      ctaLabel: 'Send request',
      fields: [
        {
          id: 'prompt',
          label: 'Task or question',
          kind: 'textArea',
          placeholder: '@code-reviewer Review the changes in src/main.rs',
          required: true
        }
      ]
    },
    {
      id: 'delegate',
      label: 'Delegate to Specialist',
      description: 'Assign a scoped goal to a dedicated specialist agent',
      category: 'Collaboration',
      ctaLabel: 'Delegate task',
      fields: [
        {
          id: 'agent',
          label: 'Agent',
          kind: 'select',
          required: true,
          options: [
            { value: 'code-reviewer', label: 'Code Reviewer' },
            { value: 'security-expert', label: 'Security Expert' }
          ]
        },
        {
          id: 'goal',
          label: 'Delegated goal',
          kind: 'textArea',
          required: true
        }
      ]
    }
  ],
  '/api/mcp/connections': [
    {
      id: 'filesystem-1',
      name: 'Local Filesystem',
      type: 'filesystem',
      status: 'connected',
      url: 'file:///',
      lastConnected: new Date().toISOString(),
      requestCount: 42,
      avgResponseTime: 15.7
    },
    {
      id: 'github-1',
      name: 'GitHub Integration',
      type: 'github',
      status: 'connected',
      url: 'https://api.github.com',
      lastConnected: new Date().toISOString(),
      requestCount: 28,
      avgResponseTime: 120.5
    }
  ],
  '/api/user': {
    id: 'default-user',
    name: 'Codex User',
    email: 'user@codex.local'
  }
};

// API routes
app.get('/api/system/metrics', async (req, res) => {
  try {
    const metrics = await getRealSystemMetrics();
    res.json(metrics);
  } catch (error) {
    console.error('Error in /api/system/metrics:', error);
    res.status(500).json({ error: 'Failed to get system metrics' });
  }
});

app.get('/api/actions', (req, res) => {
  res.json(mockResponses['/api/actions']);
});

app.post('/api/actions/:id/execute', (req, res) => {
  // Simulate action execution with realistic delay
  const delay = Math.random() * 2000 + 1000; // 1-3 seconds
  setTimeout(() => {
    const executionId = 'exec-' + Date.now();
    res.json({
      id: executionId,
      action_id: req.params.id,
      command: ['codex', req.params.id],
      executed_at: new Date().toISOString(),
      duration_ms: delay,
      status: 'completed',
      exit_code: 0,
      stdout: `Action ${req.params.id} executed successfully\nCompleted in ${delay.toFixed(0)}ms`,
      stderr: ''
    });
  }, delay);
});

app.get('/api/mcp/connections', (req, res) => {
  res.json(mockResponses['/api/mcp/connections']);
});

app.get('/api/user', (req, res) => {
  res.json(mockResponses['/api/user']);
});

app.get('/api/conversations', (req, res) => {
  res.json(conversations);
});

let conversations = [];
let messages = {};

app.post('/api/conversations', (req, res) => {
  const conversation = {
    id: 'conv-' + Date.now(),
    model: req.body.model || 'gpt-4',
    status: 'active',
    created_at: new Date().toISOString(),
    last_activity: new Date().toISOString(),
    message_count: req.body.initial_message ? 1 : 0,
    summary: null
  };
  conversations.push(conversation);
  messages[conversation.id] = [];
  res.json(conversation);
});

app.get('/api/conversations/:id/messages', (req, res) => {
  const conversationId = req.params.id;
  res.json(messages[conversationId] || []);
});

app.post('/api/conversations/:id/messages', (req, res) => {
  const conversationId = req.params.id;
  const message = {
    id: 'msg-' + Date.now(),
    role: req.body.role || 'user',
    content: req.body.content,
    timestamp: new Date().toISOString()
  };

  if (!messages[conversationId]) {
    messages[conversationId] = [];
  }
  messages[conversationId].push(message);

  // Update conversation
  const conversation = conversations.find(c => c.id === conversationId);
  if (conversation) {
    conversation.last_activity = new Date().toISOString();
    conversation.message_count = messages[conversationId].length;
  }

  res.json(message);
});

// WebSocket support for real-time updates
const http = require('http');
const { Server } = require('ws');

const server = http.createServer(app);
const wss = new Server({ server });

wss.on('connection', (ws) => {
  console.log('WebSocket client connected');

  // Send periodic real system metrics updates
  const interval = setInterval(async () => {
    try {
      const metrics = await getRealSystemMetrics();
      ws.send(JSON.stringify({
        type: 'system_metrics',
        data: metrics
      }));
    } catch (error) {
      console.error('Error sending WebSocket metrics:', error);
      // Send fallback data
      ws.send(JSON.stringify({
        type: 'system_metrics',
        data: {
          cpu_usage: Math.floor(Math.random() * 100),
          memory_usage: Math.floor(Math.random() * 100),
          disk_usage: Math.floor(Math.random() * 100),
          gpu_usage: Math.floor(Math.random() * 100),
          gpu_memory_used: Math.floor(Math.random() * 8000) + 1000,
          gpu_memory_total: 8192,
          gpu_memory_usage: Math.floor(Math.random() * 100),
          gpu_temperature: Math.floor(Math.random() * 30) + 50,
          gpu_name: 'Mock GPU',
          gpu_vendor: 'unknown',
          active_processes: Math.floor(Math.random() * 500) + 50,
          uptime: Math.floor(Math.random() * 86400)
        }
      }));
    }
  }, 2000); // Update every 2 seconds for more responsive UI

  ws.on('close', () => {
    clearInterval(interval);
    console.log('WebSocket client disconnected');
  });
});

const PORT = 8787;
server.listen(PORT, () => {
  console.log(`Mock API server running on port ${PORT}`);
});

module.exports = app;
