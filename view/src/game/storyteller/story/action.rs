use crate::game::events::{ModelEvent, ViewEvent};

pub trait Action {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>);
}

pub struct ShowDialogueAction{
    character: &'static str,
    text: &'static str
}

impl ShowDialogueAction {
    pub fn new(character: &'static str, text: &'static str) -> Box<dyn Action> {
        Box::new(Self { character, text })
    }
}

impl Action for ShowDialogueAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ViewEvent::ShowDialogue { character: self.character, text: self.text };
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