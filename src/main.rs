use limit_order_book::simulator::{run_scenario, scenarios};
use std::env;

fn main() {
    // Read scenario name from terminal.
    // Example: cargo run -- buy-sweeps-asks
    let scenario_name = env::args()
        .nth(1)
        .unwrap_or_else(|| "simple-cross".to_string());

    // Choose which predefined scenario to run.
    let commands = match scenario_name.as_str() {
        "simple-cross" => scenarios::simple_cross(),
        "buy-sweeps-asks" => scenarios::buy_sweeps_asks(),
        "cancel-and-modify" => scenarios::cancel_and_modify_flow(),
        _ => {
            eprintln!("unknown scenario: {scenario_name}");
            eprintln!("available scenarios:");
            eprintln!("  simple-cross");
            eprintln!("  buy-sweeps-asks");
            eprintln!("  cancel-and-modify");
            std::process::exit(1);
        }
    };

    // Run the selected commands against a fresh order book.
    let result = run_scenario(&commands).expect("scenario should run");
    println!("events:");
    for event in &result.events {
        println!("  {event:?}");
    }

    // Print useful output for demo/debugging.
    println!("scenario: {scenario_name}");
    println!("trades: {:?}", result.trades);
    println!("best bid: {:?}", result.book.best_bid());
    println!("best ask: {:?}", result.book.best_ask());
    println!("resting orders: {}", result.book.resting_order_count());
    println!("snapshot: {:?}", result.book.snapshot());
}