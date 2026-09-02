#!/usr/bin/env python3

# This script analyzes one completed limit order book run.
# Input: runs/<run-folder>/summary.json, events.json, snapshot.json
# Output: runs/<run-folder>/analysis.md

import json
import sys
from pathlib import Path

def load_json(path: Path):
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

def display_value(value):
    #Convert Python None into cleaner report text.
    if value is None:
        return "not available"

    return value

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
        f"- Best bid: `{display_value(book_metrics.get('best_bid'))}`",
        f"- Best ask: `{display_value(book_metrics.get('best_ask'))}`",
        f"- Spread: `{display_value(book_metrics.get('spread'))}`",
        f"- Mid price: `{display_value(book_metrics.get('mid_price'))}`",
        f"- Imbalance: `{display_value(book_metrics.get('imbalance'))}`",
        "",
        "## Trade Metrics",
        "",
        f"- Trade count: `{display_value(trade_metrics.get('trade_count'))}`",
        f"- Total traded quantity: `{display_value(trade_metrics.get('total_traded_quantity'))}`",
        f"- Total notional: `{display_value(trade_metrics.get('total_notional'))}`",
        f"- Last trade price: `{display_value(trade_metrics.get('last_trade_price'))}`",
        f"- VWAP: `{display_value(trade_metrics.get('vwap'))}`",
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

def interpret(summary, events, snapshot):
    # This is deterministic analysis.
    # Later LangChain/LangGraph can use this same data as context.
    findings = []

    book_metrics = summary.get("book_metrics", {})
    trade_metrics = summary.get("trade_metrics", {})


    trade_count = trade_metrics.get("trade_count", 0)
    resting_orders = summary.get("resting_orders", 0)
    spread = book_metrics.get("spread")
    vwap = trade_metrics.get("vwap")

    if trade_count == 0:
        findings.append("- No trades were produced. This run mainly tested resting book liquidity.")
    else:
        findings.append(f"- The run produced `{trade_count}` trades.")

    if resting_orders == 0:
        findings.append("- No resting orders remain, so all visible liquidity was consumed or no liquidity rested.")
    else:
        findings.append(f"- `{resting_orders}` orders remain resting in the final book.")

    if spread is None:
        findings.append("- Spread is not available because one or both sides of the book are empty.")
    else:
        findings.append(f"- Final spread is `{spread}` ticks.")

    if vwap is not None:
        findings.append(f"- VWAP is `{vwap}`, which summarizes the average executed price weighted by quantity.")

    return "\n".join(findings)

def main():
    # Require the user to pass a run directory.
    if len(sys.argv) != 2:
        print("usage: python3 ai/analyze_run.py runs/<run-folder>")
        sys.exit(1)

    run_dir = Path(sys.argv[1])

    summary = load_json(run_dir / "summary.json")
    events = load_json(run_dir / "events.json")
    snapshot = load_json(run_dir / "snapshot.json")

    report = build_report(summary, events, snapshot)

    output_path = run_dir / "analysis.md"
    output_path.write_text(report, encoding="utf-8")

    print(f"analysis written to: {output_path}")


if __name__ == "__main__":
    main()