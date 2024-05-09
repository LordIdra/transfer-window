use std::fs;

use eframe::Frame;
use log::error;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics}, path_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}, vessel_component::{system_slot::{Slot, SlotLocation}, timeline::{start_burn::BurnEvent, fire_torpedo::FireTorpedoEvent, TimelineEvent, TimelineEventType}, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model};
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

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(5.9722e24, 0.5, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let orbit = Orbit::new(earth, 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, -1.082e3), 0.0).with_end_at(1.0e10);
    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(0.07346e24, 1737.4e3, OrbitableComponentPhysics::Orbit(orbit))));

    let orbit = Orbit::circle(earth, VesselClass::Light.mass(), 5.9722e24, vec2(0.1e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _spacecraft_1 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Spacecraft 1".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(earth, VesselClass::Light.mass(), 5.9722e24, vec2(0.2e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _spacecraft_2 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Spacecraft 2".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(moon, VesselClass::Light.mass(), 0.07346e24, vec2(0.3e8, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _spacecraft_3 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Spacecraft 3".to_string()))
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

pub fn cancel_last_event(controller: &mut Controller, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Cancel last event");
    let model = controller.model_mut();
    
    model.cancel_last_event(entity);
}

pub fn create_burn(controller: &mut Controller, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Create burn");
    let model = controller.model_mut();
    
    let event_type = TimelineEventType::Burn(BurnEvent::new(model, entity, time));
    model.add_event(entity, TimelineEvent::new(time, event_type));
}

pub fn adjust_burn(controller: &mut Controller, entity: Entity, time: f64, amount: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Adjust burn");
    let model = controller.model_mut();
    
    model.event_at_time(entity, time)
        .type_()
        .as_burn()
        .unwrap()
        .adjust(model, amount);
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

    model.set_slot(entity, location, slot);
}

pub fn create_fire_torpedo(controller: &mut Controller, entity: Entity, location: SlotLocation, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Fire torpedo");
    let model = controller.model_mut();

    let event_type = TimelineEventType::FireTorpedo(FireTorpedoEvent::new(model, entity, time, location));
    model.add_event(entity, TimelineEvent::new(time, event_type));
}

pub fn adjust_fire_torpedo(controller: &mut Controller, entity: Entity, time: f64, amount: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Adjust burn");
    let model = controller.model_mut();
    
    model.event_at_time(entity, time)
        .type_()
        .as_fire_torpedo()
        .unwrap()
        .adjust(model, amount);
}