# Production Deployment Guide - Rust2024

**Version**: 2.11.1  
**Last Updated**: 2026-01-26

## Overview

This guide covers production deployment best practices for Codex with Rust 2024 edition, including security, performance, monitoring, and operational procedures.

## Prerequisites

- Rust 1.90.0 or later
- Windows 11 (for Windows-specific features)
- CUDA 12.0+ (optional, for GPU acceleration)
- VirtualDesktop (optional, for VR streaming)

## Build Configuration

### Release Profile

The release profile is optimized for production:

```toml
[profile.release]
lto = "fat"              # Link-Time Optimization
strip = "symbols"        # Remove debug symbols
codegen-units = 1        # Maximum optimization
```

### Building for Production

```powershell
# Full workspace build
cd codex-rs
cargo build --release --workspace

# Specific package
cargo build --release -p codex-cli

# With custom features
cargo build --release --features custom-features
```

### Build Verification

```powershell
# Run security audit
.\scripts\security-audit.ps1

# Run tests
.\scripts\test-strategy.ps1 -Coverage

# Performance profiling
.\scripts\performance-profile.ps1 -Target codex-cli
```

## Security

### Dependency Audit

Regular security audits are essential:

```powershell
# Run cargo-deny
cd codex-rs
cargo deny check

# Run cargo-audit
cargo audit
```

### Security Checklist

- [ ] All dependencies are up to date
- [ ] No known CVEs in dependencies
- [ ] `unsafe` blocks are minimized and documented
- [ ] Input validation is implemented
- [ ] Authentication and authorization are configured
- [ ] Secrets are not hardcoded
- [ ] HTTPS/TLS is used for all network communication
- [ ] Logs do not contain sensitive information

## Performance Optimization

### Build Optimization

The release profile uses:
- **LTO (Link-Time Optimization)**: `fat` for maximum optimization
- **Codegen units**: `1` for better optimization (slower build)
- **Strip symbols**: Reduces binary size

### Runtime Performance

- Use `sccache` for faster incremental builds
- Monitor memory usage
- Profile with `cargo flamegraph` for bottlenecks
- Use CUDA acceleration when available

### Performance Monitoring

```powershell
# Generate flamegraph
cargo install flamegraph
cargo flamegraph --release -p codex-cli

# Run benchmarks
cargo bench -p codex-core
```

## Logging and Monitoring

### Log Configuration

Production logging setup:

```powershell
# Set up production logging
.\scripts\production-logging-setup.ps1 -LogLevel info -LogFormat json
```

### Environment Variables

```powershell
$env:RUST_LOG = "info"
$env:CODEX_LOG_DIR = "$env:CODEX_HOME\logs"
$env:CODEX_LOG_FORMAT = "json"
```

### OpenTelemetry Integration

OpenTelemetry is integrated for distributed tracing:

```rust
use codex_otel::config::OtelSettings;
use codex_otel::otel_provider::OtelProvider;

let settings = OtelSettings {
    environment: "production".to_string(),
    service_name: "codex-cli".to_string(),
    service_version: env!("CARGO_PKG_VERSION").to_string(),
    // ... configure exporter
};
```

### Audit Logging

Audit logs track:
- Agent execution history
- LLM API calls
- Tool invocations
- Security events

Logs are written in JSON Lines format for easy parsing.

## Testing Strategy

### Test Execution

```powershell
# Run all tests
.\scripts\test-strategy.ps1

# With coverage
.\scripts\test-strategy.ps1 -Coverage

# Specific package
.\scripts\test-strategy.ps1 -Package codex-core
```

### Test Coverage

Target: 80%+ coverage for production code

```powershell
# Generate coverage report
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --all-features --out Html
```

## CI/CD Pipeline

### Quality Gates

The production quality gate includes:
1. Build verification (`cargo build --release`)
2. Test execution (`cargo test`)
3. Linting (`cargo clippy`)
4. Format check (`cargo fmt --check`)
5. Security audit (`cargo-deny`, `cargo-audit`)
6. Coverage report (monitoring)

### Deployment Process

1. **Development**: Feature branches with CI checks
2. **Staging**: Automatic deployment after PR merge
3. **Production**: Manual approval required

## Monitoring and Alerts

### Key Metrics

- Error rate
- Response time
- Memory usage
- CPU usage
- API call latency

### Alert Thresholds

- Error rate > 1%
- Response time > 5s (p95)
- Memory usage > 80%
- CPU usage > 90%

## Troubleshooting

### Common Issues

1. **Build failures**
   - Check Rust version (1.90.0+)
   - Clear cargo cache: `cargo clean`
   - Verify dependencies: `cargo tree`

2. **Performance issues**
   - Profile with `cargo flamegraph`
   - Check memory usage
   - Verify CUDA availability

3. **Security warnings**
   - Run `cargo audit`
   - Update dependencies
   - Review `deny.toml` exceptions

## Rollback Procedure

1. Identify the problematic commit
2. Revert to previous stable version
3. Rebuild and redeploy
4. Verify functionality
5. Document the issue

## Maintenance

### Regular Tasks

- Weekly: Security audit (`cargo audit`)
- Monthly: Dependency updates
- Quarterly: Performance review
- As needed: Bug fixes and patches

## Support

For issues or questions:
- Check logs: `$CODEX_HOME/logs/`
- Review audit logs: `$CODEX_HOME/logs/audit/`
- Check OpenTelemetry traces
- Review GitHub Issues
