# Orchestrator Status API

The Codex orchestrator exposes a status API that provides real-time information about the orchestration state, including lock status, active agents, token usage, and pair programming sessions.

## Overview

The status API is designed to be consumed by:
- GUI applications for real-time monitoring
- CLI tools for status checks
- External monitoring systems

## Status Endpoint

**Mock Endpoint (GUI Development):**
```
GET http://localhost:8787/status
```

**Production (Future):**
The API will be exposed via:
- HTTP JSON endpoint on localhost
- Unix domain socket (Linux/Mac)
- Named pipe (Windows)

## Status Response Format

```json
{
  "lock": {
    "holder": "12345@my-computer",
    "pid": 12345,
    "hostname": "my-computer",
    "since": 1698765432,
    "stale": false
  },
  "agents": [
    {
      "id": "main-supervisor",
      "name": "Main Supervisor",
      "status": "active",
      "tasks_completed": 42,
      "tasks_failed": 2
    }
  ],
  "tokens": {
    "used": 15000,
    "budget": 100000,
    "by_agent": [
      ["main-supervisor", {
        "prompt_tokens": 10000,
        "completion_tokens": 5000,
        "total_tokens": 15000,
        "last_updated": 1698765432
      }]
    ]
  },
  "sessions": [
    {
      "session_id": "session-001",
      "participants": ["user1", "user2"],
      "roles": ["driver", "navigator"],
      "started_at": 1698765432,
      "ended_at": null,
      "notes": "Working on feature X"
    }
  ]
}
```

## Data Model

### Lock Status

| Field | Type | Description |
|-------|------|-------------|
| `holder` | string | Format: `{pid}@{hostname}` |
| `pid` | number | Process ID of lock holder |
| `hostname` | string | Hostname of machine holding lock |
| `since` | number | Unix timestamp when lock was acquired |
| `stale` | boolean | Whether the lock is stale |

If no lock exists, this field is `null`.

### Agent Status

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique agent identifier |
| `name` | string | Human-readable agent name |
| `status` | string | Current status: `active`, `idle`, `error` |
| `tasks_completed` | number | Number of completed tasks |
| `tasks_failed` | number | Number of failed tasks |

### Token Status

| Field | Type | Description |
|-------|------|-------------|
| `used` | number | Total tokens used across all agents |
| `budget` | number | Total token budget |
| `by_agent` | array | Per-agent token usage breakdown |

#### Token Usage Detail

| Field | Type | Description |
|-------|------|-------------|
| `prompt_tokens` | number | Tokens used for prompts |
| `completion_tokens` | number | Tokens used for completions |
| `total_tokens` | number | Total tokens (prompt + completion) |
| `last_updated` | number | Unix timestamp of last update |

### Pair Session

| Field | Type | Description |
|-------|------|-------------|
| `session_id` | string | Unique session identifier |
| `participants` | array | List of participant names |
| `roles` | array | List of roles (same length as participants) |
| `started_at` | number | Unix timestamp when session started |
| `ended_at` | number | Unix timestamp when session ended (null if active) |
| `notes` | string | Session notes |

## Usage in GUI

### React Hook

```typescript
import { useOrchestratorStatus } from '@/hooks/useOrchestratorStatus';

function MyComponent() {
  const { status, isLoading, error, refetch } = useOrchestratorStatus({
    pollingInterval: 5000, // Poll every 5 seconds
    enabled: true,
  });

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <h2>Token Usage: {status.tokens.used} / {status.tokens.budget}</h2>
      <h3>Active Agents: {status.agents.length}</h3>
    </div>
  );
}
```

### Dashboard Component

```typescript
import { OrchestratorStatusDashboard } from '@/components/organisms/OrchestratorStatusDashboard';

function StatusPage() {
  return <OrchestratorStatusDashboard />;
}
```

## Polling Strategy

The GUI implements adaptive polling:

1. **Normal Operation**: Poll every 5 seconds
2. **High Activity**: Poll every 2 seconds when changes detected
3. **Idle**: Poll every 10 seconds when no activity
4. **Error Recovery**: Exponential backoff on errors

## Future Enhancements

### WebSocket Support

Real-time updates via WebSocket:
```javascript
const ws = new WebSocket('ws://localhost:8787/status');
ws.onmessage = (event) => {
  const status = JSON.parse(event.data);
  // Handle status update
};
```

### Server-Sent Events (SSE)

Alternative to WebSocket:
```javascript
const eventSource = new EventSource('http://localhost:8787/status/stream');
eventSource.onmessage = (event) => {
  const status = JSON.parse(event.data);
  // Handle status update
};
```

## Security Considerations

- Status API is only accessible from localhost
- No authentication required for local access
- Sensitive data (API keys, credentials) not included in status
- Can be disabled via `CODEX_DISABLE_STATUS_API=true`

## Performance

- Minimal overhead: <1ms per status request
- Cached for 100ms to prevent excessive queries
- Async updates don't block main orchestrator
