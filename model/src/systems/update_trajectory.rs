use crate::{components::ComponentType, model::{state_query::StateQuery, Model}, storage::entity_allocator::Entity};

impl Model {
    fn update_path_component(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update path component");
        let new_fuel_kg = self.snapshot_at(time).fuel_kg(entity);
        self.vessel_component_mut(entity).set_fuel_kg(new_fuel_kg);

        let mut should_recompute_perceived_segments = self.path_component(entity).perceived_segments().is_empty() 
        || !self.path_component(entity).current_segment().is_orbit();
        self.path_component_mut(entity).current_segment_mut().next(time);

        loop {
            if !self.path_component(entity).current_segment().is_finished() {
                break;
            }

            self.path_component_mut(entity).on_segment_finished(time);
            should_recompute_perceived_segments = should_recompute_perceived_segments || !self.path_component(entity).current_segment().is_orbit();
        }

        if self.vessel_component(entity).should_recompute_trajectory() && self.recompute_trajectory(entity) {
            should_recompute_perceived_segments = true;
        }

        if should_recompute_perceived_segments {
            let perceived_segments = self.compute_perceived_path(entity);
            self.path_component_mut(entity).set_perceived_segments(perceived_segments);
        } else {
            // update perceived segments
            self.path_component_mut(entity).current_perceived_segment_mut().next(time);
            loop {
                let current_segment = self.path_component(entity).current_perceived_segment();
                if !current_segment.is_finished() {
                    break;
                }

                self.path_component_mut(entity).on_segment_finished(time);
            }
        }
    }

    fn update_orbitable_component(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update orbitable component");
        if let Some(orbit) = self.orbitable_component_mut(entity).orbit_mut() {
            orbit.next(time);
        }
    }

    pub(crate) fn update_trajectory(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update trajectory");
        let time = self.time();
        for entity in self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            if !self.vessel_component(entity).is_ghost() {
                self.update_path_component(entity, time);
            }
        }
        for entity in self.entities(vec![ComponentType::OrbitableComponent]) {
            self.update_orbitable_component(entity, time);
        }
    }
}
