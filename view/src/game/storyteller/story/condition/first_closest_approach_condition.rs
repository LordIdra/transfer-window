use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::View;

use super::ConditionCheck;

pub struct FirstClosestApproachCondition {
    entity: Entity,
    max_distance: f64,
}

impl FirstClosestApproachCondition {
    pub fn new(entity: Entity, max_distance: f64) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity, max_distance })
    }
}

impl ConditionCheck for FirstClosestApproachCondition {
    fn met(&self, view: &View) -> bool {
        let Some(target) = view.model.target(self.entity) else {
            return false;
        };
        let Some(time) = view.model.find_next_closest_approach(self.entity, target, view.model.time(), Some(Faction::Player)) else {
            return false;
        };
        let distance = view.model.distance_at_time(self.entity, target, time, Some(Faction::Player));
        distance < self.max_distance
    }
}

