use crate::BookEvent;
use std::{fs::File, path::Path};

pub fn save_events_to_file(
    events: &[BookEvent],
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, events)?;
    Ok(())
}

pub fn load_events_from_file(
    path: impl AsRef<Path>,
) -> Result<Vec<BookEvent>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let events = serde_json::from_reader(file)?;
    Ok(events)
}

#[test]
fn save_and_load_events_round_trips() {
    let mut book = crate::OrderBook::new();

    book.add_order(crate::Order::new(1, crate::Side::Buy, 100, 10))
        .expect("order should be accepted");

    let path = std::env::temp_dir().join("limit_order_book_events_test.json");

    save_events_to_file(book.events(), &path).expect("save should succeed");
    let loaded = load_events_from_file(&path).expect("load should succeed");

    assert_eq!(loaded, book.events());
}
