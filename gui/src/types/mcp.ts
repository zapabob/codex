// MCP Protocol Types - Full Type Definitions

// ============================================================================
// JSON-RPC 2.0 Base Types
// ============================================================================

export type JSONRPCId = string | number;

export interface JSONRPCRequest {
  jsonrpc: '2.0';
  id: JSONRPCId;
  method: string;
  params?: Record<string, unknown> | unknown[];
}

export interface JSONRPCResponse {
  jsonrpc: '2.0';
  id: JSONRPCId;
  result?: unknown;
  error?: JSONRPCError;
}

export interface JSONRPCNotification {
  jsonrpc: '2.0';
  method: string;
  params?: Record<string, unknown>;
}

export interface JSONRPCError {
  code: number;
  message: string;
  data?: unknown;
}

export type JSONRPCMessage = JSONRPCRequest | JSONRPCResponse | JSONRPCNotification;

// ============================================================================
// MCP Core Types
// ============================================================================

export interface MCPClientInfo {
  name: string;
  version: string;
}

export interface MCPServerInfo {
  name: string;
  version: string;
  capabilities: MCPServerCapabilities;
}

export interface MCPServerCapabilities {
  tools?: Record<string, MCPTool>;
  resources?: Record<string, MCPResource>;
  prompts?: Record<string, MCPPrompt>;
  notifications?: MCPNotificationCapabilities;
}

export interface MCPNotificationCapabilities {
  tools?: {
    listChanged?: boolean;
  };
  resources?: {
    listChanged?: boolean;
    subscribe?: boolean;
  };
  prompts?: {
    listChanged?: boolean;
  };
}

export interface MCPTool {
  name: string;
  description: string;
  inputSchema: MCPJSONSchema;
}

export interface MCPToolResult {
  content: Array<{
    type: 'text' | 'image' | 'resource';
    text?: string;
    mimeType?: string;
    uri?: string;
  }>;
  isError?: boolean;
}

export interface MCPResource {
  uri: string;
  name: string;
  description?: string;
  mimeType?: string;
}

export interface MCPPrompt {
  name: string;
  description?: string;
  arguments?: Array<{
    name: string;
    description?: string;
    required?: boolean;
  }>;
}

export interface MCPJSONSchema {
  type: 'object' | 'string' | 'number' | 'boolean' | 'array' | 'null';
  properties?: Record<string, unknown>;
  required?: string[];
  additionalProperties?: boolean;
}

// ============================================================================
// Authentication Types
// ============================================================================

export type AuthMode = 'oauth2' | 'api-key' | 'both' | 'anonymous';

export type AuthProvider = 'openai' | 'google' | 'github';

export interface AuthConfig {
  mode: AuthMode;
  providers: AuthProvider[];
  apiKeyEnabled: boolean;
  sessionTimeoutMinutes: number;
  refreshTokenEnabled: boolean;
}

export interface OAuth2Config {
  clientId: string;
  authorizationEndpoint: string;
  tokenEndpoint: string;
  scopes: string[];
  redirectUri: string;
}

export interface AuthState {
  isAuthenticated: boolean;
  mode: AuthMode;
  provider: AuthProvider | null;
  accessToken: string | null;
  refreshToken: string | null;
  expiresAt: Date | null;
  user: AuthUser | null;
}

export interface AuthUser {
  id: string;
  email: string;
  name: string;
  avatarUrl?: string;
  provider: AuthProvider;
}

export interface TokenPayload {
  sub: string;
  email: string;
  name: string;
  exp: number;
  iat: number;
  provider: AuthProvider;
}

// ============================================================================
// Chat Types
// ============================================================================

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
  attachments?: ChatAttachment[];
  metadata?: ChatMetadata;
}

export interface ChatAttachment {
  id: string;
  type: 'file' | 'image' | 'code';
  name: string;
  mimeType?: string;
  url?: string;
  content?: string;
}

export interface ChatMetadata {
  model?: string;
  tokens?: number;
  duration?: number;
  agent?: string;
}

export interface ChatThread {
  id: string;
  title: string;
  messages: ChatMessage[];
  createdAt: Date;
  updatedAt: Date;
  pinned: boolean;
  projectId?: string;
  status: 'active' | 'archived' | 'compacting';
}

// ============================================================================
// Project & Worktree Types
// ============================================================================

export interface Project {
  id: string;
  name: string;
  path: string;
  currentBranch: string;
  status: 'healthy' | 'conflict' | 'building' | 'error';
  worktrees: WorktreeInfo[];
}

export interface WorktreeInfo {
  path: string;
  branch: string;
  status: 'active' | 'idle' | 'running';
  taskId?: string;
  lastActivity: Date;
}

export interface WorktreeConfig {
  basePath: string;
  maxConcurrent: number;
  autoCleanup: boolean;
  cleanupAfterDays: number;
}

// ============================================================================
// Terminal Types
// ============================================================================

export interface TerminalSession {
  id: string;
  worktreePath: string;
  pty?: string;
  status: 'running' | 'idle' | 'busy';
  history: TerminalLine[];
}

export interface TerminalLine {
  id: string;
  input: boolean;
  content: string;
  timestamp: Date;
}

export interface TerminalResize {
  cols: number;
  rows: number;
}

// ============================================================================
// Agent Types
// ============================================================================

export type AgentType = 'backend' | 'frontend' | 'qa' | 'orchestrator';

export type AgentStatusType = 'idle' | 'processing' | 'waiting' | 'error';

export interface AgentStatus {
  agentId: AgentType;
  status: AgentStatusType;
  currentTask?: string;
  messageQueue: number;
  performance: AgentPerformance;
}

export interface AgentPerformance {
  cpuUsage: number;
  memoryUsage: number;
  taskCount: number;
}

export interface A2AMessage {
  id: string;
  from: AgentType;
  to: AgentType;
  type: 'task' | 'result' | 'error' | 'coordination';
  payload: unknown;
  timestamp: Date;
}

// ============================================================================
// Task Types
// ============================================================================

export type TaskStatus = 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';
export type TaskPriority = 'low' | 'medium' | 'high' | 'critical';

export interface Task {
  id: string;
  name: string;
  worktree: string;
  status: TaskStatus;
  priority: TaskPriority;
  dependencies: string[];
  result?: TaskResult;
  startedAt?: Date;
  completedAt?: Date;
  createdAt: Date;
}

export interface TaskResult {
  success: boolean;
  output: string;
  error?: string;
  duration: number;
}

// ============================================================================
// QA Types
// ============================================================================

export type QASeverity = 'error' | 'warning' | 'info';

export interface QALintResult {
  id: string;
  file: string;
  line: number;
  column: number;
  severity: QASeverity;
  rule: string;
  message: string;
  autoFix: boolean;
  status: 'pending' | 'fixed' | 'ignored';
  codeFix?: string;
}

export interface QAAgentStatus {
  enabled: boolean;
  lastScan: Date | null;
  issuesCount: number;
  autoFixEnabled: boolean;
}

// ============================================================================
// Skills Types
// ============================================================================

export interface Skill {
  id: string;
  name: string;
  description: string;
  category: 'filesystem' | 'git' | 'testing' | 'deployment' | 'external' | 'code-review' | 'architecture';
  version: string;
  installed: boolean;
  autoInstall: boolean;
  mcpTools: string[];
  dependencies: string[];
  parameters?: SkillParameter[];
  actions?: SkillAction[];
  manifest: SkillManifest;
}

export interface SkillManifest {
  name: string;
  version: string;
  description?: string;
  author?: string;
  repository?: string;
  keywords?: string[];
}

export interface SkillParameter {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'array' | 'object';
  description?: string;
  required?: boolean;
  default?: unknown;
}

export interface SkillAction {
  name: string;
  description?: string;
  parameters?: SkillParameter[];
}

export interface MCPServerRequirement {
  serverName: string;
  minVersion: string;
  optional: boolean;
  autoInstall: boolean;
}

// ============================================================================
// Figma Types
// ============================================================================

export interface FigmaDesignContext {
  projectId: string;
  projectName: string;
  lastModified: Date;
  variables: DesignVariables;
  components: ComponentDefinition[];
  textStyles: TextStyle[];
  layout: LayoutInfo;
}

export interface DesignVariables {
  colors: ColorVariable[];
  typography: TypographyToken[];
  spacing: SpacingToken[];
  effects: EffectToken[];
}

export interface ColorVariable {
  name: string;
  value: string;
  description?: string;
}

export interface TypographyToken {
  name: string;
  fontFamily: string;
  fontSize: number;
  fontWeight: number;
  lineHeight: number;
  letterSpacing: number;
}

export interface SpacingToken {
  name: string;
  value: number;
  unit: 'px' | 'rem' | 'em';
}

export interface EffectToken {
  name: string;
  type: 'shadow' | 'blur' | 'glow';
  value: string;
}

export interface ComponentDefinition {
  name: string;
  description?: string;
  properties: ComponentProperty[];
  variants: ComponentVariant[];
}

export interface ComponentProperty {
  name: string;
  type: string;
  defaultValue?: unknown;
  description?: string;
}

export interface ComponentVariant {
  name: string;
  properties: Record<string, unknown>;
}

export interface TextStyle {
  name: string;
  fontFamily: string;
  fontSize: number;
  fontWeight: number;
  lineHeight: number;
}

export interface LayoutInfo {
  gridColumns: number;
  gridGutter: number;
  containerMaxWidth: number;
}

// ============================================================================
// Action Types
// ============================================================================

export type ActionTriggerType = 'manual' | 'push' | 'pr' | 'schedule' | 'webhook';
export type ActionFailureBehavior = 'continue' | 'fail' | 'rollback';

export interface Action {
  id: string;
  name: string;
  description: string;
  trigger: ActionTrigger;
  steps: ActionStep[];
  environment: EnvConfig;
  artifacts: Artifact[];
  enabled: boolean;
  createdAt: Date;
  updatedAt: Date;
}

export interface ActionTrigger {
  type: ActionTriggerType;
  config: Record<string, unknown>;
}

export interface ActionStep {
  id: string;
  name: string;
  run: string;
  env: Record<string, string>;
  timeout: number;
  onFailure: ActionFailureBehavior;
}

export interface EnvConfig {
  variables: Record<string, string>;
  secrets: string[];
}

export interface Artifact {
  path: string;
  type: 'file' | 'directory' | 'archive';
}

// ============================================================================
// Security Types
// ============================================================================

export type IsolationLevel = 'process' | 'container' | 'vm';
export type NetworkPolicy = 'allow' | 'block' | 'prompt';
export type FilesystemPolicy = 'readonly' | 'limited' | 'full';
export type PermissionStatus = 'pending' | 'approved' | 'denied';

export interface SandboxConfig {
  enabled: boolean;
  isolationLevel: IsolationLevel;
  networkPolicy: NetworkPolicy;
  filesystemPolicy: FilesystemPolicy;
  allowedCommands: string[];
  timeout: number;
  auditEnabled: boolean;
}

export interface PermissionRequest {
  id: string;
  command: string;
  reason: string;
  risk: 'low' | 'medium' | 'high';
  requestedAt: Date;
  status: PermissionStatus;
}

export interface SecurityRule {
  id: string;
  pattern: string;
  description: string;
  alwaysAllow: boolean;
  conditions?: string[];
}

// ============================================================================
// Review Types
// ============================================================================

export type CommentType = 'suggestion' | 'question' | 'approval' | 'issue';

export interface DiffComment {
  id: string;
  file: string;
  lineStart: number;
  lineEnd: number;
  side: 'left' | 'right';
  author: string;
  content: string;
  type: CommentType;
  createdAt: Date;
  resolved: boolean;
  replies: CommentReply[];
}

export interface CommentReply {
  id: string;
  author: string;
  content: string;
  createdAt: Date;
}

export interface ReviewSession {
  id: string;
  prNumber?: number;
  status: 'in_progress' | 'changes_requested' | 'approved' | 'merged';
  comments: DiffComment[];
  reviewers: Reviewer[];
  checkResults: CheckResult[];
}

export interface Reviewer {
  id: string;
  name: string;
  status: 'pending' | 'approved' | 'changes_requested';
}

export interface CheckResult {
  name: string;
  status: 'pending' | 'running' | 'passed' | 'failed';
  duration?: number;
}

// ============================================================================
// Scheduler Types
// ============================================================================

export interface ScheduledTask {
  id: string;
  name: string;
  schedule: string;
  enabled: boolean;
  lastRun: Date | null;
  nextRun: Date | null;
  history: RunRecord[];
  inboxResults: InboxItem[];
}

export interface RunRecord {
  id: string;
  status: 'success' | 'failure' | 'warning';
  startedAt: Date;
  completedAt: Date;
  output: string;
}

export interface InboxItem {
  id: string;
  taskId: string;
  type: 'success' | 'failure' | 'warning';
  summary: string;
  details: string;
  receivedAt: Date;
  read: boolean;
}

// ============================================================================
// Voice Types
// ============================================================================

export interface VoiceConfig {
  enabled: boolean;
  language: string;
  continuous: boolean;
  commands: VoiceCommand[];
}

export interface VoiceCommand {
  phrase: string;
  action: string;
  parameters?: Record<string, unknown>;
}

// ============================================================================
// Git4D Types
// ============================================================================

export interface Git4DScene {
  nodes: GitNode[];
  edges: GitEdge[];
  camera: CameraState;
  selection: GitNode | null;
}

export type GitNodeType = 'file' | 'directory' | 'commit' | 'branch';

export interface GitNode {
  id: string;
  type: GitNodeType;
  position: Vector3D;
  metadata: {
    name: string;
    size: number;
    lastModified: Date;
    branch?: string;
    commitHash?: string;
    depth?: number;
  };
  color: ColorRGB;
  connections: string[];
}

export interface GitEdge {
  id: string;
  source: string;
  target: string;
  type: 'parent' | 'child' | 'dependency';
}

export interface Vector3D {
  x: number;
  y: number;
  z: number;
}

export interface ColorRGB {
  r: number;
  g: number;
  b: number;
}

export interface CameraState {
  position: Vector3D;
  target: Vector3D;
  zoom: number;
}

// ============================================================================
// Notification Types
// ============================================================================

export interface Notification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
  action?: NotificationAction;
}

export interface NotificationAction {
  label: string;
  handler: string;
  parameters?: Record<string, unknown>;
}

// ============================================================================
// Error Types
// ============================================================================

export interface CodexError {
  code: string;
  message: string;
  details?: Record<string, unknown>;
  stack?: string;
}

export function createError(code: string, message: string, details?: Record<string, unknown>): CodexError {
  return {
    code,
    message,
    details,
    stack: typeof process !== 'undefined' ? new Error().stack : undefined,
  };
}
