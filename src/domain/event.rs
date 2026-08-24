//! Event types for the order book

use crate::Quantity;
use crate::domain::{Order, OrderId, Price, Trade};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookEvent {
    OrderAccepted {
        order: Order,
    },
    OrderCancelled {
        order_id: OrderId,
    },
    OrderModified {
        order_id: OrderId,
        new_price: Price,
        new_quantity: Quantity,
    },
    TradeExecuted {
        trade: Trade,
    },
}
