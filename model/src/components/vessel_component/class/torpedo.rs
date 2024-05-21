use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Torpedo {
    guidance_enabled: bool, // TODO remove this
}

impl Torpedo {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self { guidance_enabled: false }
    }

    pub fn guidance_enabled(&self) -> bool {
        self.guidance_enabled
    }

    pub fn enable_guidance(&mut self) {
        self.guidance_enabled = true;
    }
}