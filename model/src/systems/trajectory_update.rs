use crate::{components::ComponentType, storage::entity_allocator::Entity, Model};

fn update_path_component(model: &mut Model, entity: Entity, time: f64, simulation_dt: f64) {
    model.path_component_mut(entity).current_segment_mut().next(simulation_dt);
    loop {
        let current_segment = model.path_component(entity).current_segment();
        if !current_segment.is_finished() {
            break;
        }

        model.path_component_mut(entity).on_segment_finished(time);

        // Add one because one of the orbits will be duration zero right at the end
        // due to how trajectory prediction works
    }
    model.recompute_trajectory(entity);
}

fn update_orbitable_component(model: &mut Model, entity: Entity, simulation_dt: f64) {
    if let Some(orbit) = model.orbitable_component_mut(entity).orbit_mut() {
        orbit.next(simulation_dt);
    }
}

pub fn update(model: &mut Model, dt: f64) {
    let time = model.time();
    let time_step = model.time_step().time_step();
    let simulation_dt = dt * time_step;
    for entity in model.entities(vec![ComponentType::PathComponent]) {
        update_path_component(model, entity, time, simulation_dt);
    }
    for entity in model.entities(vec![ComponentType::OrbitableComponent]) {
        update_orbitable_component(model, entity, simulation_dt);
    }
}