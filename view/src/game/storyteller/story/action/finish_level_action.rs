use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct FinishLevelAction {
    level: String,
}

impl FinishLevelAction {
    pub fn new(level: String) -> Box<dyn Action> {
        Box::new(Self { level })
    }
}

impl Action for FinishLevelAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ViewEvent::FinishLevel(self.level.clone());
        (vec![], vec![event])
    }
}