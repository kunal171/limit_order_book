//! Matching engine modules.
//!
//! `order_book` exposes the public engine API, while `matching` keeps the
//! private price-time priority matching helpers out of the main API file.

mod matching;
mod order_book;
mod config;

pub use order_book::OrderBook;
