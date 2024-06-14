use crate::{components::ComponentType, storage::entity_allocator::Entity, Model};

fn update_path_component(model: &mut Model, entity: Entity, time: f64) {
    model.path_component_mut(entity).current_segment_mut().next(time);
    loop {
        let current_segment = model.path_component(entity).current_segment();
        if !current_segment.is_finished() {
            break;
        }

        model.path_component_mut(entity).on_segment_finished(time);
    }

    if model.vessel_component(entity).should_recompute_trajectory() {
        model.recompute_trajectory(entity);
    }
}

fn update_orbitable_component(model: &mut Model, entity: Entity, time: f64) {
    if let Some(orbit) = model.orbitable_component_mut(entity).orbit_mut() {
        orbit.next(time);
    }
}

impl Model {
    pub(crate) fn update_trajectory(&mut self) {
        let time = self.time();
        for entity in self.entities(vec![ComponentType::VesselComponent]) {
            if !self.vessel_component(entity).is_ghost() {
                update_path_component(self, entity, time);
                let perceived_segments = self.compute_perceived_path(entity);
                self.path_component_mut(entity).set_perceived_segments(perceived_segments);
            }
        }
        for entity in self.entities(vec![ComponentType::OrbitableComponent]) {
            update_orbitable_component(self, entity, time);
        }
    }
}