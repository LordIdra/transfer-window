use std::collections::HashMap;

use condition::Condition;
use state::{State, StateCreator};
use transfer_window_model::{story_event::StoryEvent, Model};
use transition::Transition;

use crate::game::events::{ModelEvent, ViewEvent};

pub mod action;
pub mod condition;
pub(super) mod state;
pub mod transition;

pub struct Story {
    state_creators: HashMap<&'static str, StateCreator>,
    state: State,
    state_string: &'static str,
}

impl Story {
    pub fn new(root: &'static str) -> Self {
        let state_creators = HashMap::new();
        let state = State::default().transition(Transition::new(root, Condition::none()));
        let state_string = "uninitialized";
        Self { state_creators, state, state_string }
    }

    pub fn empty() -> Self {
        let mut story = Self::new("root");
        story.add("root", |_: &Model| State::default());
        story
    }

    pub(super) fn add(&mut self, name: &'static str, factory: impl Fn(&Model) -> State + 'static) {
        assert!(!self.state_creators.contains_key(name), "Duplicate state {name}");
        self.state_creators.insert(name, StateCreator::new(Box::new(factory)));
    }

    pub fn update(&mut self, model: &Model, events: &Vec<StoryEvent>) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let mut model_events = vec![];
        let mut view_events = vec![];
        if let Some((state_string, objective)) = self.state.try_transition(events) {
            self.state_string = state_string;
            self.state = self.state_creators.get(state_string)
                .unwrap_or_else(|| panic!("State does not exist {state_string}"))
                .create(model);
            
            let (new_model_events, new_view_events) = self.state.trigger();
            model_events.extend(new_model_events);
            view_events.extend(new_view_events);

            if let Some(objective) = objective {
                view_events.push(ViewEvent::FinishObjective(objective));
            }
        }
        (model_events, view_events)
    }
    
    #[cfg(test)]
    pub fn states(&self) -> &HashMap<&'static str, StateCreator> {
        &self.state_creators
    }
}