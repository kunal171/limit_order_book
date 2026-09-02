#!/usr/bin/env bash
set -euo pipefail

# Scenario to run.
SCENARIO="${1:-synthetic-crossing}"

# Number of synthetic orders.
COUNT="${2:-100}"

# Output directory. Use auto to create a fresh timestamped folder.
OUTPUT_DIR="${3:-auto}"

# Create the auto folder here so this script knows the exact run directory.
if [ -z "$OUTPUT_DIR" ] || [ "$OUTPUT_DIR" = "auto" ]; then
  OUTPUT_DIR="runs/windmill-analysis-$(date +%Y%m%d-%H%M%S)"
fi

# Create output directory before redirecting stdout into files inside it.
mkdir -p "$OUTPUT_DIR"

# Run simulation, but suppress stdout because summary.json is already saved.
./scripts/run_simulation.sh "$SCENARIO" "$COUNT" "$OUTPUT_DIR" > "$OUTPUT_DIR/simulation_stdout.json"

# Run analyzer, but save its message to a log file.
./scripts/analyze_run.sh "$OUTPUT_DIR" > "$OUTPUT_DIR/analysis.log"

# Print only one final JSON object for Windmill.
printf '{\n'
printf '  "run_dir": "%s",\n' "$OUTPUT_DIR"
printf '  "summary_path": "%s/summary.json",\n' "$OUTPUT_DIR"
printf '  "analysis_path": "%s/analysis.md",\n' "$OUTPUT_DIR"
printf '  "analysis_log_path": "%s/analysis.log"\n' "$OUTPUT_DIR"
printf '}\n'