//! Domain types for the order book.
//!
//! This module contains the core nouns of the system: orders, trades, prices,
//! quantities, and sides. It should stay mostly free of matching-engine logic.

pub mod order;
pub mod trade;
pub mod types;

pub use order::Order;
pub use trade::Trade;
pub use types::{OrderId, Price, Quantity, Side};
