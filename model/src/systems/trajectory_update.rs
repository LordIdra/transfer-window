use crate::{components::ComponentType, Model};

pub fn update(model: &mut Model, dt: f64) {
    let time = model.get_time();
    let time_step = model.get_time_step().get_time_step();
    for entity in model.get_entities(vec![ComponentType::TrajectoryComponent]) {
        model.get_trajectory_component_mut(entity).next(time, dt * time_step);
    }
}