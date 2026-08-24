use crate::Quantity;
use crate::domain::{Order, OrderId, Price, Side, Trade};
use crate::error::OrderBookError;
use std::collections::{BTreeMap, HashMap, VecDeque};

/// A simple price-time priority limit order book.
///
/// `BTreeMap` keeps price levels sorted.
/// `VecDeque` preserves FIFO order inside each price level.
#[derive(Debug, Default)]
pub struct OrderBook {
    pub(super) bids: BTreeMap<Price, VecDeque<Order>>,
    pub(super) asks: BTreeMap<Price, VecDeque<Order>>,
    pub(super) order_sides: HashMap<OrderId, Side>,
}

impl OrderBook {
    /// Create an empty book.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an order and return all trades caused by that order.
    pub fn add_order(&mut self, order: Order) -> Result<Vec<Trade>, OrderBookError> {
        if order.remaining_qty == 0 {
            return Err(OrderBookError::ZeroQuantity);
        }
        if self.order_sides.contains_key(&order.id) {
            return Err(OrderBookError::DuplicateOrderId);
        }
        let trades = match order.side {
            Side::Buy => self.match_buy_order(order),
            Side::Sell => self.match_sell_order(order),
        };

        Ok(trades)
    }

    /// Highest resting buy price.
    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    /// Lowest resting sell price.
    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    /// Number of resting orders across both sides.
    pub fn resting_order_count(&self) -> usize {
        self.bids.values().map(VecDeque::len).sum::<usize>()
            + self.asks.values().map(VecDeque::len).sum::<usize>()
    }

    // Cancel the order
    pub fn cancel_order(&mut self, order_id: OrderId) -> Result<(), OrderBookError> {
        let side = self
            .order_sides
            .get(&order_id)
            .copied()
            .ok_or(OrderBookError::UnknownOrderId)?;

        self.remove_order_from_side(order_id, side)
            .ok_or(OrderBookError::UnknownOrderId)?;

        self.order_sides.remove(&order_id);

        Ok(())
    }

    //Remove order from the given side
    fn remove_order_from_side(&mut self, order_id: OrderId, side: Side) -> Option<Order> {
        // find the order book of side
        let levels = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let mut removed_order = None;
        let mut empty_price_level = None;

        for (price, orders) in levels.iter_mut() {
            if let Some(index) = orders.iter().position(|order| order.id == order_id) {
                removed_order = orders.remove(index);
                if orders.is_empty() {
                    empty_price_level = Some(*price);
                }

                break;
            }
        }

        if let Some(price) = empty_price_level {
            levels.remove(&price);
        }
        removed_order
    }

    fn remove_order_by_id(&mut self, order_id: OrderId) -> Option<Order> {
        let side = self.order_sides.get(&order_id).copied()?;
        let order = self.remove_order_from_side(order_id, side)?;

        self.order_sides.remove(&order_id);

        Some(order)
    }

    pub fn modify_order(
        &mut self,
        order_id: OrderId,
        new_price: Price,
        new_quantity: Quantity,
    ) -> Result<Vec<Trade>, OrderBookError> {
        if new_quantity == 0 {
            return Err(OrderBookError::ZeroQuantity);
        }
        let old_order = self
            .remove_order_by_id(order_id)
            .ok_or(OrderBookError::UnknownOrderId)?;

        let updated_order = Order::new(old_order.id, old_order.side, new_price, new_quantity);

        self.add_order(updated_order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Side;

    #[test]
    fn buy_order_rests_when_there_is_no_matching_ask() {
        let mut book = OrderBook::new();

        let trades = book
            .add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid buy order should be accepted");

        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn sell_order_matches_existing_bid() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid buy order should be accepted");

        let trades = book
            .add_order(Order::new(2, Side::Sell, 99, 4))
            .expect("valid sell order should be accepted");

        assert_eq!(trades, vec![Trade::new(1, 2, 100, 4)]);
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn orders_at_same_price_match_fifo() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 100, 5))
            .expect("first valid buy order should be accepted");
        book.add_order(Order::new(2, Side::Buy, 100, 5))
            .expect("second valid buy order should be accepted");

        let trades = book
            .add_order(Order::new(3, Side::Sell, 100, 8))
            .expect("valid sell order should be accepted");

        assert_eq!(
            trades,
            vec![Trade::new(1, 3, 100, 5), Trade::new(2, 3, 100, 3)]
        );
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn better_price_matches_before_older_worse_price() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 99, 5))
            .expect("valid buy order should be accepted");
        book.add_order(Order::new(2, Side::Buy, 101, 5))
            .expect("better-priced valid buy order should be accepted");

        let trades = book
            .add_order(Order::new(3, Side::Sell, 99, 5))
            .expect("valid sell order should be accepted");

        assert_eq!(trades, vec![Trade::new(2, 3, 101, 5)]);
        assert_eq!(book.best_bid(), Some(99));
    }

    #[test]
    fn sell_order_rests_when_there_is_no_matching_bid() {
        let mut book = OrderBook::new();

        let trades = book
            .add_order(Order::new(1, Side::Sell, 101, 10))
            .expect("valid sell order should be accepted");

        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.best_ask(), Some(101));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn sell_does_not_match_bid_below_limit() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid buy order should be accepted");

        let trades = book
            .add_order(Order::new(2, Side::Sell, 101, 5))
            .expect("valid sell order should be accepted");

        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.best_ask(), Some(101));
        assert_eq!(book.resting_order_count(), 2);
    }

    #[test]
    fn buy_does_not_match_ask_above_limit() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Sell, 100, 10))
            .expect("valid sell order should be accepted");

        let trades = book
            .add_order(Order::new(2, Side::Buy, 99, 5))
            .expect("valid buy order should be accepted");

        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), Some(99));
        assert_eq!(book.best_ask(), Some(100));
        assert_eq!(book.resting_order_count(), 2);
    }

    #[test]
    fn buy_sweeps_multiple_ask_levels() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Sell, 100, 5))
            .expect("valid sell order should be accepted");
        book.add_order(Order::new(2, Side::Sell, 101, 5))
            .expect("valid sell order should be accepted");
        book.add_order(Order::new(3, Side::Sell, 102, 5))
            .expect("valid sell order should be accepted");

        let trades = book
            .add_order(Order::new(4, Side::Buy, 102, 12))
            .expect("valid buy order should be accepted");

        assert_eq!(
            trades,
            vec![
                Trade::new(1, 4, 100, 5),
                Trade::new(2, 4, 101, 5),
                Trade::new(3, 4, 102, 2),
            ]
        );
        assert_eq!(book.best_ask(), Some(102));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn sell_sweeps_multiple_bid_levels() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 100, 5))
            .expect("valid buy order should be accepted");
        book.add_order(Order::new(2, Side::Buy, 101, 5))
            .expect("valid buy order should be accepted");
        book.add_order(Order::new(3, Side::Buy, 102, 5))
            .expect("valid buy order should be accepted");

        let trades = book
            .add_order(Order::new(4, Side::Sell, 99, 12))
            .expect("valid sell order should be accepted");

        assert_eq!(
            trades,
            vec![
                Trade::new(3, 4, 102, 5),
                Trade::new(2, 4, 101, 5),
                Trade::new(1, 4, 100, 2),
            ]
        );
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn rejects_zero_quantity_order() {
        let mut book = OrderBook::new();

        let result = book.add_order(Order::new(1, Side::Buy, 100, 0));

        assert_eq!(result, Err(OrderBookError::ZeroQuantity));
        assert_eq!(book.resting_order_count(), 0);
    }

    #[test]
    fn rejects_duplicate_resting_order_id() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("first order should be accepted");

        let result = book.add_order(Order::new(1, Side::Sell, 101, 5));

        assert_eq!(result, Err(OrderBookError::DuplicateOrderId));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn cancel_resting_bid_order() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid order should be accepted");

        let result = book.cancel_order(1);

        assert_eq!(result, Ok(()));
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.resting_order_count(), 0);
    }

    #[test]
    fn cancel_resting_ask_order() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Sell, 101, 10))
            .expect("valid order should be accepted");

        let result = book.cancel_order(1);

        assert_eq!(result, Ok(()));
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.resting_order_count(), 0);
    }

    #[test]
    fn cancel_unknown_order_returns_error() {
        let mut book = OrderBook::new();

        let result = book.cancel_order(999);

        assert_eq!(result, Err(OrderBookError::UnknownOrderId));
    }

    #[test]
    fn cancel_one_order_keeps_price_level_when_other_orders_remain() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("first order should be accepted");
        book.add_order(Order::new(2, Side::Buy, 100, 5))
            .expect("second order should be accepted");

        let result = book.cancel_order(1);

        assert_eq!(result, Ok(()));
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn modify_unknown_order_returns_error() {
        let mut book = OrderBook::new();

        let result = book.modify_order(999, 100, 10);

        assert_eq!(result, Err(OrderBookError::UnknownOrderId));
    }

    #[test]
    fn modify_order_rejects_zero_quantity() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid order should be accepted");

        let result = book.modify_order(1, 100, 0);

        assert_eq!(result, Err(OrderBookError::ZeroQuantity));
    }

    #[test]
    fn modify_zero_quantity_does_not_remove_existing_order() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid order should be accepted");

        let result = book.modify_order(1, 100, 0);

        assert_eq!(result, Err(OrderBookError::ZeroQuantity));
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn modify_resting_bid_price() {
        let mut book = OrderBook::new();

        book.add_order(Order::new(1, Side::Buy, 100, 10))
            .expect("valid order should be accepted");

        let trades = book
            .modify_order(1, 105, 10)
            .expect("modify should succeed");

        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), Some(105));
        assert_eq!(book.resting_order_count(), 1);
    }
}
