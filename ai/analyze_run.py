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

def count_events(events):
    # Count How many times each event type occurred
    counts = {} 

    for event in events: 
        name = event_name(event)
        counts[name] = counts.get(name, 0) + 1

    return counts

def build_report(summary, events, snapshot):
    #Pull important values from summary.json.

    scenario = summary.get("scenario")
    order_count = summary.get("order_count")
    resting_orders = summary.get("resting_orders")

    book_metrics = summary.get("book_metrics", {})
    trade_metrics = summary.get("trade_metrics", {})

    event_counts = count_events(events)

    # Build a human-readable Markdown report.
    lines = [
        "# Run Analysis",
        "",
        "## Summary",
        "",
        f"- Scenario: `{scenario}`",
        f"- Orders processed: `{order_count}`",
        f"- Resting orders after run: `{resting_orders}`",
        f"- Total events: `{len(events)}`",
        "",
        "## Book Metrics",
        "",
        f"- Best bid: `{book_metrics.get('best_bid')}`",
        f"- Best ask: `{book_metrics.get('best_ask')}`",
        f"- Spread: `{book_metrics.get('spread')}`",
        f"- Mid price: `{book_metrics.get('mid_price')}`",
        f"- Imbalance: `{book_metrics.get('imbalance')}`",
        "",
        "## Trade Metrics",
        "",
        f"- Trade count: `{trade_metrics.get('trade_count')}`",
        f"- Total traded quantity: `{trade_metrics.get('total_traded_quantity')}`",
        f"- Total notional: `{trade_metrics.get('total_notional')}`",
        f"- Last trade price: `{trade_metrics.get('last_trade_price')}`",
        f"- VWAP: `{trade_metrics.get('vwap')}`",
        "",
        "## Event Counts",
        "",
    ]

    for name, count in sorted(event_counts.items()):
        lines.append(f"- {name}: `{count}`")

    lines.extend([
        "",
        "## Interpretation",
        "",
        interpret(summary, events, snapshot),
        "",
    ])

    return "\n".join(lines)