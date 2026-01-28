// Codex API Types
// Auto-generated from Rust types - DO NOT EDIT MANUALLY

export interface Model {
  id: string;
  model: string;
  displayName: string;
  description: string;
  supportedReasoningEfforts: ReasoningEffortOption[];
  defaultReasoningEffort: ReasoningEffort;
  isDefault: boolean;
}

export interface ReasoningEffortOption {
  reasoningEffort: ReasoningEffort;
  description: string;
}

export type ReasoningEffort = "low" | "medium" | "high";

export interface Conversation {
  id: string;
  model: string;
  status: "active" | "completed" | "error";
  createdAt: Date;
  lastActivity: Date;
  messageCount: number;
  summary?: string;
}

export interface Message {
  id: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: Date;
  attachments?: FileAttachment[];
}

export interface FileAttachment {
  name: string;
  type: "image" | "file";
  url: string;
  size?: number;
}

export interface Agent {
  id: string;
  name: string;
  type: "code-reviewer" | "test-gen" | "sec-audit" | "researcher" | "performance" | "debug" | "docs";
  status: "idle" | "working" | "completed" | "error";
  description: string;
  lastUsed?: Date;
}

export interface MCPConnection {
  id: string;
  name: string;
  type: "filesystem" | "github" | "sequential-thinking" | "playwright" | "gemini" | "chrome-mcp";
  status: "connected" | "disconnected" | "error";
  url?: string;
  lastConnected?: Date;
  requestCount?: number;
  avgResponseTime?: number;
}

export interface SecurityScan {
  id: string;
  type: "dependency" | "code" | "secrets";
  status: "running" | "completed" | "failed";
  findings: SecurityFinding[];
  startedAt: Date;
  completedAt?: Date;
}

export interface SecurityFinding {
  severity: "critical" | "high" | "medium" | "low" | "info";
  title: string;
  description: string;
  location?: {
    file: string;
    line?: number;
    column?: number;
  };
  recommendation?: string;
}

export interface ResearchResult {
  id: string;
  query: string;
  status: "searching" | "analyzing" | "completed" | "failed";
  sources: ResearchSource[];
  summary?: string;
  startedAt: Date;
  completedAt?: Date;
}

export interface WebResearchResult {
  id: string;
  query: string;
  status: "completed" | "failed";
  output: string;
  startedAt: Date;
  completedAt?: Date;
  error?: string;
}

export interface ResearchSource {
  url: string;
  title: string;
  snippet: string;
  confidence: number;
  publishedAt?: Date;
}

export interface SystemMetrics {
  cpuUsage: number;
  memoryUsage: number;
  diskUsage: number;
  networkUsage?: number;
  activeProcesses: number;
  uptime: number;
  gpuUsage?: number;
  gpuMemoryUsed?: number;
  gpuMemoryTotal?: number;
  gpuMemoryUsage?: number;
  gpuTemperature?: number;
  gpuName?: string;
  gpuVendor?: 'nvidia' | 'amd' | 'intel' | 'unknown';
}

export interface NotificationItem {
  id: string;
  type: "info" | "warning" | "error" | "success";
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
}

// API Request/Response Types
export interface APIRequest<T = unknown> {
  method: string;
  params?: T;
  id?: string | number;
}

export interface APIResponse<T = unknown> {
  id?: string | number;
  result?: T;
  error?: APIError;
}

export interface APIError {
  code: number;
  message: string;
  data?: unknown;
}

// WebSocket message data types
export type ConversationUpdateData = {
  conversationId: string;
  updates: Record<string, unknown>;
};

export type AgentStatusData = {
  agentId: string;
  status: string;
  progress?: number;
};

export type SystemMetricsData = {
  cpu: number;
  memory: number;
  disk?: number;
};

export type NotificationData = {
  message: string;
  severity?: "info" | "warning" | "error";
};

export type WebSocketMessageData = 
  | ConversationUpdateData 
  | AgentStatusData 
  | SystemMetricsData 
  | NotificationData
  | Record<string, unknown>;

export interface WebSocketMessage {
  type: "conversation_update" | "agent_status" | "system_metrics" | "notification";
  data: WebSocketMessageData;
}

// Configuration Types
export interface AppConfig {
  theme: "light" | "dark" | "system";
  language: "ja" | "en";
  notifications: {
    enabled: boolean;
    sound: boolean;
    desktop: boolean;
  };
  shortcuts: {
    [key: string]: string;
  };
  api: {
    timeout: number;
    retryAttempts: number;
    baseUrl?: string;
  };
}

// Form Types
export interface LoginForm {
  method: "api-key" | "oauth";
  apiKey?: string;
  email?: string;
}

export interface NewConversationForm {
  model: string;
  initialMessage: string;
  attachments?: File[];
}

export interface AgentConfigForm {
  type: string;
  name: string;
  description?: string;
  parameters?: Record<string, unknown>;
}

// Component Props Types
export interface DashboardProps {
  user: User | null;
  conversations: Conversation[];
  agents: Agent[];
  metrics: SystemMetrics;
  notifications: NotificationItem[];
}

export interface User {
  id: string;
  email?: string;
  name?: string;
  avatar?: string;
  plan: "free" | "plus" | "pro";
}

// Utility Types
export type LoadingState = "idle" | "loading" | "success" | "error";

export interface AsyncState<T> {
  state: LoadingState;
  data?: T;
  error?: string;
}

// Event Types
export type ConversationEventData = {
  messageId?: string;
  message?: string;
  status?: string;
  agentId?: string;
  [key: string]: unknown;
};

export type AgentEventData = {
  status?: string;
  taskId?: string;
  error?: string;
  [key: string]: unknown;
};

export interface ConversationEvent {
  type: "message_added" | "status_changed" | "agent_assigned";
  conversationId: string;
  data: ConversationEventData;
  timestamp: Date;
}

export interface AgentEvent {
  type: "status_changed" | "task_completed" | "error_occurred";
  agentId: string;
  data: AgentEventData;
  timestamp: Date;
}

// WebXR API Types
export interface NavigatorXR {
  xr?: XRSystem;
}

export interface XRSystem {
  isSessionSupported(sessionType: XRSessionMode): Promise<boolean>;
  requestSession(mode: XRSessionMode, options?: XRSessionInit): Promise<XRSession>;
}

export type XRSessionMode = 'inline' | 'immersive-vr' | 'immersive-ar';

export interface XRSessionInit {
  requiredFeatures?: string[];
  optionalFeatures?: string[];
}

export interface XRSession extends EventTarget {
  mode: XRSessionMode;
  inputSources: XRInputSource[];
  requestReferenceSpace(type: XRReferenceSpaceType): Promise<XRReferenceSpace>;
  end(): Promise<void>;
}

export type XRReferenceSpaceType = 'viewer' | 'local' | 'local-floor' | 'bounded-floor' | 'unbounded';

export interface XRReferenceSpace extends XRSpace {
  getOffsetReferenceSpace(originOffset: XRRigidTransform): XRReferenceSpace;
}

export interface XRSpace extends EventTarget {}

export interface XRRigidTransform {
  position: DOMPointReadOnly;
  orientation: DOMPointReadOnly;
}

export interface XRInputSource {
  handedness: XRHandedness;
  targetRayMode: XRTargetRayMode;
  targetRaySpace: XRSpace;
  gripSpace?: XRSpace;
  profiles: string[];
  gamepad?: Gamepad;
  hand?: XRHand;
}

export type XRHandedness = 'none' | 'left' | 'right';
export type XRTargetRayMode = 'gaze' | 'tracked-pointer' | 'screen';

export interface XRHand {
  size: number;
  getJointPose(joint: XRHandJoint, baseSpace: XRSpace): XRJointPose | null;
}

export type XRHandJoint = 
  | 'wrist'
  | 'thumb-metacarpal' | 'thumb-phalanx-proximal' | 'thumb-phalanx-distal' | 'thumb-tip'
  | 'index-finger-metacarpal' | 'index-finger-phalanx-proximal' | 'index-finger-phalanx-intermediate' | 'index-finger-phalanx-distal' | 'index-finger-tip'
  | 'middle-finger-metacarpal' | 'middle-finger-phalanx-proximal' | 'middle-finger-phalanx-intermediate' | 'middle-finger-phalanx-distal' | 'middle-finger-tip'
  | 'ring-finger-metacarpal' | 'ring-finger-phalanx-proximal' | 'ring-finger-phalanx-intermediate' | 'ring-finger-phalanx-distal' | 'ring-finger-tip'
  | 'pinky-finger-metacarpal' | 'pinky-finger-phalanx-proximal' | 'pinky-finger-phalanx-intermediate' | 'pinky-finger-phalanx-distal' | 'pinky-finger-tip';

export interface XRJointPose {
  transform: XRRigidTransform;
  radius: number;
}

// Git4D API Request Types
export interface Git4DLaunchRequest {
  mode: 'desktop' | 'vr' | 'ar';
  repositoryPath: string;
  virtualDesktop?: boolean;
}

export interface Git4DLaunchResponse {
  sessionId: string;
  platform?: string;
  device_name?: string;
}

// Plan Management Types
export interface Plan {
  id: string;
  title: string;
  goal: string;
  approach: string;
  mode: 'single' | 'orchestrated' | 'competition';
  state: 'Drafting' | 'Pending' | 'Approved' | 'Rejected' | 'Executing' | 'Completed' | 'Failed';
  created_at: string;
  updated_at: string;
  approved_by?: string | null;
  rejected_reason?: string | null;
  budget: {
    session_cap?: number;
    cap_min?: number;
  };
  work_items: Array<{
    name: string;
    files_touched: string[];
    diff_contract: string;
    tests: string[];
  }>;
  risks: Array<{
    item: string;
    mitigation: string;
  }>;
}

export interface CreatePlanRequest {
  title: string;
  mode?: 'single' | 'orchestrated' | 'competition';
  budget_tokens?: number;
  budget_time?: number;
}
