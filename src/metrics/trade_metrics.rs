use crate::{Price, Quantity, Trade};

/// Summary of trades produced by a scenario.
///
/// VWAP means volume-weighted average price:
/// total traded value / total traded quantity.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeMetrics {
    pub trade_count: usize,
    pub total_traded_quantity: Quantity,
    pub total_notional: u128,
    pub last_trade_price: Option<Price>,
    pub vwap: Option<f64>,
}

/// Calculate execution metrics from generated trades.
pub fn calculate_trade_metrics(trades: &[Trade]) -> TradeMetrics {
    let trade_count = trades.len();

    let total_traded_quantity = trades.iter().map(|trade| trade.quantity).sum();

    let total_notional = trades
        .iter()
        .map(|trade| trade.price as u128 * trade.quantity as u128)
        .sum();

    let last_trade_price = trades.last().map(|trade| trade.price);

    let vwap = if total_traded_quantity == 0 {
        None
    } else {
        Some(total_notional as f64 / total_traded_quantity as f64)
    };

    TradeMetrics {
        trade_count,
        total_traded_quantity,
        total_notional,
        last_trade_price,
        vwap,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_trade_metrics_for_multiple_trades() {
        let trades = vec![
            Trade::new(1, 4, 100, 5),
            Trade::new(2, 4, 101, 5),
            Trade::new(3, 4, 102, 2),
        ];

        let metrics = calculate_trade_metrics(&trades);

        assert_eq!(metrics.trade_count, 3);
        assert_eq!(metrics.total_traded_quantity, 12);
        assert_eq!(metrics.total_notional, 1209);
        assert_eq!(metrics.last_trade_price, Some(102));
        assert_eq!(metrics.vwap, Some(100.75));
    }

    #[test]
    fn returns_empty_trade_metrics_when_no_trades_exist() {
        let metrics = calculate_trade_metrics(&[]);

        assert_eq!(metrics.trade_count, 0);
        assert_eq!(metrics.total_traded_quantity, 0);
        assert_eq!(metrics.total_notional, 0);
        assert_eq!(metrics.last_trade_price, None);
        assert_eq!(metrics.vwap, None);
    }
}
