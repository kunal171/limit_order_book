use std::collections::{BTreeMap, VecDeque};

use crate::order::Order;
use crate::trade::Trade;
use crate::types::{Price, Side};

/// A simple price-time priority limit order book.
///
/// `BTreeMap` keeps price levels sorted.
/// `VecDeque` preserves FIFO order inside each price level.
#[derive(Debug, Default)]
pub struct OrderBook {
    bids: BTreeMap<Price, VecDeque<Order>>,
    asks: BTreeMap<Price, VecDeque<Order>>,
}

impl OrderBook {
    /// Create an empty book.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an order and return all trades caused by that order.
    pub fn add_order(&mut self, order: Order) -> Vec<Trade> {
        match order.side {
            Side::Buy => self.match_buy_order(order),
            Side::Sell => self.match_sell_order(order),
        }
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

    /// Match an incoming buy order against the cheapest asks first.
    fn match_buy_order(&mut self, mut incoming: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        while !incoming.is_filled() {
            let Some(best_ask_price) = self.best_ask() else {
                break;
            };

            // A buy can only trade with asks priced at or below its limit price.
            if best_ask_price > incoming.price {
                break;
            }

            self.match_at_ask_level(best_ask_price, &mut incoming, &mut trades);
        }

        // Any unfilled quantity becomes a resting bid.
        if !incoming.is_filled() {
            self.bids
                .entry(incoming.price)
                .or_default()
                .push_back(incoming);
        }

        trades
    }

    /// Match an incoming sell order against the most expensive bids first.
    fn match_sell_order(&mut self, mut incoming: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        while !incoming.is_filled() {
            let Some(best_bid_price) = self.best_bid() else {
                break;
            };

            // A sell can only trade with bids priced at or above its limit price.
            if best_bid_price < incoming.price {
                break;
            }

            self.match_at_bid_level(best_bid_price, &mut incoming, &mut trades);
        }

        // Any unfilled quantity becomes a resting ask.
        if !incoming.is_filled() {
            self.asks
                .entry(incoming.price)
                .or_default()
                .push_back(incoming);
        }

        trades
    }

    /// Match against one ask price level.
    fn match_at_ask_level(&mut self, price: Price, incoming: &mut Order, trades: &mut Vec<Trade>) {
        let mut should_remove_level = false;

        if let Some(level) = self.asks.get_mut(&price) {
            while !incoming.is_filled() {
                let Some(mut resting) = level.pop_front() else {
                    break;
                };

                let traded_qty = incoming.remaining_qty.min(resting.remaining_qty);
                incoming.remaining_qty -= traded_qty;
                resting.remaining_qty -= traded_qty;

                trades.push(Trade::new(resting.id, incoming.id, price, traded_qty));

                // If the resting order is not filled, it keeps its FIFO position at the front.
                if !resting.is_filled() {
                    level.push_front(resting);
                    break;
                }
            }

            should_remove_level = level.is_empty();
        }

        if should_remove_level {
            self.asks.remove(&price);
        }
    }

    /// Match against one bid price level.
    fn match_at_bid_level(&mut self, price: Price, incoming: &mut Order, trades: &mut Vec<Trade>) {
        let mut should_remove_level = false;

        if let Some(level) = self.bids.get_mut(&price) {
            while !incoming.is_filled() {
                let Some(mut resting) = level.pop_front() else {
                    break;
                };

                let traded_qty = incoming.remaining_qty.min(resting.remaining_qty);
                incoming.remaining_qty -= traded_qty;
                resting.remaining_qty -= traded_qty;

                trades.push(Trade::new(resting.id, incoming.id, price, traded_qty));

                // If the resting order is not filled, it keeps its FIFO position at the front.
                if !resting.is_filled() {
                    level.push_front(resting);
                    break;
                }
            }

            should_remove_level = level.is_empty();
        }

        if should_remove_level {
            self.bids.remove(&price);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Side;

    #[test]
    fn buy_order_rests_when_there_is_no_matching_ask() {
        let mut book = OrderBook::new();

        let trades = book.add_order(Order::new(1, Side::Buy, 100, 10));

        assert!(trades.is_empty());
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn sell_order_matches_existing_bid() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 100, 10));

        let trades = book.add_order(Order::new(2, Side::Sell, 99, 4));

        assert_eq!(trades, vec![Trade::new(1, 2, 100, 4)]);
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.resting_order_count(), 1);
    }

    #[test]
    fn orders_at_same_price_match_fifo() {
        let mut book = OrderBook::new();
        book.add_order(Order::new(1, Side::Buy, 100, 5));
        book.add_order(Order::new(2, Side::Buy, 100, 5));

        let trades = book.add_order(Order::new(3, Side::Sell, 100, 8));

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
        book.add_order(Order::new(1, Side::Buy, 99, 5));
        book.add_order(Order::new(2, Side::Buy, 101, 5));

        let trades = book.add_order(Order::new(3, Side::Sell, 99, 5));

        assert_eq!(trades, vec![Trade::new(2, 3, 101, 5)]);
        assert_eq!(book.best_bid(), Some(99));
    }
}
