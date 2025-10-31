# Orchestration Architecture / オーケストレーションアーキテクチャ

## English

### Overview

The Codex orchestrator implements a hierarchical coordination system to manage concurrent access to repository resources. It ensures data consistency by serializing all write operations through a single-writer queue while allowing parallel read operations.

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Codex Orchestration Layer                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │   CLI    │  │   GUI    │  │ Agent 1  │  │ Agent 2  │ ...          │
│  │  Client  │  │  Client  │  │  Client  │  │  Client  │              │
│  └─────┬────┘  └─────┬────┘  └─────┬────┘  └─────┬────┘              │
│        │             │             │             │                     │
│        └─────────────┴─────────────┴─────────────┘                     │
│                            │                                           │
│                            │ JSON Lines / UDS / TCP                    │
│                            ▼                                           │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │             Orchestrator Server (Rust + Tokio)                  │ │
│  │                                                                  │ │
│  │  ┌────────────────┐  ┌────────────────┐  ┌─────────────────┐   │ │
│  │  │ Authentication │  │ Rate Limiting  │  │  Idempotency    │   │ │
│  │  │  (HMAC-SHA256) │  │   (429 errors) │  │  Cache (10min)  │   │ │
│  │  └────────────────┘  └────────────────┘  └─────────────────┘   │ │
│  │                                                                  │ │
│  │  ┌───────────────────────────────────────────────────────────┐  │ │
│  │  │         RPC Handler (Operation Router)                    │  │ │
│  │  │                                                             │  │ │
│  │  │  • lock.*    • status.*  • fs.*    • vcs.*                │  │ │
│  │  │  • agent.*   • task.*    • tokens.* • session.*           │  │ │
│  │  │  • subscribe / unsubscribe                                 │  │ │
│  │  └───────────────────────────────────────────────────────────┘  │ │
│  │         │                    │                    │              │ │
│  │         │ Reads              │ Writes             │ Events       │ │
│  │         ▼                    ▼                    │              │ │
│  │  ┌─────────────┐     ┌──────────────────┐        │              │ │
│  │  │ Lock State  │     │ Single-Writer    │        │              │ │
│  │  │ Token Budget│     │     Queue        │        │              │ │
│  │  │ Agent List  │     │ (capacity: 1024) │        │              │ │
│  │  └─────────────┘     └────────┬─────────┘        │              │ │
│  │                               │                  │              │ │
│  │                               │ Task             │              │ │
│  │                               ▼                  │              │ │
│  │                      ┌────────────────┐          │              │ │
│  │                      │ Task Executor  │──────────┤              │ │
│  │                      │                │          │              │ │
│  │                      │ • fs.write     │          │              │ │
│  │                      │ • fs.patch     │          │ Pub/Sub      │ │
│  │                      │ • vcs.commit   │          │ Broadcast    │ │
│  │                      │ • vcs.push     │          │              │ │
│  │                      └────────┬───────┘          │              │ │
│  │                               │                  │              │ │
│  │                               │ Apply            ▼              │ │
│  │                               ▼         ┌────────────────────┐  │ │
│  │                      ┌────────────────┐ │ Event Broadcaster │  │ │
│  │                      │  Repository    │ │                    │  │ │
│  │                      │  File System   │ │ • lock.changed     │  │ │
│  │                      │  VCS (Git)     │ │ • fs.changed       │  │ │
│  │                      └────────────────┘ │ • task.completed   │  │ │
│  │                                         └────────────────────┘  │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Hierarchical Roles

#### Main Orchestrator
- **Responsibility:** Serialize all write operations
- **Implementation:** Rust server with Tokio async runtime
- **Location:** Runs as a daemon process or spawned by CLI
- **State:** 
  - Lock state (owner, timestamp)
  - Token budget (per-agent usage tracking)
  - Agent registry
  - Task queue (FIFO with priority support)

#### Sub-Agents (Clients)
- **Types:** CLI, GUI, code reviewers, test generators, etc.
- **Capabilities:** Declared during registration
- **Constraints:** 
  - Must route all writes through orchestrator
  - Can perform reads independently
  - Receive real-time events via pub/sub
- **Lifecycle:**
  1. Connect to orchestrator
  2. Register (capabilities, version, heartbeat interval)
  3. Subscribe to relevant events
  4. Perform operations via RPC
  5. Send periodic heartbeats
  6. Disconnect gracefully

### Orchestration Phases

#### 1. Planning Phase
**Goal:** Decompose task into sub-tasks

**Participants:**
- Planning agent (main coordinator)
- Task analyzer

**Operations:**
- `task.submit` - Submit task with dependencies
- Subscribe to `task.progress` events

**Flow:**
```
Planning Agent → task.submit → Orchestrator
                                    ↓
                              Queue Task
                                    ↓
                          Broadcast task.progress
                                    ↓
                              Planning Agent
```

#### 2. Implementation Phase
**Goal:** Execute code changes

**Participants:**
- Code generation agents
- File system operations

**Operations:**
- `lock.acquire` - Acquire exclusive write lock
- `fs.write` - Queue file writes with preimage validation
- `vcs.diff` - Preview changes
- `lock.release` - Release lock

**Flow:**
```
Code Agent → lock.acquire → Orchestrator
                                ↓
                           Check lock
                                ↓
                           Grant lock
                                ↓
                      Broadcast lock.changed
                                ↓
         Code Agent → fs.write (multiple files) → Queue
                                                     ↓
                                                Process sequentially
                                                     ↓
                                              Validate preimage
                                                     ↓
                                                Apply changes
                                                     ↓
                                          Broadcast fs.changed
```

**Conflict Resolution:**
```
Agent A: fs.write("file.txt", content_A, sha="abc123")
         ↓ Queued (position 1)
         ↓ Processing...
         ↓ Applied successfully
         ↓ File SHA now "def456"

Agent B: fs.write("file.txt", content_B, sha="abc123")
         ↓ Queued (position 2)
         ↓ Processing...
         ↓ Preimage mismatch!
         ↓ Response: 409 Conflict
         {
           "status": "error",
           "code": 409,
           "message": "File was modified by Agent A",
           "data": {
             "expected": "abc123",
             "actual": "def456"
           }
         }
         ↓ Agent B must:
           1. Re-read file
           2. Merge/rebase changes
           3. Resubmit with new preimage SHA
```

#### 3. Review Phase
**Goal:** Validate changes

**Participants:**
- Code review agents
- Security scanners

**Operations:**
- `vcs.diff` - Get uncommitted changes
- `agent.register` - Register reviewer
- Subscribe to `vcs.changed` events

**Flow:**
```
Review Agent → vcs.diff → Orchestrator
                              ↓
                         Get git diff
                              ↓
                         Return diff
                              ↓
                       Review Agent (analyze)
                              ↓
                   Submit feedback via comments
```

#### 4. Commit Phase
**Goal:** Persist changes to VCS

**Participants:**
- Commit agent
- Push agent

**Operations:**
- `vcs.commit` - Commit with message
- `vcs.push` - Push to remote
- Idempotency keys prevent duplicate commits

**Flow:**
```
Commit Agent → vcs.commit(msg, idem_key="commit-feature-x")
                              ↓
                         Check idem_key
                              ↓
                    (First time) Queue commit
                              ↓
                         Process task
                              ↓
                       Execute git commit
                              ↓
                    Broadcast vcs.changed
                              ↓
                     Cache response (10min)
```

**Duplicate Prevention:**
```
Time T+0: vcs.commit(msg, idem_key="commit-abc")
          → Executed, response cached

Time T+5s: vcs.commit(msg, idem_key="commit-abc")
           → Returns cached response (no duplicate commit)

Time T+15min: Cache expired, new commit would be allowed
```

### Concurrency Control

#### Optimistic Locking
- **Mechanism:** Preimage SHA validation
- **When:** File writes, patches
- **Benefit:** No lock contention for independent changes
- **Cost:** Retry on conflict (409 response)

**Example:**
```javascript
// Agent A
const content = await client.fsRead('config.yaml');
const sha = sha256(content);
const newContent = modifyContent(content);

// May fail if another agent modified the file
await client.fsWrite('config.yaml', newContent, sha, 'update-config-001');
```

#### Pessimistic Locking
- **Mechanism:** Explicit lock acquisition
- **When:** Multi-file changes, critical sections
- **Benefit:** Guaranteed exclusive access
- **Cost:** Serialization, potential contention

**Example:**
```javascript
// Agent B
await client.acquireLock('agent-b', 30000);
try {
  // Exclusive access for 30 seconds
  await client.fsWrite('file1.txt', content1);
  await client.fsWrite('file2.txt', content2);
  await client.vcsCommit('Atomic update of file1 and file2');
} finally {
  await client.releaseLock('agent-b');
}
```

### Backpressure Handling

When the queue reaches capacity (default: 1024):

```json
{
  "status": "error",
  "code": 429,
  "message": "Queue full, retry after 5 seconds",
  "data": {
    "retry_after": 5,
    "queue_size": 1024,
    "queue_capacity": 1024
  }
}
```

**Client Strategy:**
1. Exponential backoff: 5s, 10s, 20s, ...
2. Max retries: 3-5
3. Surface error to user if persistent

### Event-Driven Updates

Real-time notification of state changes:

```javascript
// GUI subscribes to all relevant events
await client.subscribe([
  'lock.changed',
  'fs.changed',
  'vcs.changed',
  'tokens.updated',
  'agent.join',
  'agent.leave',
  'task.progress',
]);

client.on('event:lock.changed', (data) => {
  updateLockIndicator(data.locked, data.owner);
});

client.on('event:tokens.updated', (data) => {
  updateBudgetDisplay(data.remaining);
});
```

### Monitoring & Observability

**Status Endpoint:**
```javascript
const status = await client.getStatus();
// {
//   queue_size: 42,
//   idempotency_cache_size: 128,
//   connected_agents: 5,
//   lock_owner: "agent-123"
// }
```

**Agent Heartbeats:**
```javascript
setInterval(async () => {
  await client.heartbeat({
    tasks_completed: taskCount,
    avg_response_ms: avgResponseTime,
    memory_mb: process.memoryUsage().heapUsed / 1024 / 1024,
  });
}, 30000); // Every 30 seconds
```

---

## 日本語

### 概要

Codexオーケストレーターは、リポジトリリソースへの並行アクセスを管理するための階層的調整システムを実装しています。すべての書き込み操作をシングルライターキューを通じて直列化することでデータ整合性を保証し、並列読み取り操作を許可します。

### アーキテクチャ図

[英語セクションと同じ図を参照]

### 階層的役割

#### メインオーケストレーター
- **責任:** すべての書き込み操作を直列化
- **実装:** Tokio非同期ランタイムを使用したRustサーバー
- **場所:** デーモンプロセスとして実行、またはCLIによって起動
- **状態:**
  - ロック状態 (所有者、タイムスタンプ)
  - トークン予算 (エージェントごとの使用量追跡)
  - エージェントレジストリ
  - タスクキュー (優先度サポート付きFIFO)

#### サブエージェント (クライアント)
- **種類:** CLI、GUI、コードレビュアー、テストジェネレーター等
- **能力:** 登録時に宣言
- **制約:**
  - すべての書き込みをオーケストレーター経由でルーティング必須
  - 読み取りは独立して実行可能
  - pub/sub経由でリアルタイムイベントを受信
- **ライフサイクル:**
  1. オーケストレーターに接続
  2. 登録 (能力、バージョン、ハートビート間隔)
  3. 関連イベントをサブスクライブ
  4. RPC経由で操作を実行
  5. 定期的なハートビート送信
  6. 正常に切断

### オーケストレーションフェーズ

#### 1. 計画フェーズ
**目標:** タスクをサブタスクに分解

**参加者:**
- プランニングエージェント (メインコーディネーター)
- タスクアナライザー

**操作:**
- `task.submit` - 依存関係を持つタスクを送信
- `task.progress`イベントをサブスクライブ

#### 2. 実装フェーズ
**目標:** コード変更を実行

**参加者:**
- コード生成エージェント
- ファイルシステム操作

**操作:**
- `lock.acquire` - 排他的書き込みロックを取得
- `fs.write` - プリイメージ検証付きでファイル書き込みをキューに追加
- `vcs.diff` - 変更をプレビュー
- `lock.release` - ロックを解放

**競合解決:**
```
エージェントA: fs.write("file.txt", content_A, sha="abc123")
         ↓ キューに追加 (位置 1)
         ↓ 処理中...
         ↓ 正常に適用
         ↓ ファイルSHAは現在 "def456"

エージェントB: fs.write("file.txt", content_B, sha="abc123")
         ↓ キューに追加 (位置 2)
         ↓ 処理中...
         ↓ プリイメージ不一致!
         ↓ レスポンス: 409 Conflict
         {
           "status": "error",
           "code": 409,
           "message": "ファイルがエージェントAによって変更されました",
           "data": {
             "expected": "abc123",
             "actual": "def456"
           }
         }
         ↓ エージェントBは以下を実行する必要があります:
           1. ファイルを再読み込み
           2. 変更をマージ/リベース
           3. 新しいプリイメージSHAで再送信
```

#### 3. レビューフェーズ
**目標:** 変更を検証

**参加者:**
- コードレビューエージェント
- セキュリティスキャナー

**操作:**
- `vcs.diff` - コミットされていない変更を取得
- `agent.register` - レビュアーを登録
- `vcs.changed`イベントをサブスクライブ

#### 4. コミットフェーズ
**目標:** VCSに変更を永続化

**参加者:**
- コミットエージェント
- プッシュエージェント

**操作:**
- `vcs.commit` - メッセージ付きでコミット
- `vcs.push` - リモートにプッシュ
- 冪等性キーが重複コミットを防止

**重複防止:**
```
時刻 T+0: vcs.commit(msg, idem_key="commit-abc")
          → 実行、レスポンスをキャッシュ

時刻 T+5s: vcs.commit(msg, idem_key="commit-abc")
           → キャッシュされたレスポンスを返す (重複コミットなし)

時刻 T+15分: キャッシュ有効期限切れ、新しいコミットが許可される
```

### 並行性制御

#### 楽観的ロック
- **メカニズム:** プリイメージSHA検証
- **使用時:** ファイル書き込み、パッチ
- **利点:** 独立した変更のロック競合なし
- **コスト:** 競合時の再試行 (409レスポンス)

#### 悲観的ロック
- **メカニズム:** 明示的なロック取得
- **使用時:** 複数ファイル変更、クリティカルセクション
- **利点:** 排他的アクセスを保証
- **コスト:** 直列化、潜在的な競合

### バックプレッシャー処理

キューが容量に達したとき (デフォルト: 1024):

```json
{
  "status": "error",
  "code": 429,
  "message": "キューが満杯です、5秒後に再試行してください",
  "data": {
    "retry_after": 5,
    "queue_size": 1024,
    "queue_capacity": 1024
  }
}
```

**クライアント戦略:**
1. 指数バックオフ: 5秒、10秒、20秒、...
2. 最大再試行回数: 3-5回
3. 継続する場合はユーザーにエラーを表示

### イベント駆動更新

状態変化のリアルタイム通知:

```javascript
// GUIはすべての関連イベントをサブスクライブ
await client.subscribe([
  'lock.changed',
  'fs.changed',
  'vcs.changed',
  'tokens.updated',
  'agent.join',
  'agent.leave',
  'task.progress',
]);

client.on('event:lock.changed', (data) => {
  updateLockIndicator(data.locked, data.owner);
});

client.on('event:tokens.updated', (data) => {
  updateBudgetDisplay(data.remaining);
});
```
