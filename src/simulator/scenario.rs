use crate::{Order, OrderId, Price, Quantity};

/// A command that the simulator sends into the order book.
///
/// Important:
/// This is input to the engine.
/// BookEvent is output from the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioCommand {
    Add(Order),

    Cancel {
        order_id: OrderId,
    },

    Modify {
        order_id: OrderId,
        new_price: Price,
        new_quantity: Quantity,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Side;

    #[test]
    fn scenario_command_can_hold_add_order() {
        let command = ScenarioCommand::Add(Order::new(1, Side::Buy, 100, 10));

        assert_eq!(
            command,
            ScenarioCommand::Add(Order::new(1, Side::Buy, 100, 10))
        );
    }
}
