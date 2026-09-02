#!/usr/bin/env bash
set -euo pipefail

# Require one argument: the run directory to analyze.
# Example: ./scripts/analyze_run.sh runs/windmill-scheduled-20260902-083000
RUN_DIR="${1:?usage: ./scripts/analyze_run.sh runs/<run-folder>}"

# Run the Python analyzer.
# It reads summary.json, events.json, snapshot.json and writes analysis.md.
python3 ai/analyze_run.py "$RUN_DIR"