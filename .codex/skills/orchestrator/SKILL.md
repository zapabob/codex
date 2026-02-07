---
name: orchestrator
description: Intelligent multi-agent workflow orchestrator that coordinates specialized Codex skills for complex task execution. Implements official OpenAI Agents SDK patterns including guardrails, handoffs, and worker agents with MCP-centric architecture for seamless integration.
---

# Orchestrator Agent Skill

## Overview

Intelligent multi-agent workflow orchestrator that coordinates specialized Codex skills for complex task execution. Implements official OpenAI Agents SDK patterns including guardrails, handoffs, and worker agents with MCP-centric architecture for seamless integration.

## Capabilities

- **Task Decomposition**: Complex tasks âEdependency-aware subtasks with intelligent planning
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
ð¯ Orchestrator Workflow Report
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Task: Implement user authentication system
Started: 2026-01-04 14:30:00 UTC
Duration: 245.8 seconds
Status: âECOMPLETED

ð Workflow Statistics
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Total Tasks: 8
Completed Tasks: 8 (100.0%)
Failed Tasks: 0 (0.0%)
Parallel Execution: 3 concurrent workers
MCP Calls: 12 successful
Handoffs: 3 inter-agent transitions

ð Task Execution Flow
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. ð supervisor_analysis (Supervisor) - 2.3s âE
   âââ Analyzed requirements and created execution plan

2. ðEEarchitect_analysis (Architect) - 8.7s âE
   âââ Designed system architecture and data models

3. ð security_review (QA Engineer) - 12.4s âE
   âââ Security audit and vulnerability assessment

4. ð code_generation (Executor) - 45.2s âE
   âââ Generated authentication API and models

5. ð§ª test_generation (Test Gen) - 23.1s âE
   âââ Created comprehensive test suite

6. ð code_review (Code Reviewer) - 18.9s âE
   âââ Code quality analysis and best practices check

7. ð performance_analysis (QA Engineer) - 15.6s âE
   âââ Performance benchmarking and optimization

8. ð deployment_prep (Build Manager) - 9.2s âE
   âââ Build configuration and deployment preparation

ð¡EEGuardrail Results
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Security Guardrail: âEPASSED (0 violations)
Quality Guardrail: âEPASSED (2 warnings addressed)
Performance Guardrail: âEPASSED (all metrics within limits)

ð¤EAgent Handoffs
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. Supervisor âEArchitect: Requirements analysis complete
2. Architect âEExecutor: Design specifications ready
3. Executor âEQA Engineer: Implementation ready for review

ð Generated Artifacts
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âââ ð src/auth/mod.rs (2.3KB) - Authentication module
âââ ð src/auth/models.rs (1.8KB) - User and session models
âââ ð src/auth/handlers.rs (4.2KB) - API endpoint handlers
âââ ð tests/auth_tests.rs (3.1KB) - Comprehensive test suite
âââ ð docs/api/auth.md (2.9KB) - API documentation
âââ ð artifacts/workflow_report.json (8.7KB) - Execution details

â EEResolved Issues
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
- Addressed SQL injection vulnerability in login handler
- Fixed race condition in session management
- Improved error handling for edge cases

âEQuality Metrics
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Code Coverage: 92%
Cyclomatic Complexity: 4.2 (target: <10)
Security Score: A+ (Excellent)
Performance Grade: A (Very Good)
Maintainability: 87/100

ð¯ Next Recommended Actions
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. Deploy to staging environment for integration testing
2. Monitor performance metrics in production
3. Schedule regular security audits
4. Consider implementing rate limiting for auth endpoints

ð¡ Workflow Insights
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
- Most efficient path: Architect âEExecutor âEQA Engineer
- Bottleneck identified: Test generation (23.1s - consider parallelization)
- Skill utilization: QA Engineer (3 calls), Executor (2 calls), Architect (1 call)
- MCP performance: Average response time 2.1s, 100% success rate

ð Performance Trends
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Task Completion Rate: 100% (8/8 successful)
Average Task Duration: 17.2s
Parallelization Efficiency: 78% (vs sequential: 138.4s saved)
Resource Utilization: CPU 65%, Memory 2.1GB peak

ð Workflow Quality Score: A+ (Excellent)
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
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
    "optimal_path": "Architect âEExecutor âEQA Engineer",
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
Task A âETask B âETask C
```
- Dependencies are strictly enforced
- Each task completes before the next begins
- Best for: Linear workflows, strict ordering requirements

### Parallel Execution Pattern
```
     ââ Task A ââE
Task X ââ¼â Task B ââ¼â Task D
     ââ Task C ââE
```
- Independent tasks run concurrently
- Dependencies determine execution order
- Best for: Independent subtasks, performance optimization

### Handoff Pattern (Agents SDK)
```
Agent A âEHandoff âEAgent B âEHandoff âEAgent C
```
- Context and state transfer between agents
- Specialized agent for each phase
- Best for: Complex multi-expert workflows

### Guardrail Pattern
```
Task âEGuardrail Check âE[Pass: Continue | Fail: Block/Retry]
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
# âEAutomatically coordinates architect, executor, qa-engineer, etc.

# Code refactoring campaign
codex $orchestrator "Modernize legacy codebase to current standards"
# âEParallel execution across multiple modules

# Security remediation
codex $orchestrator "Fix all OWASP Top 10 vulnerabilities"
# âEComprehensive security audit and fixes
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
