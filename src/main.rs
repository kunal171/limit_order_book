use limit_order_book::simulator::{run_scenario, scenarios};
use limit_order_book::{
    calculate_book_metrics, load_events_from_file, replay_events, save_events_to_file,
};
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

    let save_events_path = args
        .windows(2)
        .find(|window| window[0] == "--save-events")
        .map(|window| window[1].clone());

    let replay_events_path = args
        .windows(2)
        .find(|window| window[0] == "--replay-events")
        .map(|window| window[1].clone());

    if let Some(path) = replay_events_path {
        // Load previously saved events from disk.
        let events = load_events_from_file(&path).expect("failed to load events");

        // Rebuild the order book from the event stream.
        let book = replay_events(&events).expect("failed to replay events");
        //Calculate Metrics
        let metrics = calculate_book_metrics(&book.snapshot());

        println!("replayed events from: {path}");
        println!("best bid: {:?}", book.best_bid());
        println!("best ask: {:?}", book.best_ask());
        println!("resting orders: {}", book.resting_order_count());
        println!("snapshot: {:?}", book.snapshot());
        println!("metrics: {:?}", metrics);
        return;
    }

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
    let metrics = calculate_book_metrics(&result.book.snapshot());

    if let Some(path) = &save_events_path {
        save_events_to_file(&result.events, path).expect("failed to save events");

        // Keep JSON mode clean so tools can parse stdout.
        if !output_json {
            println!("saved events to: {path}");
        }
    }

    if output_json {
        let output = json!({
            "scenario" : scenario_name,
            "events" : &result.events,
            "trades" : &result.trades,
            "best_bid" : &result.book.best_bid(),
            "best_ask" : &result.book.best_ask(),
            "resting_orders": &result.book.resting_order_count(),
            "snapshot": &result.book.snapshot(),
            "metrics": {
                "best_bid": metrics.best_bid,
                "best_ask": metrics.best_ask,
                "spread": metrics.spread,
                "mid_price": metrics.mid_price,
                "total_bid_quantity": metrics.total_bid_quantity,
                "total_ask_quantity": metrics.total_ask_quantity,
                "bid_price_levels": metrics.bid_price_levels,
                "ask_price_levels": metrics.ask_price_levels,
            },
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
    println!("metrics: {:?}", metrics);
}
