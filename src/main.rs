use limit_order_book::simulator::{run_scenario, scenarios};
use serde_json::json;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Read scenario name from terminal.
    // Example: cargo run -- buy-sweeps-asks
    let scenario_name = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "simple-cross".to_string());

    let output_json = args.iter().any(|arg| arg == "--json");

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

    if output_json {
        let output = json!({
            "scenario" : scenario_name,
            "events" : &result.events,
            "trades" : &result.trades,
            "best_bid" : &result.book.best_bid(),
            "best_ask" : &result.book.best_ask(),
            "resting_orders": &result.book.resting_order_count(),
            "snapshot": &result.book.snapshot(),
        });

        println!(
            "{}",
            serde_json::to_string_pretty(&output).expect("output should serialize to json")
        );

        return;
    }

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
