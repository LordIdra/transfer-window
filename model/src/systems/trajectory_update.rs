use crate::{components::ComponentType, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

fn update_path_component(model: &mut Model, entity: Entity, time: f64, simulation_dt: f64) {
    model.get_path_component_mut(entity).get_current_segment_mut().next(simulation_dt);
    loop {
        let current_segment = model.get_path_component(entity).get_current_segment();
        if !current_segment.is_finished() {
            break;
        }

        model.get_path_component_mut(entity).on_segment_finished(time);

        // Add one because one of the orbits will be duration zero right at the end
        // due to how trajectory prediction works
        let segments_to_predict = SEGMENTS_TO_PREDICT as i32 + 1 - model.get_path_component(entity).get_remaining_orbits_after_final_burn() as i32;
        if segments_to_predict > 0 {
            model.predict(entity, 1.0e10, segments_to_predict as usize);
        }
    }
}

fn update_orbitable_component(model: &mut Model, entity: Entity, simulation_dt: f64) {
    if let Some(orbit) = model.get_orbitable_component_mut(entity).get_orbit_mut() {
        orbit.next(simulation_dt);
    }
}

pub fn update(model: &mut Model, dt: f64) {
    let time = model.get_time();
    let time_step = model.get_time_step().get_time_step();
    let simulation_dt = dt * time_step;
    for entity in model.get_entities(vec![ComponentType::PathComponent]) {
        update_path_component(model, entity, time, simulation_dt);
    }
    for entity in model.get_entities(vec![ComponentType::OrbitableComponent]) {
        update_orbitable_component(model, entity, simulation_dt);
    }
}