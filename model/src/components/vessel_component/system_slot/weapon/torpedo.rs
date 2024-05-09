use serde::{Deserialize, Serialize};


const STOCKPILE: usize = 3;
const COOLDOWN: f64 = 3600.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Torpedo {
    stockpile: usize,
    cooldown: f64,
}

impl Torpedo {
    pub fn new() -> Self {
        Self { stockpile: STOCKPILE, cooldown: 0.0 }
    }

    pub fn deplete(&mut self) {
        self.stockpile -= 1;
    }

    pub fn max_stockpile(&self) -> usize {
        STOCKPILE
    }

    pub fn stockpile(&self) -> usize {
        self.stockpile
    }
}