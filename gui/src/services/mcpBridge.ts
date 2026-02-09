// ============================================================================
// MCP Bridge Service - Bidirectional MCP Communication with OAuth2 + Skills
// ============================================================================

import type {
  JSONRPCRequest,
  JSONRPCResponse,
  JSONRPCNotification,
  JSONRPCId,
  MCPClientInfo,
  MCPServerInfo,
  MCPServerCapabilities,
  MCPTool,
  MCPToolResult,
  AuthState,
  AuthProvider,
  TokenPayload,
  Skill,
  AgentStatus,
  ChatMessage,
  ChatThread,
  WorktreeInfo,
  TerminalSession,
  Task,
  QALintResult,
  Notification,
} from '../types/mcp';
import type { Bridge } from '../lib/api/Bridge';

// ============================================================================
// Configuration
// ============================================================================

interface MCPBridgeConfig {
  endpoint: string;
  clientInfo: MCPClientInfo;
  autoReconnect: boolean;
  reconnectIntervalMs: number;
  heartbeatIntervalMs: number;
  authMode: 'oauth2' | 'api-key' | 'both';
}

const DEFAULT_CONFIG: MCPBridgeConfig = {
  endpoint: 'ws://localhost:8765',
  clientInfo: { name: 'codex-gui', version: '2.14.1' },
  autoReconnect: true,
  reconnectIntervalMs: 3000,
  heartbeatIntervalMs: 30000,
  authMode: 'oauth2',
};

// ============================================================================
// Result Types
// ============================================================================

type RequestResolver = (value: unknown) => void;
type RequestRejecter = (reason: unknown) => void;

interface PendingRequest {
  id: JSONRPCId;
  method: string;
  resolve: RequestResolver;
  reject: RequestRejecter;
  timeout: ReturnType<typeof setTimeout>;
}

interface NotificationHandler {
  (params: unknown): void;
}

// ============================================================================
// MCP Bridge Class
// ============================================================================

export class MCPBridge {
  private config: MCPBridgeConfig;
  private bridge: Bridge;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private heartbeatTimer: ReturnType<typeof setTimeout> | null = null;
  private pendingRequests = new Map<JSONRPCId, PendingRequest>();
  private notificationHandlers = new Map<string, Set<NotificationHandler>>();
  private eventHandlers = new Map<string, Set<NotificationHandler>>();
  private isConnected = false;
  private isConnecting = false;
  private serverInfo: MCPServerInfo | null = null;
  private authState: AuthState = {
    isAuthenticated: false,
    mode: 'anonymous',
    provider: null,
    accessToken: null,
    refreshToken: null,
    expiresAt: null,
    user: null,
  };

  // Event emitters for UI integration
  private onConnectionChange: ((connected: boolean) => void) | null = null;
  private onAgentStatusChange: ((agents: AgentStatus[]) => void) | null = null;
  private onNotification: ((notification: Notification) => void) | null = null;

  constructor(config: Partial<MCPBridgeConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.bridge = new Bridge(this.config.endpoint);
    this.setupBridgeHandlers();
  }

  // ============================================================================
  // Connection Management
  // ============================================================================

  async connect(): Promise<void> {
    if (this.isConnected || this.isConnecting) {
      return;
    }

    this.isConnecting = true;

    try {
      // Add auth header if authenticated
      const headers = this.authState.accessToken
        ? { Authorization: `Bearer ${this.authState.accessToken}` }
        : {};

      await this.bridge.connect();
      this.isConnected = true;
      this.isConnecting = false;

      // Start heartbeat
      this.startHeartbeat();

      // Notify connection change
      this.onConnectionChange?.(true);

      // Log server info
      console.log('[MCP] Connected to server:', this.serverInfo?.name, this.serverInfo?.version);
    } catch (error) {
      this.isConnecting = false;
      console.error('[MCP] Connection failed:', error);
      this.scheduleReconnect();
      throw error;
    }
  }

  disconnect(): void {
    this.stopHeartbeat();
    this.stopReconnect();

    if (this.bridge) {
      this.bridge.notify('notifications/shutdown');
    }

    this.isConnected = false;
    this.onConnectionChange?.(false);
  }

  private scheduleReconnect(): void {
    if (!this.config.autoReconnect || this.reconnectTimer) {
      return;
    }

    console.log(`[MCP] Scheduling reconnect in ${this.config.reconnectIntervalMs}ms`);
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.connect().catch(() => {});
    }, this.config.reconnectIntervalMs);
  }

  private stopReconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
  }

  private startHeartbeat(): void {
    this.stopHeartbeat();
    this.heartbeatTimer = setInterval(() => {
      if (this.isConnected) {
        this.bridge.notify('notifications/ping');
      }
    }, this.config.heartbeatIntervalMs);
  }

  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }

  // ============================================================================
  // Bridge Handlers Setup
  // ============================================================================

  private setupBridgeHandlers(): void {
    // Handle incoming messages
    this.bridge.onMessage((message: JSONRPCResponse | JSONRPCNotification) => {
      this.handleMessage(message);
    });

    // Handle notifications
    this.bridge.onNotification('notifications/message', (params) => {
      this.handleChatMessage(params as { message: ChatMessage });
    });

    this.bridge.onNotification('notifications/agent_status', (params) => {
      this.handleAgentStatus(params as { agents: AgentStatus[] });
    });

    this.bridge.onNotification('notifications/qa_result', (params) => {
      this.handleQAResult(params as { result: QALintResult });
    });

    this.bridge.onNotification('notifications/terminal_output', (params) => {
      this.handleTerminalOutput(params as { sessionId: string; output: string });
    });

    this.bridge.onNotification('notifications/task_progress', (params) => {
      this.handleTaskProgress(params as { task: Task });
    });

    this.bridge.onNotification('notifications/error', (params) => {
      this.handleError(params as { error: { code: string; message: string } });
    });

    this.bridge.onNotification('notifications/info', (params) => {
      this.handleInfo(params as Notification);
    });
  }

  // ============================================================================
  // Message Handling
  // ============================================================================

  private handleMessage(message: JSONRPCResponse | JSONRPCNotification): void {
    // Handle response
    if ('id' in message && message.id !== null) {
      const pending = this.pendingRequests.get(message.id);
      if (pending) {
        clearTimeout(pending.timeout);
        this.pendingRequests.delete(message.id);

        if ('error' in message && message.error) {
          pending.reject(new Error(`${message.error.code}: ${message.error.message}`));
        } else if ('result' in message) {
          pending.resolve(message.result);
        }
      }
    }

    // Handle notification - route to specific handlers
    if (!('id' in message) || message.id === null) {
      if ('method' in message) {
        const handlers = this.notificationHandlers.get(message.method);
        handlers?.forEach((handler) => handler(message.params));
      }
    }
  }

  private handleChatMessage(data: { message: ChatMessage }): void {
    const handlers = this.eventHandlers.get('chat/message');
    handlers?.forEach((h) => h(data));
  }

  private handleAgentStatus(data: { agents: AgentStatus[] }): void {
    this.onAgentStatusChange?.(data.agents);
    const handlers = this.eventHandlers.get('agents/status');
    handlers?.forEach((h) => h(data));
  }

  private handleQAResult(data: { result: QALintResult }): void {
    const handlers = this.eventHandlers.get('qa/result');
    handlers?.forEach((h) => h(data));
  }

  private handleTerminalOutput(data: { sessionId: string; output: string }): void {
    const handlers = this.eventHandlers.get('terminal/output');
    handlers?.forEach((h) => h(data));
  }

  private handleTaskProgress(data: { task: Task }): void {
    const handlers = this.eventHandlers.get('tasks/progress');
    handlers?.forEach((h) => h(data));
  }

  private handleError(data: { error: { code: string; message: string } }): void {
    console.error('[MCP] Error:', data.error);
    this.onNotification?.({
      id: crypto.randomUUID(),
      type: 'error',
      title: 'MCP Error',
      message: data.error.message,
      timestamp: new Date(),
      read: false,
    });
  }

  private handleInfo(data: Notification): void {
    this.onNotification?.(data);
  }

  // ============================================================================
  // Request Methods
  // ============================================================================

  async request<T = unknown>(
    method: string,
    params?: Record<string, unknown> | unknown[]
  ): Promise<T> {
    if (!this.isConnected) {
      throw new Error('MCP Bridge not connected');
    }

    const id = this.nextRequestId();
    const request: JSONRPCRequest = {
      jsonrpc: '2.0',
      id,
      method,
      params,
    };

    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pendingRequests.delete(id);
        reject(new Error(`Request timeout: ${method}`));
      }, 60000); // 60 second timeout

      this.pendingRequests.set(id, {
        id,
        method,
        resolve: resolve as RequestResolver,
        reject: reject as RequestRejecter,
        timeout,
      });

      this.bridge.request(method, params).catch((error) => {
        this.pendingRequests.delete(id);
        clearTimeout(timeout);
        reject(error);
      });
    });
  }

  notify(method: string, params?: Record<string, unknown>): void {
    if (this.isConnected) {
      this.bridge.notify(method, params);
    }
  }

  private nextRequestId(): JSONRPCId {
    return Date.now() + Math.random();
  }

  // ============================================================================
  // Chat Methods
  // ============================================================================

  async *streamChat(
    messages: Array<{ role: string; content: string }>
  ): AsyncGenerator<string, void, unknown> {
    const requestId = this.nextRequestId();

    await this.request('chat/stream', { messages, requestId });

    for await (const notification of this.subscribeTo('notifications/chat_chunk')) {
      const chunk = notification as { chunk: string; done?: boolean };
      yield chunk.chunk;
      if (chunk.done) break;
    }
  }

  async createThread(title?: string): Promise<ChatThread> {
    return this.request<ChatThread>('chat/thread/create', { title });
  }

  async listThreads(): Promise<ChatThread[]> {
    return this.request<ChatThread[]>('chat/thread/list');
  }

  async deleteThread(threadId: string): Promise<void> {
    return this.request<void>('chat/thread/delete', { threadId });
  }

  // ============================================================================
  // Worktree Methods
  // ============================================================================

  async createWorktree(branch: string, path: string): Promise<WorktreeInfo> {
    return this.request<WorktreeInfo>('worktree/create', { branch, path });
  }

  async listWorktrees(): Promise<WorktreeInfo[]> {
    return this.request<WorktreeInfo[]>('worktree/list');
  }

  async deleteWorktree(path: string): Promise<void> {
    return this.request<void>('worktree/delete', { path });
  }

  async getWorktreeStatus(path: string): Promise<WorktreeInfo> {
    return this.request<WorktreeInfo>('worktree/status', { path });
  }

  // ============================================================================
  // Terminal Methods
  // ============================================================================

  async createTerminal(worktreePath: string): Promise<TerminalSession> {
    const session = await this.request<TerminalSession>('terminal/create', { worktreePath });

    // Subscribe to output
    this.subscribeTo(`notifications/terminal_output/${session.id}`);

    return session;
  }

  async sendTerminalInput(sessionId: string, input: string): Promise<void> {
    return this.request<void>('terminal/input', { sessionId, input });
  }

  async resizeTerminal(sessionId: string, cols: number, rows: number): Promise<void> {
    return this.request<void>('terminal/resize', { sessionId, cols, rows });
  }

  async closeTerminal(sessionId: string): Promise<void> {
    return this.request<void>('terminal/close', { sessionId });
  }

  // ============================================================================
  // Agent Methods
  // ============================================================================

  async getAgentStatus(): Promise<AgentStatus[]> {
    return this.request<AgentStatus[]>('agents/status');
  }

  async executeTask(
    task: string,
    worktree?: string,
    priority?: 'low' | 'medium' | 'high'
  ): Promise<Task> {
    return this.request<Task>('agents/execute', { task, worktree, priority });
  }

  async listTasks(): Promise<Task[]> {
    return this.request<Task[]>('tasks/list');
  }

  async cancelTask(taskId: string): Promise<void> {
    return this.request<void>('tasks/cancel', { taskId });
  }

  // ============================================================================
  // QA Methods
  // ============================================================================

  async runQAScan(worktreePath: string, files?: string[]): Promise<QALintResult[]> {
    return this.request<QALintResult[]>('qa/scan', { worktreePath, files });
  }

  async applyQAFix(resultId: string): Promise<void> {
    return this.request<void>('qa/fix', { resultId });
  }

  async ignoreQARule(resultId: string, reason: string): Promise<void> {
    return this.request<void>('qa/ignore', { resultId, reason });
  }

  // ============================================================================
  // Skills Methods
  // ============================================================================

  async listSkills(): Promise<Skill[]> {
    return this.request<Skill[]>('skills/list');
  }

  async installSkill(skillId: string): Promise<void> {
    return this.request<void>('skills/install', { skillId });
  }

  async uninstallSkill(skillId: string): Promise<void> {
    return this.request<void>('skills/uninstall', { skillId });
  }

  async executeSkill(
    skillId: string,
    parameters?: Record<string, unknown>
  ): Promise<unknown> {
    return this.request('skills/execute', { skillId, parameters });
  }

  async getSkillDependencies(skillId: string): Promise<{
    installed: string[];
    missing: string[];
    optional: string[];
  }> {
    return this.request('skills/dependencies', { skillId });
  }

  // ============================================================================
  // File System Methods
  // ============================================================================

  async readFile(path: string, worktreePath?: string): Promise<string> {
    return this.request<string>('fs/read', { path, worktreePath });
  }

  async writeFile(
    path: string,
    content: string,
    worktreePath?: string
  ): Promise<void> {
    return this.request<void>('fs/write', { path, content, worktreePath });
  }

  async listFiles(path: string, worktreePath?: string): Promise<string[]> {
    return this.request<string[]>('fs/list', { path, worktreePath });
  }

  async globFiles(
    pattern: string,
    worktreePath?: string
  ): Promise<string[]> {
    return this.request<string[]>('fs/glob', { pattern, worktreePath });
  }

  // ============================================================================
  // Git Methods
  // ============================================================================

  async getGitStatus(worktreePath?: string): Promise<{
    branch: string;
    modified: string[];
    staged: string[];
    untracked: string[];
  }> {
    return this.request('git/status', { worktreePath });
  }

  async gitCommit(
    message: string,
    worktreePath?: string,
    allowEmpty?: boolean
  ): Promise<string> {
    return this.request<string>('git/commit', { message, worktreePath, allowEmpty });
  }

  async gitBranch(name: string, worktreePath?: string): Promise<void> {
    return this.request<void>('git/branch', { name, worktreePath });
  }

  async gitCheckout(branch: string, worktreePath?: string): Promise<void> {
    return this.request<void>('git/checkout', { branch, worktreePath });
  }

  async gitPull(worktreePath?: string): Promise<void> {
    return this.request<void>('git/pull', { worktreePath });
  }

  async gitPush(worktreePath?: string, remote?: string, branch?: string): Promise<void> {
    return this.request<void>('git/push', { worktreePath, remote, branch });
  }

  // ============================================================================
  // Action Methods
  // ============================================================================

  async listActions(): Promise<Array<{
    id: string;
    name: string;
    description: string;
    enabled: boolean;
  }>> {
    return this.request('actions/list');
  }

  async runAction(
    actionId: string,
    parameters?: Record<string, unknown>
  ): Promise<{
    executionId: string;
    status: string;
  }> {
    return this.request('actions/run', { actionId, parameters });
  }

  async createAction(
    action: Record<string, unknown>
  ): Promise<{ id: string }> {
    return this.request('actions/create', { action });
  }

  async getActionLogs(executionId: string): Promise<string[]> {
    return this.request<string[]>('actions/logs', { executionId });
  }

  // ============================================================================
  // Figma Methods
  // ============================================================================

  async extractFigmaDesign(
    figmaUrl: string,
    scope?: {
      variables?: boolean;
      components?: boolean;
      textStyles?: boolean;
      layout?: boolean;
    }
  ): Promise<Record<string, unknown>> {
    return this.request('figma/extract', { figmaUrl, scope });
  }

  async generateDesignTokens(figmaUrl: string): Promise<Record<string, unknown>> {
    return this.request('figma/tokens', { figmaUrl });
  }

  async exportComponentCode(figmaUrl: string, componentId: string): Promise<string> {
    return this.request<string>('figma/export', { figmaUrl, componentId });
  }

  // ============================================================================
  // Subscription Methods
  // ============================================================================

  private async *subscribeTo(
    method: string
  ): AsyncGenerator<unknown, void, unknown> {
    const queue: Array<{ resolve: (v: unknown) => void; reject: (e: unknown) => void }> = [];

    const handler = (params: unknown) => {
      if (queue.length > 0) {
        const next = queue.shift();
        next?.resolve(params);
      }
    };

    const unsubscribe = this.onNotification(method, handler);

    try {
      while (true) {
        yield new Promise<unknown>((resolve, reject) => {
          queue.push({ resolve, reject });
        });
      }
    } finally {
      unsubscribe();
    }
  }

  // ============================================================================
  // Event Handler Registration
  // ============================================================================

  onConnectionChanged(handler: (connected: boolean) => void): () => void {
    this.onConnectionChange = handler;
    return () => {
      this.onConnectionChange = null;
    };
  }

  onAgentStatusChanged(handler: (agents: AgentStatus[]) => void): () => void {
    this.onAgentStatusChange = handler;
    return () => {
      this.onAgentStatusChange = null;
    };
  }

  onNotificationReceived(handler: (notification: Notification) => void): () => void {
    this.onNotification = handler;
    return () => {
      this.onNotification = null;
    };
  }

  onEvent(method: string, handler: NotificationHandler): () => void {
    if (!this.eventHandlers.has(method)) {
      this.eventHandlers.set(method, new Set());
    }
    this.eventHandlers.get(method)!.add(handler);
    return () => {
      this.eventHandlers.get(method)?.delete(handler);
    };
  }

  private onNotification(method: string, handler: NotificationHandler): () => void {
    if (!this.notificationHandlers.has(method)) {
      this.notificationHandlers.set(method, new Set());
    }
    this.notificationHandlers.get(method)!.add(handler);
    return () => {
      this.notificationHandlers.get(method)?.delete(handler);
    };
  }

  // ============================================================================
  // Properties
  // ============================================================================

  get connected(): boolean {
    return this.isConnected;
  }

  get serverCapabilities(): MCPServerCapabilities | null {
    return this.serverInfo?.capabilities ?? null;
  }

  get authenticated(): boolean {
    return this.authState.isAuthenticated;
  }

  get authProvider(): AuthProvider | null {
    return this.authState.provider;
  }
}

// ============================================================================
// Singleton Instance
// ============================================================================

let bridgeInstance: MCPBridge | null = null;

export function getBridge(config?: Partial<MCPBridgeConfig>): MCPBridge {
  if (!bridgeInstance) {
    bridgeInstance = new MCPBridge(config);
  }
  return bridgeInstance;
}

export function resetBridge(): void {
  bridgeInstance?.disconnect();
  bridgeInstance = null;
}

export { MCPBridge as default };
