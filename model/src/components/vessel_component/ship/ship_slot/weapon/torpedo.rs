use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Torpedo {
    max_stockpile: usize,
    stockpile: usize,
}

impl Torpedo {
    #[allow(clippy::new_without_default)]
    pub fn new(max_stockpile: usize) -> Self {
        let stockpile = max_stockpile;
        Self { max_stockpile, stockpile }
    }

    pub fn decrement_stockpile(&mut self) {
        self.stockpile -= 1;
    }

    pub fn increment_stockpile(&mut self) {
        self.stockpile += 1;
        assert!(self.stockpile <= self.max_stockpile);
    }

    pub fn max_stockpile(&self) -> usize {
        self.max_stockpile
    }

    pub fn stockpile(&self) -> usize {
        self.stockpile
    }
}