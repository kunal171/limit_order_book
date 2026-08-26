use crate::simulator::ScenarioCommand;
use crate::{Order, OrderId, Price, Quantity, Side};

/// Configuration for generating synthetic orders.
///
/// This gives us repeatable fake market data for testing,
/// metrics, replay, and later benchmarks.
///
#[derive(Debug, Copy, Clone)]
pub struct GeneratorConfig {
    pub order_count: usize,
    pub start_order_id: OrderId,
    pub base_price: Price,
    pub tick_size: Price,
    pub price_levels: usize,
    pub quantity: Quantity,
}

/// Generate deterministic buy/sell orders around a base price.
///
/// Buys are placed below base price.
/// Sells are placed above base price.
/// This avoids accidental crossing and builds a visible two-sided book.
pub fn generate_two_sided_orders(config: GeneratorConfig) -> Vec<ScenarioCommand> {
    let price_levels = config.price_levels.max(1);

    (0..config.order_count)
        .map(|index| {
            let side = if index % 2 == 0 {
                Side::Buy
            } else {
                Side::Sell
            };

            let order_id = config.start_order_id + index as u64;
            let level = (index % price_levels) as u64 + 1;

            let price = match side {
                Side::Buy => config.base_price.saturating_sub(level * config.tick_size),
                Side::Sell => config.base_price + level * config.tick_size,
            };

            ScenarioCommand::Add(Order::new(order_id, side, price, config.quantity))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_expected_number_of_orders() {
        let commands = generate_two_sided_orders(GeneratorConfig {
            order_count: 10,
            start_order_id: 1,
            base_price: 100,
            tick_size: 1,
            price_levels: 3,
            quantity: 5,
        });

        assert_eq!(commands.len(), 10);
    }

    #[test]
    fn generated_orders_are_deterministic() {
        let config = GeneratorConfig {
            order_count: 4,
            start_order_id: 1,
            base_price: 100,
            tick_size: 1,
            price_levels: 2,
            quantity: 5,
        };

        let first = generate_two_sided_orders(config);
        let second = generate_two_sided_orders(config);

        assert_eq!(first, second);
    }
}
