use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Torpedo {
    guidance_enabled: bool,
}

impl Torpedo {
    pub fn default() -> Self {
        Self { guidance_enabled: false }
    }
}