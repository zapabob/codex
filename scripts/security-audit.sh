#!/bin/bash
# Security Audit Script for Codex
# Author: zapabob
# Date: 2025-10-08 11:48:07 UTC

set -e

echo "🔒 Security Audit - $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "User: zapabob"
echo ""

# Change to codex-rs directory
cd codex-rs

# Install cargo-audit if not present
echo "📦 Checking cargo-audit installation..."
if ! command -v cargo-audit &> /dev/null; then
    echo "Installing cargo-audit..."
    cargo install cargo-audit --quiet
fi
echo "✅ cargo-audit ready"
echo ""

# Run cargo audit
echo "🔍 Running cargo audit..."
cargo audit
echo "✅ Cargo audit complete"
echo ""

# Run Clippy with security lints
echo "🔬 Running Clippy (security lints)..."
cargo clippy --all-features --all-targets -- -D warnings
echo "✅ Clippy check complete"
echo ""

# Check for unwrap/expect usage
echo "🔍 Checking for unwrap/expect usage..."
if grep -r "\.unwrap()" src/ --include="*.rs" 2>/dev/null; then
    echo "⚠️  Found .unwrap() usage"
    exit 1
fi

if grep -r "\.expect(" src/ --include="*.rs" 2>/dev/null; then
    echo "⚠️  Found .expect() usage"
    exit 1
fi
echo "✅ No unwrap/expect found"
echo ""

# Check for unsafe blocks
echo "🔍 Checking for unsafe blocks..."
UNSAFE_COUNT=$(grep -r "unsafe" src/ --include="*.rs" 2>/dev/null | wc -l || echo "0")
if [ "$UNSAFE_COUNT" -gt 0 ]; then
    echo "⚠️  Found $UNSAFE_COUNT unsafe blocks"
    echo "Please review unsafe code carefully"
else
    echo "✅ No unsafe blocks found"
fi
echo ""

# Run tests with all features
echo "🧪 Running tests (all features)..."
cargo test --all-features
echo "✅ All tests passed"
echo ""
echo "🎉 Security audit complete!"
echo ""
echo "Summary:"
echo "- Cargo audit: ✅"
echo "- Clippy: ✅"
echo "- No unwrap/expect: ✅"
echo "- Unsafe blocks: $UNSAFE_COUNT"
echo "- Tests: ✅"