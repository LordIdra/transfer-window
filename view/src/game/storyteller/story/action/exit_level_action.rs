use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct ExitLevelAction;

impl ExitLevelAction {
    pub fn new() -> Box<dyn Action> {
        Box::new(Self)
    }
}

impl Action for ExitLevelAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ViewEvent::ExitLevel;
        (vec![], vec![event])
    }
}