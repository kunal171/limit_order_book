# Limit Order Book

A Rust limit order book and market microstructure lab.

The project starts with a deterministic matching engine and grows in phases into
a research platform for event replay, simulation, benchmarks, AI-assisted
analysis, Windmill workflows, and later HFT-style performance experiments.

## Current Status

Current branch:

```text
phase-05-market-data-simulator
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

  replay/
    persistence.rs  Save/load events as JSON
    replay.rs       Rebuild book state from events

  simulator/
    scenario.rs     ScenarioCommand input enum
    scenarios.rs    Predefined market scenarios
    runner.rs       Runs scenario commands against a fresh book

  error.rs          OrderBookError
  lib.rs            Library exports
  main.rs           CLI demo/simulator entrypoint
```

The public library exports the common types from `lib.rs`, so users can write:

```rust
use limit_order_book::{Order, OrderBook, Side};
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

## Example Output

```bash
cargo run -- buy-sweeps-asks
```

This scenario places multiple sell orders and then sends one buy order that
sweeps through those ask levels. The result shows generated trades, best bid,
best ask, resting order count, and the final book snapshot.

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

Detailed roadmap:

```text
docs/ROADMAP.md
```

## Long-Term Direction

The final project direction:

```text
Rust matching engine
-> event replay and market simulation
-> benchmark reports
-> market data metrics
-> AI scenario analysis
-> LangGraph research workflows
-> Windmill scheduled runs and dashboards
-> HFT-style optimization experiments
```

The hot path remains Rust-only.
