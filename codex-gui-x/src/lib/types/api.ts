/**
 * API型定義
 * Codex APIのリクエスト/レスポンス型定義
 */

// APIリクエスト型
export interface APIRequest<T = unknown> {
  method: string;
  params?: T;
  id?: string | number;
}

// APIレスポンス型
export interface APIResponse<T = unknown> {
  id?: string | number;
  result?: T;
  error?: APIError;
}

// APIエラー型
export interface APIError {
  code: number;
  message: string;
  data?: unknown;
}

// WebSocketメッセージ型
export interface WebSocketMessage {
  type: string;
  data?: unknown;
  [key: string]: unknown;
}

// アカウント情報型
export interface AccountInfo {
  id: string;
  email: string;
  name?: string;
  [key: string]: unknown;
}

// エージェント実行コンテキスト型
export interface AgentContext {
  [key: string]: unknown;
}

// エージェント実行結果型
export interface AgentResult {
  success: boolean;
  output?: string;
  error?: string;
  [key: string]: unknown;
}

// ツール型
export interface Tool {
  name: string;
  description?: string;
  parameters?: Record<string, unknown>;
  [key: string]: unknown;
}

// セッション型
export interface Session {
  id: string;
  name?: string;
  created_at?: string;
  [key: string]: unknown;
}

// タスク型
export interface Task {
  id: string;
  name?: string;
  status?: string;
  [key: string]: unknown;
}

// CLI bridgeレスポンス型（snake_case形式）
export interface CLIBridgeTool {
  id: string;
  name?: string;
  status?: string;
  capabilities?: string[];
  active_sessions?: number;
  activeSessions?: number;
  max_sessions?: number;
  maxSessions?: number;
  avg_response_time?: number;
  success_rate?: number;
  resource_usage?: number;
  performance?: {
    avgResponseTime?: number;
    successRate?: number;
    resourceUsage?: number;
  };
}

export interface CLIBridgeSession {
  id: string;
  toolId?: string;
  tool_id?: string;
  taskId?: string;
  task_id?: string;
  status?: string;
  startTime?: string | number;
  start_time?: string | number;
  endTime?: string;
  end_time?: string;
  progress?: number;
  output?: string;
  error?: string;
}

export interface CLIBridgeTask {
  id: string;
  title?: string;
  description?: string;
  complexity?: string;
  priority?: string;
  requirements?: string[];
  subtasks?: unknown[];
  status?: string;
  created_at?: string | number;
  createdAt?: string | number;
  assignedTools?: string[];
  assigned_tools?: string[];
  progress?: number;
}

// ファジー検索結果型
export interface FuzzySearchResult {
  path: string;
  score?: number;
  [key: string]: unknown;
}
