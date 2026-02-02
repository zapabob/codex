---
name: qa-engineer
description: Comprehensive Quality Assurance agent that performs advanced code analysis using mathematical optimization, quantum computing principles, and software engineering best practices. Automatically evaluates code quality, performance, and architectural integrity before integration, ensuring production-ready code through rigorous automated review.
---

# QA Engineer Agent Skill

## Overview

Comprehensive Quality Assurance agent that performs advanced code analysis using mathematical optimization, quantum computing principles, and software engineering best practices. Automatically evaluates code quality, performance, and architectural integrity before integration, ensuring production-ready code through rigorous automated review.

## Capabilities

- **Mathematical Optimization Analysis**: Algorithmic complexity, computational complexity, Big O notation validation
- **Quantum Optimization Review**: Quantum algorithm efficiency, circuit optimization, quantum-classical hybrid analysis
- **Software Engineering Best Practices**: SOLID principles, DRY/KISS/YAGNI, clean architecture compliance
- **Code Quality Metrics**: Readability, maintainability, extensibility, testability assessment
- **Performance Optimization**: Memory usage, CPU efficiency, scalability analysis
- **Security & Reliability**: Vulnerability assessment, error handling, fault tolerance evaluation

## Tools Required

### MCP Tools
- `codex_read_file` - Source code analysis and metrics calculation
- `codex_grep` - Pattern recognition for anti-patterns and best practices
- `codex_codebase_search` - Semantic analysis across the entire codebase
- `grep` - Regular expression pattern matching
- `read_file` - File content access for detailed analysis
- `codebase_search` - Project-wide structural analysis

### File System Access
- **Read**: Full codebase access for comprehensive analysis
- **Write**: Limited to `./qa-reports`, `./optimization-results`, `./review-artifacts`

### Network Access
- `https://docs.rs/*` - Rust crate documentation and performance benchmarks
- `https://doc.rust-lang.org/*` - Standard library optimization guides
- `https://docs.python.org/*` - Python performance and optimization docs
- `https://developer.mozilla.org/*` - Web performance and optimization
- `https://arxiv.org/*` - Research papers on optimization algorithms
- `https://quantum-computing.ibm.com/*` - Quantum computing documentation
- `https://qiskit.org/*` - Quantum development framework docs

### Shell Commands
- `cargo bench` - Performance benchmarking
- `cargo flamegraph` - Performance profiling
- `python -m cProfile` - Python performance profiling
- `node --prof` - Node.js performance profiling
- `hyperfine` - Command-line benchmarking tool
- `valgrind` - Memory profiling and leak detection

## Usage Examples

### Comprehensive QA Review
```bash
codex $qa-engineer "Perform complete QA analysis: mathematical optimization, quantum efficiency, software engineering best practices, and code quality metrics"
```

### Pre-merge Quality Gate
```bash
codex $qa-engineer "Review pull request changes for algorithmic complexity, performance bottlenecks, and architectural integrity before merge"
```

### Optimization Analysis
```bash
codex $qa-engineer "Analyze code for mathematical optimization opportunities, quantum algorithm improvements, and software engineering enhancements"
```

### CI/CD Quality Assurance
```bash
codex $qa-engineer "Automated QA pipeline: complexity analysis, best practices compliance, and optimization recommendations"
```

## Output Format

### Comprehensive QA Report
```
🔬 QA Engineering Analysis Report
Generated: 2026-01-04 12:00:00 UTC

📊 Quality Metrics Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Algorithmic Complexity: A+ (Excellent)
Quantum Optimization: A  (Very Good)
Software Engineering: A+ (Excellent)
Code Quality: A- (Good)
Performance: A  (Very Good)
Security: A+ (Excellent)

📈 Optimization Scores
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Mathematical Complexity: O(log n) ↁEO(1) potential
Quantum Gate Count: 15 gates ↁE8 gates optimization
Memory Efficiency: 94% ↁE98% improvement potential
CPU Utilization: 67% ↁE45% reduction opportunity

🏗�E�EArchitecture Assessment
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
SOLID Principles: ✁EAll compliant
Clean Architecture: ✁ELayers properly separated
Design Patterns: ✁EAppropriate usage
Dependency Injection: ✁EImplemented correctly

🔍 Code Quality Analysis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Readability Score: 9.2/10
Maintainability Index: 87/100
Extensibility Rating: 8.8/10
Testability Score: 9.5/10

⚡ Performance Optimization Opportunities
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. [HIGH] Algorithm Optimization
   - Location: src/algorithms/sort.rs:45
   - Current: O(n²) bubble sort
   - Recommended: O(n log n) quicksort
   - Impact: 75% performance improvement

2. [MEDIUM] Memory Optimization
   - Location: src/data_structures/cache.rs:123
   - Issue: Unnecessary allocations
   - Solution: Object pooling pattern
   - Impact: 40% memory reduction

3. [MEDIUM] Quantum Optimization
   - Location: src/quantum/algorithm.rs:67
   - Current: 15 quantum gates
   - Optimized: 8 quantum gates
   - Impact: 47% gate count reduction

🏆 Best Practices Compliance
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✁EDRY Principle: No code duplication detected
✁EKISS Principle: Simple, clear implementations
✁EYAGNI Principle: No unnecessary complexity
✁ESingle Responsibility: All classes focused
✁EOpen/Closed: Extensible without modification

🚨 Critical Issues Found
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. [CRITICAL] Algorithmic Bottleneck
   - Exponential time complexity in hot path
   - Recommendation: Dynamic programming approach
   - Risk: Performance degradation under load

2. [HIGH] Memory Leak Potential
   - Improper resource cleanup in error paths
   - Recommendation: RAII pattern implementation
   - Risk: Memory exhaustion in long-running processes

📋 Recommendations Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Immediate Actions: 2 critical fixes required
Short-term: 3 high-priority optimizations
Long-term: 5 medium-priority improvements

Integration Status: 🚫 BLOCKED - Critical issues must be resolved
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### JSON Output (for CI/CD Integration)
```json
{
  "timestamp": "2026-01-04T12:00:00Z",
  "quality_scores": {
    "algorithmic_complexity": "A+",
    "quantum_optimization": "A",
    "software_engineering": "A+",
    "code_quality": "A-",
    "performance": "A",
    "security": "A+"
  },
  "optimization_opportunities": [
    {
      "priority": "HIGH",
      "category": "algorithmic",
      "location": "src/algorithms/sort.rs:45",
      "current_complexity": "O(n²)",
      "recommended_complexity": "O(n log n)",
      "impact_percentage": 75,
      "description": "Replace bubble sort with quicksort"
    }
  ],
  "architecture_assessment": {
    "solid_principles": true,
    "clean_architecture": true,
    "design_patterns": true,
    "dependency_injection": true
  },
  "code_quality_metrics": {
    "readability": 9.2,
    "maintainability": 87,
    "extensibility": 8.8,
    "testability": 9.5
  },
  "critical_issues": [
    {
      "severity": "CRITICAL",
      "category": "performance",
      "description": "Exponential time complexity in hot path",
      "recommendation": "Implement dynamic programming"
    }
  ],
  "integration_status": {
    "can_merge": false,
    "blocking_issues": 2,
    "required_actions": [
      "Fix algorithmic bottleneck",
      "Implement proper resource cleanup"
    ]
  }
}
```

## Advanced QA Criteria

### Mathematical Optimization Analysis
- **Time Complexity**: Big O notation validation
- **Space Complexity**: Memory usage optimization
- **Algorithmic Efficiency**: Optimal algorithm selection
- **Computational Complexity**: Theoretical performance bounds

### Quantum Optimization Review
- **Quantum Gate Count**: Minimize quantum operations
- **Circuit Depth**: Optimize quantum circuit layers
- **Qubit Efficiency**: Minimize qubit requirements
- **Error Correction**: Fault-tolerant quantum computing

### Software Engineering Best Practices
- **SOLID Principles**: Single responsibility, open-closed, etc.
- **GRASP Patterns**: Information expert, creator, etc.
- **Design Patterns**: Appropriate pattern usage
- **Architectural Patterns**: Layered, hexagonal, clean architecture

### Code Quality Dimensions
- **Readability**: Naming conventions, structure clarity
- **Maintainability**: Change ease, bug fix simplicity
- **Extensibility**: New feature addition facility
- **Testability**: Automated testing feasibility

## Progressive QA Levels

### Level 1: Basic QA (Fast Feedback)
- Syntax validation, basic complexity checks
- Critical security vulnerabilities
- Fundamental best practices compliance

### Level 2: Standard QA (Comprehensive)
- Full algorithmic analysis
- Architecture pattern review
- Performance benchmarking
- Code quality metrics calculation

### Level 3: Advanced QA (Deep Analysis)
- Quantum optimization opportunities
- Mathematical proof validation
- Long-term maintainability assessment
- Scalability and performance modeling

## Configuration

### QA Rules Customization
```json
{
  "qa_levels": {
    "enable_level_1": true,
    "enable_level_2": true,
    "enable_level_3": false
  },
  "optimization_criteria": {
    "mathematical_complexity_threshold": "O(n log n)",
    "quantum_gate_limit": 50,
    "memory_efficiency_target": 0.9,
    "performance_baseline": "current_average"
  },
  "quality_gates": {
    "block_on_critical_issues": true,
    "require_minimum_score": 7.0,
    "enforce_best_practices": true
  },
  "languages": ["rust", "python", "typescript"],
  "output_formats": ["markdown", "json", "sarif"]
}
```

## Integration Points

### GitHub Actions Quality Gate
```yaml
- name: QA Engineering Review
  uses: openai/codex-qa-engineer@v1
  with:
    level: 'standard'
    block_on_critical: true
    output_format: 'sarif'
    optimization_targets: 'mathematical,quantum,performance'
```

### Pre-commit Hooks
```bash
#!/bin/bash
# .git/hooks/pre-commit
codex $qa-engineer "Quick QA check on staged changes"
if [ $? -ne 0 ]; then
    echo "QA check failed - fix issues before commit"
    exit 1
fi
```

### VS Code Extension
```json
{
  "contributes": {
    "commands": [
      {
        "command": "codex.qaReview",
        "title": "Run QA Engineering Review"
      },
      {
        "command": "codex.qaOptimize",
        "title": "Apply QA Optimization Suggestions"
      }
    ]
  }
}
```

## Background QA Service

### Continuous Monitoring
```bash
# Start background QA service
codex-qa-service --watch --level=standard --notify

# Monitor specific directories
codex-qa-service --watch src/ tests/ --quantum-analysis
```

### Pre-merge Validation
```bash
# Automatic pre-merge QA check
git config merge.qa.enabled true
git config merge.qa.level comprehensive
```

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
- [Introduction to Algorithms](https://mitpress.mit.edu/9780262033848/introduction-to-algorithms/)
- [Quantum Computation and Quantum Information](https://www.cambridge.org/us/academic/subjects/physics/quantum-physics-quantum-information-and-quantum-computation/quantum-computation-and-quantum-information-10th-anniversary-edition)
- [Clean Code](https://www.oreilly.com/library/view/clean-code/9780136083238/)
- [Design Patterns](https://www.oreilly.com/library/view/design-patterns/0201633612/)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-qa-engineer-skill`
**Version**: 1.0.0
**Compatibility**: Codex v2.9.0+
