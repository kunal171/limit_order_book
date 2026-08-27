use limit_order_book::simulator::{
    GeneratorConfig, generate_crossing_orders, generate_two_sided_orders, run_scenario, scenarios,
};
use limit_order_book::{
    BookEvent, calculate_book_metrics, calculate_trade_metrics, load_events_from_file,
    replay_events, save_events_to_file,
};
use serde_json::json;
use std::{
    env,
    fs::{self, File},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Read scenario name from terminal.
    // Example: cargo run -- buy-sweeps-asks
    let scenario_name = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "simple-cross".to_string());

    let output_json = args.iter().any(|arg| arg == "--json");

    // Optional synthetic order count.
    // Example: cargo run -- synthetic --count 100
    let synthetic_count = args
        .windows(2)
        .find(|window| window[0] == "--count")
        .and_then(|window| window[1].parse::<usize>().ok())
        .unwrap_or(100);

    let save_events_path = args
        .windows(2)
        .find(|window| window[0] == "--save-events")
        .map(|window| window[1].clone());

    let replay_events_path = args
        .windows(2)
        .find(|window| window[0] == "--replay-events")
        .map(|window| window[1].clone());

    // Optional output directory for run artifacts.
    // Example:
    // cargo run -- synthetic-crossing --count 100 --output-dir runs/run-001
    let output_dir = args
        .windows(2)
        .find(|window| window[0] == "--output-dir")
        .map(|window| window[1].clone());

    if let Some(path) = replay_events_path {
        // Load previously saved events from disk.
        let events = load_events_from_file(&path).expect("failed to load events");

        // Rebuild the order book from the event stream.
        let book = replay_events(&events).expect("failed to replay events");
        //Calculate Book Metrics
        let book_metrics = calculate_book_metrics(&book.snapshot());


        //Fetch Trades from the events
        let trades = events
            .iter()
            .filter_map(|event| match event {
                BookEvent::TradeExecuted { trade } => Some(trade.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        //Get trade Metrics
        let trade_metrics = calculate_trade_metrics(&trades);

        println!("replayed events from: {path}");
        println!("best bid: {:?}", book.best_bid());
        println!("best ask: {:?}", book.best_ask());
        println!("resting orders: {}", book.resting_order_count());
        println!("snapshot: {:?}", book.snapshot());
        println!("book metrics: {:?}", book_metrics);
        println!("trade metrics: {:?}", trade_metrics);
        return;
    }

    // Choose which predefined scenario to run.
    let commands = match scenario_name.as_str() {
        "simple-cross" => scenarios::simple_cross(),
        "buy-sweeps-asks" => scenarios::buy_sweeps_asks(),
        "cancel-and-modify" => scenarios::cancel_and_modify_flow(),
        "two-sided-book" => scenarios::two_sided_book(),
        "synthetic" => generate_two_sided_orders(GeneratorConfig {
            order_count: synthetic_count,
            start_order_id: 1,
            base_price: 100,
            tick_size: 1,
            price_levels: 10,
            quantity: 5,
        }),
        "synthetic-crossing" => generate_crossing_orders(GeneratorConfig {
            order_count: synthetic_count,
            start_order_id: 1,
            base_price: 100,
            tick_size: 1,
            price_levels: 10,
            quantity: 5,
        }),

        _ => {
            eprintln!("unknown scenario: {scenario_name}");
            eprintln!("available scenarios:");
            eprintln!("  simple-cross");
            eprintln!("  buy-sweeps-asks");
            eprintln!("  cancel-and-modify");
            eprintln!("  two-sided-book");
            eprintln!("  synthetic");
            eprintln!("  synthetic-crossing");
            std::process::exit(1);
        }
    };

    // Run the selected commands against a fresh order book.
    let result = run_scenario(&commands).expect("scenario should run");
    let book_metrics = calculate_book_metrics(&result.book.snapshot());
    let trade_metrics = calculate_trade_metrics(&result.trades);

    if let Some(output_dir) = &output_dir {
        // Create the run directory if it does not exist.
        fs::create_dir_all(output_dir).expect("failed to create output directory");

        //Save all events for replay/Debugging
        save_events_to_file(&result.events, Path::new(output_dir).join("evens.json"))
            .expect("failed to save events artifacts");

         // Save final book snapshot.
        write_json_file(
            Path::new(output_dir).join("snapshot.json"),
            &result.book.snapshot(),
        ).expect("failed to save snapshot artifacts");

        // Save small summary that Windmill/CI/AI can read quickly.
        let summary = json!({
            "scenario": scenario_name,
            "order_count": synthetic_count,
            "best_bid": result.book.best_bid(),
            "best_ask": result.book.best_ask(),
            "resting_orders": result.book.resting_order_count(),
            "book_metrics": {
                "best_bid": book_metrics.best_bid,
                "best_ask": book_metrics.best_ask,
                "spread": book_metrics.spread,
                "mid_price": book_metrics.mid_price,
                "total_bid_quantity": book_metrics.total_bid_quantity,
                "total_ask_quantity": book_metrics.total_ask_quantity,
                "bid_price_levels": book_metrics.bid_price_levels,
                "ask_price_levels": book_metrics.ask_price_levels,
                "imbalance": book_metrics.imbalance,
            },
            "trade_metrics": {
                "trade_count": trade_metrics.trade_count,
                "total_traded_quantity": trade_metrics.total_traded_quantity,
                "total_notional": trade_metrics.total_notional,
                "last_trade_price": trade_metrics.last_trade_price,
                "vwap": trade_metrics.vwap,
            }
        });

        write_json_file(Path::new(output_dir).join("summary.json"), &summary)
            .expect("failed to save summary artifact");

        if !output_json {
            println!("saved run artifacts to: {output_dir}");
        }
    }

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
            "book_metrics": {
                "best_bid": book_metrics.best_bid,
                "best_ask": book_metrics.best_ask,
                "spread": book_metrics.spread,
                "mid_price": book_metrics.mid_price,
                "total_bid_quantity": book_metrics.total_bid_quantity,
                "total_ask_quantity": book_metrics.total_ask_quantity,
                "bid_price_levels": book_metrics.bid_price_levels,
                "ask_price_levels": book_metrics.ask_price_levels,
                "imbalance": book_metrics.imbalance
            },

            "trade_metrics": {
                "trade_count": trade_metrics.trade_count,
                "total_traded_quantity": trade_metrics.total_traded_quantity,
                "total_notional": trade_metrics.total_notional,
                "last_trade_price": trade_metrics.last_trade_price,
                "vwap": trade_metrics.vwap,
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
    println!("metrics: {:?}", book_metrics);
    println!("trade metrics: {:?}", trade_metrics);
}


fn write_json_file<T: serde::Serialize>(
    path: impl AsRef<Path>,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the output file.
    let file = File::create(path)?;

    // Write pretty JSON so humans and tools can read it.
    serde_json::to_writer_pretty(file, value)?;

    Ok(())
}