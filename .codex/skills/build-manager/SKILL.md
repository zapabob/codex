---
name: build-manager
description: Advanced build management agent that provides high-performance incremental compilation, hot reload capabilities, and intelligent build orchestration. Features MD5-based change detection, process management, and multi-platform deployment support with tqdm progress visualization.
---

# Build Manager Agent Skill

## Overview

Advanced build management agent that provides high-performance incremental compilation, hot reload capabilities, and intelligent build orchestration. Features MD5-based change detection, process management, and multi-platform deployment support with tqdm progress visualization.

## Capabilities

- **Incremental Compilation**: MD5-based change detection for intelligent rebuilds
- **Hot Reload System**: Process termination and atomic binary replacement
- **Build Orchestration**: Parallel compilation with CPU optimization
- **Multi-Platform Support**: Cross-platform build and deployment
- **Progress Visualization**: Real-time build progress with ETA calculation
- **Dependency Management**: Cargo workspace and npm project handling

## Tools Required

### MCP Tools
- `codex_read_file` - Reading build configurations and source files
- `codex_write_file` - Generating build artifacts and cache files
- `codex_search_replace` - Updating build configurations
- `codex_codebase_search` - Analyzing project structure and dependencies
- `grep` - Searching for build-related files and patterns
- `read_file` - File access for build operations
- `write` - Creating build outputs and logs

### File System Access
- **Read**: Full codebase access for build analysis and dependency resolution
- **Write**: Limited to `./target`, `./node_modules`, `./dist`, `./build`, `./artifacts`

### Network Access
- `https://crates.io/*` - Rust crate registry for dependency resolution
- `https://registry.npmjs.org/*` - npm registry for JavaScript dependencies
- `https://github.com/*` - Git repositories for dependency fetching

### Shell Commands
- `cargo` - Rust build system operations
- `npm` - Node.js package management
- `rustc` - Rust compiler direct invocation
- `node` - JavaScript runtime operations
- `python` - Build script execution
- `git` - Version control for change detection

## Usage Examples

### Fast Incremental Build
```bash
codex $build-manager "Perform incremental build with change detection and progress visualization"
```

### Hot Reload Deployment
```bash
codex $build-manager "Build release and hot-reload with process management"
```

### Multi-Platform Build
```bash
codex $build-manager "Cross-platform build for Linux, macOS, and Windows targets"
```

### Build Optimization Analysis
```bash
codex $build-manager "Analyze build performance and suggest optimization strategies"
```

## Output Format

### Build Report
```
ð Build Manager Report - Incremental Build
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Build Type: Incremental (MD5-based change detection)
Target: release
Platform: x86_64-unknown-linux-gnu

ð Build Statistics
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Files Processed: 1,234
Changed Files: 23 (1.8%)
Build Time: 45.2s (ETA: 67% faster than full rebuild)
Peak Memory: 2.1GB
CPU Cores Used: 6/8

â¡ Performance Optimizations Applied
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âEIncremental Compilation: Enabled (CARGO_INCREMENTAL=1)
âEParallel Jobs: 6 cores (75% of available)
âEsccache: Disabled (conflicts with incremental)
âELTO: Disabled for faster iteration
âEDebug Info: Stripped for release

ð Build Artifacts
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
âââ target/release/codex (8.2MB) - Main binary
âââ target/release/deps/ (245MB) - Dependencies
âââ target/release/build/ (89MB) - Build artifacts
âââ target/release/incremental/ (156MB) - Incremental cache

ð Hot Reload Status
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
Process Detection: âEFound 3 running instances
Termination: âEGraceful shutdown completed
Binary Replacement: âEAtomic copy successful
Restart: âENew process started (PID: 12345)

â EEBuild Warnings
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
- Large binary size: Consider stripping debug symbols
- Memory usage high: Monitor for OOM in production
- Dependencies: 47 crates updated, review for breaking changes

âEBuild Verification
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
- Compilation: âEPASSED
- Linking: âEPASSED
- Tests: âEPASSED (1,247 tests in 23.4s)
- Linting: âEPASSED (clippy)
- Formatting: âEPASSED (rustfmt)
- Security: âEPASSED (no vulnerabilities)

ð¯ Next Steps
âââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââââE
1. Deploy to staging environment
2. Run integration tests
3. Monitor performance metrics
4. Consider enabling LTO for production release
```

### Performance Metrics
```json
{
  "build_type": "incremental",
  "change_detection": {
    "method": "MD5",
    "files_analyzed": 1234,
    "changed_files": 23,
    "change_ratio": 0.018
  },
  "timing": {
    "total_build_time": 45.2,
    "compilation_time": 38.7,
    "linking_time": 4.1,
    "test_time": 23.4,
    "full_build_equivalent": 183.0,
    "time_saved_percentage": 67.3
  },
  "resources": {
    "peak_memory_mb": 2147,
    "cpu_cores_used": 6,
    "cpu_utilization_avg": 78.5,
    "disk_io_mb": 456
  },
  "optimizations": {
    "incremental_compilation": true,
    "parallel_jobs": 6,
    "sccache_disabled": true,
    "lto_disabled": true,
    "debug_stripped": true
  },
  "artifacts": {
    "main_binary_size_mb": 8.2,
    "dependencies_size_mb": 245,
    "build_artifacts_size_mb": 89,
    "incremental_cache_size_mb": 156
  },
  "verification": {
    "compilation": "PASSED",
    "linking": "PASSED",
    "tests": "PASSED",
    "linting": "PASSED",
    "formatting": "PASSED",
    "security": "PASSED"
  }
}
```

## Build Optimization Strategies

### 1. Incremental Compilation
```rust
// Cargo configuration for optimal incremental builds
[profile.dev]
incremental = true
codegen-units = 16  # Parallel code generation

[profile.release]
incremental = false  # Full optimization for release
codegen-units = 1    # Single unit for better optimization
lto = true          # Link-time optimization
```

### 2. Change Detection Algorithm
```python
def detect_changes(self, source_files: List[Path]) -> List[Path]:
    """MD5-based change detection with caching"""
    changed_files = []
    cache_file = Path(".build_cache.json")

    # Load previous hashes
    previous_hashes = {}
    if cache_file.exists():
        with open(cache_file) as f:
            previous_hashes = json.load(f)

    current_hashes = {}
    for file_path in source_files:
        if file_path.exists():
            file_hash = hashlib.md5(file_path.read_bytes()).hexdigest()
            current_hashes[str(file_path)] = file_hash

            # Check if file changed
            if str(file_path) not in previous_hashes or \
               previous_hashes[str(file_path)] != file_hash:
                changed_files.append(file_path)

    # Save current hashes
    with open(cache_file, 'w') as f:
        json.dump(current_hashes, f, indent=2)

    return changed_files
```

### 3. Process Management for Hot Reload
```python
def hot_reload_binary(self, new_binary_path: Path, target_path: Path) -> bool:
    """Atomic binary replacement with process management"""

    # Find running processes
    running_processes = self.find_processes_by_binary(target_path)

    # Graceful shutdown
    for proc in running_processes:
        proc.terminate()
        try:
            proc.wait(timeout=10)  # Wait up to 10 seconds
        except subprocess.TimeoutExpired:
            proc.kill()  # Force kill if not responding

    # Atomic replacement
    temp_path = target_path.with_suffix('.tmp')
    shutil.copy2(new_binary_path, temp_path)
    temp_path.replace(target_path)  # Atomic rename

    # Restart process
    new_process = subprocess.Popen([str(target_path)])
    logger.info(f"Hot reload completed, new process PID: {new_process.pid}")

    return True
```

### 4. Progress Visualization
```python
def build_with_progress(self, build_command: List[str]) -> bool:
    """Execute build with tqdm progress visualization"""

    process = subprocess.Popen(
        build_command,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        universal_newlines=True
    )

    # Progress tracking
    total_lines = 0
    compiled_lines = 0

    with tqdm(desc="Building", unit="lines") as pbar:
        for line in process.stdout:
            total_lines += 1

            if "Compiling" in line or "Finished" in line:
                compiled_lines += 1
                pbar.update(1)
                pbar.set_description(f"Compiling: {compiled_lines}/{total_lines}")

            # Log output for debugging
            logger.debug(line.strip())

    return process.wait() == 0
```

## Configuration

### Build Profiles
```toml
[build-manager]
default_profile = "dev"
progress_enabled = true
hot_reload_enabled = true

[build-manager.profiles.dev]
incremental = true
parallel_jobs = "75%"  # Percentage of CPU cores
optimization_level = 0

[build-manager.profiles.release]
incremental = false
parallel_jobs = "100%"
optimization_level = 3
lto = true
strip_debug = true

[build-manager.profiles.ci]
incremental = false
parallel_jobs = "50%"
progress_enabled = false
verbose_output = true
```

### Platform-Specific Settings
```json
{
  "platforms": {
    "linux": {
      "compiler_flags": ["-march=native", "-mtune=native"],
      "linker_flags": ["-fuse-ld=lld"],
      "package_managers": ["apt", "snap"]
    },
    "macos": {
      "compiler_flags": ["-march=native"],
      "linker_flags": ["-fuse-ld=lld"],
      "package_managers": ["brew", "port"]
    },
    "windows": {
      "compiler_flags": ["/arch:AVX2"],
      "linker_flags": ["/LTCG"],
      "package_managers": ["choco", "winget"]
    }
  }
}
```

## Integration Points

### Development Workflow
```bash
# Fast development iteration
codex $build-manager "Incremental dev build"
# âEOnly rebuilds changed files

# Production release
codex $build-manager "Optimized release build with LTO"
# âEFull optimization for performance

# Hot reload deployment
codex $build-manager "Build and hot-reload to staging"
# âEZero-downtime deployment
```

### CI/CD Integration
```yaml
# GitHub Actions
- name: Fast Build
  run: codex $build-manager "Incremental CI build"

- name: Release Build
  run: codex $build-manager "Optimized release build"

- name: Deploy
  run: codex $build-manager "Hot reload to production"
```

### IDE Integration
```json
// VS Code tasks.json
{
  "tasks": [
    {
      "label": "Fast Build",
      "type": "shell",
      "command": "codex",
      "args": ["$build-manager", "Incremental dev build"],
      "group": "build"
    },
    {
      "label": "Hot Reload",
      "type": "shell",
      "command": "codex",
      "args": ["$build-manager", "Build and hot-reload"],
      "group": "build"
    }
  ]
}
```

## Performance Benchmarks

### Build Time Comparison
```
Full Rebuild:      183.2s
Incremental (10% changes):   45.2s (75% faster)
Incremental (1% changes):    12.8s (93% faster)
Cached Build:       8.3s (95% faster)
```

### Resource Usage
```
Memory Usage:     2.1GB peak (vs 4.8GB full rebuild)
CPU Utilization:  78.5% avg (6 cores)
Disk I/O:         456MB transferred
Network:          0MB (offline build)
```

## Troubleshooting

### Common Issues

#### Incremental Build Corruption
```bash
# Clear incremental cache
rm -rf target/incremental/
codex $build-manager "Clean rebuild"
```

#### Hot Reload Failures
```bash
# Check process permissions
ps aux | grep codex
sudo codex $build-manager "Force restart"
```

#### Memory Issues
```bash
# Reduce parallel jobs
export CARGO_BUILD_JOBS=2
codex $build-manager "Low memory build"
```

#### Cache Conflicts
```bash
# Reset build cache
rm .build_cache.json
rm -rf target/
codex $build-manager "Fresh build"
```

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
- [Cargo Build System](https://doc.rust-lang.org/cargo/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Build Systems Ã  la Carte](https://www.microsoft.com/en-us/research/uploads/prod/2018/03/build-systems.pdf)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-build-manager-skill`
**Version**: 2.10.0
**Compatibility**: Codex v2.10.0+
