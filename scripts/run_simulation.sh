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

# Run the simulation and write artifacts.
./target/release/limit_order_book "$SCENARIO" \
  --count "$COUNT" \
  --output-dir "$OUTPUT_DIR" \
  --json