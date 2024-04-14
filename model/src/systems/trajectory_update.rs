use crate::{components::ComponentType, Model, SEGMENTS_TO_PREDICT};

pub fn update(model: &mut Model, dt: f64) {
    let time = model.get_time();
    let time_step = model.get_time_step().get_time_step();
    for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
        model.get_trajectory_component_mut(entity).next(time, dt * time_step);
        if model.try_get_vessel_component(entity).is_some() {
            // Cast to u64 to prevent potential crash if there are ever more segments than expected for whatever reason
            let segments_to_predict = SEGMENTS_TO_PREDICT as i32 - model.get_trajectory_component(entity).get_remaining_orbits_after_final_burn() as i32;
            if segments_to_predict > 0 {
                model.predict(entity, 1.0e10, segments_to_predict as usize);
            }
        }
    }
}