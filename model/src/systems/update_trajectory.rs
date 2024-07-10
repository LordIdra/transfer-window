use crate::components::ComponentType;
use crate::storage::entity_allocator::Entity;
use crate::Model;

fn update_path_component(model: &mut Model, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update path component");
    let mut should_recompute_perceived_segments =
        model.path_component(entity).perceived_segments().is_empty()
            || !model.path_component(entity).current_segment().is_orbit();
    model
        .path_component_mut(entity)
        .current_segment_mut()
        .next(time);
    loop {
        let current_segment = model.path_component(entity).current_segment();
        if !current_segment.is_finished() {
            break;
        }

        model.path_component_mut(entity).on_segment_finished(time);
        should_recompute_perceived_segments = should_recompute_perceived_segments
            || !model.path_component(entity).current_segment().is_orbit();
    }

    if model.vessel_component(entity).should_recompute_trajectory()
        && model.recompute_trajectory(entity)
    {
        should_recompute_perceived_segments = true;
    }

    if should_recompute_perceived_segments {
        let perceived_segments = model.compute_perceived_path(entity);
        model
            .path_component_mut(entity)
            .set_perceived_segments(perceived_segments);
    } else {
        // update perceived segments
        model
            .path_component_mut(entity)
            .current_perceived_segment_mut()
            .next(time);
        loop {
            let current_segment = model.path_component(entity).current_perceived_segment();
            if !current_segment.is_finished() {
                break;
            }

            model.path_component_mut(entity).on_segment_finished(time);
        }
    }
}

fn update_orbitable_component(model: &mut Model, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update orbitable component");
    if let Some(orbit) = model.orbitable_component_mut(entity).orbit_mut() {
        orbit.next(time);
    }
}

impl Model {
    pub(crate) fn update_trajectory(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Update trajectory");
        let time = self.time();
        for entity in self.entities(vec![
            ComponentType::VesselComponent,
            ComponentType::PathComponent,
        ]) {
            if !self.vessel_component(entity).is_ghost() {
                update_path_component(self, entity, time);
            }
        }
        for entity in self.entities(vec![ComponentType::OrbitableComponent]) {
            update_orbitable_component(self, entity, time);
        }
    }
}
