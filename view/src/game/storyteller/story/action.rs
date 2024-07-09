use crate::game::{events::{ModelEvent, ViewEvent}, overlay::dialogue::Dialogue};

pub trait Action {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>);
}

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