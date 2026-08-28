#!/usr/bin/env bash
set -euo pipefail

# Scenario to run, default is synthetic-crossing.
SCENARIO="${1:-synthetic-crossing}"

# Number of synthetic orders, default is 100.
COUNT="${2:-100}"

# Output directory, default is a timestamped run folder.
OUTPUT_DIR="${3:-runs/run-$(date +%Y%m%d-%H%M%S)}"

# Build release binary if it does not exist yet.
if [ ! -x "./target/release/limit_order_book" ]; then
  cargo build --release
fi

# Make sure the output directory exists before writing run.log.
mkdir -p "$OUTPUT_DIR"

# Run the simulation and save verbose output to run.log.
# Windmill should receive only the small summary JSON on stdout.
./target/release/limit_order_book "$SCENARIO" \
  --count "$COUNT" \
  --output-dir "$OUTPUT_DIR" \
  > "$OUTPUT_DIR/run.log"

# Print only the summary JSON.
# This becomes the clean Windmill job result.
cat "$OUTPUT_DIR/summary.json"
