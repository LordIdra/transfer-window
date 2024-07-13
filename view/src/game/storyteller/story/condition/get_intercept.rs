use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::View;

use super::ConditionCheck;

pub struct GetInterceptCondition {
    entity: Entity,
}

impl GetInterceptCondition {
    pub fn new(entity: Entity) -> Box<dyn ConditionCheck> {
        Box::new(Self { entity })
    }
}

impl ConditionCheck for GetInterceptCondition {
    fn met(&self, view: &View) -> bool {
        view.model.vessel_component(self.entity).timeline().last_event().is_some_and(|event| event.is_intercept())
    }
}

