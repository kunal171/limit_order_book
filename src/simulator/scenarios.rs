use crate::simulator::ScenarioCommand;
use crate::{Order, Side};

/// Simple scenario:
/// One sell order rests first, then one buy order matches it.
pub fn simple_cross() -> Vec<ScenarioCommand> {
    vec![
        ScenarioCommand::Add(Order::new(1, Side::Sell, 100, 5)),
        ScenarioCommand::Add(Order::new(2, Side::Buy, 100, 2)),
    ]
}

/// A buy order walks through multiple ask price levels.
///
/// Important market concept:
/// A buy order matches from the cheapest sell price first.
pub fn buy_sweeps_asks() -> Vec<ScenarioCommand> {
    vec![
        ScenarioCommand::Add(Order::new(1, Side::Sell, 100, 5)),
        ScenarioCommand::Add(Order::new(2, Side::Sell, 101, 5)),
        ScenarioCommand::Add(Order::new(3, Side::Sell, 102, 5)),
        ScenarioCommand::Add(Order::new(4, Side::Buy, 102, 12)),
    ]
}

/// Scenario with both sides resting.
///
/// This is useful for market-data metrics because spread and mid price
/// only exist when both best bid and best ask are present.
pub fn two_sided_book() -> Vec<ScenarioCommand> {
    vec![
        ScenarioCommand::Add(Order::new(1, Side::Buy, 100, 10)),
        ScenarioCommand::Add(Order::new(2, Side::Buy, 99, 5)),
        ScenarioCommand::Add(Order::new(3, Side::Sell, 105, 7)),
        ScenarioCommand::Add(Order::new(4, Side::Sell, 106, 3)),
    ]
}

/// Scenario that checks normal lifecycle:
/// add order -> cancel order -> modify order.
pub fn cancel_and_modify_flow() -> Vec<ScenarioCommand> {
    vec![
        ScenarioCommand::Add(Order::new(1, Side::Buy, 100, 10)),
        ScenarioCommand::Add(Order::new(2, Side::Buy, 101, 5)),
        ScenarioCommand::Cancel { order_id: 1 },
        ScenarioCommand::Modify {
            order_id: 2,
            new_price: 103,
            new_quantity: 4,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Trade;
    use crate::simulator::run_scenario;

    #[test]
    fn buy_sweep_scenario_produces_expected_trades() {
        let result = run_scenario(&buy_sweeps_asks()).expect("scenario should run");

        assert_eq!(
            result.trades,
            vec![
                Trade::new(1, 4, 100, 5),
                Trade::new(2, 4, 101, 5),
                Trade::new(3, 4, 102, 2),
            ]
        );

        assert_eq!(result.book.best_ask(), Some(102));
        assert_eq!(result.book.resting_order_count(), 1);
    }
}
