# AI Work Pause Note

This document records the AI integration work completed so far, why it is being
paused, and where to resume later.

## Status

AI integration is paused for now.

Current priority:

```text
make the Rust limit order book stronger as a systems/HFT-style project
```

The AI layer should remain outside the matching hot path. It should analyze
completed runs, not decide order matching or mutate live book state.

## What Exists Today

The current AI work is a deterministic analysis foundation, not a full LLM
integration.

Files:

```text
ai/README.md
ai/analyze_run.py
ai/requirements.txt
scripts/analyze_run.sh
scripts/run_and_analyze.sh
```

### ai/analyze_run.py

Purpose:

```text
read run artifacts -> produce analysis.md
```

Inputs:

```text
summary.json
events.json
snapshot.json
```

Output:

```text
analysis.md
```

Current behavior:

```text
validates that the run directory exists
validates required run artifacts exist
loads JSON artifacts
counts event types
formats book metrics
formats trade metrics
turns None/null values into "not available"
generates deterministic interpretation bullets
```

Important functions:

```text
load_json(path)
event_name(event)
count_events(events)
display_value(value)
build_report(summary, events, snapshot)
interpret(summary, events, snapshot)
validate_run_dir(run_dir)
main()
```

### scripts/analyze_run.sh

Purpose:

```text
shell wrapper around ai/analyze_run.py
```

Usage:

```bash
./scripts/analyze_run.sh runs/<run-folder>
```

Why it exists:

```text
Windmill and local scripts should call a simple shell entrypoint.
The Python analyzer can change internally without changing orchestration calls.
```

### scripts/run_and_analyze.sh

Purpose:

```text
run Rust simulation -> write artifacts -> run analyzer -> print final JSON
```

Usage:

```bash
./scripts/run_and_analyze.sh synthetic-crossing 100 auto
```

Expected final stdout:

```json
{
  "run_dir": "runs/windmill-analysis-...",
  "summary_path": "runs/windmill-analysis-.../summary.json",
  "analysis_path": "runs/windmill-analysis-.../analysis.md",
  "analysis_log_path": "runs/windmill-analysis-.../analysis.log"
}
```

Why it exists:

```text
Windmill should receive one clean JSON result.
Verbose simulation/analyzer output should be stored as artifacts, not mixed into stdout.
```

## Verified Flow

This flow was tested:

```bash
./scripts/run_and_analyze.sh synthetic-crossing 10 auto
```

It created:

```text
analysis.md
analysis.log
events.json
run.log
simulation_stdout.json
snapshot.json
summary.json
```

The generated analysis included:

```text
scenario summary
book metrics
trade metrics
event counts
interpretation bullets
```

Example findings:

```text
The run produced trades.
No resting orders remain.
Spread is not available because one or both sides are empty.
VWAP summarizes the average executed price weighted by quantity.
```

## What Is Intentionally Deferred

Do not continue these yet:

```text
LangChain integration
LangGraph workflow
LLM-generated reports
AI scenario generation
AI natural-language query layer
Windmill flow that calls an LLM
AI dashboard/report UI
```

These can wait until the Rust system is more impressive and measurable.

## Why Pause Here

The strongest next direction is HFT-style systems work:

```text
matching hot path clarity
latency measurement
allocation reduction
bounded event handling
better benchmark baselines
data layout improvements
order id index improvements
replay correctness after optimization
```

AI is useful later, but it should analyze a strong system. It should not become
the center of the project before the engine itself has stronger systems depth.

## Rules When Resuming AI

Keep these rules:

```text
AI must not be inside the matching hot path.
AI must not mutate the order book directly.
AI must use structured artifacts instead of guessing from vague text.
AI output should cite evidence from summary/events/snapshot.
Windmill should orchestrate AI work after simulation completes.
Every AI result should be reproducible from saved artifacts.
```

Good AI flow:

```text
Rust engine run
-> save summary/events/snapshot
-> deterministic analyzer creates findings
-> optional LLM explains findings
-> Windmill stores and links report
```

Bad AI flow:

```text
order arrives
-> LLM decides matching behavior
-> book state mutates from AI output
```

## Resume Plan Later

When we come back to AI, resume in this order:

```text
1. Clean analyzer error output so Windmill does not show Python tracebacks.
2. Add test fixtures for ai/analyze_run.py.
3. Add a small LangChain wrapper that reads analysis.md and summary.json.
4. Add an LLM prompt that only explains evidence from artifacts.
5. Add LangGraph state for repeated experiments.
6. Add human approval before generated scenarios are run.
7. Add a Windmill flow: run simulation -> analyze -> generate AI report.
```

## Next Project Direction

After this pause, move to HFT/system-strengthening work.

Suggested next branch:

```text
phase-11-hft-systems
```

Suggested first HFT/system tasks:

```text
record current benchmark baselines
identify allocations in add_order/cancel/modify
make event storage configurable or bounded
separate hot-path matching from optional observability
add benchmark notes with before/after numbers
```

Detailed HFT roadmap:

```text
docs/HFT_ROADMAP.md
```
