use super::types::{OrderId, Price, Quantity, Side};

/// A limit order currently entering or resting in the book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub price: Price,
    pub remaining_qty: Quantity,
}

impl Order {
    /// Create a new limit order.
    pub fn new(id: OrderId, side: Side, price: Price, quantity: Quantity) -> Self {
        Self {
            id,
            side,
            price,
            remaining_qty: quantity,
        }
    }

    /// Return true when the order has no quantity left to trade.
    pub fn is_filled(&self) -> bool {
        self.remaining_qty == 0
    }
}
