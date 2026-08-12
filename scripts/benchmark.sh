#!/usr/bin/env bash

set -euo pipefail

echo "======================================"
echo "ACE Benchmark Suite"
echo "======================================"

echo
echo "Rust version:"
rustc --version

echo
echo "Cargo version:"
cargo --version

echo
echo "Building release binaries..."
cargo build --workspace --release

echo
echo "Running benchmarks..."
cargo bench --workspace --all-features

echo
echo "Benchmark run completed."