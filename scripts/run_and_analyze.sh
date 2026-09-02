#!/usr/bin/env bash
set -euo pipefail

# Scenario to run.
SCENARIO="${1:-synthetic-crossing}"

# Number of synthetic orders.
COUNT="${2:-100}"

# Output directory. Use auto to create a fresh timestamped folder.
OUTPUT_DIR="${3:-auto}"

# Run the Rust simulation first.
RUN_PREFIX=windmill-analysis ./scripts/run_simulation.sh "$SCENARIO" "$COUNT" "$OUTPUT_DIR"

# If OUTPUT_DIR was auto, find the latest generated folder.
if [ "$OUTPUT_DIR" = "auto" ]; then
  OUTPUT_DIR="$(ls -td runs/windmill-analysis-* | head -1)"
fi

# Generate analysis.md from the simulation artifacts.
./scripts/analyze_run.sh "$OUTPUT_DIR"

# Return a small final JSON object for Windmill.
printf '{\n'
printf '  "run_dir": "%s",\n' "$OUTPUT_DIR"
printf '  "summary_path": "%s/summary.json",\n' "$OUTPUT_DIR"
printf '  "analysis_path": "%s/analysis.md"\n' "$OUTPUT_DIR"
printf '}\n'