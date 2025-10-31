/**
 * Protocol client for orchestrator communication
 */

import { EventEmitter } from 'events';
import { Transport, TransportConfig } from './transport';
import {
  Envelope,
  PROTOCOL_VERSION,
  ResponseBody,
  LockStatus,
  TaskStatus,
  StatusResponse,
  FsReadRequest,
  FsWriteRequest,
  FsPatchRequest,
  VcsCommitRequest,
  VcsPushRequest,
  AgentRegisterRequest,
  TokenReportUsageRequest,
  SubscribeRequest,
  Topic,
} from './types';

export interface ProtocolClientConfig {
  transport?: TransportConfig;
  requestTimeout?: number;
}

/**
 * Protocol client for communicating with the orchestrator
 */
export class ProtocolClient extends EventEmitter {
  private transport: Transport;
  private config: ProtocolClientConfig;
  private pendingRequests = new Map<string, {
    resolve: (value: any) => void;
    reject: (error: Error) => void;
    timeout: NodeJS.Timeout;
  }>();
  private sessionId?: string;

  constructor(config: ProtocolClientConfig = {}) {
    super();
    this.config = {
      requestTimeout: config.requestTimeout || 30000,
      ...config,
    };
    this.transport = new Transport(config.transport);
    this.setupTransportHandlers();
  }

  /**
   * Setup transport event handlers
   */
  private setupTransportHandlers(): void {
    this.transport.on('message', (envelope: Envelope) => {
      this.handleMessage(envelope);
    });

    this.transport.on('connected', () => {
      this.emit('connected');
    });

    this.transport.on('disconnected', () => {
      this.emit('disconnected');
      // Clear all pending requests
      for (const [id, pending] of this.pendingRequests.entries()) {
        clearTimeout(pending.timeout);
        pending.reject(new Error('Connection lost'));
        this.pendingRequests.delete(id);
      }
    });

    this.transport.on('error', (error) => {
      this.emit('error', error);
    });

    this.transport.on('reconnecting', (attempt) => {
      this.emit('reconnecting', attempt);
    });
  }

  /**
   * Handle incoming message
   */
  private handleMessage(envelope: Envelope): void {
    if (envelope.type === 'response') {
      this.handleResponse(envelope);
    } else if (envelope.type === 'event') {
      this.handleEvent(envelope);
    }
  }

  /**
   * Handle response message
   */
  private handleResponse(envelope: Envelope): void {
    const body = envelope.body as ResponseBody;
    const requestId = body.request_id;

    if (!requestId) {
      this.emit('error', new Error('Response missing request_id'));
      return;
    }

    const pending = this.pendingRequests.get(requestId);
    if (!pending) {
      return; // Possibly already timed out
    }

    this.pendingRequests.delete(requestId);
    clearTimeout(pending.timeout);

    if (body.status === 'ok') {
      pending.resolve(body.data);
    } else {
      const error = new Error(body.message || 'Request failed');
      (error as any).code = body.code;
      pending.reject(error);
    }
  }

  /**
   * Handle event message
   */
  private handleEvent(envelope: Envelope): void {
    this.emit('event', {
      topic: envelope.op,
      data: envelope.body,
      timestamp: envelope.ts,
    });

    // Also emit specific topic events
    this.emit(`event:${envelope.op}`, envelope.body);
  }

  /**
   * Connect to the orchestrator
   */
  async connect(): Promise<void> {
    await this.transport.connect();
  }

  /**
   * Disconnect from the orchestrator
   */
  disconnect(): void {
    this.transport.disconnect();
  }

  /**
   * Send a request and wait for response
   */
  private async request<T = any>(
    op: string,
    body: any = {},
    idemKey?: string
  ): Promise<T> {
    if (!this.transport.isConnected()) {
      await this.connect();
    }

    const envelope: Envelope = {
      v: PROTOCOL_VERSION,
      id: this.generateId(),
      ts: new Date().toISOString(),
      type: 'request',
      op,
      body,
    };

    if (idemKey) {
      envelope.idem_key = idemKey;
    }

    if (this.sessionId) {
      envelope.session = this.sessionId;
    }

    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pendingRequests.delete(envelope.id);
        reject(new Error(`Request timeout: ${op}`));
      }, this.config.requestTimeout);

      this.pendingRequests.set(envelope.id, { resolve, reject, timeout });

      this.transport.send(envelope).catch((error) => {
        this.pendingRequests.delete(envelope.id);
        clearTimeout(timeout);
        reject(error);
      });
    });
  }

  /**
   * Generate a unique ID
   */
  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  // Lock operations
  
  async getLockStatus(): Promise<LockStatus> {
    return this.request('lock.status');
  }

  async acquireLock(owner: string, timeoutMs?: number): Promise<void> {
    await this.request('lock.acquire', { owner, timeout_ms: timeoutMs });
  }

  async releaseLock(owner: string): Promise<void> {
    await this.request('lock.release', { owner });
  }

  // Status operations

  async getStatus(): Promise<StatusResponse> {
    return this.request('status.get');
  }

  // File system operations

  async fsRead(path: string): Promise<string> {
    const req: FsReadRequest = { path };
    return this.request('fs.read', req);
  }

  async fsWrite(
    path: string,
    content: string,
    preimageSha?: string,
    idemKey?: string
  ): Promise<TaskStatus> {
    const req: FsWriteRequest = { path, content, preimage_sha: preimageSha };
    return this.request('fs.write', req, idemKey);
  }

  async fsPatch(
    unifiedDiff: string,
    baseCommit?: string,
    idemKey?: string
  ): Promise<TaskStatus> {
    const req: FsPatchRequest = { unified_diff: unifiedDiff, base_commit: baseCommit };
    return this.request('fs.patch', req, idemKey);
  }

  // VCS operations

  async vcsDiff(): Promise<string> {
    return this.request('vcs.diff');
  }

  async vcsCommit(message: string, idemKey?: string): Promise<TaskStatus> {
    const req: VcsCommitRequest = { message };
    return this.request('vcs.commit', req, idemKey);
  }

  async vcsPush(
    remote: string,
    branch: string,
    idemKey?: string
  ): Promise<TaskStatus> {
    const req: VcsPushRequest = { remote, branch };
    return this.request('vcs.push', req, idemKey);
  }

  // Agent operations

  async registerAgent(
    capabilities: string[],
    heartbeatMs: number,
    version: string
  ): Promise<void> {
    const req: AgentRegisterRequest = {
      capabilities,
      heartbeat_ms: heartbeatMs,
      version,
    };
    await this.request('agent.register', req);
  }

  async heartbeat(stats: Record<string, any>): Promise<void> {
    await this.request('agent.heartbeat', { stats });
  }

  async listAgents(): Promise<any[]> {
    const result = await this.request('agent.list');
    return result.agents || [];
  }

  // Task operations

  async submitTask(
    kind: string,
    payload: any,
    deps: string[] = []
  ): Promise<TaskStatus> {
    return this.request('task.submit', { kind, payload, deps });
  }

  async cancelTask(id: string): Promise<void> {
    await this.request('task.cancel', { id });
  }

  // Token operations

  async reportUsage(
    agentId: string,
    promptTokens: number,
    completionTokens: number,
    model: string
  ): Promise<void> {
    const req: TokenReportUsageRequest = {
      agent_id: agentId,
      prompt_tokens: promptTokens,
      completion_tokens: completionTokens,
      model,
    };
    await this.request('tokens.reportUsage', req);
  }

  async getBudget(): Promise<any> {
    const result = await this.request('tokens.getBudget');
    return result.budget || {};
  }

  // Session operations

  async startSession(meta: Record<string, string>): Promise<string> {
    const result = await this.request('session.start', { meta });
    this.sessionId = result.session_id;
    return this.sessionId;
  }

  async endSession(id: string): Promise<void> {
    await this.request('session.end', { id });
    if (this.sessionId === id) {
      this.sessionId = undefined;
    }
  }

  // Pub/Sub operations

  async subscribe(topics: Topic[]): Promise<void> {
    const req: SubscribeRequest = { topics };
    await this.request('subscribe', req);
  }

  async unsubscribe(topics: Topic[]): Promise<void> {
    await this.request('unsubscribe', { topics });
  }
}
