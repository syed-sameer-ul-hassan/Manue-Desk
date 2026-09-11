#!/usr/bin/env bash
set -e

echo "=== Building Release Binary ==="
cargo build --release

echo ""
echo "=== Generating .deb Package ==="
cargo deb

echo ""
echo "=== Generating .rpm Package ==="
cargo generate-rpm

echo ""
echo "=========================================="
echo "Packaging complete!"
echo "Generated Packages:"
ls -la target/debian/*.deb 2>/dev/null || true
ls -la target/generate-rpm/*.rpm 2>/dev/null || true
echo "=========================================="
