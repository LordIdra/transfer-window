use transfer_window_model::story_event::StoryEvent;

use crate::game::View;

use super::condition::Condition;
use super::transition::Transition;

pub struct StateCreator {
    factory: Box<dyn Fn(&View) -> State>,
}

impl StateCreator {
    pub fn new(factory: Box<dyn Fn(&View) -> State>) -> Self {
        Self { factory }
    }

    pub fn create(&self, view: &View) -> State {
        (self.factory)(view)
    }
}

#[derive(Default)]
pub struct State {
    transition: Option<Transition>,
}


impl State {
    pub fn new(to: &'static str, condition: Condition) -> Self {
        let transition = Transition::new(to, condition);
        Self { transition: Some(transition) }
    }

    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }

    pub fn try_transition(&self, events: &Vec<StoryEvent>) -> Option<(&'static str, Option<&'static str>)> {
        if self.transition.as_ref().is_some_and(|transition| transition.can_transition(events)) {
            Some((self.transition.as_ref().unwrap().to(), self.transition.as_ref().unwrap().objective()))
        } else {
            None
        }
    }

    pub fn get_transition(&self) -> &Option<Transition> {
        &self.transition
    }
}