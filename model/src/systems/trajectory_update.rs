use crate::{components::ComponentType, storage::entity_allocator::Entity, Model, SEGMENTS_TO_PREDICT};

fn update_trajectory_component(model: &mut Model, entity: Entity, time: f64, delta_time: f64) {
    model.get_trajectory_component_mut(entity).get_current_segment_mut().next(delta_time);
    while model.get_trajectory_component(entity).get_current_segment().is_finished() {
        model.get_trajectory_component_mut(entity).on_segment_finished(time);
        // Add one because one of the orbits will be duration zero right at the end
        // due to how trajectory prediction works
        let segments_to_predict = SEGMENTS_TO_PREDICT as i32 + 1 - model.get_trajectory_component(entity).get_remaining_orbits_after_final_burn() as i32;
        if segments_to_predict > 0 {
            model.predict(entity, 1.0e10, segments_to_predict as usize);
        }
    }
}

pub fn update(model: &mut Model, dt: f64) {
    let time = model.get_time();
    let time_step = model.get_time_step().get_time_step();
    for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
        update_trajectory_component(model, entity, time, dt * time_step);
    }
}