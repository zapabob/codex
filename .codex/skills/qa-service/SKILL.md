---
name: qa-service
description: Continuous Quality Assurance service agent that monitors development activities in real-time, automatically triggering quality analysis on code changes. Provides background processing for comprehensive QA validation across parallel development environments, ensuring consistent quality standards throughout the development lifecycle.
---

# QA Service Agent Skill

## Overview

Continuous Quality Assurance service agent that monitors development activities in real-time, automatically triggering quality analysis on code changes. Provides background processing for comprehensive QA validation across parallel development environments, ensuring consistent quality standards throughout the development lifecycle.

## Capabilities

- **Real-time Monitoring**: File system change detection with intelligent debouncing
- **Automatic QA Execution**: Background quality analysis triggered by code modifications
- **Multi-worktree Support**: Parallel QA processing across isolated development environments
- **Incremental Analysis**: Smart analysis focusing on changed components only
- **Quality Trend Tracking**: Historical QA metrics and trend analysis
- **Integration Status Monitoring**: Continuous validation of merge readiness

## Tools Required

### MCP Tools
- `codex_read_file` - Reading source files for analysis
- `codex_write_file` - Generating QA reports and metrics
- `codex_codebase_search` - Understanding project structure changes
- `grep` - Pattern matching for file change detection
- `read_file` - File access for change monitoring
- `write` - Creating QA artifacts and status files

### File System Access
- **Read**: Full codebase access for monitoring and analysis
- **Write**: Limited to `./qa-service-logs`, `./qa-metrics`, `./qa-reports`

### Network Access
- **None required** - Local file system monitoring only

### Shell Commands
- File system monitoring (varies by platform)
- Process management for service lifecycle
- Log rotation and cleanup operations

## Usage Examples

### Start Background QA Monitoring
```bash
codex $qa-service "Start continuous QA monitoring for development environment"
```

### Monitor Parallel Development
```bash
codex $qa-service "Monitor multiple worktrees with automatic QA validation"
```

### Quality Trend Analysis
```bash
codex $qa-service "Analyze QA trends and generate quality improvement recommendations"
```

### Service Management
```bash
codex $qa-service "Manage QA service lifecycle and configuration"
```

## Output Format

### QA Service Status Report
```
ð¬ QA Service Status Report - Continuous Quality Assurance
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Service Status: Active (Running)
Uptime: 2h 34m 12s
Monitoring: 4 worktrees
Last Activity: 2026-01-04 15:47:23 UTC

ð Service Statistics
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Total QA Runs: 127
Successful QA Runs: 118 (92.9% success rate)
Failed QA Runs: 9 (7.1% failure rate)
Files Processed: 2,847
Change Events Detected: 456
Average QA Duration: 34.2 seconds

ð¯ Active Monitoring
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Worktrees Being Monitored:
âââ auth-feature (3 changes pending)
âE  âââ Last QA: âEPASSED (2 min ago)
âE  âââ Quality Score: A- (Good)
âE  âââ Next Scheduled: Auto (on changes)
âââ payment-feature (1 change pending)
âE  âââ Last QA: ð RUNNING (45s elapsed)
âE  âââ Quality Score: B+ (Good)
âE  âââ Next Scheduled: Auto (on changes)
âââ ui-feature (12 changes pending)
âE  âââ Last QA: âEFAILED (5 min ago)
âE  âââ Quality Score: C (Needs Attention)
âE  âââ Next Scheduled: Manual review required
âââ api-feature (0 changes pending)
    âââ Last QA: âEPASSED (12 min ago)
    âââ Quality Score: A (Excellent)
    âââ Next Scheduled: Auto (on changes)

âï¸EService Configuration
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Monitoring Paths: ./worktrees/
QA Interval: 300 seconds (5 minutes)
File Change Debounce: 2.0 seconds
Max Concurrent QA: 2
Auto-restart on Failure: Enabled
Log Retention: 7 days

ð Quality Trends (Last 24 hours)
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âââââââââââââââââââââââ¬ââââââââââ¬ââââââââââ¬ââââââââââââââ¬ââââââââââââââE
âETime Period         âEQA Runs âESuccess âEAvg Score   âEIssues FoundâE
âââââââââââââââââââââââ¼ââââââââââ¼ââââââââââ¼ââââââââââââââ¼ââââââââââââââ¤
âE00:00 - 06:00       âE12      âE100%    âEA-          âE3           âE
âE06:00 - 12:00       âE28      âE93%     âEB+          âE12          âE
âE12:00 - 18:00       âE45      âE91%     âEB           âE18          âE
âE18:00 - 24:00       âE42      âE88%     âEB-          âE25          âE
âââââââââââââââââââââââ´ââââââââââ´ââââââââââ´ââââââââââââââ´ââââââââââââââE

ð¨ Active Alerts
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. â EEui-feature: Multiple QA failures detected
   - Last 3 QA runs failed
   - Issues: Security vulnerabilities, performance bottlenecks
   - Recommendation: Code review and fixes required

2. ð Quality Trend: Declining code quality scores
   - Average score dropped from A- to B- in 6 hours
   - Primary issues: Algorithmic complexity, error handling
   - Recommendation: Team training on best practices

ð Current QA Executions
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. payment-feature (PID: 18473)
   - Progress: 68% complete
   - ETA: 23 seconds
   - Current Phase: Security Analysis

2. background-audit (PID: 18492)
   - Progress: 34% complete
   - ETA: 4 minutes 12 seconds
   - Current Phase: Performance Benchmarking

ð¡ Service Recommendations
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. Address ui-feature QA failures immediately
2. Schedule team training on security best practices
3. Consider increasing QA frequency for high-activity worktrees
4. Review and optimize QA analysis performance

ð Service Artifacts
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âââ qa-service-logs/
âE  âââ qa-service.log (2.3MB)
âE  âââ error.log (145KB)
âE  âââ audit.log (892KB)
âââ qa-metrics/
âE  âââ quality-trends.json
âE  âââ performance-metrics.json
âE  âââ worktree-stats.json
âââ qa-reports/
    âââ latest-qa-summary.md
    âââ quality-dashboard.html
    âââ alert-history.json

â¡ Performance Metrics
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
CPU Usage: 12.3% average
Memory Usage: 245MB peak
Disk I/O: 1.2GB transferred
Network: 0MB (local monitoring only)
Uptime: 99.7% availability

ð¯ Next Maintenance Window
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Scheduled: 2026-01-05 02:00 UTC
Tasks: Log rotation, metric aggregation, service restart
Estimated Duration: 15 minutes
```

### QA Execution Details
```json
{
  "qa_service_execution": {
    "execution_id": "qa-exec-2026-01-04-154723",
    "worktree": "payment-feature",
    "trigger": "file_change",
    "start_time": "2026-01-04T15:47:23Z",
    "estimated_completion": "2026-01-04T15:48:15Z",
    "progress_percentage": 68,
    "current_phase": "security_analysis",
    "phases_completed": [
      "file_analysis",
      "syntax_validation",
      "dependency_check"
    ],
    "phases_remaining": [
      "security_analysis",
      "performance_analysis",
      "code_quality_analysis",
      "integration_testing"
    ],
    "resource_usage": {
      "cpu_percent": 45.2,
      "memory_mb": 156,
      "disk_io_kb": 2340
    },
    "change_summary": {
      "files_changed": 3,
      "lines_added": 127,
      "lines_deleted": 23,
      "critical_files_modified": ["src/auth/mod.rs", "src/payment/handler.rs"]
    }
  }
}
```

## Service Architecture

### Monitoring Engine
- **File System Watcher**: Real-time change detection with platform-specific implementations
- **Event Debouncing**: Intelligent filtering to prevent excessive QA triggers
- **Queue Management**: Prioritized processing of QA requests
- **Resource Limiting**: Controlled concurrent QA execution

### QA Orchestration
- **Worktree Isolation**: Independent QA analysis per development environment
- **Incremental Analysis**: Smart analysis focusing on changes and dependencies
- **Result Caching**: Performance optimization through intelligent caching
- **Failure Recovery**: Robust error handling and retry mechanisms

### Reporting System
- **Real-time Status**: Live monitoring dashboards and status updates
- **Historical Trends**: Long-term quality metrics and trend analysis
- **Alert System**: Configurable notifications for quality issues
- **Integration Reports**: CI/CD and development tool integration

## Configuration

### Service Configuration
```json
{
  "qa_service": {
    "monitoring_paths": ["./worktrees/", "./src/"],
    "qa_interval_seconds": 300,
    "file_change_debounce_seconds": 2.0,
    "max_concurrent_qa_runs": 2,
    "auto_restart_on_failure": true,
    "log_retention_days": 7,
    "enable_performance_monitoring": true
  }
}
```

### Worktree Integration
```json
{
  "worktree_integration": {
    "auto_discover_worktrees": true,
    "worktree_base_path": "./worktrees/",
    "qa_on_worktree_creation": true,
    "qa_on_file_changes": true,
    "qa_on_schedule": true,
    "merge_blocking_qa": true
  }
}
```

### Alert Configuration
```json
{
  "alerts": {
    "qa_failure_threshold": 3,
    "quality_score_decline_threshold": 1.0,
    "alert_channels": ["slack", "discord", "email"],
    "alert_cooldown_minutes": 30,
    "escalation_rules": {
      "critical_issues": "immediate",
      "quality_regression": "team_lead",
      "performance_degradation": "devops_team"
    }
  }
}
```

### Quality Gates
```json
{
  "quality_gates": {
    "block_on_critical_issues": true,
    "block_on_high_issues": false,
    "require_minimum_score": "B-",
    "max_qa_duration_seconds": 600,
    "require_clean_build": true,
    "require_test_coverage": 0.8
  }
}
```

## Integration Points

### Development Workflow Integration
```bash
# Start QA monitoring for current session
codex $qa-service "Start monitoring development session"

# Get real-time QA status
codex $qa-service "Show current QA status and active analyses"

# Pause/resume monitoring
codex $qa-service "Pause QA monitoring during refactoring"
codex $qa-service "Resume QA monitoring"
```

### Worktree Manager Integration
```bash
# QA service automatically monitors new worktrees
codex $worktree-manager "Create feature worktree"
# âEQA service automatically starts monitoring

# Manual QA trigger
codex $qa-service "Run QA analysis on auth-feature worktree"
```

### CI/CD Pipeline Integration
```yaml
# GitHub Actions integration
- name: QA Service Status Check
  run: |
    codex $qa-service "Check service health and report status"

- name: Generate QA Trend Report
  run: |
    codex $qa-service "Generate quality trend report for CI"
```

### IDE Integration
```json
// VS Code extension configuration
{
  "codex.qaService": {
    "enableRealTimeMonitoring": true,
    "showQualityIndicators": true,
    "alertOnQAFailures": true,
    "autoFixSuggestions": true
  }
}
```

## Performance Optimization

### Resource Management
- **CPU Limiting**: Configurable CPU usage limits per QA run
- **Memory Management**: Automatic cleanup and memory monitoring
- **Disk I/O Optimization**: Efficient file access patterns
- **Concurrent Limiting**: Controlled parallelism to prevent resource exhaustion

### Analysis Optimization
- **Incremental Processing**: Only analyze changed files and dependencies
- **Caching Strategies**: Intelligent caching of analysis results
- **Parallel Analysis**: Concurrent processing of independent components
- **Early Termination**: Stop analysis on critical failures

### Monitoring Optimization
- **Event Filtering**: Smart filtering of irrelevant file changes
- **Batch Processing**: Group related changes for efficient processing
- **Priority Queuing**: High-priority changes processed first
- **Load Balancing**: Distribute QA load across available resources

## Troubleshooting

### Service Startup Issues
```bash
# Check service status
codex $qa-service "Check service status"

# Restart service
codex $qa-service "Restart QA service"

# Check logs
tail -f qa-service-logs/qa-service.log
```

### File Monitoring Problems
```bash
# Verify file system permissions
ls -la ./worktrees/

# Test file change detection
echo "test" > ./worktrees/test-worktree/test.txt
codex $qa-service "Check for pending changes"
```

### QA Execution Failures
```bash
# Check QA process status
ps aux | grep qa-service

# View detailed error logs
cat qa-service-logs/error.log | tail -50

# Run diagnostic QA
codex $qa-service "Run diagnostic QA analysis"
```

### Performance Issues
```bash
# Monitor resource usage
codex $qa-service "Show performance metrics"

# Adjust configuration
codex $qa-service "Update config max_concurrent_qa_runs=1"

# Clear caches
codex $qa-service "Clear analysis caches"
```

## Best Practices

### Service Management
1. **Regular Monitoring**: Daily status checks and performance monitoring
2. **Log Rotation**: Automatic cleanup of old log files
3. **Backup Configuration**: Regular backup of service configuration
4. **Version Updates**: Keep QA rules and analysis engines updated

### Quality Assurance
1. **Baseline Establishment**: Establish quality baselines for each worktree
2. **Trend Monitoring**: Regular review of quality trends and metrics
3. **Alert Tuning**: Fine-tune alert thresholds based on team tolerance
4. **Feedback Integration**: Use QA results to improve development practices

### Resource Optimization
1. **Load Balancing**: Distribute QA load across development phases
2. **Off-hours Processing**: Schedule intensive analysis during low-activity periods
3. **Caching Strategies**: Maximize cache hit rates for repeated analyses
4. **Resource Limits**: Set appropriate limits to prevent system impact

### Team Integration
1. **Training Programs**: Educate team on QA feedback interpretation
2. **Process Integration**: Make QA checks part of standard development workflow
3. **Communication**: Clear communication of QA requirements and expectations
4. **Continuous Improvement**: Regular review and improvement of QA processes

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
- [Continuous Integration Best Practices](https://martinfowler.com/articles/continuousIntegration.html)
- [Quality Assurance in Software Development](https://www.iso.org/standard/35784.html)
- [Real-time Monitoring Patterns](https://microservices.io/patterns/observability/)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-qa-service-skill`
**Version**: 2.10.0
**Compatibility**: Codex v2.10.0+
