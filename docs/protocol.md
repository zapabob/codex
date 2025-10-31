# Protocol Specification / プロトコル仕様

## English

### Overview

The Codex Orchestrator Protocol is a versioned JSON-based RPC protocol for coordinating write operations across multiple agents, the CLI, and GUI. It ensures data consistency through single-writer serialization and provides real-time event notifications.

### Transport Layer

The protocol supports three transport modes in order of preference:

1. **Unix Domain Socket** (Unix/Linux/macOS)
   - Path: `.codex/orchestrator.sock`
   - Permissions: `0700` (owner read/write/execute only)
   - Most efficient local transport

2. **Windows Named Pipe** (Windows)
   - Name: `\\.\pipe\codex-orchestrator`
   - Local machine only

3. **TCP Fallback** (all platforms)
   - Host: `127.0.0.1` (localhost only)
   - Port: Ephemeral (stored in `.codex/orchestrator.port`)
   - Used when UDS/Pipe unavailable

### Protocol Framing

Messages use **JSON Lines** format:
- One JSON object per line
- Lines terminated with `\n` (LF)
- UTF-8 encoding

### Message Envelope

All messages share a common envelope structure:

```json
{
  "v": "1.0",
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "ts": "2025-10-31T16:20:28.933Z",
  "type": "request",
  "op": "fs.write",
  "session": "session-id-optional",
  "agent": {
    "id": "agent-123",
    "role": "code-reviewer"
  },
  "idem_key": "unique-operation-key",
  "body": { }
}
```

**Fields:**
- `v` (string, required): Protocol version (currently "1.0")
- `id` (string, required): Unique message ID (UUID recommended)
- `ts` (string, required): RFC3339 timestamp
- `type` (string, required): Message type: "request" | "response" | "event"
- `op` (string, required): Operation name (e.g., "fs.write", "lock.acquire")
- `session` (string, optional): Session ID for grouping related operations
- `agent` (object, optional): Agent metadata
  - `id` (string): Agent identifier
  - `role` (string): Agent role/type
- `idem_key` (string, optional): Idempotency key for deduplication
- `body` (object, required): Operation-specific payload

### Response Format

Responses include status and data:

```json
{
  "v": "1.0",
  "id": "response-uuid",
  "ts": "2025-10-31T16:20:29.100Z",
  "type": "response",
  "op": "fs.write",
  "body": {
    "status": "ok",
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "data": {
      "task_id": "task-789",
      "position": 3,
      "total": 1024
    }
  }
}
```

**Body Fields:**
- `status` (string): "ok" | "error"
- `code` (number, optional): HTTP-style status code on error
- `message` (string, optional): Error message
- `request_id` (string): ID of the original request
- `data` (any): Operation-specific result

### Error Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 400  | Bad Request | Invalid request format or parameters |
| 401  | Unauthorized | Missing or invalid authentication |
| 403  | Forbidden | Operation not allowed by policy |
| 404  | Not Found | Resource does not exist |
| 409  | Conflict | Preimage/base commit mismatch (optimistic lock failure) |
| 429  | Rate Limited | Queue full, retry after N seconds |
| 500  | Internal Error | Server-side error |
| 503  | Service Unavailable | Server not ready |

**409 Conflict** is used for optimistic concurrency control:
```json
{
  "status": "error",
  "code": 409,
  "message": "Preimage SHA mismatch: expected abc123, got def456. File was modified by another agent.",
  "data": {
    "expected": "abc123",
    "actual": "def456"
  }
}
```

**429 Rate Limit** indicates queue backpressure:
```json
{
  "status": "error",
  "code": 429,
  "message": "Queue full (capacity: 1024), retry after 5 seconds",
  "data": {
    "retry_after": 5
  }
}
```

### Operations

#### Lock Operations

**lock.status** - Get lock status
```json
Request:  { "op": "lock.status", "body": {} }
Response: {
  "data": {
    "locked": true,
    "owner": "agent-123",
    "acquired_at": "2025-10-31T16:15:00.000Z"
  }
}
```

**lock.acquire** - Acquire lock
```json
Request:  {
  "op": "lock.acquire",
  "body": { "owner": "agent-123", "timeout_ms": 30000 }
}
Response: { "data": {} }
```

**lock.release** - Release lock
```json
Request:  { "op": "lock.release", "body": { "owner": "agent-123" } }
Response: { "data": {} }
```

#### File System Operations (Queued)

**fs.read** - Read file (immediate)
```json
Request:  { "op": "fs.read", "body": { "path": "src/main.rs" } }
Response: { "data": "file content..." }
```

**fs.write** - Write file (queued)
```json
Request:  {
  "op": "fs.write",
  "idem_key": "write-main-rs-v1",
  "body": {
    "path": "src/main.rs",
    "content": "fn main() {...}",
    "preimage_sha": "abc123"  // optional, for conflict detection
  }
}
Response: {
  "data": {
    "task_id": "task-001",
    "position": 1,
    "total": 1024
  }
}
```

**fs.patch** - Apply unified diff (queued)
```json
Request:  {
  "op": "fs.patch",
  "idem_key": "patch-feature-x",
  "body": {
    "unified_diff": "diff --git a/file.txt...",
    "base_commit": "HEAD"  // optional git ref
  }
}
```

#### VCS Operations (Queued)

**vcs.diff** - Get git diff (immediate)
```json
Request:  { "op": "vcs.diff", "body": {} }
Response: { "data": "diff --git a/..." }
```

**vcs.commit** - Commit changes (queued)
```json
Request:  {
  "op": "vcs.commit",
  "idem_key": "commit-feature-x",
  "body": { "message": "feat: add feature X" }
}
```

**vcs.push** - Push to remote (queued)
```json
Request:  {
  "op": "vcs.push",
  "idem_key": "push-origin-main",
  "body": { "remote": "origin", "branch": "main" }
}
```

#### Agent Operations

**agent.register** - Register agent
```json
Request:  {
  "op": "agent.register",
  "body": {
    "capabilities": ["code-review", "testing"],
    "heartbeat_ms": 30000,
    "version": "1.0.0"
  }
}
```

**agent.heartbeat** - Send heartbeat
```json
Request:  {
  "op": "agent.heartbeat",
  "body": {
    "stats": {
      "tasks_completed": 42,
      "avg_response_ms": 1500
    }
  }
}
```

**agent.list** - List agents
```json
Request:  { "op": "agent.list", "body": {} }
Response: { "data": { "agents": [...] } }
```

#### Pub/Sub

**subscribe** - Subscribe to events
```json
Request:  {
  "op": "subscribe",
  "body": { "topics": ["lock.changed", "fs.changed"] }
}
```

**Event notification:**
```json
{
  "type": "event",
  "op": "lock.changed",
  "ts": "2025-10-31T16:20:30.000Z",
  "body": {
    "locked": true,
    "owner": "agent-456"
  }
}
```

**Event Topics:**
- `lock.changed` - Lock state changed
- `fs.changed` - File system changed
- `vcs.changed` - VCS state changed
- `tokens.updated` - Token budget updated
- `agent.join` - Agent registered
- `agent.leave` - Agent disconnected
- `task.progress` - Task progress update
- `task.completed` - Task completed
- `task.failed` - Task failed

### Security

#### HMAC Authentication

All requests must include an HMAC-SHA256 signature in a custom header or field:

```json
{
  "op": "fs.write",
  "auth": {
    "timestamp": "2025-10-31T16:20:28.933Z",
    "signature": "base64-encoded-hmac"
  },
  "body": {...}
}
```

**Signature Calculation:**
```
payload = message_json + timestamp_rfc3339
signature = HMAC-SHA256(secret, payload)
```

**Secret Management:**
- Generated on first run: `.codex/secret` (base64-encoded, 256 bits)
- Permissions: `0600` (owner read/write only)
- Rotation: Manual via `orchestrator secret rotate`

**Timestamp Validation:**
- Maximum skew: 5 minutes
- Prevents replay attacks

#### Local-Only Binding

- Unix sockets: Limited to local user by file permissions
- Named pipes: Local machine only
- TCP: Bound to `127.0.0.1` (loopback only)
- No remote network exposure

### Idempotency

Operations with `idem_key` are cached for 10 minutes:

```json
{
  "op": "fs.write",
  "idem_key": "write-config-2025-10-31-001",
  "body": {...}
}
```

- First request: Executed normally
- Duplicate request: Returns cached response
- Cache window: 10 minutes
- Storage: In-memory (lost on restart)

---

## 日本語

### 概要

Codex Orchestrator Protocolは、複数のエージェント、CLI、GUIの書き込み操作を調整するためのバージョン管理されたJSONベースのRPCプロトコルです。シングルライター直列化によりデータ整合性を保証し、リアルタイムイベント通知を提供します。

### トランスポート層

プロトコルは優先順位順に3つのトランスポートモードをサポートします:

1. **Unixドメインソケット** (Unix/Linux/macOS)
   - パス: `.codex/orchestrator.sock`
   - パーミッション: `0700` (所有者のみ読み書き実行可能)
   - 最も効率的なローカルトランスポート

2. **Windows名前付きパイプ** (Windows)
   - 名前: `\\.\pipe\codex-orchestrator`
   - ローカルマシンのみ

3. **TCPフォールバック** (全プラットフォーム)
   - ホスト: `127.0.0.1` (localhostのみ)
   - ポート: エフェメラル (`.codex/orchestrator.port`に保存)
   - UDS/Pipeが利用できない場合に使用

### プロトコルフレーミング

メッセージは**JSON Lines**形式を使用:
- 1行に1つのJSONオブジェクト
- `\n` (LF)で終端
- UTF-8エンコーディング

### メッセージエンベロープ

全メッセージは共通のエンベロープ構造を共有:

```json
{
  "v": "1.0",
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "ts": "2025-10-31T16:20:28.933Z",
  "type": "request",
  "op": "fs.write",
  "session": "session-id-optional",
  "agent": {
    "id": "agent-123",
    "role": "code-reviewer"
  },
  "idem_key": "unique-operation-key",
  "body": { }
}
```

**フィールド:**
- `v` (文字列、必須): プロトコルバージョン (現在 "1.0")
- `id` (文字列、必須): 一意のメッセージID (UUID推奨)
- `ts` (文字列、必須): RFC3339タイムスタンプ
- `type` (文字列、必須): メッセージタイプ: "request" | "response" | "event"
- `op` (文字列、必須): 操作名 (例: "fs.write", "lock.acquire")
- `session` (文字列、オプション): 関連操作をグループ化するセッションID
- `agent` (オブジェクト、オプション): エージェントメタデータ
  - `id` (文字列): エージェント識別子
  - `role` (文字列): エージェントの役割/タイプ
- `idem_key` (文字列、オプション): 重複排除用の冪等性キー
- `body` (オブジェクト、必須): 操作固有のペイロード

### レスポンス形式

レスポンスはステータスとデータを含む:

```json
{
  "v": "1.0",
  "id": "response-uuid",
  "ts": "2025-10-31T16:20:29.100Z",
  "type": "response",
  "op": "fs.write",
  "body": {
    "status": "ok",
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "data": {
      "task_id": "task-789",
      "position": 3,
      "total": 1024
    }
  }
}
```

**Bodyフィールド:**
- `status` (文字列): "ok" | "error"
- `code` (数値、オプション): エラー時のHTTP形式ステータスコード
- `message` (文字列、オプション): エラーメッセージ
- `request_id` (文字列): 元のリクエストのID
- `data` (任意): 操作固有の結果

### エラーコード

| コード | 意味 | 説明 |
|------|---------|-------------|
| 400  | Bad Request | 無効なリクエスト形式またはパラメータ |
| 401  | Unauthorized | 認証情報の欠落または無効 |
| 403  | Forbidden | ポリシーにより操作が許可されない |
| 404  | Not Found | リソースが存在しない |
| 409  | Conflict | プリイメージ/ベースコミットの不一致 (楽観的ロック失敗) |
| 429  | Rate Limited | キューが満杯、N秒後に再試行 |
| 500  | Internal Error | サーバー側エラー |
| 503  | Service Unavailable | サーバー未準備 |

**409 Conflict**は楽観的並行性制御に使用:
```json
{
  "status": "error",
  "code": 409,
  "message": "プリイメージSHAが一致しません: 期待値 abc123、実際 def456。ファイルが別のエージェントによって変更されました。",
  "data": {
    "expected": "abc123",
    "actual": "def456"
  }
}
```

**429 Rate Limit**はキューのバックプレッシャーを示す:
```json
{
  "status": "error",
  "code": 429,
  "message": "キューが満杯です (容量: 1024)、5秒後に再試行してください",
  "data": {
    "retry_after": 5
  }
}
```

### セキュリティ

#### HMAC認証

全リクエストはHMAC-SHA256署名を含む必要があります:

```json
{
  "op": "fs.write",
  "auth": {
    "timestamp": "2025-10-31T16:20:28.933Z",
    "signature": "base64エンコードされたHMAC"
  },
  "body": {...}
}
```

**署名計算:**
```
payload = message_json + timestamp_rfc3339
signature = HMAC-SHA256(secret, payload)
```

**シークレット管理:**
- 初回実行時に生成: `.codex/secret` (base64エンコード、256ビット)
- パーミッション: `0600` (所有者のみ読み書き)
- ローテーション: `orchestrator secret rotate`で手動実行

**タイムスタンプ検証:**
- 最大ずれ: 5分
- リプレイ攻撃を防止

#### ローカル専用バインディング

- Unixソケット: ファイルパーミッションによりローカルユーザーに制限
- 名前付きパイプ: ローカルマシンのみ
- TCP: `127.0.0.1` (ループバックのみ) にバインド
- リモートネットワークへの露出なし

### 冪等性

`idem_key`を持つ操作は10分間キャッシュされます:

```json
{
  "op": "fs.write",
  "idem_key": "write-config-2025-10-31-001",
  "body": {...}
}
```

- 最初のリクエスト: 通常通り実行
- 重複リクエスト: キャッシュされたレスポンスを返す
- キャッシュウィンドウ: 10分
- ストレージ: インメモリ (再起動時に消失)
