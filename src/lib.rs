//! Core library for the limit order book.
//!
//! Keep the matching engine separate from `main.rs` so we can test it directly
//! and later reuse it from an API, CLI, benchmark, or market-data simulator.

pub mod domain;
pub mod engine;
pub mod error;

pub use domain::{BookEvent, Order, OrderId, Price, Quantity, Side, Trade};
pub use engine::OrderBook;
pub use error::OrderBookError;
