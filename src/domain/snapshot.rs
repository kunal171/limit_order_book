use crate::domain::{Order, Price};
use serde::{Deserialize, Serialize};

/// Full readable state of the order book.
///
/// This is used for tests, replay verification, debugging, and later APIs.
/// We keep it separate from the internal BTreeMap/VecDeque structure so callers
/// do not depend on our private storage layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookSnapshot {
    pub bids: Vec<(Price, Vec<Order>)>,
    pub asks: Vec<(Price, Vec<Order>)>,
}
