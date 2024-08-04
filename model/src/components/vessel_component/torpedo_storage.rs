use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TorpedoStorage {
    capacity: usize,
    torpedoes: usize,
}

impl TorpedoStorage {
    pub fn new(capacity: usize) -> TorpedoStorage {
        let torpedoes = capacity;
        TorpedoStorage { capacity, torpedoes }
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn torpedoes(&self) -> usize {
        self.torpedoes
    }

    pub fn increment(&mut self) {
        self.torpedoes += 1;
        assert!(self.torpedoes <= self.capacity());
    }

    pub fn decrement(&mut self) {
        self.torpedoes -= 1;
    }
}