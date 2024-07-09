use std::collections::HashMap;

use condition::NoneCondition;
use state::State;
use story_event::StoryEvent;
use transition::Transition;

use crate::game::events::{ModelEvent, ViewEvent};

pub mod action;
pub mod condition;
pub mod state;
pub mod story_event;
pub mod transition;

pub struct Story {
    states: HashMap<&'static str, State>,
    state: &'static str,
}

impl Story {
    pub fn new(root: &'static str) -> Self {
        let mut states = HashMap::new();
        states.insert("uninitialized", State::new("uninitialized").transition(Transition::new(root, NoneCondition::new())));
        let state = "uninitialized";
        Self { states, state }
    }

    pub(super) fn add(&mut self, state: State) {
        self.states.insert(state.name(), state);
    }

    fn state(&self) -> &State {
        self.states.get(self.state).unwrap()
    }

    pub fn update(&mut self, events: &Vec<StoryEvent>) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        if let Some(new_state) = self.state().try_transition(events) {
            self.state = new_state;
            return self.state().trigger();
        }
        (vec![], vec![])
    }
    
    #[cfg(test)]
    pub fn states(&self) -> &HashMap<&'static str, State> {
        &self.states
    }
}