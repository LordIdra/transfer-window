use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct SetFocusAction{
    entity: Entity,
}

impl SetFocusAction {
    pub fn new(entity: Entity) -> Box<dyn Action> {
        Box::new(Self { entity })
    }
}

impl Action for SetFocusAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ViewEvent::SetCameraFocus(self.entity);
        (vec![], vec![event])
    }
}