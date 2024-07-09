use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct CloseDialogueAction;

impl CloseDialogueAction {
    pub fn new() -> Box<dyn Action> {
        Box::new(Self {})
    }
}

impl Action for CloseDialogueAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        (vec![], vec![ViewEvent::CloseDialogue])
    }
}