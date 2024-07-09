use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct FinishLevelAction;

impl FinishLevelAction {
    pub fn new() -> Box<dyn Action> {
        Box::new(Self {})
    }
}

impl Action for FinishLevelAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ViewEvent::FinishLevel;
        (vec![], vec![event])
    }
}