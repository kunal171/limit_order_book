# Performance Baseline

This file records the saved performance baseline for the HFT-style systems
phase.

Purpose:

```text
measure first, optimize second
```

Any future optimization should compare against this file before we decide
whether the change actually helped.

## Run Details

```text
Date: 2026-09-04
Branch: phase-11-hft-systems
Commit: 732bf74
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

Benchmark scope:

```text
small hot-path operations
1,000 order synthetic workloads
10,000 and 100,000 order larger workloads
deep price-level cancel/modify operations
100-symbol synthetic workload
```

Criterion reports each result as:

```text
[lower bound, estimate, upper bound]
```

Use the middle number as the main baseline estimate.

| Benchmark | Lower | Estimate | Upper | What It Measures |
| --- | ---: | ---: | ---: | --- |
| `two_sided_1000_orders` | 57.791 us | 58.861 us | 59.881 us | Runs 1,000 deterministic non-crossing orders |
| `crossing_1000_orders` | 53.381 us | 53.879 us | 54.379 us | Runs 1,000 deterministic orders that create trades |
| `add_one_resting_order` | 81.117 ns | 82.705 ns | 84.896 ns | Adds one order that rests in the book |
| `single_trade` | 104.94 ns | 106.66 ns | 108.42 ns | Matches one crossing order against one resting order |
| `multi_level_sweep` | 244.02 ns | 249.19 ns | 255.03 ns | Sweeps multiple price levels with one order |
| `cancel_order` | 106.84 ns | 109.09 ns | 111.57 ns | Cancels one resting order |
| `modify_order` | 169.09 ns | 176.70 ns | 186.97 ns | Modifies one resting order |
| `two_sided_10000_orders` | 676.66 us | 695.44 us | 717.79 us | Runs 10,000 deterministic non-crossing orders |
| `two_sided_100000_orders` | 7.5620 ms | 7.6894 ms | 7.8263 ms | Runs 100,000 deterministic non-crossing orders |
| `cancel_from_10000_deep_level` | 4.5778 us | 4.7926 us | 5.0213 us | Cancels near the end of a 10,000-order FIFO price level |
| `modify_from_10000_deep_level` | 5.0434 us | 5.3255 us | 5.6412 us | Modifies near the end of a 10,000-order FIFO price level |
| `multi_symbol_100x1000_orders` | 8.1417 ms | 8.2719 ms | 8.4149 ms | Runs 100 books with 1,000 orders each |

## Throughput Estimate

These are rough estimates from the Criterion middle value.

| Benchmark | Approximate Throughput |
| --- | ---: |
| `two_sided_1000_orders` | 16.99 million orders/sec |
| `crossing_1000_orders` | 18.56 million orders/sec |
| `two_sided_10000_orders` | 14.38 million orders/sec |
| `two_sided_100000_orders` | 13.01 million orders/sec |
| `multi_symbol_100x1000_orders` | 12.09 million orders/sec |

Interpretation:

```text
The large synthetic workloads scale reasonably well for the current
correctness-first design.
```

Important caution:

```text
These are in-process benchmarks. They do not include networking, protocol
parsing, risk checks, persistence, thread handoff, or p99/p999 latency.
```

## Benchmark Notes

Criterion also printed local comparison messages:

```text
two_sided_1000_orders: No change in performance detected
crossing_1000_orders: No change in performance detected
add_one_resting_order: Change within noise threshold
single_trade: No change in performance detected
multi_level_sweep: No change in performance detected
cancel_order: Performance has regressed
modify_order: Performance has regressed
```

Important:

```text
Those messages compare against previous local Criterion history on this machine.
For this phase, the table above is the saved baseline.
```

Outliers were present in multiple benchmarks. That is expected on a normal
developer machine because background processes, CPU frequency scaling, scheduler
noise, and thermals can affect nanosecond-level measurements.

The `1_000_000` resting-order workload is not part of this saved baseline yet.
That should be added later as a separate heavy benchmark so normal local
benchmark runs do not become too slow.

## Baseline Observations

Large workloads:

```text
1,000 non-crossing orders: 58.861 us
10,000 non-crossing orders: 695.44 us
100,000 non-crossing orders: 7.6894 ms
```

This is roughly linear, which is a good sign for the current synthetic workload.

Deep cancel/modify:

```text
cancel near end of 10,000-order level: 4.7926 us
modify near end of 10,000-order level: 5.3255 us
```

This confirms the expected weakness:

```text
order_sides finds the side quickly, but remove_order_from_side still scans
inside the FIFO queue at that price level.
```

Multi-symbol:

```text
100 symbols x 1,000 orders: 8.2719 ms
```

This is a simple simulation where each symbol is represented by an independent
`OrderBook`. It is useful as a baseline, but it is not a real multi-symbol
gateway or sharded matching architecture yet.

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
