/**
 * Protocol types for orchestrator communication
 */

export const PROTOCOL_VERSION = '1.0';

export type MessageType = 'request' | 'response' | 'event';

export interface AgentInfo {
  id: string;
  role: string;
}

export interface Envelope {
  v: string;
  id: string;
  ts: string; // ISO 8601 / RFC3339
  type: MessageType;
  op: string;
  session?: string;
  agent?: AgentInfo;
  idem_key?: string;
  body: any;
}

export interface ResponseBody {
  status: 'ok' | 'error';
  code?: number;
  message?: string;
  request_id?: string;
  data: any;
}

export interface LockStatus {
  locked: boolean;
  owner?: string;
  acquired_at?: string;
}

export interface TaskStatus {
  id: string;
  position: number;
  total: number;
}

export interface StatusResponse {
  queue_size: number;
  idempotency_cache_size: number;
}

// Event topics
export const Topics = {
  LOCK_CHANGED: 'lock.changed',
  FS_CHANGED: 'fs.changed',
  VCS_CHANGED: 'vcs.changed',
  TOKENS_UPDATED: 'tokens.updated',
  AGENT_JOIN: 'agent.join',
  AGENT_LEAVE: 'agent.leave',
  TASK_PROGRESS: 'task.progress',
  TASK_COMPLETED: 'task.completed',
  TASK_FAILED: 'task.failed',
} as const;

export type Topic = typeof Topics[keyof typeof Topics];

// Operation types
export type LockOp = 'lock.status' | 'lock.acquire' | 'lock.release';
export type StatusOp = 'status.get';
export type FsOp = 'fs.read' | 'fs.write' | 'fs.patch';
export type VcsOp = 'vcs.diff' | 'vcs.commit' | 'vcs.push';
export type AgentOp = 'agent.register' | 'agent.heartbeat' | 'agent.list';
export type TaskOp = 'task.submit' | 'task.cancel';
export type TokenOp = 'tokens.reportUsage' | 'tokens.getBudget';
export type SessionOp = 'session.start' | 'session.end';
export type PubSubOp = 'subscribe' | 'unsubscribe';

export type Operation = 
  | LockOp 
  | StatusOp 
  | FsOp 
  | VcsOp 
  | AgentOp 
  | TaskOp 
  | TokenOp 
  | SessionOp 
  | PubSubOp;

// Request payloads
export interface FsReadRequest {
  path: string;
}

export interface FsWriteRequest {
  path: string;
  content: string;
  preimage_sha?: string;
}

export interface FsPatchRequest {
  unified_diff: string;
  base_commit?: string;
}

export interface VcsCommitRequest {
  message: string;
}

export interface VcsPushRequest {
  remote: string;
  branch: string;
}

export interface AgentRegisterRequest {
  capabilities: string[];
  heartbeat_ms: number;
  version: string;
}

export interface TokenReportUsageRequest {
  agent_id: string;
  prompt_tokens: number;
  completion_tokens: number;
  model: string;
}

export interface SubscribeRequest {
  topics: Topic[];
}

// Error codes
export const ErrorCodes = {
  BAD_REQUEST: 400,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
  NOT_FOUND: 404,
  CONFLICT: 409,
  RATE_LIMIT: 429,
  INTERNAL_ERROR: 500,
  SERVICE_UNAVAILABLE: 503,
} as const;
