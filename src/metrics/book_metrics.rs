use crate::{BookSnapshot, Price, Quantity};

/// Read-only market data summary derived from a book snapshot.
///
/// Important:
/// This is analytics/output data, not matching logic.
/// The matching engine should stay focused only on order matching.
#[derive(Debug, Clone, PartialEq)]
pub struct BookMetrics {
    pub best_bid: Option<Price>,
    pub best_ask: Option<Price>,
    pub spread: Option<Price>,
    pub mid_price: Option<f64>,
    pub total_bid_quantity: Quantity,
    pub total_ask_quantity: Quantity,
    pub bid_price_levels: usize,
    pub ask_price_levels: usize,
}

///
/// Calculate market data metrics from a snapshot.
///
pub fn calculate_book_metrics(snapshot: &BookSnapshot) -> BookMetrics {
    let best_bid = snapshot.bids.first().map(|(price, _orders)| *price);
    let best_ask = snapshot.asks.first().map(|(price, _orders)| *price);

    let spread = match (best_bid, best_ask) {
        (Some(bid), Some(ask)) if ask >= bid => Some(ask - bid),
        _ => None,
    };

    let mid_price = match (best_bid, best_ask) {
        (Some(bid), Some(ask)) => Some((bid as f64 + ask as f64) / 2.0),
        _ => None,
    };

    let total_bid_quantity = snapshot
        .bids
        .iter()
        .flat_map(|(_price, orders)| orders)
        .map(|order| order.remaining_qty)
        .sum();

    let total_ask_quantity = snapshot
        .asks
        .iter()
        .flat_map(|(_price, orders)| orders)
        .map(|order| order.remaining_qty)
        .sum();

    BookMetrics {
        best_bid,
        best_ask,
        spread,
        mid_price,
        total_bid_quantity,
        total_ask_quantity,
        bid_price_levels: snapshot.bids.len(),
        ask_price_levels: snapshot.asks.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Order, OrderBook, Side};

    #[test]
    fn calculates_metrics_for_book_with_bids_and_asks() {
        let mut book = OrderBook::new();

        // Two bid levels: best bid should be highest price.
        book.add_order(Order::new(1, Side::Buy, 100, 10)).unwrap();
        book.add_order(Order::new(2, Side::Buy, 99, 5)).unwrap();

        // Two ask levels: best ask should be lowest price.
        book.add_order(Order::new(3, Side::Sell, 105, 7)).unwrap();
        book.add_order(Order::new(4, Side::Sell, 106, 3)).unwrap();

        let metrics = calculate_book_metrics(&book.snapshot());

        assert_eq!(metrics.best_bid, Some(100));
        assert_eq!(metrics.best_ask, Some(105));
        assert_eq!(metrics.spread, Some(5));
        assert_eq!(metrics.mid_price, Some(102.5));
        assert_eq!(metrics.total_bid_quantity, 15);
        assert_eq!(metrics.total_ask_quantity, 10);
        assert_eq!(metrics.bid_price_levels, 2);
        assert_eq!(metrics.ask_price_levels, 2);
    }

    #[test]
    fn spread_and_mid_price_are_none_when_one_side_is_empty() {
        let mut book = OrderBook::new();

        // Only bid side exists, so there is no real market spread yet.
        book.add_order(Order::new(1, Side::Buy, 100, 10)).unwrap();

        let metrics = calculate_book_metrics(&book.snapshot());

        assert_eq!(metrics.best_bid, Some(100));
        assert_eq!(metrics.best_ask, None);
        assert_eq!(metrics.spread, None);
        assert_eq!(metrics.mid_price, None);
        assert_eq!(metrics.total_bid_quantity, 10);
        assert_eq!(metrics.total_ask_quantity, 0);
    }
}
