# Codex Agents SDK Orchestrator

## Overview

**Official OpenAI Codex Agents SDK Orchestrator** - Multi-agent workflow supervisor that coordinates specialized skills for complex task execution. Implements the official Skills + Agents SDK architecture pattern.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐
│   Supervisor    │────│   Codex MCP      │
│   Orchestrator  │    │   Server         │
└─────────────────┘    └──────────────────┘
         │                        │
         ├────────────────────────┤
         │                        │
    ┌────▼────┐    ┌─────────────┐
    │  Skill  │    │   Skill     │
    │ Worker A│    │  Worker B   │
    └─────────┘    └─────────────┘
```

## Core Components

### Supervisor Orchestrator (`supervisor.py`)
- **Task Decomposition**: Complex tasks → subtasks with dependencies
- **Dependency Management**: Automatic task ordering and parallel execution
- **Progress Monitoring**: Real-time status tracking and reporting
- **Result Aggregation**: Comprehensive workflow reports

### MCP Bridge (`mcp_bridge.py`)
- **Codex Integration**: WebSocket-based MCP communication
- **Tool Execution**: Direct tool invocation through Codex
- **Fallback Support**: Direct skill execution when MCP unavailable
- **Error Handling**: Robust connection management and retries

### Worker Skills (`.codex/skills/`)
- **Official Format**: SKILL.md + scripts/ + references/
- **Specialization**: Each skill handles specific domain tasks
- **Parallel Execution**: Independent operation with result sharing

## Quick Start

### 1. Start Codex MCP Server
```bash
# Terminal 1: Start MCP server
codex mcp-server --port 3000
```

### 2. Run Orchestrator
```bash
# Terminal 2: Execute complex task
cd tools/orchestrator
python supervisor.py "Implement user authentication system with role-based access control"
```

### 3. View Results
```bash
# Check generated artifacts
ls artifacts/

# View workflow report
cat artifacts/workflow_report.json
```

## Workflow Execution

The orchestrator automatically:

1. **Analyzes** the input task using architect skill
2. **Decomposes** into subtasks with dependencies
3. **Assigns** tasks to appropriate skills (parallel when possible)
4. **Monitors** execution and handles errors
5. **Aggregates** results into comprehensive report

### Example Workflow
```
Input: "Build a REST API with authentication"

Tasks Generated:
├── architect_analysis (analyze requirements)
├── code_review (review existing code)
├── api_design (design REST endpoints)
├── auth_implementation (implement auth)
├── testing (generate tests)
└── documentation (create API docs)
```

## Skills Architecture

### Directory Structure
```
.codex/skills/
├── architect/           # System analysis & design
│   ├── SKILL.md        # Skill definition
│   ├── scripts/        # Execution scripts
│   │   └── run_architect.py
│   └── references/     # Documentation links
├── code-reviewer/      # Code quality analysis
├── executor/           # Implementation execution
├── researcher/         # Information gathering
└── test-gen/           # Test generation
```

### Skill Definition (SKILL.md)
Each skill includes:
- **Capabilities**: What the skill does
- **Tools Required**: MCP tools and permissions
- **Usage Examples**: How to invoke the skill
- **Output Format**: Expected results structure

## Advanced Configuration

### Custom MCP Endpoints
```python
# Custom MCP server URL
orchestrator = CodexOrchestrator(
    codex_mcp_url="ws://custom-server:3000"
)
```

### Skill Override
```python
# Use custom skills directory
import os
os.environ['CODEX_SKILLS_DIR'] = '/path/to/custom/skills'
```

### Parallel Limits
```python
# Limit concurrent skill execution
workflow = await orchestrator.orchestrate_workflow(
    task, max_parallel=2
)
```

## Integration Examples

### CI/CD Pipeline Integration
```bash
#!/bin/bash
# Start MCP server in background
codex mcp-server --port 3000 &

# Run orchestrator
python tools/orchestrator/supervisor.py "Review pull request #$PR_NUMBER"

# Check results
if [ $? -eq 0 ]; then
    echo "Review completed successfully"
    cat artifacts/workflow_report.json
else
    echo "Review failed"
    exit 1
fi
```

### IDE Extension Integration
```typescript
// VS Code extension integration
import { CodexOrchestrator } from './orchestrator';

export async function executeComplexRefactor(task: string) {
    const orchestrator = new CodexOrchestrator();
    const result = await orchestrator.orchestrate(task);

    // Update UI with results
    updateProgress(result.execution_summary);
    showArtifacts(result.artifacts);
}
```

## Development & Testing

### Prerequisites
- Python 3.8+
- Codex CLI (latest version)
- MCP-compatible skills in `.codex/skills/`

### Setup
```bash
# Install dependencies
pip install websockets asyncio

# Clone and setup skills
git clone https://github.com/zapabob/codex-skills.git .codex/skills/
```

### Testing
```bash
# Unit tests
python -m pytest tests/

# Integration test
python supervisor.py "Create a simple hello world function"

# MCP bridge test
python mcp_bridge.py
```

### Adding New Skills
```bash
# Create skill directory
mkdir .codex/skills/my-skill

# Create SKILL.md
cat > .codex/skills/my-skill/SKILL.md << EOF
# My Skill

## Overview
Description of what this skill does.

## Capabilities
- Feature 1
- Feature 2

## Tools Required
### MCP Tools
- tool_name
EOF

# Create execution script
mkdir .codex/skills/my-skill/scripts
# Add run_my-skill.py
```

## Performance & Observability

### Metrics Collected
- Task execution time
- Success/failure rates
- Skill utilization statistics
- MCP communication latency

### Logging
```bash
# Enable debug logging
export RUST_LOG=debug
codex mcp-server --port 3000

# View orchestrator logs
python supervisor.py "task" 2>&1 | tee orchestrator.log
```

## Troubleshooting

### Common Issues

**MCP Server Connection Failed**
```bash
# Check if server is running
netstat -an | grep 3000

# Restart MCP server
codex mcp-server --port 3000
```

**Skill Not Found**
```bash
# Check skill directory structure
ls -la .codex/skills/

# Validate SKILL.md format
python -c "import yaml; yaml.safe_load(open('.codex/skills/architect/SKILL.md'))"
```

**Permission Errors**
```bash
# Check file permissions
ls -la .codex/skills/*/scripts/*.py

# Ensure executable permissions
chmod +x .codex/skills/*/scripts/*.py
```

## Contributing

1. Follow the official Skills format
2. Add comprehensive tests
3. Update documentation
4. Submit PR with example usage

## License

Apache License 2.0 - See project LICENSE file.