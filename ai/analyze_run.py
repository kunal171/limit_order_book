#!/usr/bin/env python3

# This script analyzes one completed limit order book run.
# Input: runs/<run-folder>/summary.json, events.json, snapshot.json
# Output: runs/<run-folder>/analysis.md

import json
import sys
from pathlib import Path

def load_json (path: Path):
    # Read a JSON file and convert it into Python dict/list data.
    with path.open("r", encoding="utf-8") as file:
        return json.load(file)

def event_name(event):
    # Rust serde enum JSON can appear in different shapes.
    # This keeps our analyzer tolerant instead of tightly coupled.
    if isinstance(event, dict) and len(event) == 1:
        return next(iter(event.keys()))

    if isinstance(event, dict) and "type" in event:
        return event["type"]

    return "Unknown"