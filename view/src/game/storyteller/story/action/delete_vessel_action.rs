use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::events::{ModelEvent, ViewEvent};

use super::Action;

pub struct DeleteVesselAction{
    entity: Entity,
}

impl DeleteVesselAction {
    pub fn new(entity: Entity) -> Box<dyn Action> {
        Box::new(Self { entity })
    }
}

impl Action for DeleteVesselAction {
    fn trigger(&self) -> (Vec<ModelEvent>, Vec<ViewEvent>) {
        let event = ModelEvent::DeleteVessel { entity: self.entity };
        (vec![event], vec![])
    }
}