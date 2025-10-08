#!/bin/bash
set -e

echo "🔒 Security Audit - 2025-10-08 11:12:07 UTC"
echo "User: zapabob"
echo ""

# Rust dependencies
echo "📦 Cargo Audit..."
cd codex-rs
cargo install cargo-audit --quiet 2>/dev/null || true
cargo audit || echo "⚠️  Vulnerabilities found in Rust dependencies"

# Node.js dependencies
echo ""
echo "📦 NPM Audit..."
cd ../codex-web
npm audit --production || echo "⚠️  Vulnerabilities found in Node.js dependencies"

# Clippy
echo ""
echo "🔬 Clippy..."
cd ../codex-rs
cargo clippy --all-features --all-targets -- -D warnings || echo "⚠️  Clippy warnings found"

# ESLint
echo ""
echo "🔬 ESLint..."
cd ../codex-web
npm run lint || echo "⚠️  ESLint errors found"

echo ""
echo "✅ Security audit complete!"