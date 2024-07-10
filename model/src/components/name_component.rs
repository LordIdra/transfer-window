use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NameComponent {
    name: String,
}

impl NameComponent {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
