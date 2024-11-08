use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::model::state_query::StateQuery;
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
        let Some(time) = view.model.snapshot_at_observe(view.model.time(), Faction::Player).find_next_closest_approach(self.entity, target) else {
            return false;
        };
        let distance = view.model.snapshot_at_observe(time, Faction::Player).distance(self.entity, target);
        distance < self.max_distance
    }
}

