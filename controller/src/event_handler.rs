use std::fs;

use eframe::Frame;
use log::error;
use nalgebra_glm::vec2;
use transfer_window_model::{components::{mass_component::MassComponent, name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{orbit::Orbit, segment::Segment, TrajectoryComponent}}, storage::entity_builder::EntityBuilder, Model};
use transfer_window_view::{game::Scene, View};

use crate::Controller;

pub fn quit(frame: &mut Frame) {
    frame.close();
}

pub fn new_game(controller: &mut Controller) {
    let mut model = Model::default();

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html
    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_mass_component(MassComponent::new(1_988_500e24))
        .with_orbitable_component(OrbitableComponent::new(695_700e3))
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let mut trajectory_component = TrajectoryComponent::default();
    let mut orbit = Orbit::new(sun, 5.9722e24, 1_988_500e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0);
    orbit.end_at(1.0e10);
    trajectory_component.add_segment(Segment::Orbit(orbit));
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_mass_component(MassComponent::new(5.9722e24))
        .with_orbitable_component(OrbitableComponent::new(1.0))
        .with_trajectory_component(trajectory_component));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let mut trajectory_component = TrajectoryComponent::default();
    let mut orbit = Orbit::new(earth, 0.07346e24, 5.9722e24, vec2(0.3633e10, 0.0), vec2(0.0, 1.082e3), 0.0);
    orbit.end_at(1.0e10);
    trajectory_component.add_segment(Segment::Orbit(orbit));
    let _moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_mass_component(MassComponent::new(0.07346e24))
        .with_orbitable_component(OrbitableComponent::new(1737.4e3))
        .with_trajectory_component(trajectory_component));

    controller.model = Some(model);
    controller.view = View::GameScene(Scene::new(&controller.gl, Some(earth)));
}

pub fn save_game(controller: &Controller, name: &str) {
    let model = controller.get_model();

    let serialized = model.serialize();
    let Ok(serialized) = serialized else {
        error!("Failed to handle save_game; error while serializing: {}", serialized.err().unwrap());
        return;
    };

    if let Err(error) = fs::write("saves/".to_string() + name + ".json", serialized) {
        error!("Failed to handle save_game; error while saving: {}", error);
    }
}

pub fn load_game(controller: &mut Controller, name: &str) {
     let serialized = fs::read_to_string("saves/".to_string() + name + ".json");
     let Ok(serialized) = serialized else {
         error!("Failed to handle load game; error while loading file: {}", serialized.err().unwrap());
         return;
     };

     let model = Model::deserialize(serialized.as_str());
     let Ok(model) = model else {
        error!("Failed to handle load game; error while deseraizing: {}", model.err().unwrap());
        return;
     };

     controller.model = Some(model);
     controller.view = View::GameScene(Scene::new(&controller.gl, None));
}

pub fn toggle_paused(controller: &mut Controller) {
    controller.get_model_mut().toggle_paused();
}

pub fn increase_time_step_level(controller: &mut Controller) {
    controller.get_model_mut().increase_time_step_level();
}

pub fn decrease_time_step_level(controller: &mut Controller) {
    controller.get_model_mut().decrease_time_step_level();
}

pub fn debug_add_entity(controller: &mut Controller, entity_builder: EntityBuilder) {
    controller.get_model_mut().allocate(entity_builder);
}