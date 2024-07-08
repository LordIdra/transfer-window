use crate::game::events::{ModelEvent, ViewEvent};

use super::{action::Action, story_event::StoryEvent, transition::Transition};

pub struct State {
    transitions: Vec<Transition>,
    actions: Vec<Box<dyn Action>>,
}

impl Default for State {
    fn default() -> Self {
        let transitions = vec![];
        let actions = vec![];
        State { transitions, actions }
    }
}

impl State {
    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    pub fn with_action(mut self, action: Box<dyn Action>) -> Self {
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