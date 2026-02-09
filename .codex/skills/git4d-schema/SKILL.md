---
name: git4d-schema
description: "Git4D Schema Auditor - Validate Git4D configurations, schema definitions, and DevOps pipeline integrity."
---

# Git4D Schema Auditor Agent Skill

## Overview

Git4D Schema Auditor validates Git4D configurations, schema definitions, and DevOps pipeline integrity. Ensures all Git4D configurations follow best practices and maintain pipeline consistency.

## Capabilities

- **Schema Validation**: Validate Git4D schema definitions
- **Configuration Audit**: Audit Git4D configuration files
- **Pipeline Integrity**: Verify DevOps pipeline configurations
- **Best Practices Enforcement**: Ensure Git4D best practices
- **Version Compatibility**: Check schema version compatibility
- **Documentation Validation**: Validate documentation consistency

## Tools Required

### MCP Tools

- `git4d_validate_schema` - Validate Git4D schema
- `git4d_audit_config` - Audit configuration files
- `git4d_check_pipeline` - Check pipeline integrity
- `git4d_enforce_practices` - Enforce best practices
- `git4d_check_version` - Check version compatibility
- `git4d_validate_docs` - Validate documentation

### File System Access

- **Read**: Full repository access
- **Write**: `./git4d-results/`, `./artifacts/`, `./reports/`

### Network Access

- **None required** - Local validation

### Shell Commands

- `git` - Git operations
- `python` - Python for validation scripts
- `jsonlint` - JSON validation

## Usage Examples

### Basic Schema Validation

```bash
codex $git4d-schema "Validate Git4D schema definitions"
```

### Configuration Audit

```bash
codex $git4d-schema "Audit all Git4D configuration files"
```

### Pipeline Integrity Check

```bash
codex $git4d-schema "Verify DevOps pipeline integrity"
```

## Git4D Schema Structure

```
.git4d/
├── schema/
│   ├── v1.0/
│   │   ├── pipeline.schema.json
│   │   ├── config.schema.json
│   │   └── runtime.schema.json
│   └── current -> v1.0/
├── configs/
│   ├── pipeline.yaml
│   ├── runtime.yaml
│   └── deployment.yaml
└── .git4drc
```

## Schema Validation Rules

| Rule            | Severity | Description                        |
| --------------- | -------- | ---------------------------------- |
| required_fields | Error    | Required fields must exist         |
| type_check      | Error    | Fields must have correct types     |
| enum_values     | Error    | Enum fields must have valid values |
| pattern_match   | Warning  | String fields must match patterns  |
| best_practice   | Info     | Should follow best practices       |

## Configuration Validation

### Pipeline Configuration

```yaml
pipeline:
  stages:
    - name: build
      type: docker
      image: rust:1.75
    - name: test
      type: shell
      script: cargo test
    - name: deploy
      type: approval
      requires: test
```

### Runtime Configuration

```yaml
runtime:
  gpu:
    required: true
    min_memory: "16GB"
    cuda_version: "12.0"
  cpu:
    cores: 4
    architecture: "x86_64"
```

## Output Format

The git4d-schema agent provides:

- Schema validation report
- Configuration audit results
- Pipeline integrity status
- Best practices scorecard
- Version compatibility matrix

## References

- [Git4D GitHub](https://github.com/zapabob/codex-git4d)
- [Schema Validation](https://json-schema.org/)
- [YAML Best Practices](https://yaml.org/spec/)
- [DevOps Pipeline](https://docs.github.com/en/actions/pipelines)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-git4d-schema-skill`
**Version**: 1.0.0
**Compatibility**: Codex v2.14.0+
