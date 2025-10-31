# Codex Protocol Client

TypeScript client SDK for communicating with the Codex orchestrator server.

## Installation

```bash
npm install @codex/protocol-client
```

## Usage

### Basic Usage

```typescript
import { ProtocolClient } from '@codex/protocol-client';

const client = new ProtocolClient();

// Connect to orchestrator
await client.connect();

// Get lock status
const lockStatus = await client.getLockStatus();
console.log(lockStatus);

// Write a file (queued operation)
const task = await client.fsWrite(
  'path/to/file.txt',
  'content',
  undefined, // preimage SHA (optional)
  'my-unique-key' // idempotency key (optional)
);
console.log('Task queued:', task);

// Subscribe to events
await client.subscribe(['lock.changed', 'fs.changed']);

client.on('event', (event) => {
  console.log('Event received:', event);
});

// Disconnect
client.disconnect();
```

### React Integration

```typescript
import { useProtocol, useProtocolEvent, Topics } from '@codex/protocol-client';

function MyComponent() {
  const { client, state, subscribe } = useProtocol({
    autoConnect: true,
    subscribeTopics: [Topics.LOCK_CHANGED, Topics.TOKENS_UPDATED],
  });

  // Listen to specific events
  useProtocolEvent(client, Topics.LOCK_CHANGED, (data) => {
    console.log('Lock changed:', data);
  });

  const handleAcquireLock = async () => {
    if (!client) return;
    await client.acquireLock('my-agent');
  };

  return (
    <div>
      <p>Status: {state.connected ? 'Connected' : 'Disconnected'}</p>
      <button onClick={handleAcquireLock}>Acquire Lock</button>
    </div>
  );
}
```

## API Reference

### ProtocolClient

#### Connection Methods
- `connect(): Promise<void>` - Connect to the orchestrator
- `disconnect(): void` - Disconnect from the orchestrator

#### Lock Operations
- `getLockStatus(): Promise<LockStatus>` - Get current lock status
- `acquireLock(owner: string, timeoutMs?: number): Promise<void>` - Acquire the repository lock
- `releaseLock(owner: string): Promise<void>` - Release the repository lock

#### Status Operations
- `getStatus(): Promise<StatusResponse>` - Get orchestrator status

#### File System Operations (Queued)
- `fsRead(path: string): Promise<string>` - Read a file
- `fsWrite(path: string, content: string, preimageSha?: string, idemKey?: string): Promise<TaskStatus>` - Write a file
- `fsPatch(unifiedDiff: string, baseCommit?: string, idemKey?: string): Promise<TaskStatus>` - Apply a patch

#### VCS Operations (Queued)
- `vcsDiff(): Promise<string>` - Get git diff
- `vcsCommit(message: string, idemKey?: string): Promise<TaskStatus>` - Commit changes
- `vcsPush(remote: string, branch: string, idemKey?: string): Promise<TaskStatus>` - Push to remote

#### Agent Operations
- `registerAgent(capabilities: string[], heartbeatMs: number, version: string): Promise<void>` - Register an agent
- `heartbeat(stats: Record<string, any>): Promise<void>` - Send heartbeat
- `listAgents(): Promise<any[]>` - List all registered agents

#### Token Operations
- `reportUsage(agentId: string, promptTokens: number, completionTokens: number, model: string): Promise<void>` - Report token usage
- `getBudget(): Promise<any>` - Get token budget

#### Session Operations
- `startSession(meta: Record<string, string>): Promise<string>` - Start a new session
- `endSession(id: string): Promise<void>` - End a session

#### Pub/Sub Operations
- `subscribe(topics: Topic[]): Promise<void>` - Subscribe to event topics
- `unsubscribe(topics: Topic[]): Promise<void>` - Unsubscribe from event topics

### Events

The client emits the following events:
- `connected` - When connection is established
- `disconnected` - When connection is lost
- `error` - When an error occurs
- `reconnecting` - When reconnecting (with attempt number)
- `event` - When any event is received
- `event:<topic>` - When a specific event topic is received

### Event Topics

Available event topics:
- `lock.changed` - Lock state changed
- `fs.changed` - File system changed
- `vcs.changed` - VCS state changed
- `tokens.updated` - Token budget updated
- `agent.join` - Agent joined
- `agent.leave` - Agent left
- `task.progress` - Task progress update
- `task.completed` - Task completed
- `task.failed` - Task failed

## Configuration

### TransportConfig

```typescript
interface TransportConfig {
  socketPath?: string; // Unix socket path (default: .codex/orchestrator.sock)
  tcpHost?: string; // TCP host (default: 127.0.0.1)
  tcpPort?: number; // TCP port (read from .codex/orchestrator.port if not specified)
  reconnectInterval?: number; // Reconnect interval in ms (default: 5000)
  maxReconnectAttempts?: number; // Max reconnect attempts (default: 10)
}
```

### ProtocolClientConfig

```typescript
interface ProtocolClientConfig {
  transport?: TransportConfig;
  requestTimeout?: number; // Request timeout in ms (default: 30000)
}
```

## Error Handling

The client throws errors with an optional `code` property:

```typescript
try {
  await client.fsWrite('file.txt', 'content', 'wrong-sha');
} catch (error) {
  if (error.code === 409) {
    console.error('Conflict: preimage mismatch');
  } else if (error.code === 429) {
    console.error('Rate limited, retry later');
  }
}
```

Error codes:
- `400` - Bad request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not found
- `409` - Conflict (preimage/base mismatch)
- `429` - Rate limit (queue full)
- `500` - Internal server error
- `503` - Service unavailable

## License

Apache-2.0
