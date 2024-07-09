use crate::game::{events::{ModelEvent, ViewEvent}, overlay::dialogue::Dialogue};

use super::Action;

pub struct ShowDialogueAction{
    dialogue: Dialogue,
}

impl ShowDialogueAction {
    pub fn new(dialogue: Dialogue) -> Box<dyn Action> {
        Box::new(Self { dialogue })
    }
}

impl Action for ShowDialogueAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ViewEvent::ShowDialogue(self.dialogue.clone());
        (vec![], vec![event])
    }
}