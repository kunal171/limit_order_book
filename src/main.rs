use limit_order_book::{Order, OrderBook, Side};

fn main() {
    // This binary is only a small demo. The real matching logic lives in the library.
    let mut book = OrderBook::new();

    // Add one resting buy order.
    book.add_order(Order::new(1, Side::Buy, 100, 10))
        .expect("demo buy order should be accepted");

    // This sell order crosses the buy price, so it creates a trade.
    let trades = book
        .add_order(Order::new(2, Side::Sell, 99, 4))
        .expect("demo sell order should be accepted");

    println!("trades: {trades:?}");
    println!("best bid: {:?}", book.best_bid());
    println!("best ask: {:?}", book.best_ask());
}
