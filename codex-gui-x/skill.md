---
name: gui-unified-implementation-plan
description: "Comprehensive UI/UX Integration and Advanced Features Implementation Plan - Command Center, Worktree Orchestration, Actions, Skills+MCP, Guardrails, Automations"
---

# Codex GUI Unified Implementation Plan

# Codex GUI統合実装計画

## Overview | 概要

本計画書は、Codex GUIの包括的な実装計画を概述する。ChatGPT風のUI、Git Worktree並列実行、Actions統合、Skills+MCP連携、ガードレール、自動化を含む。

This document outlines the comprehensive implementation plan for Codex GUI. Includes ChatGPT-style UI, Git Worktree parallel execution, Actions integration, Skills+MCP integration, guardrails, and automations.

---

## Architecture | アーキテクチャ

### Command Center Design | 司令塔設計

```
┌─────────────────────────────────────────────────────────────┐
│                      Codex Command Center                    │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────────────────────────────┐   │
│ │  Projects   │ │         Active Worktrees             │   │
│ │  Selector   │ │  ┌─────┬─────┬─────┬─────┐         │   │
│ │             │ │  │ wt1 │ wt2 │ wt3 │ wt4 │ ...     │   │
│ │  • PRJ-A   │ │  └─────┴─────┴─────┴─────┘         │   │
│ │  • PRJ-B   │ │    Running Tasks (Parallel)          │   │
│ │  • PRJ-C   │ │                                     │   │
│ └─────────────┘ └─────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────┐   │
│ │  Chat Interface (ChatGPT Style)                      │   │
│ │  ┌─────────────────────────────────────────────┐   │   │
│ │  │  Thread: [Pinned] Verification Task #123    │   │   │
│ │  │  ├─ User: Please verify this PR...         │   │   │
│ │  │  ├─ AI: Running tests...                   │   │   │
│ │  │  └─ [Streaming Response]                    │   │   │
│ │  └─────────────────────────────────────────────┘   │   │
│ │  ┌─────────────────────────────────────────────┐   │   │
│ │  │  Input: [Voice][Attach][@Mention] ──────▶ │   │   │
│ │  └─────────────────────────────────────────────┘   │   │
│ └─────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  Sidebar: Threads │ Actions Panel │ Notifications         │
└─────────────────────────────────────────────────────────────┘
```

---

## Implementation Plan | 実装計画

### Phase 1: UI Foundation (Week 1)

#### 1.1 ChatGPT-Style UI Components

| Component     | File                                    | Status               |
| ------------- | --------------------------------------- | -------------------- |
| ChatContainer | `src/components/chat/ChatContainer.tsx` | Redesign             |
| ChatBubble    | `src/components/chat/ChatBubble.tsx`    | Redesign (No Avatar) |
| ChatSidebar   | `src/components/chat/ChatSidebar.tsx`   | Rename & Redesign    |
| InputArea     | `src/components/chat/InputArea.tsx`     | Enhance              |
| WelcomeScreen | `src/components/chat/WelcomeScreen.tsx` | New                  |

#### 1.2 Theme System

| File                        | Purpose                        |
| --------------------------- | ------------------------------ |
| `src/theme/chatGPTTheme.ts` | ChatGPT-style dark/light theme |
| `src/theme/index.ts`        | Theme provider                 |

---

### Phase 2: Worktree Orchestration (Week 2)

#### 2.1 Worktree Manager

```
src/
├── components/
│   ├── worktree/
│   │   ├── WorktreeDashboard.tsx    # Main dashboard
│   │   ├── WorktreeCard.tsx         # Individual worktree card
│   │   ├── WorktreeStatus.tsx        # Status indicator
│   │   ├── BranchManager.tsx         # Branch management
│   │   └── ConflictResolver.tsx      # Conflict handling
│   └── orchestrator/
│       └── TaskOrchestrator.tsx      # Parallel task runner
```

#### 2.2 Worktree Store

```typescript
// src/store/useWorktreeStore.ts
interface WorktreeState {
  worktrees: Worktree[];
  activeWorktreeId: string | null;
  pinnedThreads: string[];
  runningTasks: Map<string, TaskStatus>;

  // Actions
  createWorktree: (repo: string, branch: string) => Promise<Worktree>;
  deleteWorktree: (id: string) => Promise<void>;
  switchWorktree: (id: string) => void;
  pinThread: (threadId: string) => void;
  unpinThread: (threadId: string) => void;
  runTask: (task: Task, worktreeId: string) => Promise<void>;
}
```

#### 2.3 Worktree Conflict Prevention

```
Conflict Prevention Rules:
1. One branch per worktree (enforced)
2. Lock mechanism for shared resources
3. Auto-sync before new checkout
4. Branch existence check before create
```

---

### Phase 3: Actions Integration (Week 3)

#### 3.1 Built-in Terminal

```
src/
├── components/
│   ├── terminal/
│   │   ├── IntegratedTerminal.tsx   # xterm.js based
│   │   ├── TerminalPanel.tsx         # Side panel terminal
│   │   └── TerminalTabs.tsx          # Multiple terminals
│   └── actions/
│       ├── ActionButtons.tsx         # Quick action buttons
│       ├── ActionRunner.tsx          # Action execution
│       └── ActionHistory.tsx         # Action history
```

#### 3.2 Actions Configuration

```yaml
# .codex/actions/common.yaml
actions:
  build:
    command: pnpm build
    working_directory: ${WORKTREE_ROOT}
    env:
      NODE_ENV: production
    description: Build the project

  test:
    command: pnpm test
    working_directory: ${WORKTREE_ROOT}
    env:
      CI: true
    description: Run tests

  dev:
    command: pnpm dev
    working_directory: ${WORKTREE_ROOT}
    description: Start dev server

  lint:
    command: pnpm lint
    working_directory: ${WORKTREE_ROOT}
    description: Run linter

  format:
    command: pnpm format
    working_directory: ${WORKTREE_ROOT}
    description: Format code

  typecheck:
    command: pnpm typecheck
    working_directory: ${WORKTREE_ROOT}
    description: Type check
```

#### 3.3 Environment Setup Automation

```yaml
# .codex/actions/setup.yaml
setup:
  - name: Detect package manager
    run: |
      if [ -f package.json ]; then
        if [ -f pnpm-lock.yaml ]; then echo "PNPM"; fi
      fi

  - name: Install dependencies
    command: pnpm install
    condition: package_manager == "pnpm"

  - name: Setup environment
    command: |
      cp .env.example .env 2>/dev/null || true
      echo "Environment configured"
```

---

### Phase 4: Skills + MCP Integration (Week 4)

#### 4.1 Skills Catalog

```
src/
├── components/
│   ├── skills/
│   │   ├── SkillsCatalog.tsx        # Skills browser
│   │   ├── SkillCard.tsx            # Individual skill card
│   │   ├── SkillDetail.tsx          # Skill details
│   │   └── MCPStatus.tsx            # MCP server status
│   └── mcp/
│       ├── MCPServerManager.tsx     # MCP server management
│       ├── MCPDependencyChecker.tsx  # Dependency checker
│       └── OAuthFlow.tsx            # OAuth authentication
```

#### 4.2 MCP Auto-Setup

```typescript
// src/services/mcpAutoSetup.ts
interface MCPSetupResult {
  success: boolean;
  installed: string[];
  missing: string[];
  requiresAuth: string[];
}

async function setupMCPServer(serverName: string): Promise<MCPSetupResult> {
  // 1. Check dependencies
  const deps = await checkDependencies(serverName);

  // 2. Auto-install missing dependencies
  if (deps.missing.length > 0) {
    await autoInstall(deps.missing);
  }

  // 3. Check OAuth requirements
  if (deps.requiresAuth.length > 0) {
    await initiateOAuth(deps.requiresAuth);
  }

  // 4. Start server
  await startMCPServer(serverName);

  return { success: true, ... };
}
```

#### 4.3 Skills Configuration

```yaml
# .codex/skills/catalog.yaml
skills:
  vrchat-dev:
    name: VRChat Development
    description: VRChat world and avatar development
    mcp_servers:
      - vrchat
      - blender
    auto_setup: true
    oauth:
      - vrchat (optional)

  blender-cad:
    name: Blender CAD
    description: Blender CAD modeling automation
    mcp_servers:
      - blender
    auto_setup: true

  code-reviewer:
    name: Code Review
    description: Automated code review
    mcp_servers: []
    auto_setup: false
```

---

### Phase 5: Guardrails & Notifications (Week 5)

#### 5.1 Sandbox Security

```
src/
├── components/
│   ├── security/
│   │   ├── Sandbox.tsx              # Isolated execution
│   │   ├── NetworkPolicy.tsx        # Network control
│   │   ├── PermissionRequest.tsx    # Permission dialog
│   │   └── GuardrailSettings.tsx   # Settings panel
```

#### 5.2 Guardrail Configuration

```yaml
# .codex/rules/guardrails.yaml
guardrails:
  network:
    default_policy: block
    allowed_domains:
      - github.com
      - api.openai.com
      - localhost
    require_approval:
      - api.external-service.com

  execution:
    max_cpu_percent: 80
    max_memory_mb: 4096
    max_duration_minutes: 60
    require_approval:
      - cargo build --release
      - npm publish

  file_system:
    default_policy: read-only
    writable_paths:
      - ${WORKTREE_ROOT}/src
      - ${WORKTREE_ROOT}/artifacts
    require_approval:
      - ${WORKTREE_ROOT}/package.json (modify)
```

#### 5.3 Notification System

```typescript
// src/store/useNotificationStore.ts
interface Notification {
  id: string;
  type: "info" | "success" | "warning" | "error" | "approval";
  title: string;
  message: string;
  timestamp: Date;
  read: boolean;
  action?: {
    label: string;
    onClick: () => void;
  };
}

interface NotificationSettings {
  turnComplete: boolean;
  permissionRequests: boolean;
  buildComplete: boolean;
  mention: boolean;
  inboxDigest: "realtime" | "hourly" | "daily";
}
```

---

### Phase 6: Automations (Week 6)

#### 6.1 Scheduled Tasks

```
src/
├── components/
│   ├── automation/
│   │   ├── AutomationScheduler.tsx  # Schedule management
│   │   ├── AutomationList.tsx       # List automations
│   │   ├── AutomationEditor.tsx     # Create/edit
│   │   └── InboxView.tsx            # Results inbox
│   └── worktree/
│       └── ScheduledTaskRunner.tsx  # Background runner
```

#### 6.2 Automation Configuration

```yaml
# .codex/automations/schedules.yaml
automations:
  pr-comment-handler:
    schedule: "*/30 * * * *" # Every 30 minutes
    worktree: automation-pr-${WORKTREE_ID}
    steps:
      - name: Fetch PR comments
        run: gh pr view ${PR_NUMBER} --comments
      - name: Process unhandled
        run: |
          # AI analyzes comments
          # Executes appropriate actions
      - name: Update status
        run: gh pr comment ${PR_NUMBER} --body "Processed"
    notify:
      - type: completion
        to: inbox

  ci-health-check:
    schedule: "0 */4 * * *" # Every 4 hours
    worktree: automation-ci-${WORKTREE_ID}
    steps:
      - name: Check CI status
        run: gh run list --limit 10
      - name: Report failures
        run: |
          # AI analyzes failures
          # Suggests fixes
    notify:
      - type: failure
        to: inbox

  nightly-build:
    schedule: "0 2 * * *" # 2 AM daily
    worktree: automation-build-${WORKTREE_ID}
    steps:
      - name: Checkout main
        run: git checkout main
      - name: Pull latest
        run: git pull
      - name: Full build
        run: pnpm build
      - name: Run tests
        run: pnpm test
    notify:
      - type: failure
        to: inbox
```

---

### Phase 7: Approval Rules (Week 7)

#### 7.1 Approval Rules Configuration

```yaml
# .codex/rules/approvals.yaml
rules:
  # Prefix-based rules
  - prefix: "ghpr"
    description: "GitHub PR commands"
    always_allow: true
    commands:
      - gh pr checkout
      - gh pr view
      - gh pr diff

  - prefix: "gh"
    description: "GitHub CLI general"
    always_allow: true
    commands:
      - gh repo view
      - gh run view

  # Specific safe commands
  - command: "git push"
    always_allow: true
    conditions:
      - branch != "main"
      - branch != "develop"

  - command: "git checkout"
    always_allow: true

  - command: "pnpm install"
    always_allow: true

  - command: "cargo test"
    always_allow: true

  - command: "cargo clippy"
    always_allow: true

  # Commands requiring approval
  - command: "git push"
    conditions:
      - branch == "main"
      - branch == "develop"
    require_approval: true

  - command: "npm publish"
    require_approval: true

  - command: "cargo publish"
    require_approval: true
```

---

### Phase 8: Figma Integration (Week 8)

#### 8.1 Figma Import Components

```
src/
├── components/
│   ├── figma/
│   │   ├── FigmaImportPanel.tsx     # Import UI
│   │   ├── FigmaFileBrowser.tsx     # File selector
│   │   ├── DesignExtractor.tsx      # Extract design tokens
│   │   └── VariableMapper.tsx       # Map to code
```

#### 8.2 Figma MCP Service

```typescript
// src/services/figmaMCP.ts
interface FigmaDesign {
  fileKey: string;
  nodes: FigmaNode[];
  variables: DesignVariable[];
  styles: DesignStyle[];
  textContent: FigmaText[];
  fonts: FontInfo[];
}

async function extractDesignContext(fileKey: string): Promise<FigmaDesign> {
  // 1. Get file structure
  const file = await mcp.figma_get_file({ file_key: fileKey });

  // 2. Extract design tokens
  const variables = await extractVariables(file);

  // 3. Extract text content
  const textContent = await extractText(file);

  // 4. Extract fonts
  const fonts = await extractFonts(file);

  // 5. Generate implementation code
  return generateCodeContext(file, variables, textContent, fonts);
}
```

#### 8.3 Design Token Output

```typescript
// Output example for React Component
const designTokens = {
  colors: {
    primary: "#0061A4",
    secondary: "#565F71",
    background: "#FFFFFF",
  },
  spacing: {
    small: "8px",
    medium: "16px",
    large: "24px",
  },
  typography: {
    heading1: { size: "32px", weight: 700, font: "Inter" },
    body: { size: "16px", weight: 400, font: "Inter" },
  },
};
```

---

### Phase 9: Voice Input (Week 9)

#### 9.1 Voice Input Components

```
src/
├── components/
│   ├── voice/
│   │   ├── VoiceInput.tsx          # Main voice input
│   │   ├── VoiceButton.tsx          # Microphone button
│   │   ├── VoiceSettings.tsx         # Settings
│   │   └── VoiceIndicator.tsx       # Recording state
```

#### 9.2 Voice Service

```typescript
// src/services/voiceInput.ts
class VoiceInputService {
  private recognition: SpeechRecognition | null = null;
  private isListening: boolean = false;

  async startListening(): Promise<void> {
    if (!("webkitSpeechRecognition" in window)) {
      throw new Error("Speech recognition not supported");
    }

    this.recognition = new webkitSpeechRecognition();
    this.recognition.continuous = true;
    this.recognition.interimResults = true;

    this.recognition.onresult = (event) => {
      const transcript = event.results[event.results.length - 1][0].transcript;
      this.emit("transcript", transcript);
    };

    this.recognition.start();
    this.isListening = true;
  }

  stopListening(): void {
    this.recognition?.stop();
    this.isListening = false;
  }

  // Continuous dictation during task execution
  onDictation(callback: (text: string) => void): void {
    this.on("transcript", callback);
  }
}
```

---

### Phase 10: Worktree Environment Variables (Week 10)

#### 10.1 Environment Setup

```yaml
# .codex/actions/env-setup.yaml
env_setup:
  strategy: copy_and_modify

  steps:
    - name: Create worktree .env
      run: |
        if [ -f .env.example ]; then
          cp .env.example ${WORKTREE_ROOT}/.env
          echo "Created .env from template"
        fi

    - name: Append project env
      run: |
        cat >> ${WORKTREE_ROOT}/.env << 'EOF'
        CODE_WORKTREE_ID=${WORKTREE_ID}
        CODE_ORIGIN_REPO=${REPO_NAME}
        EOF

    - name: Pull shared env (optional)
      run: |
        if command -v codex-env &> /dev/null; then
          codex-env pull >> ${WORKTREE_ROOT}/.env
        fi
```

#### 10.2 Environment Variable Options

```
Option 1: .env Copy
- Pros: Complete isolation, easy debugging
- Cons: Duplication, sync challenges

Option 2: Shared Paths
- Pros: Single source of truth
- Cons: Permission issues

Option 3: CLI Pull
- Pros: On-demand, secure
- Cons: Extra dependency

Decision: Support all three via .codex/config.yaml
```

---

## File Structure | ファイル構造

```
gui/
├── src/
│   ├── components/
│   │   ├── chat/
│   │   │   ├── ChatContainer.tsx
│   │   │   ├── ChatBubble.tsx        # ChatGPT style
│   │   │   ├── ChatSidebar.tsx       # Threads + Settings
│   │   │   ├── InputArea.tsx          # Voice + Attach + @
│   │   │   └── WelcomeScreen.tsx      # Suggestions
│   │   ├── worktree/
│   │   │   ├── WorktreeDashboard.tsx
│   │   │   ├── WorktreeCard.tsx
│   │   │   ├── BranchManager.tsx
│   │   │   └── ConflictResolver.tsx
│   │   ├── terminal/
│   │   │   ├── IntegratedTerminal.tsx
│   │   │   └── TerminalPanel.tsx
│   │   ├── skills/
│   │   │   ├── SkillsCatalog.tsx
│   │   │   ├── MCPServerManager.tsx
│   │   │   └── MCPDependencyChecker.tsx
│   │   ├── security/
│   │   │   ├── Sandbox.tsx
│   │   │   ├── NetworkPolicy.tsx
│   │   │   └── PermissionRequest.tsx
│   │   ├── automation/
│   │   │   ├── AutomationScheduler.tsx
│   │   │   └── InboxView.tsx
│   │   ├── figma/
│   │   │   ├── FigmaImportPanel.tsx
│   │   │   └── DesignExtractor.tsx
│   │   └── voice/
│   │       ├── VoiceInput.tsx
│   │       └── VoiceIndicator.tsx
│   ├── store/
│   │   ├── useChatStore.ts
│   │   ├── useWorktreeStore.ts       # New
│   │   ├── useTerminalStore.ts
│   │   ├── useNotificationStore.ts   # New
│   │   └── useAutomationStore.ts     # New
│   ├── services/
│   │   ├── mcpAutoSetup.ts           # New
│   │   ├── figmaMCP.ts               # New
│   │   ├── voiceInput.ts             # New
│   │   └── worktreeManager.ts        # New
│   ├── theme/
│   │   ├── chatGPTTheme.ts           # New
│   │   └── index.ts
│   ├── hooks/
│   │   ├── useWorktree.ts            # New
│   │   ├── useTerminal.ts
│   │   └── useVoiceInput.ts          # New
│   ├── types/
│   │   ├── mcp.ts
│   │   ├── worktree.ts               # New
│   │   ├── automation.ts             # New
│   │   └── notification.ts           # New
│   └── App.tsx
├── .codex/
│   ├── rules/
│   │   ├── guardrails.yaml           # New
│   │   ├── approvals.yaml            # New
│   │   └── security.yaml             # New
│   ├── actions/
│   │   ├── common.yaml               # New
│   │   ├── setup.yaml                # New
│   │   └── automation.yaml          # New
│   ├── skills/
│   │   └── catalog.yaml              # New
│   └── automations/
│       ├── schedules.yaml             # New
│       └── pr-handler.yaml           # Example
└── package.json
```

---

## Dependencies | 依存関係

```json
{
  "dependencies": {
    "@mui/material": "^7.3.7",
    "@emotion/react": "^11.14.0",
    "@emotion/styled": "^11.14.1",
    "@react-three/fiber": "^9.5.0",
    "@react-three/drei": "^10.7.7",
    "@react-three/xr": "^6.6.29",
    "three": "^0.182.0",
    "zustand": "^5.0.0",
    "xterm": "^5.3.0",
    "xterm-addon-fit": "^0.8.0",
    "react-router-dom": "^7.0.0",
    "framer-motion": "^11.0.0",
    "react-markdown": "^9.0.0",
    "react-syntax-highlighter": "^15.5.0"
  }
}
```

---

## Success Criteria | 成功基準

### Functional Requirements

1. **Command Center**
   - [ ] Multi-project selector works
   - [ ] Pinned threads persist
   - [ ] Auto-compaction runs without issues

2. **Worktree Orchestration**
   - [ ] Create worktree from any branch
   - [ ] Run parallel tasks without conflicts
   - [ ] Conflict resolution UI works

3. **Actions Integration**
   - [ ] Built-in terminal works
   - [ ] Action buttons execute correctly
   - [ ] Environment setup is automated

4. **Skills + MCP**
   - [ ] Skills catalog displays all skills
   - [ ] MCP servers auto-install dependencies
   - [ ] OAuth flow works for Figma

5. **Guardrails**
   - [ ] Network policy blocks by default
   - [ ] Permission requests show approval UI
   - [ ] Notifications work correctly

6. **Automations**
   - [ ] Scheduled tasks run on time
   - [ ] Results appear in inbox
   - [ ] PR handler works as expected

7. **Approval Rules**
   - [ ] Safe commands run without prompt
   - [ ] Risky commands require approval
   - [ ] Rules are configurable

8. **Figma Integration**
   - [ ] Import designs from Figma
   - [ ] Extract variables correctly
   - [ ] Generate usable code

9. **Voice Input**
   - [ ] Voice dictation works
   - [ ] Continuous input during tasks
   - [ ] Settings are accessible

### Quality Requirements

1. **Performance**
   - Worktree creation: < 5 seconds
   - Terminal response: < 100ms
   - Voice recognition: Real-time

2. **Reliability**
   - Uptime: 99%
   - Build success: 95%
   - Test coverage: 70%

---

## Timeline | タイムライン

| Week | Phase                  | Deliverables                           |
| ---- | ---------------------- | -------------------------------------- |
| 1    | UI Foundation          | ChatGPT-style UI components            |
| 2    | Worktree Orchestration | Worktree dashboard, parallel execution |
| 3    | Actions Integration    | Terminal, action runner                |
| 4    | Skills + MCP           | Skills catalog, MCP auto-setup         |
| 5    | Guardrails             | Sandbox, notifications                 |
| 6    | Automations            | Scheduler, inbox                       |
| 7    | Approval Rules         | Rules engine                           |
| 8    | Figma Integration      | Design import                          |
| 9    | Voice Input            | Voice dictation                        |
| 10   | Environment Setup      | Env var management                     |

---

## Migration Plan | 移行計画

### Backward Compatibility

```yaml
# .codex/config.yaml
migration:
  legacy_mode: false # Set to true during transition
  legacy_paths:
    - gui/src/components/chat/MessageBubble.tsx
    - gui/src/components/chat/ThreadList.tsx

  feature_flags:
    command_center: true
    worktree_orchestration: true
    actions_integration: true
    skills_mcp_integration: true
    guardrails: true
    automations: true
    figma_integration: true
    voice_input: true
```

### Rollback Plan

```bash
# Rollback command
codex gui rollback --version 2.14.1

# Emergency revert
git checkout HEAD -- gui/
git checkout HEAD -- .codex/
```

---

## References | 参考文献

- [ChatGPT UI Patterns](https://platform.openai.com/docs/assistants/tools/chatgpt-ui)
- [Git Worktree Documentation](https://git-scm.com/docs/git-worktree)
- [xterm.js Documentation](https://xtermjs.org/)
- [Web Speech API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Speech_API)
- [Figma API](https://www.figma.com/developers/api)

---

## Meta Prompt for Next Agent | 次のエージェントへのメタ指示

### Current Status | 現在のステータス

Implementing comprehensive Codex GUI unification plan covering UI/UX, worktree orchestration, actions integration, skills+MCP, guardrails, automations, Figma integration, voice input, and environment management.

**実装状況**: Planning complete, implementation started.

### Completed Items | 完了済み項目

| Item                        | Status           |
| --------------------------- | ---------------- |
| UI Foundation Plan          | ✅ Plan Complete |
| Worktree Orchestration Plan | ✅ Plan Complete |
| Actions Integration Plan    | ✅ Plan Complete |
| Skills + MCP Plan           | ✅ Plan Complete |
| Guardrails Plan             | ✅ Plan Complete |
| Automations Plan            | ✅ Plan Complete |
| Figma Integration Plan      | ✅ Plan Complete |
| Voice Input Plan            | ✅ Plan Complete |

### Next Tasks | 次のタスク

1. **Phase 1 Implementation**: Start with UI Foundation
   - Create ChatGPT-style theme
   - Redesign ChatBubble
   - Create WelcomeScreen

2. **Worktree Store**: Implement useWorktreeStore

3. **Migration Setup**: Configure feature flags

### Key Files | 主要ファイル

| File                                | Purpose                   |
| ----------------------------------- | ------------------------- |
| `gui/src/theme/chatGPTTheme.ts`     | ChatGPT-style theme       |
| `gui/src/store/useWorktreeStore.ts` | Worktree state management |
| `gui/.codex/rules/guardrails.yaml`  | Security rules            |
| `gui/.codex/actions/common.yaml`    | Action definitions        |

### Continue Implementation | 実装の継続

1. Read skill.md (this file) for context
2. Start with Phase 1 UI Foundation
3. Implement worktree store after UI components
4. Test each phase before moving to next

---

**Document Version**: 1.0
**Last Updated**: 2026-02-09
**Author**: Codex Implementation Team
**For**: Next agent continuing the implementation
