import {
  APIRequest,
  APIResponse,
  APIError,
  Conversation,
  Message,
  Model,
  Agent,
  MCPConnection,
  SecurityScan,
  ResearchResult,
  WebResearchResult,
  SystemMetrics,
  LoginForm,
  NewConversationForm,
} from '../types';
import { AITool, AISession, DevelopmentTask } from '../types/ai-tools';

class CodexAPIError extends Error {
  constructor(
    public code: number,
    message: string,
    public data?: any
  ) {
    super(message);
    this.name = 'CodexAPIError';
  }
}

export class CodexAPIClient {
  private baseUrl: string;
  private wsConnection?: WebSocket;
  private requestId = 0;
  private pendingRequests = new Map<string | number, {
    resolve: (value: any) => void;
    reject: (error: Error) => void;
  }>();

  constructor(baseUrl = 'http://localhost:8787') {
    this.baseUrl = baseUrl;
  }

  // HTTP Request helper
  private async httpRequest<T>(
    method: string,
    params?: any,
    endpoint?: string
  ): Promise<T> {
    const id = ++this.requestId;
    const request: APIRequest = {
      method,
      params,
      id,
    };

    try {
      const response = await fetch(endpoint || this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      });

      if (!response.ok) {
        throw new CodexAPIError(
          response.status,
          `HTTP ${response.status}: ${response.statusText}`
        );
      }

      const data: APIResponse<T> = await response.json();

      if (data.error) {
        throw new CodexAPIError(data.error.code, data.error.message, data.error.data);
      }

      return data.result!;
    } catch (error) {
      if (error instanceof CodexAPIError) {
        throw error;
      }
      throw new CodexAPIError(-1, `Network error: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  // WebSocket connection for real-time updates
  connectWebSocket(onMessage: (message: any) => void): void {
    try {
      this.wsConnection = new WebSocket(`ws://localhost:8787`);

      this.wsConnection.onopen = () => {
        console.log('WebSocket connected');
      };

      this.wsConnection.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          onMessage(message);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      this.wsConnection.onclose = () => {
        console.log('WebSocket disconnected');
        // Auto-reconnect after 5 seconds
        setTimeout(() => this.connectWebSocket(onMessage), 5000);
      };

      this.wsConnection.onerror = (error) => {
        console.error('WebSocket error:', error);
      };
    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
    }
  }

  disconnectWebSocket(): void {
    if (this.wsConnection) {
      this.wsConnection.close();
      this.wsConnection = undefined;
    }
  }

  // Authentication
  async login(credentials: LoginForm): Promise<{ token?: string; authUrl?: string }> {
    const params = credentials.method === 'api-key'
      ? { type: 'apiKey', apiKey: credentials.apiKey }
      : { type: 'chatgpt' };

    return this.httpRequest('account/login', params);
  }

  async logout(): Promise<void> {
    return this.httpRequest('account/logout');
  }

  async getAccount(): Promise<any> {
    return this.httpRequest('account/read');
  }

  // Conversations
  async createConversation(config: NewConversationForm): Promise<Conversation> {
    const params = {
      model: config.model,
      initialMessage: config.initialMessage,
      // Add attachments handling if needed
    };
    return this.httpRequest('newConversation', params);
  }

  async sendMessage(
    conversationId: string,
    content: string,
    attachments?: File[]
  ): Promise<Message> {
    const params = {
      conversationId,
      items: [{
        type: 'text',
        text: content,
      }],
      // Handle attachments if needed
    };
    return this.httpRequest('sendUserTurn', params);
  }

  async listConversations(): Promise<Conversation[]> {
    return this.httpRequest('listConversations');
  }

  async resumeConversation(path: string): Promise<Conversation> {
    return this.httpRequest('resumeConversation', { path });
  }

  async archiveConversation(conversationId: string, path: string): Promise<void> {
    return this.httpRequest('archiveConversation', { conversationId, rolloutPath: path });
  }

  // Models
  async listModels(): Promise<Model[]> {
    const response = await this.httpRequest<{ items: Model[] }>('model/list');
    return response.items;
  }

  // Agents - Get from backend API
  async getAgents(): Promise<Agent[]> {
    try {
      // Fetch actions from backend API
      const response = await fetch(`${this.baseUrl}/api/actions`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new CodexAPIError(
          response.status,
          `Failed to fetch actions: ${response.statusText}`
        );
      }

      const actions = await response.json() as Array<{
        id: string;
        label: string;
        description: string;
        category: string;
      }>;

      // Map actions to agents
      return actions.map(action => ({
        id: action.id,
        name: action.label,
        type: action.id,
        status: 'idle' as const,
        description: action.description,
      }));
    } catch (error) {
      console.error('Failed to fetch agents from backend:', error);
      // Fallback to default agents if backend is unavailable
    return [
      {
        id: 'code-reviewer',
        name: 'Code Reviewer',
        type: 'code-reviewer',
        status: 'idle',
        description: 'コードの品質とセキュリティをレビューします',
      },
      {
        id: 'test-gen',
        name: 'Test Generator',
        type: 'test-gen',
        status: 'idle',
        description: '自動的にテストコードを生成します',
      },
      {
        id: 'sec-audit',
        name: 'Security Auditor',
        type: 'sec-audit',
        status: 'idle',
        description: 'セキュリティ脆弱性をスキャンします',
      },
      {
        id: 'researcher',
        name: 'Deep Researcher',
        type: 'researcher',
        status: 'idle',
        description: '高度な研究と分析を行います',
      },
    ];
    }
  }

  async runAgent(agentId: string, context: any): Promise<any> {
    try {
      // Map agent context to action values
      const values: Record<string, string> = {};
      
      // Map context to action-specific fields
      if (agentId === 'review') {
        values.task = context.code || context.path || context.task || '';
      } else if (agentId === 'audit') {
        values.task = context.path || context.task || '';
      } else if (agentId === 'research') {
        values.topic = context.query || context.topic || '';
        values.depth = context.depth?.toString() || '3';
        values.breadth = context.breadth?.toString() || '8';
      } else if (agentId === 'web-research') {
        values.query = context.query || context.topic || context.prompt || '';
      } else if (agentId === 'delegate') {
        values.agent = context.agent || 'code-reviewer';
        values.goal = context.goal || context.code || context.task || '';
        if (context.scope) {
          values.scope = context.scope;
        }
      } else if (agentId === 'ask') {
        values.prompt = context.prompt || context.code || context.query || '';
      } else {
        // Fallback: try to map common fields
        if (context.code) values.code = context.code;
        if (context.task) values.task = context.task;
        if (context.query) values.query = context.query;
        if (context.prompt) values.prompt = context.prompt;
      }

      // Call backend API to execute action
      const response = await fetch(`${this.baseUrl}/api/actions/${agentId}/execute`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ values }),
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new CodexAPIError(
          response.status,
          errorData.message || `Failed to execute action: ${response.statusText}`
        );
    }

      const result = await response.json() as {
        id: string;
        action_id: string;
        command: string[];
        executed_at: string;
        duration_ms: number;
        status: 'completed' | 'failed';
        exit_code: number | null;
        stdout: string;
        stderr: string;
      };

      // Map result to appropriate return type based on agent type
      if (agentId === 'audit') {
        return {
          id: result.id,
          type: 'code',
          status: result.status === 'completed' ? 'completed' : 'failed',
          findings: this.parseSecurityFindings(result.stdout, result.stderr),
          startedAt: new Date(result.executed_at),
          completedAt: new Date(new Date(result.executed_at).getTime() + result.duration_ms),
        } as SecurityScan;
      } else if (agentId === 'research') {
        return {
          id: result.id,
          query: values.topic || '',
          status: result.status === 'completed' ? 'completed' : 'failed',
          sources: this.parseResearchSources(result.stdout),
          startedAt: new Date(result.executed_at),
          completedAt: new Date(new Date(result.executed_at).getTime() + result.duration_ms),
        } as ResearchResult;
      } else if (agentId === 'web-research') {
        return {
          id: result.id,
          query: values.query || '',
          status: result.status === 'completed' ? 'completed' : 'failed',
          output: result.status === 'completed' ? result.stdout : result.stderr,
          startedAt: new Date(result.executed_at),
          completedAt: new Date(new Date(result.executed_at).getTime() + result.duration_ms),
          error: result.status === 'completed' ? undefined : result.stderr,
        } as WebResearchResult;
      } else {
        // Generic result for other agent types
        return {
          status: result.status === 'completed' ? 'completed' : 'failed',
          output: result.stdout,
          error: result.stderr,
          exitCode: result.exit_code,
          duration: result.duration_ms,
        };
  }
    } catch (error) {
      if (error instanceof CodexAPIError) {
        throw error;
      }
      throw new CodexAPIError(-1, `Failed to run agent: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  async runResearch(payload: { query: string }): Promise<ResearchResult> {
    return this.runAgent('research', { query: payload.query }) as Promise<ResearchResult>;
  }

  async runWebResearch(payload: { query: string }): Promise<WebResearchResult> {
    return this.runAgent('web-research', { query: payload.query }) as Promise<WebResearchResult>;
  }

  private parseSecurityFindings(stdout: string, stderr: string): Array<{
    id: string;
    severity: 'critical' | 'high' | 'medium' | 'low';
    title: string;
    description: string;
    location?: { file: string; line?: number };
    recommendation: string;
  }> {
    const findings: Array<{
      id: string;
      severity: 'critical' | 'high' | 'medium' | 'low';
      title: string;
      description: string;
      location?: { file: string; line?: number };
      recommendation: string;
    }> = [];

    // Try to parse JSON output
    try {
      const jsonMatch = stdout.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        const parsed = JSON.parse(jsonMatch[0]);
        if (Array.isArray(parsed.findings)) {
          return parsed.findings;
        }
      }
    } catch (e) {
      // Not JSON, continue with text parsing
    }

    // Parse text output for security findings
    const lines = (stdout + '\n' + stderr).split('\n');
    let currentFinding: any = null;

    for (const line of lines) {
      const severityMatch = line.match(/(critical|high|medium|low)/i);
      if (severityMatch) {
        if (currentFinding) {
          findings.push(currentFinding);
        }
        currentFinding = {
          id: `finding-${findings.length + 1}`,
          severity: severityMatch[1].toLowerCase() as 'critical' | 'high' | 'medium' | 'low',
          title: line.trim(),
          description: '',
          recommendation: '',
        };
      } else if (currentFinding) {
        if (line.includes('file:') || line.includes('File:')) {
          const fileMatch = line.match(/(?:file|File):\s*([^\s:]+)(?::(\d+))?/);
          if (fileMatch) {
            currentFinding.location = {
              file: fileMatch[1],
              line: fileMatch[2] ? parseInt(fileMatch[2], 10) : undefined,
            };
          }
        } else if (line.trim()) {
          if (!currentFinding.description) {
            currentFinding.description = line.trim();
          } else {
            currentFinding.recommendation = line.trim();
          }
        }
      }
    }

    if (currentFinding) {
      findings.push(currentFinding);
    }

    return findings;
  }

  private parseResearchSources(stdout: string): Array<{
    id: string;
    title: string;
    url: string;
    snippet: string;
    publishedAt?: string;
  }> {
    const sources: Array<{
      id: string;
      title: string;
      url: string;
      snippet: string;
      publishedAt?: string;
    }> = [];

    // Try to parse JSON output
    try {
      const jsonMatch = stdout.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        const parsed = JSON.parse(jsonMatch[0]);
        if (Array.isArray(parsed.sources)) {
          return parsed.sources;
        }
      }
    } catch (e) {
      // Not JSON, continue with text parsing
    }

    // Parse text output for research sources
    const lines = stdout.split('\n');
    let currentSource: any = null;

    for (const line of lines) {
      const urlMatch = line.match(/https?:\/\/[^\s]+/);
      if (urlMatch) {
        if (currentSource) {
          sources.push(currentSource);
        }
        currentSource = {
          id: `source-${sources.length + 1}`,
          title: line.replace(urlMatch[0], '').trim() || `Source ${sources.length + 1}`,
          url: urlMatch[0],
          snippet: '',
        };
      } else if (currentSource && line.trim()) {
        currentSource.snippet = (currentSource.snippet + ' ' + line.trim()).trim();
      }
    }

    if (currentSource) {
      sources.push(currentSource);
    }

    return sources;
  }

  // MCP Connections - Get from backend (if available) or return default connections
  async getMCPConnections(): Promise<MCPConnection[]> {
    try {
      // Try to fetch MCP connections from backend API
      // Note: This endpoint may not exist yet, so we'll use a fallback
      const response = await fetch(`${this.baseUrl}/api/mcp/connections`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        const connections = await response.json() as MCPConnection[];
        return connections;
      }
    } catch (error) {
      console.warn('MCP connections endpoint not available, using defaults:', error);
    }

    // Fallback to default MCP connections
    return [
      {
        id: 'filesystem',
        name: 'File System',
        type: 'filesystem',
        status: 'connected',
        lastConnected: new Date(),
      },
      {
        id: 'github',
        name: 'GitHub',
        type: 'github',
        status: 'connected',
        lastConnected: new Date(),
      },
      {
        id: 'gemini',
        name: 'Gemini AI',
        type: 'gemini',
        status: 'connected',
        lastConnected: new Date(),
      },
    ];
  }

  // System metrics - Get from backend (if available) or return default metrics
  async getSystemMetrics(): Promise<SystemMetrics> {
    try {
      // Try to fetch system metrics from backend API
      const response = await fetch(`${this.baseUrl}/api/system/metrics`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        const data = await response.json();
        // Map backend response to SystemMetrics interface
        const metrics: SystemMetrics = {
          cpuUsage: data.cpu_usage || data.cpuUsage || 0,
          memoryUsage: data.memory_usage || data.memoryUsage || 0,
          diskUsage: data.disk_usage || data.diskUsage || 0,
          networkUsage: data.network_usage || data.networkUsage,
          activeProcesses: data.active_processes || data.activeProcesses || 0,
          uptime: data.uptime || 0,
          gpuUsage: data.gpu_usage || data.gpuUsage,
          gpuMemoryUsed: data.gpu_memory_used || data.gpuMemoryUsed,
          gpuMemoryTotal: data.gpu_memory_total || data.gpuMemoryTotal,
          gpuMemoryUsage: data.gpu_memory_usage || data.gpuMemoryUsage,
          gpuTemperature: data.gpu_temperature || data.gpuTemperature,
          gpuName: data.gpu_name || data.gpuName,
          gpuVendor: data.gpu_vendor || data.gpuVendor,
        };
        return metrics;
      }
    } catch (error) {
      console.warn('System metrics endpoint not available, trying CLI fallback:', error);
      
      // Fallback: Try to get metrics from CLI/TUI via codex command
      try {
        const cliResponse = await this.getSystemMetricsFromCLI();
        if (cliResponse) {
          return cliResponse;
        }
      } catch (cliError) {
        console.warn('CLI fallback also failed:', cliError);
      }
    }

    // Final fallback to default metrics (will be updated by WebSocket if available)
    return {
      cpuUsage: 0,
      memoryUsage: 0,
      diskUsage: 0,
      activeProcesses: 0,
      uptime: 0,
    };
  }

  // Get system metrics from CLI/TUI via codex command
  private async getSystemMetricsFromCLI(): Promise<SystemMetrics | null> {
    try {
      // Try to use DualBridge if available (via CodexContext)
      // This will be called from components that have access to the bridge
      // For now, we'll use a direct API call to the mock server or Rust backend
      
      // Option 1: Try Rust backend API endpoint
      const response = await fetch(`${this.baseUrl}/api/system/metrics`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        const data = await response.json();
        return {
          cpuUsage: data.cpu_usage || data.cpuUsage || 0,
          memoryUsage: data.memory_usage || data.memoryUsage || 0,
          diskUsage: data.disk_usage || data.diskUsage || 0,
          activeProcesses: data.active_processes || data.activeProcesses || 0,
          uptime: data.uptime || 0,
          gpuUsage: data.gpu_usage || data.gpuUsage,
          gpuMemoryUsed: data.gpu_memory_used || data.gpuMemoryUsed,
          gpuMemoryTotal: data.gpu_memory_total || data.gpuMemoryTotal,
          gpuMemoryUsage: data.gpu_memory_usage || data.gpuMemoryUsage,
          gpuTemperature: data.gpu_temperature || data.gpuTemperature,
          gpuName: data.gpu_name || data.gpuName,
          gpuVendor: data.gpu_vendor || data.gpuVendor,
        };
      }
    } catch (error) {
      console.warn('CLI system metrics fetch failed:', error);
    }
    return null;
  }

  // Execute codex CLI command via bridge
  async executeCodexCommand(args: string[]): Promise<any> {
    try {
      // This method can be used by components with access to DualBridge
      // For components without bridge access, we'll use HTTP API
      const response = await fetch(`${this.baseUrl}/api/cli/execute`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          command: 'codex',
          args: args,
        }),
      });

      if (response.ok) {
        return await response.json();
      }
    } catch (error) {
      console.error('Failed to execute codex command:', error);
      throw error;
    }
    return null;
  }

  async listAITools(): Promise<AITool[]> {
    try {
      const response = await fetch(`${this.baseUrl}/api/ai/tools`, { method: 'GET' });
      if (response.ok) {
        const payload = await response.json();
        const tools = Array.isArray(payload.tools) ? payload.tools : payload;
        return tools.map((tool: any) => ({
          id: tool.id,
          name: tool.name ?? tool.id,
          status: tool.status ?? 'available',
          capabilities: tool.capabilities ?? [],
          activeSessions: tool.activeSessions ?? 0,
          maxSessions: tool.maxSessions ?? 1,
          performance: {
            avgResponseTime: tool.performance?.avgResponseTime ?? 0,
            successRate: tool.performance?.successRate ?? 0,
            resourceUsage: tool.performance?.resourceUsage ?? 0,
          },
        }));
      }
    } catch (error) {
      console.warn('Backend AI tools endpoint unavailable, trying CLI bridge:', error);
    }

    try {
      const status = await this.executeCommand(['codex', 'status', '--json'], process.cwd());
      const parsed = JSON.parse(status.stdout || '{}');
      if (Array.isArray(parsed.tools)) {
        return parsed.tools.map((tool: any) => ({
          id: tool.id,
          name: tool.name ?? tool.id,
          status: tool.status ?? 'available',
          capabilities: tool.capabilities ?? [],
          activeSessions: tool.active_sessions ?? tool.activeSessions ?? 0,
          maxSessions: tool.max_sessions ?? tool.maxSessions ?? 1,
          performance: {
            avgResponseTime: tool.avg_response_time ?? tool.performance?.avgResponseTime ?? 0,
            successRate: tool.success_rate ?? tool.performance?.successRate ?? 0,
            resourceUsage: tool.resource_usage ?? tool.performance?.resourceUsage ?? 0,
          },
        }));
      }
    } catch (error) {
      console.error('CLI bridge could not provide AI tools:', error);
    }

    throw new CodexAPIError(-1, 'Failed to list AI tools from GUI');
  }

  async listAISessions(): Promise<AISession[]> {
    try {
      const response = await fetch(`${this.baseUrl}/api/ai/sessions`, { method: 'GET' });
      if (response.ok) {
        const payload = await response.json();
        const sessions = Array.isArray(payload.sessions) ? payload.sessions : payload;
        return sessions.map((session: any) => ({
          id: session.id,
          toolId: session.toolId ?? session.tool_id,
          taskId: session.taskId ?? session.task_id,
          status: session.status ?? 'running',
          startTime: new Date(session.startTime ?? session.start_time ?? Date.now()),
          endTime: session.endTime ? new Date(session.endTime) : session.end_time ? new Date(session.end_time) : undefined,
          progress: session.progress ?? 0,
          output: session.output ?? '',
          error: session.error,
        }));
      }
    } catch (error) {
      console.warn('Backend AI sessions endpoint unavailable, falling back to CLI bridge:', error);
    }

    try {
      const status = await this.executeCommand(['codex', 'status', '--json'], process.cwd());
      const parsed = JSON.parse(status.stdout || '{}');
      if (Array.isArray(parsed.sessions)) {
        return parsed.sessions.map((session: any) => ({
          id: session.id,
          toolId: session.toolId ?? session.tool_id,
          taskId: session.taskId ?? session.task_id,
          status: session.status ?? 'running',
          startTime: new Date(session.startTime ?? session.start_time ?? Date.now()),
          endTime: session.endTime ? new Date(session.endTime) : session.end_time ? new Date(session.end_time) : undefined,
          progress: session.progress ?? 0,
          output: session.output ?? '',
          error: session.error,
        }));
      }
    } catch (error) {
      console.error('CLI bridge could not provide AI sessions:', error);
    }

    return [];
  }

  async listDevelopmentTasks(): Promise<DevelopmentTask[]> {
    try {
      const response = await fetch(`${this.baseUrl}/api/ai/tasks`, { method: 'GET' });
      if (response.ok) {
        const payload = await response.json();
        const tasks = Array.isArray(payload.tasks) ? payload.tasks : payload;
        return tasks.map((task: any) => ({
          id: task.id,
          title: task.title ?? task.id,
          description: task.description ?? '',
          complexity: task.complexity ?? 'medium',
          priority: task.priority ?? 'medium',
          requirements: task.requirements ?? [],
          subtasks: task.subtasks ?? [],
          status: task.status ?? 'pending',
          createdAt: new Date(task.createdAt ?? Date.now()),
          assignedTools: task.assignedTools ?? [],
          progress: task.progress ?? 0,
        }));
      }
    } catch (error) {
      console.warn('Backend AI tasks endpoint unavailable, checking CLI bridge:', error);
    }

    try {
      const status = await this.executeCommand(['codex', 'status', '--json'], process.cwd());
      const parsed = JSON.parse(status.stdout || '{}');
      if (Array.isArray(parsed.tasks)) {
        return parsed.tasks.map((task: any) => ({
          id: task.id,
          title: task.title ?? task.id,
          description: task.description ?? '',
          complexity: task.complexity ?? 'medium',
          priority: task.priority ?? 'medium',
          requirements: task.requirements ?? [],
          subtasks: task.subtasks ?? [],
          status: task.status ?? 'pending',
          createdAt: new Date(task.created_at ?? Date.now()),
          assignedTools: task.assignedTools ?? task.assigned_tools ?? [],
          progress: task.progress ?? 0,
        }));
      }
    } catch (error) {
      console.error('CLI bridge could not provide AI tasks:', error);
    }

    return [];
  }

  // File operations
  async executeCommand(command: string[], cwd?: string): Promise<{ exitCode: number; stdout: string; stderr: string }> {
    return this.httpRequest('execOneOffCommand', {
      command,
      cwd: cwd || process.cwd(),
    });
  }

  async fuzzyFileSearch(query: string, roots: string[]): Promise<any[]> {
    return this.httpRequest('fuzzyFileSearch', { query, roots });
  }

  // Utility methods
  isConnected(): boolean {
    return this.wsConnection?.readyState === WebSocket.OPEN;
  }

  getBaseUrl(): string {
    return this.baseUrl;
  }

  setBaseUrl(url: string): void {
    this.baseUrl = url;
  }

  getBridgeWebSocketUrl(path = '/cli/bridge'): string {
    const endpoint = new URL(this.baseUrl);
    endpoint.protocol = endpoint.protocol === 'https:' ? 'wss:' : 'ws:';
    endpoint.pathname = path.startsWith('/') ? path : `/${path}`;
    return endpoint.toString();
  }
}

// Singleton instance
export const apiClient = new CodexAPIClient();

// Export types
export { CodexAPIError };
