# Performance Baseline

This file records the first saved performance baseline for the HFT-style systems
phase.

Purpose:

```text
measure first, optimize second
```

Any future optimization should compare against this file before we decide
whether the change actually helped.

## Run Details

```text
Date: 2026-09-03
Branch: phase-11-hft-systems
Commit: 8ead7bc
Project: limit_order_book v0.1.0
Rust edition: 2024
Benchmark profile: cargo bench / optimized release benchmark profile
Benchmark tool: Criterion 0.8.2
Plot backend note: Gnuplot not found, Criterion used plotters backend
```

## Machine

```text
OS: Linux Shady 7.0.0-30-generic
Kernel: #30-Ubuntu SMP PREEMPT_DYNAMIC Fri Jul 31 18:22:54 UTC 2026
Architecture: x86_64
CPU: AMD Ryzen 5 5600X 6-Core Processor
CPU threads: 12
Cores per socket: 6
Threads per core: 2
Socket(s): 1
CPU max MHz: 4654.2881
CPU min MHz: 560.7580
L1d cache: 192 KiB (6 instances)
L1i cache: 192 KiB (6 instances)
L2 cache: 3 MiB (6 instances)
L3 cache: 32 MiB (1 instance)
NUMA nodes: 1
```

## Toolchain

```text
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
```

## Correctness Check

Command:

```bash
cargo test
```

Result:

```text
39 passed
0 failed
0 ignored
0 measured
0 filtered out
```

Interpretation:

```text
The correctness safety net passes before starting HFT-style optimization work.
```

## Benchmark Results

Command:

```bash
cargo bench
```

Criterion reports each result as:

```text
[lower bound, estimate, upper bound]
```

Use the middle number as the main baseline estimate.

| Benchmark | Lower | Estimate | Upper | What It Measures |
| --- | ---: | ---: | ---: | --- |
| `two_sided_1000_orders` | 54.905 us | 55.699 us | 56.571 us | Runs 1,000 deterministic non-crossing orders |
| `crossing_1000_orders` | 52.391 us | 53.241 us | 54.213 us | Runs 1,000 deterministic orders that create trades |
| `add_one_resting_order` | 80.796 ns | 81.324 ns | 81.902 ns | Adds one order that rests in the book |
| `single_trade` | 100.34 ns | 102.90 ns | 105.63 ns | Matches one crossing order against one resting order |
| `multi_level_sweep` | 213.01 ns | 214.09 ns | 215.20 ns | Sweeps multiple price levels with one order |
| `cancel_order` | 96.780 ns | 98.734 ns | 101.03 ns | Cancels one resting order |
| `modify_order` | 147.65 ns | 148.26 ns | 149.01 ns | Modifies one resting order |

## Benchmark Notes

Criterion also printed local comparison messages:

```text
add_one_resting_order: Performance has regressed
cancel_order: Performance has improved
modify_order: Performance has improved
```

Important:

```text
Those messages compare against previous local Criterion history on this machine.
For this phase, the table above is the saved baseline.
```

Outliers were present in multiple benchmarks. That is expected on a normal
developer machine because background processes, CPU frequency scaling, scheduler
noise, and thermals can affect nanosecond-level measurements.

## Current Hot-Path Concerns

The current engine is correctness-first, not HFT-optimized.

Known concerns:

```text
add_order records events on every accepted order
event recording clones orders and trades
events: Vec<BookEvent> can grow without bound
cancel_order uses order_sides to find the side, but still scans inside the price level
modify_order removes and re-adds the order, which is simple but not yet optimized
BTreeMap is clean and correct, but may not be the fastest price ladder later
JSON output is useful for tools, but must stay outside the matching hot path
```

## First Optimization Target

Next target:

```text
make event recording configurable
```

Why:

```text
Event replay is useful for debugging, audit, Windmill, and AI analysis.
But low-latency benchmarks should not always pay for event clones and an
unbounded in-memory event log.
```

Expected design:

```text
OrderBookConfig {
    event_mode: EventMode
}

EventMode:
    Full
    TradesOnly
    Disabled
```

Expected result:

```text
Full mode keeps current replay/debug behavior.
Disabled mode gives a cleaner hot-path benchmark.
TradesOnly mode keeps execution output without recording every accepted command.
```

## Next Measurement After Optimization

After event recording config is implemented, run:

```bash
cargo test
cargo bench
```

Then compare especially:

```text
add_one_resting_order
single_trade
multi_level_sweep
crossing_1000_orders
```

The optimization is only worth keeping if correctness stays the same and the
benchmark results improve or the trade-off is clearly documented.
