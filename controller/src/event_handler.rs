use std::fs;

use eframe::Frame;
use log::error;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{orbit::Orbit, segment::Segment, PathComponent}, vessel_component::{system_slot::{Slot, SlotLocation}, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};
use transfer_window_view::{game::Scene, View};

use crate::Controller;

pub fn quit(frame: &mut Frame) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Quit");

    frame.close();
}

pub fn new_game(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("New game");

    let mut model = Model::default();

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html
    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_orbitable_component(OrbitableComponent::new(1_988_500e24, 695_700e3, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let orbit = Orbit::new(sun, 5.9722e24, 1_988_500e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0).with_end_at(1.0e10);
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(5.9722e24, 0.5, OrbitableComponentPhysics::Orbit(orbit))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let orbit = Orbit::new(earth, 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, -1.082e3), 0.0).with_end_at(1.0e10);
    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(0.07346e24, 1737.4e3, OrbitableComponentPhysics::Orbit(orbit))));

    let orbit = Orbit::new(earth, 3.0e2, 5.9722e24, vec2(0.1e9, 0.0), vec2(0.0, 2.0e3), 0.0).with_end_at(1.0e10);
    let _spacecraft_1 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Light Spacecraft".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::new(moon, 2.0e2, 0.07346e24, vec2(0.3e8, 0.0), vec2(0.0, 4.0e2), 0.0).with_end_at(1.0e10);
    let _spacecraft_2 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Light Spacecraft".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    controller.model = Some(model);
    controller.view = View::GameScene(Scene::new(&controller.gl, Some(earth)));
}

pub fn save_game(controller: &Controller, name: &str) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Save game");

    let model = controller.model();

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
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Load game");
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
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Toggle paused");
    controller.model_mut().toggle_paused();
}

pub fn increase_time_step_level(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Increase time step level");
    controller.model_mut().increase_time_step_level();
}

pub fn decrease_time_step_level(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Decrease time step level");
    controller.model_mut().decrease_time_step_level();
}

pub fn start_warp(controller: &mut Controller, end_time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Start warp");
    controller.model_mut().start_warp(end_time);
}

pub fn create_burn(controller: &mut Controller, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Create burn");
    let model = controller.model_mut();
    
    model.create_burn(entity, time);
}

pub fn delete_burn(controller: &mut Controller, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Delete burn");
    let model = controller.model_mut();
    
    model.delete_segments_after_time_and_recompute_trajectory(entity, time);
}

pub fn adjust_burn(controller: &mut Controller, entity: Entity, time: f64, amount: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Adjust burn");
    let model = controller.model_mut();
    
    model.adjust_burn(entity, time, amount);
}

pub fn destroy(controller: &mut Controller, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set target");
    let model = controller.model_mut();

    model.deallocate(entity);
}

pub fn set_target(controller: &mut Controller, entity: Entity, target: Option<Entity>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set target");
    let model = controller.model_mut();

    model.vessel_component_mut(entity).set_target(target);
}

pub fn set_slot(controller: &mut Controller, entity: Entity, location: SlotLocation, slot: Slot) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set slot");
    let model = controller.model_mut();

    model.vessel_component_mut(entity).set_slot(location, slot);
    model.recompute_entire_trajectory(entity);
}