use transfer_window_model::story_event::StoryEvent;
use transfer_window_model::Model;

use super::action::Action;
use super::transition::Transition;
use crate::game::events::{ModelEvent, ViewEvent};

pub struct StateCreator {
    factory: Box<dyn Fn(&Model) -> State>,
}

impl StateCreator {
    pub fn new(factory: Box<dyn Fn(&Model) -> State>) -> Self {
        Self { factory }
    }

    pub fn create(&self, model: &Model) -> State {
        (self.factory)(model)
    }
}

#[derive(Default)]
pub struct State {
    transition: Option<Transition>,
    actions: Vec<Box<dyn Action>>,
}

impl State {
    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }

    pub fn action(mut self, action: Box<dyn Action>) -> Self {
        self.actions.push(action);
        self
    }

    pub fn try_transition(
        &self,
        events: &Vec<StoryEvent>,
    ) -> Option<(&'static str, Option<&'static str>)> {
        if self
            .transition
            .as_ref()
            .is_some_and(|transition| transition.can_transition(events))
        {
            Some((
                self.transition.as_ref().unwrap().to(),
                self.transition.as_ref().unwrap().objective(),
            ))
        } else {
            None
        }
    }

    pub fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let mut model_events = vec![];
        let mut view_events = vec![];
        if let Some(transition) = &self.transition {
            if let Some(objective) = transition.objective() {
                view_events.push(ViewEvent::StartObjective(objective));
            }
        }
        for action in &self.actions {
            let (new_model_events, new_view_events) = action.trigger();
            model_events.extend(new_model_events);
            view_events.extend(new_view_events);
        }
        (model_events, view_events)
    }
}
