use crate::domain::{Order, Price, Trade};

use super::order_book::OrderBook;

impl OrderBook {
    /// Match an incoming buy order against the cheapest asks first.
    pub(super) fn match_buy_order(&mut self, mut incoming: Order) -> Vec<Trade> {
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
    pub(super) fn match_sell_order(&mut self, mut incoming: Order) -> Vec<Trade> {
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

                // A partially filled resting order keeps its FIFO position.
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

                // A partially filled resting order keeps its FIFO position.
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
