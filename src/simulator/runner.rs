use crate::simulator::ScenarioCommand;
use crate::{BookEvent, OrderBook, OrderBookError, Trade};

/// Result of running a simulator scenario.
///
/// `book` is the final order book state.
/// `trades` contains all trades produced while running the scenario.
pub struct ScenarioResult {
    pub book: OrderBook,
    pub trades: Vec<Trade>,
    pub events: Vec<BookEvent>
}

//Run a list of simulator commands against a fresh order book.
pub fn run_scenario(commands: &[ScenarioCommand]) -> Result<ScenarioResult, OrderBookError> {
    let mut book = OrderBook::new();
    let mut all_trades = Vec::new();

    for command in commands {
        match command {
            ScenarioCommand::Add(order) => {
                let trades = book.add_order(order.clone())?;
                all_trades.extend(trades);
            }
            ScenarioCommand::Cancel { order_id } => {
                book.cancel_order(*order_id)?;
            }
            ScenarioCommand::Modify {
                order_id,
                new_price,
                new_quantity,
            } => {
                let trades = book.modify_order(*order_id, *new_price, *new_quantity)?;
                all_trades.extend(trades);
            }
        }
    }
    let events = book.events().to_vec();

    Ok(ScenarioResult {
        book,
        trades: all_trades,
        events
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Order, Side, Trade};

    #[test]
    fn run_scenario_collects_trades_and_final_book_state() {
        let commands = vec![
            ScenarioCommand::Add(Order::new(1, Side::Sell, 100, 5)),
            ScenarioCommand::Add(Order::new(2, Side::Buy, 100, 2)),
        ];

        let result = run_scenario(&commands).expect("scenario should run");

        assert_eq!(result.trades, vec![Trade::new(1, 2, 100, 2)]);
        assert_eq!(result.book.best_ask(), Some(100));
        assert_eq!(result.book.resting_order_count(), 1);
        assert_eq!(result.events.len(), 3);
    }
}
