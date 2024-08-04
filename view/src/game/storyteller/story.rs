use std::collections::HashMap;
use std::sync::Mutex;

use condition::Condition;
use state::{State, StateCreator};
use transfer_window_model::storage::entity_allocator::Entity;
use transition::Transition;

use crate::game::events::ViewEvent;
use crate::game::View;

pub mod condition;
pub(super) mod state;
pub mod transition;

pub struct Story {
    state_creators: HashMap<&'static str, StateCreator>,
    state: Mutex<State>,
    state_string: Mutex<&'static str>,
    persistent_data: HashMap<String, Entity>,
}

impl Story {
    pub fn new(root: &'static str) -> Self {
        let state_creators = HashMap::new();
        let state = Mutex::new(State::default().transition(Transition::new(root, Condition::none())));
        let state_string = Mutex::new("uninitialized");
        let persistent_data = HashMap::new();
        Self { state_creators, state, state_string, persistent_data }
    }

    pub fn empty() -> Self {
        let mut story = Self::new("root");
        story.add("root", |_| State::default());
        story
    }

    pub(super) fn add(&mut self, name: &'static str, factory: impl Fn(&View) -> State + 'static) {
        assert!(!self.state_creators.contains_key(name), "Duplicate state {name}");
        self.state_creators.insert(name, StateCreator::new(Box::new(factory)));
    }

    pub fn add_persistent_data(&mut self, key: &str, entity: Entity) {
        self.persistent_data.insert(key.to_string(), entity);
    }

    pub(super) fn get_persistent_data(&self, key: &str) -> Entity {
        *self.persistent_data.get(key).unwrap_or_else(|| panic!("Attempt to get nonexistent persistent data with key {key}"))
    }

    pub fn update(&self, view: &View) {
        let Some((state_string, objective)) = self.state.lock().unwrap().try_transition(view) else { 
            return 
        };

        *self.state_string.lock().unwrap() = state_string;
        *self.state.lock().unwrap() = self.state_creators.get(state_string)
            .unwrap_or_else(|| panic!("State does not exist {state_string}"))
            .create(view);
        
        if let Some(transition) = self.state.lock().unwrap().get_transition() {
            if let Some(objective) = transition.objective() {
                view.add_view_event(ViewEvent::StartObjective(objective));
            }
        }

        if let Some(objective) = objective {
            view.add_view_event(ViewEvent::FinishObjective(objective));
        }
    }
    
    #[cfg(test)]
    pub fn states(&self) -> &HashMap<&'static str, StateCreator> {
        &self.state_creators
    }
}