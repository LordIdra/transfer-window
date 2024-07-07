use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TorpedoStorageType {
    TorpedoStorage1,
    TorpedoStorage2,
    Hub,
}

impl TorpedoStorageType {
    pub fn ship_types() -> [Self; 2] {
        [
            TorpedoStorageType::TorpedoStorage1,
            TorpedoStorageType::TorpedoStorage2,
        ]
    }

    pub fn capacity(&self) -> usize {
        match self {
            TorpedoStorageType::TorpedoStorage1 => 1,
            TorpedoStorageType::TorpedoStorage2 => 3,
            TorpedoStorageType::Hub => 10,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TorpedoStorage {
    type_: TorpedoStorageType,
    torpedoes: usize,
}

impl TorpedoStorage {
    pub fn new(type_: TorpedoStorageType) -> TorpedoStorage {
        let torpedoes = type_.capacity() / 2;
        TorpedoStorage {
            type_,
            torpedoes
        }
    }
    
    pub fn type_(&self) -> TorpedoStorageType {
        self.type_
    }

    pub fn capacity(&self) -> usize {
        self.type_.capacity()
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