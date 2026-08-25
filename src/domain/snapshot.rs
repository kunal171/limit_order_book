use crate::domain::{Order, Price};

/// Full readable state of the order book.
///
/// This is used for tests, replay verification, debugging, and later APIs.
/// We keep it separate from the internal BTreeMap/VecDeque structure so callers
/// do not depend on our private storage layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookSnapshot {
    pub bids: Vec<(Price, Vec<Order>)>,
    pub asks: Vec<(Price, Vec<Order>)>,
}
