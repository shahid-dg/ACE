#!/usr/bin/env bash

set -euo pipefail

INPUT="${1:-data/sample/annotations.csv}"
OUTPUT="${2:-report.html}"

echo "Generating ACE report..."
echo "Input:  ${INPUT}"
echo "Output: ${OUTPUT}"

cargo run --release -p ace -- report "$INPUT" --output "$OUTPUT"

echo
echo "Report generated: ${OUTPUT}"