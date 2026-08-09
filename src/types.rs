//! Small domain types used by the order book.

/// Unique id for an order.
pub type OrderId = u64;

/// Price stored as an integer tick value.
///
/// In real trading systems we avoid floating point prices because floats can
/// introduce rounding surprises. For example, use paise/cents/ticks instead.
pub type Price = u64;

/// Quantity stored as an integer unit.
pub type Quantity = u64;

/// Direction of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}
