use crate::game::events::{ModelEvent, ViewEvent};

use super::{action::Action, story_event::StoryEvent, transition::Transition};

pub struct State {
    name: &'static str,
    transitions: Vec<Transition>,
    actions: Vec<Box<dyn Action>>,
}


impl State {
    pub fn new(name: &'static str) -> Self {
        let transitions = vec![];
        let actions = vec![];
        State { name, transitions, actions }
    }

    pub fn transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    pub fn action(mut self, action: Box<dyn Action>) -> Self {
        self.actions.push(action);
        self
    }

    pub fn try_transition(&self, events: &Vec<StoryEvent>) -> Option<&'static str> {
        self.transitions.iter().find_map(|transition| transition.try_transition(events))
    }
    
    #[cfg(test)]
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let mut model_events = vec![];
        let mut view_events = vec![];
        for action in &self.actions {
            let (new_model_events, new_view_events) = action.trigger();
            model_events.extend(new_model_events);
            view_events.extend(new_view_events);
        }
        (model_events, view_events)
    }
}