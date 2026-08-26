# Limit Order Book

A Rust limit order book and market microstructure lab.

The project starts with a deterministic matching engine and grows in phases into
a research platform for event replay, simulation, market metrics, benchmarks,
AI-assisted analysis, Windmill workflows, and later HFT-style performance
experiments.

## Current Status

Completed through:

```text
Phase 8: Benchmarks
```

Implemented so far:

```text
price-time priority matching
limit buy and sell orders
partial fills and full fills
best bid and best ask
trade generation
zero quantity validation
duplicate active order id validation
cancel resting orders
modify resting orders
active order side index
book snapshots
event log
event replay
save and load event streams as JSON
predefined simulator scenarios
CLI runner for scenarios
JSON simulator output
book metrics
trade metrics
order book imbalance
deterministic two-sided synthetic order generation
deterministic crossing synthetic order generation
configurable synthetic order count
Criterion benchmark suite
synthetic workload benchmarks
hot-path operation benchmarks
```

## Core Idea

The matching engine follows this flow:

```text
incoming order
-> validate
-> match against the opposite side
-> emit trades
-> rest any leftover quantity
-> record events
```

AI, Windmill, databases, dashboards, and reports stay outside the matching hot
path. They are useful for orchestration and analysis, but the core book should
remain deterministic and easy to replay.

## Architecture

```text
src/
  domain/
    event.rs        BookEvent output stream
    order.rs        Order model
    snapshot.rs     Serializable book snapshot
    trade.rs        Trade model
    types.rs        OrderId, Price, Quantity, Side

  engine/
    order_book.rs   Public OrderBook API and tests
    matching.rs     Private price-time matching logic

  metrics/
    book_metrics.rs   Spread, mid price, depth, levels, imbalance
    trade_metrics.rs  Volume, notional, last trade price, VWAP

  replay/
    persistence.rs  Save/load events as JSON
    replay.rs       Rebuild book state from events

  simulator/
    generator.rs    Deterministic synthetic order generators
    scenario.rs     ScenarioCommand input enum
    scenarios.rs    Predefined market scenarios
    runner.rs       Runs scenario commands against a fresh book

  error.rs          OrderBookError
  lib.rs            Library exports
  main.rs           CLI demo/simulator entrypoint

benches/
  order_book_bench.rs  Criterion benchmarks for workloads and hot paths
```

The public library exports common types and helpers from `lib.rs`, so users can
write:

```rust
use limit_order_book::{
    calculate_book_metrics, calculate_trade_metrics, Order, OrderBook, Side,
};
```

## Important Concepts

### Price-Time Priority

The engine follows price-time priority:

```text
better price wins first
same price uses FIFO order
```

For buy orders, the best price is the highest bid. For sell orders, the best
price is the lowest ask.

### Integer Prices

Prices use integer ticks instead of floating point values.

```text
good: 10025 ticks
bad: 100.25 as f64
```

This avoids floating point rounding bugs in financial logic.

### Event Replay

The engine records events such as:

```text
OrderAccepted
OrderCancelled
OrderModified
TradeExecuted
```

Replay lets us rebuild book state from historical events:

```text
saved event stream
-> replay events
-> same final book snapshot
```

This is important for debugging, audits, crash recovery, simulation, and later
analytics.

### Market Metrics

Book metrics describe the final visible market:

```text
best bid
best ask
spread
mid price
total bid quantity
total ask quantity
bid and ask price levels
order book imbalance
```

Trade metrics describe what happened during execution:

```text
trade count
total traded quantity
total notional
last trade price
VWAP
```

VWAP means volume-weighted average price:

```text
total traded notional / total traded quantity
```

### Synthetic Workloads

Synthetic generators create repeatable order flows for testing, metrics, replay,
and later benchmarks.

The current generators are deterministic:

```text
synthetic
-> creates a two-sided resting book without trades

synthetic-crossing
-> builds ask liquidity and then sends crossing buy orders
-> creates trades and execution metrics
```

Deterministic means the same config creates the same sequence every time. That
is useful before adding randomness because tests and benchmark comparisons stay
stable.

### Benchmarks

Benchmarks measure both full workloads and individual hot-path operations.

Full workload benchmarks:

```text
two_sided_1000_orders
crossing_1000_orders
```

Hot-path benchmarks:

```text
add_one_resting_order
single_trade
multi_level_sweep
cancel_order
modify_order
```

The full workload benchmarks show end-to-end scenario cost. The hot-path
benchmarks isolate specific order book operations so later optimizations have a
clear baseline.

## Run

Run tests:

```bash
cargo test
```

Format code:

```bash
cargo fmt
```

Run the default scenario:

```bash
cargo run
```

Run a specific scenario:

```bash
cargo run -- simple-cross
cargo run -- buy-sweeps-asks
cargo run -- cancel-and-modify
cargo run -- two-sided-book
cargo run -- synthetic --count 100
cargo run -- synthetic-crossing --count 100
```

Print simulator output as JSON:

```bash
cargo run -- buy-sweeps-asks --json
```

Save generated events:

```bash
cargo run -- buy-sweeps-asks --save-events events.json
```

Replay saved events:

```bash
cargo run -- --replay-events events.json
```

Run benchmarks:

```bash
cargo bench
```

## Synthetic Examples

Build a two-sided resting book:

```bash
cargo run -- synthetic --count 20
```

This creates buy orders below the base price and sell orders above the base
price. It is useful for book metrics such as spread, mid price, depth, and
imbalance.

Generate trades:

```bash
cargo run -- synthetic-crossing --count 20
```

This first builds ask liquidity, then sends buy orders that cross the spread.
It is useful for trade metrics such as traded quantity, notional, last trade
price, and VWAP.

## Example Metrics

```bash
cargo run -- two-sided-book
```

This scenario leaves both bids and asks resting in the book, so spread, mid
price, depth, and imbalance are visible.

Expected market shape:

```text
best bid: 100
best ask: 105
spread: 5
mid price: 102.5
total bid quantity: 15
total ask quantity: 10
imbalance: 0.6
```

## Roadmap

The project is built in phases:

```text
Phase 1: Matching core
Phase 2: Correctness tests
Phase 3: Cancel and modify orders
Phase 4: Event log and replay
Phase 5: Market data simulator
Phase 6: Market data metrics
Phase 7: Synthetic order generator
Phase 8: Benchmarks
Phase 9: Windmill orchestration
Phase 10: AI/LangChain/LangGraph analysis
Phase 11: HFT-style optimization
```

Next phase:

```text
Phase 9: Windmill orchestration
```

Detailed roadmap:

```text
docs/ROADMAP.md
```

## Long-Term Direction

The final project direction:

```text
Rust matching engine
-> event replay and market simulation
-> market data metrics
-> synthetic workloads
-> benchmark reports
-> AI scenario analysis
-> LangGraph research workflows
-> Windmill scheduled runs and dashboards
-> HFT-style optimization experiments
```

The hot path remains Rust-only.
