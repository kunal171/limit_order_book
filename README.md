# Limit Order Book

A Rust-based limit order book and market microstructure lab.

The project starts with a deterministic matching engine and will grow in phases into a research platform that combines Rust, AI-assisted analysis, Windmill workflow orchestration, and later HFT-style performance work.

## Project Vision

The core idea is to keep the matching engine simple, correct, and fast:

```text
incoming order
-> validate
-> match against opposite side
-> emit trades
-> update resting book state
```

AI, Windmill, reports, dashboards, databases, and workflows will stay outside the matching hot path. They will support research, testing, orchestration, and analysis around the engine.

## Current Status

Current branch:

```text
phase-04-event-log-replay
```

Completed so far:

```text
price-time priority matching
limit buy and sell orders
partial fills
full fills
best bid and best ask
trade generation
zero quantity validation
duplicate active order id validation
cancel resting orders
modify resting orders
active order side index
unit tests for matching, validation, cancel, and modify
```

## Architecture

```text
src/
  domain/
    order.rs        Order model
    trade.rs        Trade model
    types.rs        OrderId, Price, Quantity, Side

  engine/
    order_book.rs   Public OrderBook API and tests
    matching.rs     Private price-time matching logic

  error.rs          OrderBookError
  lib.rs            Library exports
  main.rs           Small demo binary
```

The public library exports common types from `lib.rs`, so users can write:

```rust
use limit_order_book::{Order, OrderBook, Side};
```

## Important Concepts

### Price-Time Priority

The matching engine follows price-time priority:

```text
better price wins first
same price uses FIFO order
```

For buy orders, the best price is the highest bid. For sell orders, the best price is the lowest ask.

### Integer Prices

Prices use integer ticks instead of floating point values.

```text
good: 10025 paise/ticks
bad: 100.25 as f64
```

This avoids floating point rounding issues in financial logic.

### Active Order Index

The book stores an active order side index:

```text
order_id -> side
```

This lets cancel and modify jump to the correct side instead of scanning both bids and asks. It is still an MVP structure; later HFT work can replace it with a fuller location index.

## Run

Run the demo:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

Format code:

```bash
cargo fmt
```

## Roadmap

The project is built in phases:

```text
Phase 1: Matching core
Phase 2: Correctness tests
Phase 3: Cancel and modify orders
Phase 4: Event log and replay
Phase 5: Market data simulator
Phase 6: Benchmarks
Phase 7: LangChain AI analysis layer
Phase 8: LangGraph research agent
Phase 9: Windmill orchestration
Phase 10: Advanced HFT optimization
```

Detailed roadmap:

```text
docs/ROADMAP.md
```

## Next Phase

Next implementation phase:

```text
event log and replay
```

This will introduce events such as:

```text
OrderAccepted
OrderRejected
OrderCancelled
OrderModified
TradeExecuted
BookSnapshot
```

The goal is to make the engine replayable and auditable:

```text
same input events
-> same trades
-> same final book state
```

## Long-Term Direction

The final project will become:

```text
Rust matching engine
-> replay and simulation
-> benchmark reports
-> AI scenario generation and analysis
-> LangGraph experiment workflows
-> Windmill scheduled runs and dashboards
-> HFT-style optimization experiments
```

The hot path remains Rust-only.
