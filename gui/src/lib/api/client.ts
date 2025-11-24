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
  SystemMetrics,
  LoginForm,
  NewConversationForm,
} from '../types';
import { OrchestratorClient } from '@zapabob/codex-protocol-client';

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
  private protocolClient: any = null;
  private isConnected = false;
  private requestId = 0;
  private pendingRequests = new Map<string, {
    resolve: (value: any) => void;
    reject: (error: Error) => void;
  }>();

  constructor() {
    // Initialize WebSocket connection to CLI server
    this.initializeConnection();
  }

  private initializeConnection() {
    try {
      // Direct WebSocket connection to CLI server
      this.protocolClient = new WebSocket('ws://localhost:3001');

      this.protocolClient.onopen = () => {
        console.log('Connected to Codex CLI Server');
        this.isConnected = true;
      };

      this.protocolClient.onmessage = (event: MessageEvent) => {
        try {
          const response = JSON.parse(event.data);
          this.handleResponse(response);
        } catch (error) {
          console.error('Failed to parse response:', error);
        }
      };

      this.protocolClient.onclose = () => {
        console.log('Disconnected from Codex CLI Server');
        this.isConnected = false;
        // Auto-reconnect after 5 seconds
        setTimeout(() => this.initializeConnection(), 5000);
      };

      this.protocolClient.onerror = (error) => {
        console.error('WebSocket error:', error);
      };
    } catch (error) {
      console.error('Failed to initialize connection:', error);
    }
  }

  private handleResponse(response: any) {
    const requestId = response.id?.toString();
    if (requestId && this.pendingRequests.has(requestId)) {
      const pending = this.pendingRequests.get(requestId)!;
      this.pendingRequests.delete(requestId);

      if (response.error) {
        pending.reject(new Error(`RPC Error: ${response.error.message}`));
      } else {
        pending.resolve(response.result);
      }
    }
  }

  private async sendRequest(method: string, params: any = {}): Promise<any> {
    // Wait for connection with timeout
    if (!this.isConnected || !this.protocolClient || this.protocolClient.readyState !== WebSocket.OPEN) {
      // Try to reconnect
      this.initializeConnection();
      
      // Wait for connection with timeout
      await new Promise<void>((resolve, reject) => {
        const timeout = setTimeout(() => {
          reject(new Error('Connection timeout: CLI server not available'));
        }, 5000);
        
        const checkConnection = setInterval(() => {
          if (this.isConnected && this.protocolClient?.readyState === WebSocket.OPEN) {
            clearInterval(checkConnection);
            clearTimeout(timeout);
            resolve();
          }
        }, 100);
      });
    }

    const id = (++this.requestId).toString();
    const request = {
      jsonrpc: '2.0',
      id,
      method,
      params,
    };

    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pendingRequests.delete(id);
        reject(new Error(`Request timeout for method: ${method}`));
      }, 30000); // 30 second timeout

      this.pendingRequests.set(id, {
        resolve: (result: any) => {
          clearTimeout(timeout);
          resolve(result);
        },
        reject: (error: Error) => {
          clearTimeout(timeout);
          reject(error);
        },
      });

      try {
        this.protocolClient.send(JSON.stringify(request));
      } catch (error) {
        clearTimeout(timeout);
        this.pendingRequests.delete(id);
        reject(new Error(`Failed to send request: ${error instanceof Error ? error.message : 'Unknown error'}`));
      }
    });
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

  // WebSocket connection for real-time updates (legacy method)
  connectWebSocket(onMessage: (message: any) => void): void {
    // Use the main connection for real-time updates
    if (this.protocolClient) {
      this.protocolClient.addEventListener('message', (event: MessageEvent) => {
        try {
          const message = JSON.parse(event.data);
          onMessage(message);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      });
    }
  }

  disconnectWebSocket(): void {
    // Connection is managed automatically
  }

  // Authentication
  async login(credentials: LoginForm): Promise<{ token?: string; authUrl?: string }> {
    const params = credentials.method === 'api-key'
      ? { type: 'apiKey', apiKey: credentials.apiKey }
      : { type: 'chatgpt' };

    return await this.sendRequest('account.login', params);
  }

  async logout(): Promise<void> {
    try {
      await this.sendRequest('account.logout');
    } catch (error) {
      console.warn('CLI server not available:', error);
    }
  }

  async getAccount(): Promise<any> {
    return await this.sendRequest('account.read');
  }

  // Conversations
  async createConversation(config: NewConversationForm): Promise<Conversation> {
    const params = {
      model: config.model,
      initialMessage: config.initialMessage,
    };

    const result = await this.sendRequest('conversation.create', params);
    return {
      id: result.id || `conv-${Date.now()}`,
      title: result.title || `New Conversation`,
      createdAt: new Date(result.createdAt || Date.now()),
      updatedAt: new Date(result.updatedAt || Date.now()),
      model: config.model,
      messageCount: 1,
    };
  }

  async sendMessage(
    conversationId: string,
    content: string,
    attachments?: File[]
  ): Promise<Message> {
    const params = {
      conversationId,
      content,
      // Handle attachments if needed
    };

    const result = await this.sendRequest('conversation.sendMessage', params);
    return {
      id: result.id || `msg-${Date.now()}`,
      conversationId,
      role: result.role || 'assistant',
      content: result.content || 'Response from AI',
      createdAt: new Date(result.createdAt || Date.now()),
    };
  }

  async listConversations(): Promise<Conversation[]> {
    const result = await this.sendRequest('conversation.list');
    return result.conversations || [];
  }

  async resumeConversation(path: string): Promise<Conversation> {
    try {
      const result = await this.sendRequest('conversation.resume', { path });
      return result.conversation;
    } catch (error) {
      console.warn('CLI server not available:', error);
      throw error;
    }
  }

  async archiveConversation(conversationId: string, path: string): Promise<void> {
    try {
      await this.sendRequest('conversation.archive', { conversationId, path });
    } catch (error) {
      console.warn('CLI server not available:', error);
    }
  }

  // Models
  async listModels(): Promise<Model[]> {
    const response = await this.httpRequest<{ items: Model[] }>('model/list');
    return response.items;
  }

  // Agents - Get from CLI via RPC
  async getAgents(): Promise<Agent[]> {
    try {
      const result = await this.sendRequest('agent.list');
      return result.agents || this.getDefaultAgents();
    } catch (error) {
      console.warn('CLI server not available, using default agents:', error);
      return this.getDefaultAgents();
    }
  }

  private getDefaultAgents(): Agent[] {
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

  async runAgent(agentId: string, context: any): Promise<any> {
    try {
      // Map agent context to parameters
      const params: any = { agentId };
      
      // Map context to agent-specific fields
      if (agentId === 'code-reviewer' || agentId === 'review') {
        params.task = context.code || context.path || context.task || '';
      } else if (agentId === 'sec-audit' || agentId === 'audit') {
        params.path = context.path || context.task || '';
      } else if (agentId === 'researcher' || agentId === 'research') {
        params.query = context.query || context.topic || '';
        params.depth = context.depth || 3;
      } else {
        // Generic context mapping
        Object.assign(params, context);
      }

    const result = await this.sendRequest('agent.run', params);

    // Map result to appropriate return type based on agent type
    if (agentId === 'sec-audit' || agentId === 'audit') {
      return {
        id: result.id || `scan-${Date.now()}`,
        type: 'code',
        status: result.status || 'completed',
        findings: result.findings || [],
        startedAt: new Date(result.startedAt || Date.now()),
        completedAt: new Date(result.completedAt || Date.now()),
      } as SecurityScan;
    } else if (agentId === 'researcher' || agentId === 'research') {
      return {
        id: result.id || `research-${Date.now()}`,
        query: params.query,
        status: result.status || 'completed',
        sources: result.sources || [],
        startedAt: new Date(result.startedAt || Date.now()),
        completedAt: new Date(result.completedAt || Date.now()),
      } as ResearchResult;
    } else {
      // Generic result for other agent types
      return {
        status: result.status || 'completed',
        output: result.output || '',
        error: result.error || '',
        exitCode: result.exitCode || 0,
        duration: result.duration || 0,
      };
    }
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

  // MCP Connections - Get from CLI via RPC
  async getMCPConnections(): Promise<MCPConnection[]> {
    try {
      const result = await this.sendRequest('mcp.connections');
      return result.connections || this.getDefaultMCPConnections();
    } catch (error) {
      console.warn('CLI server not available, using default MCP connections:', error);
      return this.getDefaultMCPConnections();
    }
    }

  private getDefaultMCPConnections(): MCPConnection[] {
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

  // System metrics - Get from CLI via RPC
  async getSystemMetrics(): Promise<SystemMetrics> {
    try {
      const result = await this.sendRequest('system.metrics');
      return result.metrics || this.getDefaultMetrics();
    } catch (error) {
      console.warn('CLI server not available, using default metrics:', error);
      return this.getDefaultMetrics();
    }
    }

  private getDefaultMetrics(): SystemMetrics {
    return {
      cpuUsage: Math.random() * 100,
      memoryUsage: Math.random() * 100,
      diskUsage: Math.random() * 100,
      activeProcesses: Math.floor(Math.random() * 100),
      uptime: Math.floor(Math.random() * 86400), // Random uptime in seconds
    };
  }

  // File operations
  async executeCommand(command: string[], cwd?: string): Promise<{ exitCode: number; stdout: string; stderr: string }> {
    try {
      const result = await this.sendRequest('exec.command', {
      command,
      cwd: cwd || process.cwd(),
    });
      return {
        exitCode: result.exitCode || 0,
        stdout: result.stdout || '',
        stderr: result.stderr || '',
      };
    } catch (error) {
      console.warn('CLI server not available:', error);
      return {
        exitCode: 1,
        stdout: '',
        stderr: `Command execution failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
      };
    }
  }

  async fuzzyFileSearch(query: string, roots: string[]): Promise<any[]> {
    try {
      const result = await this.sendRequest('fs.search', { query, roots });
      return result.results || [];
    } catch (error) {
      console.warn('CLI server not available:', error);
      return [];
    }
  }

  // Resource Management Methods
  async getResourceStatus(): Promise<{
    capacity: {
      maxConcurrent: number;
      activeTasks: number;
      availableSlots: number;
    };
    stats: {
      cpuUsagePercent: number;
      memoryUsedBytes: number;
      memoryTotalBytes: number;
      memoryUsagePercent: number;
      activeAgents: number;
      cpuCores: number;
    };
  }> {
    return await this.sendRequest('resource.getStatus', {});
  }

  async acquireResource(): Promise<{ success: boolean; message: string }> {
    try {
      return await this.sendRequest('resource.acquire', {});
    } catch (error) {
      console.warn('Failed to acquire resource:', error);
      throw error;
    }
  }

  async releaseResource(): Promise<{ success: boolean; message: string }> {
    try {
      return await this.sendRequest('resource.release', {});
    } catch (error) {
      console.warn('Failed to release resource:', error);
      throw error;
    }
  }

  // CLI Execution Methods
  async executeCodex(prompt: string): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
    success: boolean;
  }> {
    try {
      return await this.sendRequest('cli.codex.execute', { prompt });
    } catch (error) {
      console.warn('Failed to execute Codex:', error);
      throw error;
    }
  }

  async executeGemini(prompt: string): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
    success: boolean;
  }> {
    try {
      return await this.sendRequest('cli.gemini.execute', { prompt });
    } catch (error) {
      console.warn('Failed to execute Gemini:', error);
      throw error;
    }
  }

  async executeClaude(prompt: string): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
    success: boolean;
  }> {
    try {
      return await this.sendRequest('cli.claude.execute', { prompt });
    } catch (error) {
      console.warn('Failed to execute Claude:', error);
      throw error;
    }
  }

  // Windows 11 25H2 MCP Methods
  async detectWindowsMCP(): Promise<{
    windowsVersion: string;
    mcpStandardAvailable: boolean;
    features?: {
      autoDetection: boolean;
      standardProtocol: boolean;
      nativeIntegration: boolean;
    };
  }> {
    try {
      return await this.sendRequest('mcp.windows.detect', {});
    } catch (error) {
      console.warn('Failed to detect Windows MCP:', error);
      throw error;
    }
  }

  async autoDetectMCPServers(): Promise<{
    servers: Array<{
      name: string;
      path: string;
      type: string;
    }>;
    count: number;
  }> {
    try {
      return await this.sendRequest('mcp.windows.autoDetect', {});
    } catch (error) {
      console.warn('Failed to auto-detect MCP servers:', error);
      throw error;
    }
  }

  async manageMCPConnection(action: 'list' | 'connect' | 'disconnect', serverId?: string): Promise<{
    success: boolean;
    serverId?: string;
    message: string;
  }> {
    try {
      return await this.sendRequest('mcp.windows.manage', {
        action,
        serverId,
      });
    } catch (error) {
      console.warn('Failed to manage MCP connection:', error);
      throw error;
    }
  }

  // Malware Detection Methods
  async scanMalware(params: {
    path: string;
    type?: 'file' | 'directory';
  }): Promise<{
    success: boolean;
    threatsFound: number;
    results: Array<{
      filePath: string;
      method: string;
      threatName: string;
      confidence: number;
      severity: string;
      details: string;
      timestamp: string;
    }>;
  }> {
    return this.sendRequest('malware.scan', {
      path: params.path,
      type: params.type || 'file',
    });
  }

  async quarantineMalware(params: {
    filePath: string;
    threatName: string;
    confidence?: number;
  }): Promise<{
    success: boolean;
    entryId: string;
    originalPath: string;
    quarantinePath: string;
    threatName: string;
    quarantinedAt: string;
  }> {
    return this.sendRequest('malware.quarantine', {
      filePath: params.filePath,
      threatName: params.threatName,
      confidence: params.confidence || 0.9,
    });
  }

  async deleteQuarantinedFile(entryId: string): Promise<{
    success: boolean;
    message: string;
  }> {
    return this.sendRequest('malware.delete', { entryId });
  }

  async restoreQuarantinedFile(entryId: string): Promise<{
    success: boolean;
    message: string;
  }> {
    return this.sendRequest('malware.restore', { entryId });
  }

  async listQuarantine(): Promise<{
    entries: Array<{
      id: string;
      originalPath: string;
      quarantinePath: string;
      threatName: string;
      confidence: number;
      status: string;
      quarantinedAt: string;
    }>;
    count: number;
  }> {
    return this.sendRequest('malware.listQuarantine', {});
  }

  async getMalwareStats(): Promise<{
    totalFilesScanned: number;
    threatsDetected: number;
    signatureMatches: number;
    heuristicMatches: number;
    behavioralMatches: number;
  }> {
    return this.sendRequest('malware.getStats', {});
  }

  // Virtual OS Terminal Methods
  async createTerminalSession(params: {
    workingDirectory?: string;
  }): Promise<{
    sessionId: string;
    workingDirectory: string;
  }> {
    return this.sendRequest('virtualos.terminal.createSession', {
      workingDirectory: params.workingDirectory || '.',
    });
  }

  async executeTerminalCommand(
    sessionId: string,
    command: string[]
  ): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
    isBlocked: boolean;
    blockReason?: string;
  }> {
    return this.sendRequest('virtualos.terminal.execute', {
      sessionId,
      command,
    });
  }

  async listTerminalCommands(sessionId: string): Promise<{
    commands: string[];
  }> {
    return this.sendRequest('virtualos.terminal.listCommands', {
      sessionId,
    });
  }

  async getTerminalHistory(sessionId: string): Promise<{
    history: Array<{
      command: string[];
      workingDirectory: string;
      timestamp: string;
      result?: {
        exitCode: number;
        stdout: string;
        stderr: string;
        isBlocked: boolean;
        blockReason?: string;
      };
    }>;
  }> {
    return this.sendRequest('virtualos.terminal.getHistory', {
      sessionId,
    });
  }

  async changeTerminalDirectory(
    sessionId: string,
    path: string
  ): Promise<{
    success: boolean;
    workingDirectory: string;
  }> {
    return this.sendRequest('virtualos.terminal.changeDirectory', {
      sessionId,
      path,
    });
  }

  // System Tray and Notification Methods
  async setAutostart(enabled: boolean): Promise<{
    success: boolean;
    enabled: boolean;
    message: string;
  }> {
    return this.sendRequest('tray.setAutostart', { enabled });
  }

  async getAutostart(): Promise<{
    enabled: boolean;
    message: string;
  }> {
    return this.sendRequest('tray.getAutostart', {});
  }

  async showNotification(params: {
    title: string;
    body?: string;
    type?: 'info' | 'success' | 'warning' | 'error';
  }): Promise<{
    success: boolean;
    message: string;
  }> {
    return this.sendRequest('notification.show', {
      title: params.title,
      body: params.body || '',
      type: params.type || 'info',
    });
  }

  async setNotificationEnabled(enabled: boolean): Promise<{
    success: boolean;
    enabled: boolean;
    message: string;
  }> {
    return this.sendRequest('tray.setNotificationEnabled', { enabled });
  }

  // GPU Methods
  async getGPUStatus(): Promise<{
    gpus: Array<{
      name: string;
      vendor: string;
      usagePercent: number;
      memoryUsed: number;
      memoryTotal: number;
      memoryUsagePercent: number;
      temperature?: number;
      powerUsage?: number;
      clockSpeed?: number;
      computeCapability?: string;
      cudaVersion?: string;
      directMLVersion?: string;
    }>;
  }> {
    try {
      return await this.sendRequest('gpu.getStatus', {});
    } catch (error) {
      console.warn('Failed to get GPU status:', error);
      throw error;
    }
  }

  async getGPUAccess(): Promise<{
    hasAccess: boolean;
    permissions: {
      compute: boolean;
      memory: boolean;
      monitoring: boolean;
    };
  }> {
    try {
      return await this.sendRequest('gpu.getAccess', {});
    } catch (error) {
      console.warn('Failed to get GPU access:', error);
      throw error;
    }
  }

  async optimizeGPU(settings: {
    powerLimit?: number;
    clockSpeed?: number;
    memoryClock?: number;
  }): Promise<{
    success: boolean;
    message: string;
    settings: any;
  }> {
    try {
      return await this.sendRequest('gpu.optimize', { settings });
    } catch (error) {
      console.warn('Failed to optimize GPU:', error);
      throw error;
    }
  }

  // DeepResearch Methods
  async deepResearch(params: {
    query: string;
    depth?: number;
    strategy?: 'comprehensive' | 'focused' | 'exploratory';
    useGemini?: boolean;
  }): Promise<{
    query: string;
    summary?: string;
    sources?: Array<{
      title: string;
      url: string;
      snippet: string;
      relevance_score?: number;
    }>;
    strategy?: string;
    depth?: number;
  }> {
    try {
      return await this.sendRequest('research.deep', {
        query: params.query,
        depth: params.depth || 3,
        strategy: params.strategy || 'comprehensive',
        useGemini: params.useGemini !== false, // Default to true
      });
    } catch (error) {
      console.warn('Failed to execute DeepResearch:', error);
      throw error;
    }
  }

  // Plan Methods
  async createPlan(params: {
    title: string;
    mode: string;
    budgetTokens: number;
    budgetTime: number;
  }): Promise<any> {
    return await this.sendRequest('plan.create', params);
  }

  async listPlans(): Promise<any[]> {
    const result = await this.sendRequest('plan.list', {});
    return result.plans || [];
  }

  async showPlan(planId: string): Promise<any> {
    return await this.sendRequest('plan.show', { planId });
  }

  async executePlan(planId: string): Promise<any> {
    return await this.sendRequest('plan.execute', { planId });
  }

  async getPlanStatus(planId: string): Promise<any> {
    return await this.sendRequest('plan.status', { planId });
  }

  async approvePlan(planId: string): Promise<any> {
    return await this.sendRequest('plan.approve', { planId });
  }

  async rejectPlan(planId: string, reason: string): Promise<any> {
    return await this.sendRequest('plan.reject', { planId, reason });
  }

  // Utility methods
  isConnected(): boolean {
    return this.isConnected;
  }

  getBaseUrl(): string {
    return 'ws://localhost:3001'; // CLI server WebSocket URL
  }

  setBaseUrl(url: string): void {
    // Base URL is fixed for CLI connection
    console.warn('Base URL is fixed for CLI connection:', url);
  }
}

// Singleton instance
export const apiClient = new CodexAPIClient();

// Export types
export { CodexAPIError };
