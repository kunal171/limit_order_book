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