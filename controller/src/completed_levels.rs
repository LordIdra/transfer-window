use std::{collections::HashSet, fs, path::Path};

use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CompletedLevels(HashSet<String>);

impl CompletedLevels {
    pub fn load() -> Self {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Load completed levels");
        if !Path::new("data/completed_levels.json").exists() {
            Self::default().save();
        }

        let serialized = fs::read_to_string("data/completed_levels.json");
        let Ok(serialized) = serialized else {
            error!("FAILED TO LOAD COMPLETED LEVELS: {}", serialized.err().unwrap().to_string());
            return Self::default();
        };

        match serde_json::from_str(&serialized) {
            Ok(completed_levels) => completed_levels,
            Err(error) => {
                error!("FAILED TO DESERIALIZE COMPLETED LEVELS: {}", error.to_string());
                Self::default()
            },
        }
    }

    pub fn save(&self) {
        let path = &Path::new("data/completed_levels.json");
        let serialized = serde_json::to_string(&self).expect("Failed to serialize default CompletedLevels");
        if let Err(err) = fs::write(path, serialized) {
            error!("FAILED TO WRITE DEFAULT COMPLETED LEVELS: {}", err.to_string());
        }
    }

    pub fn add(&mut self, level: String) {
        self.0.insert(level);
    }

    pub fn get(&self) -> &HashSet<String> {
        &self.0
    }
}
