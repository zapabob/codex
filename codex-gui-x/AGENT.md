# Codex GUI Implementation Plan - Agent Specification

## Overview

This document defines the comprehensive implementation plan for integrating zapabob/codex's unique features into a ChatGPT-style GUI interface. The goal is to create a "Command Center" development environment that combines the power of Codex's autonomous agents with an intuitive, modern UI.

---

## 1. Project Foundation

### 1.1 Core Principles

- **UI Parity with ChatGPT**: Familiar chat-based interface with sidebar navigation
- **Zero-Latency Experience**: Rust backend ensures sub-millisecond response
- **Enterprise Security**: Zero-trust sandboxing with Windows native isolation
- **Autonomous Operation**: A2A swarm intelligence for parallel task execution

### 1.2 Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| Frontend | Next.js 15 + React 19 | Modern UI framework |
| UI Components | Material UI 6 | Consistent design system |
| State Management | Zustand | Lightweight global state |
| 3D Visualization | Babylon.js + WebXR | Git4D immersive view |
| Terminal | xterm.js | Integrated terminal |
| Backend | Rust 2024 + Tokio | High-performance processing |
| Protocol | MCP 1.0 + JSON-RPC | Extensible tool integration |
| GPU Acceleration | CUDA 12 | Fast codebase analysis |

---

## 2. Architecture Integration

### 2.1 Feature Map to UI Components

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Codex GUI - Command Center                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────┐  ┌──────────────────────────────────────────┐    │
│  │   CHAT       │  │              MAIN WORKSPACE              │    │
│  │   AREA       │  │  ┌────────────┐ ┌────────────────────┐  │    │
│  │              │  │  │  Projects  │ │    Active Task     │  │    │
│  │  ┌────────┐  │  │  │  Manager   │ │    Panel           │  │    │
│  │  │Thread │  │  │  └────────────┘ └────────────────────┘  │    │
│  │  │List   │  │  │  ┌────────────┐ ┌────────────────────┐  │    │
│  │  └────────┘  │  │  │  Worktree │ │    Terminal/      │  │    │
│  │              │  │  │  Status   │ │    Actions        │  │    │
│  │  ┌────────┐  │  │  └────────────┘ └────────────────────┘  │    │
│  │  │Message │  │  │  ┌────────────┐ ┌────────────────────┐  │    │
│  │  │Stream  │  │  │  │  QA Agent │ │    Diff/Review    │  │    │
│  │  └────────┘  │  │  │  Status   │ │                   │  │    │
│  │              │  │  └────────────┘ └────────────────────┘  │    │
│  └──────────────┘  └──────────────────────────────────────────┘    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                        RIGHT SIDEBAR                         │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────────┐ │   │
│  │  │Git4D     │ │Figma    │ │Skills   │ │Sandbox        │ │   │
│  │  │Visualize │ │Design   │ │Catalog  │ │Security       │ │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Backend Feature Integration

| Codex Feature | UI Integration Point | Implementation |
|--------------|---------------------|----------------|
| A2A Swarm Intelligence | Agent Monitor Panel | Real-time agent status, message broadcasting |
| Parallel Sub-Agents | Task Queue View | Concurrent task visualization |
| Git Worktree Orchestration | Project Manager | Create/switch/manage worktrees |
| QA Agent (Real-time Linting) | Code Editor | Inline lint feedback |
| Git4D 3D Visualization | Right Sidebar Tab | Immersive repo visualization |
| Slash Commands | Input Area | Command autocomplete |
| MCP Protocol | Skills Catalog | External tool integration |
| Zero-Trust Sandbox | Security Panel | Policy management |
| CUDA Acceleration | Performance Stats | GPU utilization display |

---

## 3. Implementation Phases

### Phase 1: Core UI Foundation (Weeks 1-3)

#### 3.1.1 Chat Interface

```typescript
// codex-gui-x/src/components/chat/
interface ChatState {
  threads: Thread[];
  activeThread: Thread | null;
  messages: Message[];
  isStreaming: boolean;
  agents: AgentStatus[];
}

interface Thread {
  id: string;
  title: string;
  pinned: boolean;
  projectId: string | null;
  createdAt: Date;
  lastActivity: Date;
  status: 'active' | 'archived' | 'compacting';
}
```

**Components:**
- `ChatContainer.tsx` - Main chat layout
- `ThreadList.tsx` - Left sidebar thread management
- `MessageBubble.tsx` - Chat message rendering
- `StreamingIndicator.tsx` - Real-time response animation
- `VoiceInput.tsx` - Speech-to-text input

#### 3.1.2 Project Manager

```typescript
// codex-gui-x/src/components/projects/
interface Project {
  id: string;
  name: string;
  path: string;
  worktrees: WorktreeInfo[];
  currentBranch: string;
  status: 'healthy' | 'conflict' | 'building';
}

interface WorktreeInfo {
  path: string;
  branch: string;
  status: 'active' | 'idle' | 'running';
  taskId: string | null;
}
```

**Components:**
- `ProjectSelector.tsx` - Quick project switcher
- `WorktreeManager.tsx` - Git worktree control panel
- `MultiProjectView.tsx` - Split view for multiple projects

#### 3.1.3 Terminal Integration

```typescript
// codex-gui-x/src/components/terminal/
interface TerminalSession {
  id: string;
  worktreePath: string;
  pty: PTY;
  history: TerminalLine[];
  status: 'running' | 'idle' | 'busy';
}
```

**Components:**
- `TerminalPanel.tsx` - xterm.js wrapper
- `ActionTerminal.tsx` - Integrated command execution
- `TerminalHistory.tsx` - Command history with search

---

### Phase 2: Autonomous Agent Integration (Weeks 4-6)

#### 3.2.1 A2A Swarm Dashboard

```typescript
// codex-gui-x/src/components/agents/
interface AgentStatus {
  agentId: 'backend' | 'frontend' | 'qa' | 'orchestrator';
  status: 'idle' | 'processing' | 'waiting' | 'error';
  currentTask: string | null;
  messageQueue: A2AMessage[];
  performance: {
    cpuUsage: number;
    memoryUsage: number;
    taskCount: number;
  };
}

interface A2AMessage {
  from: AgentType;
  to: AgentType;
  type: 'task' | 'result' | 'error' | 'coordination';
  payload: unknown;
  timestamp: Date;
}
```

**UI Features:**
- Real-time agent status dashboard
- Message flow visualization
- Performance metrics (CPU, memory)
- Task distribution overview

#### 3.2.2 Parallel Task Manager

```typescript
// codex-gui-x/src/components/tasks/
interface Task {
  id: string;
  name: string;
  worktree: string;
  status: 'queued' | 'running' | 'completed' | 'failed';
  priority: 'low' | 'medium' | 'high' | 'critical';
  dependencies: string[];
  result: TaskResult | null;
  startedAt: Date | null;
  completedAt: Date | null;
}
```

**UI Features:**
- Kanban-style task board
- Dependency visualization
- Auto-retry configuration
- Progress tracking

#### 3.2.3 QA Agent Panel (Real-time Linting)

```typescript
// codex-gui-x/src/components/qa/
interface QALintResult {
  file: string;
  line: number;
  severity: 'error' | 'warning' | 'info';
  rule: string;
  message: string;
  autoFix: boolean;
  status: 'pending' | 'fixed' | 'ignored';
}

interface QAAgentStatus {
  enabled: boolean;
  lastScan: Date;
  issuesCount: number;
  autoFixEnabled: boolean;
}
```

**UI Features:**
- Inline code error highlighting
- One-click auto-fix
- Issue statistics dashboard
- Rule configuration

---

### Phase 3: Git4D Visualization (Weeks 7-9)

#### 3.3.1 3D Repository View

```typescript
// codex-gui-x/src/components/git4d/
interface Git4DScene {
  nodes: GitNode[];
  edges: GitEdge[];
  camera: CameraState;
  selection: GitNode | null;
}

interface GitNode {
  id: string;
  type: 'file' | 'directory' | 'commit' | 'branch';
  position: Vector3;
  metadata: {
    name: string;
    size: number;
    lastModified: Date;
    branch?: string;
    commitHash?: string;
  };
  color: Color3;
  connections: string[];
}
```

**Features:**
- File/folder as 3D nodes
- Commit history as timeline
- Branch visualization
- VR/AR mode toggle
- Zoom and pan controls
- Click-to-navigate

**Components:**
- `Git4DView.tsx` - Babylon.js 3D canvas
- `SceneControls.tsx` - Camera and navigation
- `LayerToggle.tsx` - Show/hide layers
- `VRModeButton.tsx` - WebXR activation

#### 3.3.2 Immersive Code Review

- 3D diff visualization
- Spatial commit history
- Architectural overview mode
- Annotations in 3D space

---

### Phase 4: Actions & Automation (Weeks 10-12)

#### 3.4.1 Actions System

```typescript
// codex-gui-x/src/components/actions/
interface Action {
  id: string;
  name: string;
  description: string;
  trigger: ActionTrigger;
  steps: ActionStep[];
  environment: EnvConfig;
  artifacts: Artifact[];
}

interface ActionTrigger {
  type: 'manual' | 'push' | 'pr' | 'schedule' | 'webhook';
  config: Record<string, unknown>;
}

interface ActionStep {
  name: string;
  run: string;
  env: Record<string, string>;
  timeout: number;
  onFailure: 'continue' | 'fail' | 'rollback';
}
```

**Components:**
- `ActionsPanel.tsx` - Actions library
- `ActionEditor.tsx` - Visual action builder
- `ActionRunner.tsx` - Execution view with logs
- `BuildProgress.tsx` - Real-time build status

#### 3.4.2 Scheduler UI

```typescript
// codex-gui-x/src/components/scheduler/
interface ScheduledTask {
  id: string;
  name: string;
  schedule: string; // cron expression
  enabled: boolean;
  lastRun: Date | null;
  nextRun: Date | null;
  history: RunRecord[];
  inboxResults: InboxItem[];
}

interface InboxItem {
  id: string;
  taskId: string;
  type: 'success' | 'failure' | 'warning';
  summary: string;
  details: string;
  receivedAt: Date;
  read: boolean;
}
```

**UI Features:**
- Calendar view for schedules
- Inbox for task results
- Schedule enable/disable toggle
- History timeline

---

### Phase 5: Skills & MCP Integration (Weeks 13-15)

#### 3.5.1 Skills Catalog

```typescript
// codex-gui-x/src/components/skills/
interface Skill {
  id: string;
  name: string;
  description: string;
  category: 'filesystem' | 'git' | 'testing' | 'deployment' | 'external';
  mcpServers: MCPServerRequirement[];
  parameters: ParameterSchema;
  actions: SkillAction[];
  version: string;
}

interface MCPServerRequirement {
  serverName: string;
  minVersion: string;
  optional: boolean;
  autoInstall: boolean;
}
```

**Components:**
- `SkillBrowser.tsx` - Searchable catalog
- `SkillCard.tsx` - Skill information
- `MCPServerManager.tsx` - Server configuration
- `DependencyResolver.tsx` - Auto-install dialog

#### 3.5.2 MCP Server Management

- Server status monitoring
- OAuth authentication flow
- Configuration editor
- Connection testing

---

### Phase 6: Figma Design Integration (Weeks 16-18)

#### 3.6.1 Design Context Extraction

```typescript
// codex-gui-x/src/components/figma/
interface FigmaDesignContext {
  project: {
    id: string;
    name: string;
    lastModified: Date;
  };
  variables: {
    colors: ColorVariable[];
    typography: TypographyToken[];
    spacing: SpacingToken[];
    effects: EffectToken[];
  };
  components: ComponentDefinition[];
  textStyles: TextStyle[];
  layout: LayoutInfo;
}

interface ColorVariable {
  name: string;
  value: string;
  description: string;
}

interface TypographyToken {
  name: string;
  fontFamily: string;
  fontSize: number;
  fontWeight: number;
  lineHeight: number;
  letterSpacing: number;
}

interface ComponentDefinition {
  name: string;
  description: string;
  properties: ComponentProperty[];
  variants: ComponentVariant[];
}
```

**Components:**
- `FigmaInput.tsx` - URL/project picker
- `DesignContextViewer.tsx` - Extracted context display
- `VariableMapper.tsx` - Map Figma vars to code
- `ComponentExporter.tsx` - Generate component code

**Features:**
- Automatic variable extraction
- Component code generation
- Layout constraint mapping
- Design token export

---

### Phase 7: Security & Sandbox (Weeks 19-20)

#### 3.7.1 Security Dashboard

```typescript
// codex-gui-x/src/components/security/
interface SandboxConfig {
  enabled: boolean;
  isolationLevel: 'process' | 'container' | 'vm';
  networkPolicy: 'allow' | 'block' | 'prompt';
  filesystemPolicy: 'readonly' | 'limited' | 'full';
  allowedCommands: string[];
  timeout: number;
  auditEnabled: boolean;
}

interface PermissionRequest {
  id: string;
  command: string;
  reason: string;
  risk: 'low' | 'medium' | 'high';
  requestedAt: Date;
  status: 'pending' | 'approved' | 'denied';
}
```

**Components:**
- `SecurityPanel.tsx` - Security settings
- `PermissionRequestModal.tsx` - Permission prompts
- `AuditLog.tsx` - Security event log
- `NetworkPolicyEditor.tsx` - Firewall rules

#### 3.7.2 Rules Engine

```yaml
# .codex/rules/commands.yaml
allowed_commands:
  - pattern: "gh pr*"
    description: "GitHub PR operations"
    always_allow: true
  - pattern: "git status"
    always_allow: true
  - pattern: "git add *"
    conditions:
      - "no secrets detected"
      - "files are tracked"

prefix_rules:
  - pattern: "ghpr"
    command: "gh pr {{args}}"
    always_allow: true
  - pattern: "build"
    command: "{{workspace}}/scripts/build.sh"
```

---

### Phase 8: Diff & Code Review (Weeks 21-22)

#### 3.8.1 Inline Feedback System

```typescript
// codex-gui-x/src/components/review/
interface DiffComment {
  id: string;
  file: string;
  lineStart: number;
  lineEnd: number;
  side: 'left' | 'right' | 'both';
  author: string;
  content: string;
  type: 'suggestion' | 'question' | 'approval' | 'issue';
  createdAt: Date;
  resolved: boolean;
  replies: CommentReply[];
}

interface ReviewSession {
  id: string;
  prNumber: number;
  status: 'in_progress' | 'changes_requested' | 'approved' | 'merged';
  comments: DiffComment[];
  reviewers: Reviewer[];
  checkResults: CheckResult[];
}
```

**Components:**
- `DiffViewer.tsx` - Side-by-side diff
- `InlineComment.tsx` - Comment input
- `CommentThread.tsx` - Discussion view
- `ReviewActions.tsx` - Approve/request changes
- `SuggestionCard.tsx` - Code suggestions

---

### Phase 9: Voice & Accessibility (Week 23)

#### 3.9.1 Voice Input

```typescript
// codex-gui-x/src/components/voice/
interface VoiceConfig {
  enabled: boolean;
  language: string;
  continuous: boolean;
  commands: VoiceCommand[];
}

interface VoiceCommand {
  phrase: string;
  action: string;
  parameters?: Record<string, unknown>;
}
```

**Commands:**
- "Save this" → Save current work
- "Run build" → Execute build action
- "Create branch" → Branch creation flow
- "Add to chat" → Voice-to-message conversion

---

### Phase 10: Polish & Integration (Weeks 24-25)

- Performance optimization
- Accessibility audit
- Mobile responsiveness
- Offline support
- Documentation

---

## 4. File Structure

```
codex-gui-x/
├── src/
│   ├── app/
│   │   ├── page.tsx
│   │   ├── layout.tsx
│   │   └── globals.css
│   ├── components/
│   │   ├── chat/
│   │   │   ├── ChatContainer.tsx
│   │   │   ├── ThreadList.tsx
│   │   │   ├── MessageBubble.tsx
│   │   │   ├── InputArea.tsx
│   │   │   ├── VoiceInput.tsx
│   │   │   └── StreamingIndicator.tsx
│   │   ├── projects/
│   │   │   ├── ProjectSelector.tsx
│   │   │   ├── WorktreeManager.tsx
│   │   │   └── MultiProjectView.tsx
│   │   ├── agents/
│   │   │   ├── AgentDashboard.tsx
│   │   │   ├── TaskQueue.tsx
│   │   │   └── A2AMonitor.tsx
│   │   ├── qa/
│   │   │   ├── QAPanel.tsx
│   │   │   ├── LintResults.tsx
│   │   │   └── AutoFixButton.tsx
│   │   ├── git4d/
│   │   │   ├── Git4DView.tsx
│   │   │   ├── SceneControls.tsx
│   │   │   └── VRModeButton.tsx
│   │   ├── actions/
│   │   │   ├── ActionsPanel.tsx
│   │   │   ├── ActionEditor.tsx
│   │   │   ├── ActionRunner.tsx
│   │   │   └── TerminalPanel.tsx
│   │   ├── skills/
│   │   │   ├── SkillBrowser.tsx
│   │   │   ├── MCPServerManager.tsx
│   │   │   └── DependencyResolver.tsx
│   │   ├── figma/
│   │   │   ├── FigmaInput.tsx
│   │   │   ├── DesignContextViewer.tsx
│   │   │   └── ComponentExporter.tsx
│   │   ├── security/
│   │   │   ├── SecurityPanel.tsx
│   │   │   ├── PermissionRequestModal.tsx
│   │   │   └── AuditLog.tsx
│   │   ├── review/
│   │   │   ├── DiffViewer.tsx
│   │   │   ├── InlineComment.tsx
│   │   │   └── ReviewActions.tsx
│   │   ├── scheduler/
│   │   │   ├── ScheduleList.tsx
│   │   │   └── InboxView.tsx
│   │   └── common/
│   │       ├── Button.tsx
│   │       ├── Modal.tsx
│   │       └── LoadingSpinner.tsx
│   ├── services/
│   │   ├── api.ts
│   │   ├── worktreeService.ts
│   │   ├── figmaService.ts
│   │   ├── voiceService.ts
│   │   └── securityService.ts
│   ├── store/
│   │   ├── useChatStore.ts
│   │   ├── useProjectStore.ts
│   │   ├── useAgentStore.ts
│   │   └── useTerminalStore.ts
│   ├── hooks/
│   │   ├── useStream.ts
│   │   ├── useWorktree.ts
│   │   └── useVoiceInput.ts
│   └── utils/
│       ├── format.ts
│       └── validation.ts
├── public/
│   └── icons/
├── package.json
├── tsconfig.json
└── vite.config.ts
```

---

## 5. Implementation Checklist

### Phase 1: Core UI
- [ ] Chat container with thread management
- [ ] Project selector and worktree manager
- [ ] Terminal panel with xterm.js
- [ ] Voice input integration
- [ ] Responsive sidebar navigation

### Phase 2: Agents
- [ ] A2A swarm dashboard
- [ ] Parallel task queue
- [ ] QA agent lint display
- [ ] Performance metrics

### Phase 3: Git4D
- [ ] Babylon.js scene setup
- [ ] File/commit node rendering
- [ ] VR mode activation
- [ ] Camera controls

### Phase 4: Actions
- [ ] Actions library view
- [ ] Visual action editor
- [ ] Terminal integration
- [ ] Build progress display

### Phase 5: Skills/MCP
- [ ] Skills catalog
- [ ] MCP server management
- [ ] Dependency auto-install
- [ ] OAuth flow

### Phase 6: Figma
- [ ] Design context extraction
- [ ] Variable mapping UI
- [ ] Component export

### Phase 7: Security
- [ ] Sandbox configuration
- [ ] Permission prompts
- [ ] Audit log viewer
- [ ] Rules editor

### Phase 8: Review
- [ ] Side-by-side diff
- [ ] Inline commenting
- [ ] Suggestion cards
- [ ] Review workflow

### Phase 9: Voice
- [ ] Speech recognition
- [ ] Voice commands
- [ ] Dictation buffer

### Phase 10: Polish
- [ ] Performance tuning
- [ ] Accessibility check
- [ ] Mobile layout
- [ ] Final testing

---

## 6. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Time to first message | < 500ms | User opens app → first response |
| Thread switching | < 100ms | Context switch latency |
| Worktree creation | < 2s | Branch create to usable |
| A2A message latency | < 50ms | Agent-to-agent communication |
| Git4D load time | < 3s | 3D scene initialization |
| Bundle size | < 2MB | Initial load (gzipped) |
| Lighthouse score | > 90 | Performance/Accessibility |
| Test coverage | > 80% | Unit + E2E tests |

---

## 7. Dependencies

### External Libraries
- `react@19` - UI framework
- `@mui/material@6` - UI components
- `zustand@5` - State management
- `babylonjs@7` - 3D visualization
- `xterm@5` - Terminal emulation
- `framer-motion@11` - Animations
- `react-hook-form` - Form handling
- `zod` - Validation

### Backend Services (codex-rs)
- `codex-core` - Main agent orchestration
- `codex-mcp` - MCP protocol handler
- `codex-sandbox` - Security isolation
- `codex-qa` - Real-time linting

### External APIs
- Figma API - Design extraction
- GitHub API - PR/Issue integration
- WebXR API - VR/AR support

---

## 8. Rollout Strategy

### Beta Release (Week 8)
- Invite-only beta users
- Core chat + project features
- Git worktree management
- Basic terminal integration

### RC1 (Week 16)
- All Phase 1-4 features
- QA agent integration
- Git4D visualization
- Actions system

### Stable Release (Week 24)
- Full feature set
- Mobile web support
- Performance optimization
- Complete documentation

---

*Document Version: 1.0*
*Last Updated: 2024-02-08*
*Status: Ready for Implementation*
