# Token Management and Pair Programming Sessions

Codex provides centralized token budget management and pair programming session tracking for orchestrated workflows.

## Token Budget Management

### Overview

The token tracker monitors and controls token usage across all agents in an orchestration session, preventing budget overruns and providing warnings when approaching limits.

### Configuration

Default budget settings:
- **Total Budget**: 100,000 tokens
- **Warning Threshold**: 80,000 tokens (80%)
- **Per-Agent Limit**: 20,000 tokens

### Token Tracking

Token usage is tracked automatically:
- Prompt tokens
- Completion tokens
- Total tokens per agent
- Cumulative total across all agents

### Budget Enforcement

The system enforces budgets at multiple levels:

1. **Total Budget**: Prevents any agent from executing if total usage exceeds budget
2. **Warning Threshold**: Logs warnings when usage exceeds threshold
3. **Per-Agent Limit**: Warns when individual agents exceed their allocation

### Example Usage (Rust)

```rust
use codex_core::orchestration::{TokenTracker, TokenBudget};

// Create a token tracker with custom budget
let budget = TokenBudget {
    total_budget: 50_000,
    warning_threshold: 40_000,
    per_agent_limit: Some(10_000),
};
let tracker = TokenTracker::new(budget);

// Record token usage
tracker.record_usage("agent-1", 1000, 500)?;
tracker.record_usage("agent-2", 2000, 1000)?;

// Check total usage
let total = tracker.get_total_usage();
println!("Total tokens used: {}", total);

// Get per-agent breakdown
let usages = tracker.get_all_usages();
for (agent_id, usage) in usages {
    println!("{}: {} tokens", agent_id, usage.total_tokens);
}
```

### Environment Variables

Configure token budgets via environment variables:

```bash
# Set total budget
CODEX_TOKEN_BUDGET=100000

# Set warning threshold (as percentage)
CODEX_TOKEN_WARNING_PCT=80

# Set per-agent limit
CODEX_PER_AGENT_TOKEN_LIMIT=20000

# Disable token tracking (not recommended)
CODEX_DISABLE_TOKEN_TRACKING=false
```

## Pair Programming Sessions

### Overview

Pair programming sessions track collaborative coding sessions with metadata about participants, roles, and progress.

### Session Metadata

Each session includes:
- **Session ID**: Unique identifier
- **Participants**: List of participant names/IDs
- **Roles**: Corresponding roles (e.g., driver, navigator)
- **Started At**: Unix timestamp
- **Ended At**: Unix timestamp (null if active)
- **Notes**: Free-form text notes

### Session Lifecycle

1. **Start Session**:
   ```rust
   tracker.start_session(
       "session-001".to_string(),
       vec!["alice".to_string(), "bob".to_string()],
       vec!["driver".to_string(), "navigator".to_string()],
   );
   ```

2. **Add Notes**:
   ```rust
   tracker.add_session_notes(
       "session-001",
       "Implemented feature X, fixed bug Y"
   )?;
   ```

3. **End Session**:
   ```rust
   tracker.end_session("session-001")?;
   ```

### CLI Integration

Start a pair programming session:
```bash
codex pair --participants alice,bob --roles driver,navigator
```

View active sessions:
```bash
codex pair list
```

End a session:
```bash
codex pair end session-001
```

### Common Pair Programming Patterns

#### Driver-Navigator

```bash
codex pair \
  --participants alice,bob \
  --roles driver,navigator \
  --rotate-interval 15  # Rotate every 15 minutes
```

#### Mob Programming

```bash
codex pair \
  --participants alice,bob,charlie,diana \
  --roles driver,navigator,researcher,observer \
  --rotate-interval 10
```

#### Ping Pong

```bash
codex pair \
  --participants alice,bob \
  --roles test-writer,implementation \
  --mode ping-pong
```

## Token Usage in Pair Sessions

Token usage is automatically tracked during pair sessions:

```rust
// Get token usage for a session
let sessions = tracker.get_sessions();
for session in sessions {
    println!("Session: {}", session.session_id);
    println!("Participants: {:?}", session.participants);
    
    // Calculate per-participant token usage
    let session_start = session.started_at;
    let usages = tracker.get_all_usages();
    // Filter usages by timestamp...
}
```

## Best Practices

### Token Management

1. **Set Realistic Budgets**: Base budgets on task complexity
2. **Monitor Usage**: Check status regularly via GUI or CLI
3. **Handle Warnings**: Take action when approaching limits
4. **Reserve Buffer**: Keep 10-20% buffer for unexpected needs

### Pair Programming

1. **Rotate Regularly**: Switch roles every 10-20 minutes
2. **Take Notes**: Document decisions and insights
3. **Track Time**: Monitor session duration
4. **Review Sessions**: Analyze token usage and productivity

## Monitoring

### GUI Dashboard

The Orchestrator Status Dashboard shows:
- Real-time token usage with percentage bars
- Per-agent breakdown
- Active pair sessions
- Session history

### CLI Commands

```bash
# View token usage
codex status tokens

# View pair sessions
codex pair list --active

# View session history
codex pair history
```

## Troubleshooting

### Budget Exceeded

If you see:
```
Token budget exceeded: used 105000 / 100000
```

Solutions:
1. Increase budget: `CODEX_TOKEN_BUDGET=150000`
2. Reset usage: `codex tokens reset`
3. Optimize prompts to reduce token usage

### Session Not Found

If you see:
```
Session not found: session-001
```

Check:
1. Session ID is correct
2. Session hasn't already ended
3. Use `codex pair list` to see active sessions

## Integration with Orchestration

Token tracking and pair sessions integrate seamlessly with the auto-orchestrator:

```rust
use codex_core::orchestration::{
    AutoOrchestrator,
    TokenTracker,
    TokenBudget,
};

let budget = TokenBudget::default();
let tracker = Arc::new(TokenTracker::new(budget));

// Start pair session
tracker.start_session(
    "session-001".to_string(),
    vec!["user".to_string(), "ai-agent".to_string()],
    vec!["driver".to_string(), "navigator".to_string()],
);

// Orchestrator will use tracker for token management
let orchestrator = AutoOrchestrator::new(tracker.clone());

// Execute task
let result = orchestrator.execute_task("Implement feature X").await?;

// Record token usage
tracker.record_usage("main-agent", result.prompt_tokens, result.completion_tokens)?;

// End session
tracker.end_session("session-001")?;
```
