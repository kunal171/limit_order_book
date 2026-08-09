use crate::types::{OrderId, Price, Quantity};

/// A trade produced when an incoming order matches a resting order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub maker_order_id: OrderId,
    pub taker_order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
}

impl Trade {
    /// Create a trade at the resting order's price.
    pub fn new(
        maker_order_id: OrderId,
        taker_order_id: OrderId,
        price: Price,
        quantity: Quantity,
    ) -> Self {
        Self {
            maker_order_id,
            taker_order_id,
            price,
            quantity,
        }
    }
}
