//! Core library for the limit order book.
//!
//! Keep the matching engine separate from `main.rs` so we can test it directly
//! and later reuse it from an API, CLI, benchmark, or market-data simulator.

pub mod order;
pub mod order_book;
pub mod trade;
pub mod types;

pub use order::Order;
pub use order_book::OrderBook;
pub use trade::Trade;
pub use types::{OrderId, Price, Quantity, Side};
