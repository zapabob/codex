---
name: orchestrator
description: Intelligent multi-agent workflow orchestrator that coordinates specialized Codex skills for complex task execution. Implements official OpenAI Agents SDK patterns including guardrails, handoffs, and worker agents with MCP-centric architecture for seamless integration.
---

# Orchestrator Agent Skill

## Overview

Intelligent multi-agent workflow orchestrator that coordinates specialized Codex skills for complex task execution. Implements official OpenAI Agents SDK patterns including guardrails, handoffs, and worker agents with MCP-centric architecture for seamless integration.

## Capabilities

- **Task Decomposition**: Complex tasks ↁEdependency-aware subtasks with intelligent planning
- **Multi-Agent Coordination**: Parallel execution with worker agent assignment and handoffs
- **MCP Integration**: Direct Codex skill invocation via Model Context Protocol
- **Quality Assurance**: Built-in guardrails for security, performance, and best practices
- **Progress Tracking**: Real-time workflow monitoring with detailed reporting
- **Error Recovery**: Intelligent retry mechanisms and alternative execution paths

## Tools Required

### MCP Tools
- `codex_read_file` - Reading project files and specifications
- `codex_write_file` - Generating workflow artifacts and reports
- `codex_search_replace` - Updating configuration and code
- `codex_grep` - Searching for patterns and dependencies
- `codex_codebase_search` - Understanding project structure
- `grep` - Text pattern matching utilities
- `read_file` - File access for workflow operations
- `write` - Creating workflow outputs and logs

### File System Access
- **Read**: Full codebase access for analysis and planning
- **Write**: Limited to `./artifacts`, `./workflows`, `./reports`

### Network Access
- `ws://localhost:3000/*` - MCP server for Codex skill invocation
- `https://docs.rs/*` - Rust documentation for technical analysis
- `https://developer.mozilla.org/*` - Web standards reference

### Shell Commands
- `codex mcp-server` - Starting MCP server for skill access
- `git` - Version control operations for workflow management
- `python` - Workflow script execution
- `find` - File system traversal for project analysis

## Usage Examples

### Complex Feature Development
```bash
codex $orchestrator "Implement complete user authentication system with API endpoints, database models, and comprehensive testing"
```

### Code Refactoring Workflow
```bash
codex $orchestrator "Refactor monolithic service into microservices architecture with proper separation of concerns"
```

### Security Audit and Remediation
```bash
codex $orchestrator "Perform comprehensive security audit and implement fixes for all critical vulnerabilities"
```

### Performance Optimization
```bash
codex $orchestrator "Optimize application performance: identify bottlenecks, implement caching, and improve database queries"
```

## Output Format

### Workflow Execution Report
```
🎯 Orchestrator Workflow Report
══════════════════════════════════════════════════════════════════════════════╁E
Task: Implement user authentication system
Started: 2026-01-04 14:30:00 UTC
Duration: 245.8 seconds
Status: ✁ECOMPLETED

📊 Workflow Statistics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
Total Tasks: 8
Completed Tasks: 8 (100.0%)
Failed Tasks: 0 (0.0%)
Parallel Execution: 3 concurrent workers
MCP Calls: 12 successful
Handoffs: 3 inter-agent transitions

🔄 Task Execution Flow
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
1. 🔍 supervisor_analysis (Supervisor) - 2.3s ✁E
   └── Analyzed requirements and created execution plan

2. 🏗�E�Earchitect_analysis (Architect) - 8.7s ✁E
   └── Designed system architecture and data models

3. 🔒 security_review (QA Engineer) - 12.4s ✁E
   └── Security audit and vulnerability assessment

4. 📝 code_generation (Executor) - 45.2s ✁E
   └── Generated authentication API and models

5. 🧪 test_generation (Test Gen) - 23.1s ✁E
   └── Created comprehensive test suite

6. 🔍 code_review (Code Reviewer) - 18.9s ✁E
   └── Code quality analysis and best practices check

7. 📊 performance_analysis (QA Engineer) - 15.6s ✁E
   └── Performance benchmarking and optimization

8. 🚀 deployment_prep (Build Manager) - 9.2s ✁E
   └── Build configuration and deployment preparation

🛡�E�EGuardrail Results
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
Security Guardrail: ✁EPASSED (0 violations)
Quality Guardrail: ✁EPASSED (2 warnings addressed)
Performance Guardrail: ✁EPASSED (all metrics within limits)

🤁EAgent Handoffs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
1. Supervisor ↁEArchitect: Requirements analysis complete
2. Architect ↁEExecutor: Design specifications ready
3. Executor ↁEQA Engineer: Implementation ready for review

📁 Generated Artifacts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
├── 📄 src/auth/mod.rs (2.3KB) - Authentication module
├── 📄 src/auth/models.rs (1.8KB) - User and session models
├── 📄 src/auth/handlers.rs (4.2KB) - API endpoint handlers
├── 📄 tests/auth_tests.rs (3.1KB) - Comprehensive test suite
├── 📄 docs/api/auth.md (2.9KB) - API documentation
└── 📄 artifacts/workflow_report.json (8.7KB) - Execution details

⚠�E�EResolved Issues
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
- Addressed SQL injection vulnerability in login handler
- Fixed race condition in session management
- Improved error handling for edge cases

✁EQuality Metrics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
Code Coverage: 92%
Cyclomatic Complexity: 4.2 (target: <10)
Security Score: A+ (Excellent)
Performance Grade: A (Very Good)
Maintainability: 87/100

🎯 Next Recommended Actions
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
1. Deploy to staging environment for integration testing
2. Monitor performance metrics in production
3. Schedule regular security audits
4. Consider implementing rate limiting for auth endpoints

💡 Workflow Insights
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
- Most efficient path: Architect ↁEExecutor ↁEQA Engineer
- Bottleneck identified: Test generation (23.1s - consider parallelization)
- Skill utilization: QA Engineer (3 calls), Executor (2 calls), Architect (1 call)
- MCP performance: Average response time 2.1s, 100% success rate

📈 Performance Trends
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
Task Completion Rate: 100% (8/8 successful)
Average Task Duration: 17.2s
Parallelization Efficiency: 78% (vs sequential: 138.4s saved)
Resource Utilization: CPU 65%, Memory 2.1GB peak

🏆 Workflow Quality Score: A+ (Excellent)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━E
```

### JSON Output (for CI/CD Integration)
```json
{
  "workflow_id": "auth-system-impl-2026-01-04",
  "original_task": "Implement user authentication system",
  "execution_summary": {
    "total_tasks": 8,
    "completed_tasks": 8,
    "failed_tasks": 0,
    "total_duration": 245.8,
    "parallel_efficiency": 0.78,
    "mcp_success_rate": 1.0
  },
  "task_results": [
    {
      "id": "supervisor_analysis",
      "agent": "supervisor",
      "duration": 2.3,
      "success": true,
      "mcp_used": false
    },
    {
      "id": "architect_analysis",
      "agent": "architect",
      "duration": 8.7,
      "success": true,
      "mcp_used": true
    }
  ],
  "guardrail_results": {
    "security": {"passed": true, "violations": 0},
    "quality": {"passed": true, "violations": 0},
    "performance": {"passed": true, "violations": 0}
  },
  "quality_metrics": {
    "code_coverage": 0.92,
    "complexity_score": 4.2,
    "security_score": "A+",
    "performance_grade": "A",
    "maintainability_index": 87
  },
  "artifacts": [
    "src/auth/mod.rs",
    "src/auth/models.rs",
    "tests/auth_tests.rs",
    "docs/api/auth.md"
  ],
  "insights": {
    "optimal_path": "Architect ↁEExecutor ↁEQA Engineer",
    "bottlenecks": ["test_generation"],
    "recommendations": [
      "Deploy to staging",
      "Monitor performance",
      "Implement rate limiting"
    ]
  }
}
```

## Orchestration Patterns

### Sequential Execution Pattern
```
Task A ↁETask B ↁETask C
```
- Dependencies are strictly enforced
- Each task completes before the next begins
- Best for: Linear workflows, strict ordering requirements

### Parallel Execution Pattern
```
     ┌─ Task A ─━E
Task X ─┼─ Task B ─┼─ Task D
     └─ Task C ─━E
```
- Independent tasks run concurrently
- Dependencies determine execution order
- Best for: Independent subtasks, performance optimization

### Handoff Pattern (Agents SDK)
```
Agent A ↁEHandoff ↁEAgent B ↁEHandoff ↁEAgent C
```
- Context and state transfer between agents
- Specialized agent for each phase
- Best for: Complex multi-expert workflows

### Guardrail Pattern
```
Task ↁEGuardrail Check ↁE[Pass: Continue | Fail: Block/Retry]
```
- Pre/post-execution validation
- Automatic issue detection and correction
- Best for: Quality assurance, compliance

## Agent Roles and Capabilities

### Supervisor Agent
- **Primary Role**: Workflow planning and coordination
- **Capabilities**:
  - Task decomposition and dependency analysis
  - Agent assignment and scheduling
  - Progress monitoring and reporting
  - Error handling and recovery

### Worker Agents
- **Architect**: System design and architectural planning
- **Executor**: Code implementation and feature development
- **Code Reviewer**: Code quality analysis and best practices
- **QA Engineer**: Comprehensive testing and quality assurance
- **Build Manager**: Build orchestration and deployment

### Specialized Agents (Optional)
- **Security Auditor**: Security vulnerability assessment
- **Performance Analyst**: Performance optimization and monitoring
- **Test Generator**: Automated test suite creation
- **Documentation Specialist**: Technical documentation generation

## Configuration

### Workflow Profiles
```json
{
  "orchestrator": {
    "default_profile": "balanced",
    "max_parallel_tasks": 3,
    "mcp_timeout": 30,
    "enable_handoffs": true,
    "guardrails_enabled": true
  },
  "profiles": {
    "fast": {
      "max_parallel_tasks": 5,
      "mcp_timeout": 15,
      "skip_optional_checks": true
    },
    "thorough": {
      "max_parallel_tasks": 2,
      "mcp_timeout": 60,
      "additional_quality_checks": true
    },
    "balanced": {
      "max_parallel_tasks": 3,
      "mcp_timeout": 30,
      "standard_quality_checks": true
    }
  }
}
```

### Agent Configuration
```json
{
  "agents": {
    "supervisor": {
      "enabled": true,
      "priority": "high",
      "capabilities": ["planning", "coordination", "monitoring"]
    },
    "architect": {
      "enabled": true,
      "priority": "high",
      "skills": ["system_design", "architecture_planning"]
    },
    "executor": {
      "enabled": true,
      "priority": "high",
      "skills": ["code_generation", "implementation"]
    },
    "qa_engineer": {
      "enabled": true,
      "priority": "critical",
      "skills": ["quality_assurance", "testing", "security"]
    }
  }
}
```

### MCP Integration Settings
```json
{
  "mcp": {
    "server_url": "ws://localhost:3000",
    "connection_timeout": 10,
    "max_retries": 3,
    "fallback_enabled": true,
    "skill_discovery": true
  }
}
```

## Integration Points

### Development Workflow Integration
```bash
# Complex feature development
codex $orchestrator "Implement e-commerce checkout system"
# ↁEAutomatically coordinates architect, executor, qa-engineer, etc.

# Code refactoring campaign
codex $orchestrator "Modernize legacy codebase to current standards"
# ↁEParallel execution across multiple modules

# Security remediation
codex $orchestrator "Fix all OWASP Top 10 vulnerabilities"
# ↁEComprehensive security audit and fixes
```

### CI/CD Pipeline Integration
```yaml
# GitHub Actions
- name: Complex Feature Implementation
  run: codex $orchestrator "Implement ${{ inputs.feature_spec }}"

- name: Code Quality Improvement
  run: codex $orchestrator "Improve code quality across ${{ inputs.modules }}"

- name: Security Hardening
  run: codex $orchestrator "Implement security best practices"
```

### IDE Integration
```json
// VS Code tasks.json
{
  "tasks": [
    {
      "label": "Orchestrate Complex Task",
      "type": "shell",
      "command": "codex",
      "args": ["$orchestrator", "${input:taskDescription}"],
      "group": "build"
    }
  ]
}
```

## Performance Optimization

### Parallel Execution Tuning
```python
# Optimal parallelization based on task dependencies
max_parallel = min(len(ready_tasks), self.config.max_parallel_tasks)
if task_complexity > threshold:
    max_parallel = max(1, max_parallel // 2)  # Reduce for complex tasks
```

### MCP Performance Optimization
```python
# Connection pooling and request batching
mcp_pool = MCPConnectionPool(max_connections=5)
batched_requests = group_requests_by_skill(requests)
```

### Caching and Memoization
```python
# Cache agent responses and analysis results
@cached(ttl=3600)
def get_code_analysis(file_path: Path) -> AnalysisResult:
    return analyze_file(file_path)
```

## Monitoring and Observability

### Workflow Metrics
- Task completion rates and durations
- Agent utilization and performance
- MCP call success rates and latencies
- Guardrail violation patterns
- Handoff efficiency metrics

### Real-time Dashboard
```bash
# Start monitoring dashboard
codex $orchestrator --monitor --port 8080

# View workflow progress in browser
open http://localhost:8080/workflows
```

### Alerting System
```yaml
# Alert on workflow failures
alert_rules:
  - condition: "failure_rate > 0.1"
    action: "slack_notification"
    message: "High workflow failure rate detected"

  - condition: "avg_duration > 300"
    action: "email_alert"
    message: "Workflow performance degradation"
```

## Troubleshooting

### Common Issues

#### MCP Connection Failures
```bash
# Check MCP server status
codex mcp-server --status

# Restart MCP server
codex mcp-server --port 3000 --reset

# Test MCP connection
python -c "from mcp_bridge import create_mcp_bridge; print(create_mcp_bridge())"
```

#### Agent Handoff Failures
```bash
# Check agent availability
codex $orchestrator --list-agents

# Test specific agent
codex $architect "Test agent connectivity"

# Reset workflow state
codex $orchestrator --reset-workflow workflow_id
```

#### Performance Bottlenecks
```bash
# Profile workflow execution
codex $orchestrator --profile "complex task"

# Optimize parallel execution
export ORCHESTRATOR_MAX_PARALLEL=2  # Reduce for memory constraints

# Enable verbose logging
export ORCHESTRATOR_LOG_LEVEL=DEBUG
```

#### Guardrail False Positives
```yaml
# Customize guardrail rules
guardrails:
  security:
    exclude_patterns:
      - "test_*"
      - "mock_*"
  quality:
    min_complexity_threshold: 15
    allow_long_functions_in_legacy: true
```

## Best Practices

### Workflow Design
1. **Clear Task Boundaries**: Each task should have well-defined inputs/outputs
2. **Dependency Management**: Minimize circular dependencies
3. **Error Handling**: Implement retry logic and fallback strategies
4. **Resource Awareness**: Consider memory and CPU constraints

### Agent Coordination
1. **Skill Matching**: Assign tasks to agents with appropriate capabilities
2. **Context Preservation**: Maintain state across handoffs
3. **Communication**: Clear interfaces between agents
4. **Monitoring**: Track agent performance and reliability

### Quality Assurance
1. **Guardrail Placement**: Strategic guardrail positioning
2. **Progressive Validation**: Early detection of issues
3. **Feedback Loops**: Learn from workflow patterns
4. **Continuous Improvement**: Regular workflow optimization

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Workflow Patterns](https://www.enterpriseintegrationpatterns.com/)
- [Distributed Systems Design](https://microservices.io/)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-orchestrator-skill`
**Version**: 2.10.0
**Compatibility**: Codex v2.10.0+
