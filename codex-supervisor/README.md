# Codex Supervisor

**Official OpenAI Codex Agents SDK Supervisor** - MCP-Centric Multi-Agent Workflow Orchestrator

This is an independent package that provides advanced multi-agent orchestration capabilities for OpenAI Codex, implementing the official Agents SDK patterns including handoffs, guardrails, and worker agents.

## Overview

Codex Supervisor extends OpenAI Codex with sophisticated multi-agent workflows while maintaining compatibility with the official Skills + MCP + Agents SDK architecture. It acts as an intelligent coordinator that delegates complex tasks to specialized Codex Skills via MCP.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐
│   Supervisor    │────│   Codex MCP      │
│   Orchestrator  │    │   Server         │
│ (This Package)  │    └──────────────────┘
└─────────────────┘             │
         │                      │
         ├──────────────────────┤
         │                      │
    ┌────▼────┐    ┌─────────────┐
    │  Skill  │    │   Skill     │
    │ Worker A│    │  Worker B   │
    └─────────┘    └─────────────┘
```

## Official Agents SDK Compliance

This supervisor implements all official OpenAI Agents SDK concepts:

### 🔒 Guardrails
- **Security Guardrail**: Prevents dangerous operations and validates inputs
- **Quality Guardrail**: Ensures code quality and testing requirements
- **Custom Guardrails**: Extensible guardrail system for domain-specific rules

### 🤝 Handoffs
- **Task Handoffs**: Seamless task transitions between worker agents
- **Conditional Handoffs**: Context-aware task delegation
- **Handoff Tracking**: Complete audit trail of task transitions

### 👷 Worker Agents
- **Specialized Workers**: Domain-specific agent registration
- **Dynamic Assignment**: Intelligent task-to-worker matching
- **Parallel Execution**: Concurrent task processing with dependency management

## Installation

### As Standalone Package
```bash
# Clone this repository
git clone https://github.com/zapabob/codex-supervisor.git
cd codex-supervisor

# Install dependencies
pip install -r requirements.txt

# Optional: Install as package
pip install -e .
```

### Integration with Zapabob Codex
```bash
# If using zapabob/codex fork, the supervisor is included
cd tools/codex-supervisor
pip install -r requirements.txt
```

## Usage

### Basic Orchestration
```bash
# Start Codex MCP server first
codex mcp-server --port 3000

# Run supervisor
python supervisor.py "Implement user authentication system with role-based access control"
```

### Advanced Configuration
```python
from supervisor import CodexSupervisor

# Initialize with custom MCP endpoint
supervisor = CodexSupervisor(mcp_url="ws://localhost:3000")

# Add custom guardrails
supervisor.add_guardrail(custom_security_guardrail)

# Register specialized workers
supervisor.register_worker_agent("security-auditor", {
    "skills": ["vulnerability_scan", "compliance_check"],
    "capabilities": ["owasp_top_10", "pci_dss"]
})

# Execute complex workflow
result = await supervisor.orchestrate_workflow(
    "Build secure REST API with authentication and authorization"
)
```

### Integration with CI/CD
```yaml
# .github/workflows/code-review.yml
- name: Advanced Code Review
  run: |
    codex mcp-server --port 3000 &
    python codex-supervisor/supervisor.py "Review pull request for security, performance, and maintainability"
```

## MCP Integration

### Official Codex MCP Server
```bash
# Start Codex as MCP server
codex mcp-server --port 3000 --skills "architect,code-reviewer,executor"

# Supervisor connects via WebSocket
python supervisor.py --mcp-url ws://localhost:3000 "complex task"
```

### Tool Execution via MCP
- **codex_read_file**: Source code analysis
- **codex_write_file**: Code generation
- **codex_search_replace**: Precise modifications
- **codex_grep**: Pattern matching
- **codex_codebase_search**: Semantic search

## Skills Ecosystem

### Official Skills Integration
```bash
# Install community skills
codex $skill-install https://github.com/zapabob/codex-code-reviewer-skill
codex $skill-install https://github.com/zapabob/codex-executor-skill

# Use in supervisor workflows
python supervisor.py "Use code-reviewer and executor skills for implementation"
```

### Skill Development
```bash
# Create new skill
mkdir my-custom-skill
cat > my-custom-skill/SKILL.md << EOF
# My Custom Skill
## Overview
Specialized functionality for domain X
EOF

# Install and use
codex $skill-install ./my-custom-skill
```

## Configuration

### Environment Variables
```bash
# MCP Configuration
export CODEX_MCP_URL="ws://localhost:3000"
export CODEX_MCP_TIMEOUT="30"

# Supervisor Configuration
export SUPERVISOR_MAX_PARALLEL="5"
export SUPERVISOR_LOG_LEVEL="INFO"

# Skills Path
export CODEX_SKILLS_PATH="~/.codex/skills"
```

### Configuration File
```json
{
  "mcp": {
    "url": "ws://localhost:3000",
    "timeout": 30,
    "reconnect_attempts": 3
  },
  "supervisor": {
    "max_parallel_tasks": 5,
    "enable_handoffs": true,
    "guardrails_enabled": true
  },
  "workers": {
    "architect": {
      "skills": ["architecture_analysis"],
      "priority": "high"
    },
    "code-reviewer": {
      "skills": ["security_scan", "quality_check"],
      "priority": "high"
    }
  }
}
```

## Official Compliance

### ✅ Agents SDK Patterns
- **Guardrails**: Pre/post-execution validation
- **Handoffs**: Task transitions between agents
- **Worker Agents**: Specialized task execution
- **Structured Output**: JSON schema validation

### ✅ MCP Integration
- **Client/Server**: Full MCP protocol support
- **Tool Execution**: Direct Codex tool invocation
- **Resource Access**: MCP resource management
- **WebSocket Communication**: Real-time coordination

### ✅ Skills Architecture
- **SKILL.md Format**: Official progressive disclosure
- **Community Distribution**: GitHub-based skill sharing
- **Version Compatibility**: Codex version alignment

## Development

### Prerequisites
- Python 3.8+
- OpenAI Codex CLI
- MCP-compatible skills

### Testing
```bash
# Unit tests
python -m pytest tests/

# Integration tests
python test_workflow.py

# MCP bridge tests
python mcp_bridge.py
```

### Contributing
1. Follow official Agents SDK patterns
2. Maintain MCP compatibility
3. Add comprehensive tests
4. Update documentation

## Performance & Monitoring

### Metrics Collected
- Task execution times
- MCP communication latency
- Guardrail violation rates
- Handoff success rates
- Worker utilization

### Logging
```python
import logging
logging.basicConfig(level=logging.INFO)

# Enable detailed MCP logging
logging.getLogger('mcp_bridge').setLevel(logging.DEBUG)
```

## Troubleshooting

### Common Issues

**MCP Connection Failed**
```bash
# Check Codex server status
codex mcp-server --port 3000 --status

# Verify WebSocket connection
curl -I ws://localhost:3000
```

**Skill Not Found**
```bash
# List available skills
codex mcp tools/list | grep skills

# Check skill installation
ls ~/.codex/skills/
```

**Guardrail Blocks Execution**
```bash
# Check guardrail violations
python supervisor.py --verbose "task"

# Disable specific guardrails
export SUPERVISOR_DISABLE_GUARDRAILS="security"
```

## Migration Guide

### From Zapabob Codex
If migrating from zapabob/codex's internal orchestrator:

1. **Extract Supervisor**: Move `tools/orchestrator/` to standalone package
2. **Update Imports**: Change from internal imports to MCP-based calls
3. **Configure MCP**: Ensure Codex runs as MCP server
4. **Test Workflows**: Verify all existing workflows work via MCP

### From Other Orchestrators
```python
# Example migration from custom orchestrator
from codex_supervisor import CodexSupervisor

# Replace custom logic with official patterns
supervisor = CodexSupervisor()
supervisor.add_guardrail(my_custom_guardrail)
result = await supervisor.orchestrate_workflow(task)
```

## License

Apache License 2.0 - See LICENSE file.

## Related Projects

- [OpenAI Codex](https://github.com/openai/codex) - Official CLI
- [OpenAI Agents SDK](https://github.com/openai/openai-agents-js) - Official agents framework
- [Codex Skills](https://github.com/openai/skills) - Official skills catalog

---

**Compatibility**: Codex v2.9.0+
**Version**: 1.0.0-official
**Status**: Official Agents SDK Compliant