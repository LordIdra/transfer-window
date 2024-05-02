use std::fs;

use eframe::Frame;
use log::error;
use nalgebra_glm::{vec2, DVec2};
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::OrbitableComponent, stationary_component::StationaryComponent, trajectory_component::{burn::{rocket_equation_function::RocketEquationFunction, Burn}, orbit::Orbit, segment::Segment, TrajectoryComponent}, vessel_component::{system_slot::{Slot, SlotLocation, System}, VesselClass, VesselComponent}}, storage::{entity_allocator::Entity, entity_builder::EntityBuilder}, Model, SEGMENTS_TO_PREDICT};
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
        .with_orbitable_component(OrbitableComponent::new(1_988_500e24, 695_700e3))
        .with_stationary_component(StationaryComponent::new(vec2(0.0, 0.0))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let mut trajectory_component = TrajectoryComponent::default();
    let mut orbit = Orbit::new(sun, 5.9722e24, 1_988_500e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0);
    orbit.end_at(1.0e10);
    trajectory_component.add_segment(Segment::Orbit(orbit));
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(5.9722e24, 0.5))
        .with_trajectory_component(trajectory_component));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let mut trajectory_component = TrajectoryComponent::default();
    let mut orbit = Orbit::new(earth, 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, -1.082e3), 0.0);
    orbit.end_at(1.0e10);
    trajectory_component.add_segment(Segment::Orbit(orbit));
    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(0.07346e24, 1737.4e3))
        .with_trajectory_component(trajectory_component));

    let mut trajectory_component = TrajectoryComponent::default();
    let mut orbit = Orbit::new(earth, 3.0e2, 5.9722e24, vec2(0.1e9, 0.0), vec2(0.0, 2.0e3), 0.0);
    orbit.end_at(1.0e10);
    trajectory_component.add_segment(Segment::Orbit(orbit));
    let _spacecraft_1 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Light Spacecraft".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_trajectory_component(trajectory_component));

    let mut trajectory_component = TrajectoryComponent::default();
    let mut orbit = Orbit::new(moon, 2.0e2, 0.07346e24, vec2(0.3e8, 0.0), vec2(0.0, 4.0e2), 0.0);
    orbit.end_at(1.0e10);
    trajectory_component.add_segment(Segment::Orbit(orbit));
    let _spacecraft_2 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Light Spacecraft".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light))
        .with_trajectory_component(trajectory_component));

    controller.model = Some(model);
    controller.view = View::GameScene(Scene::new(&controller.gl, Some(earth)));
}

pub fn save_game(controller: &Controller, name: &str) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Save game");

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
    controller.get_model_mut().toggle_paused();
}

pub fn increase_time_step_level(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Increase time step level");
    controller.get_model_mut().increase_time_step_level();
}

pub fn decrease_time_step_level(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Decrease time step level");
    controller.get_model_mut().decrease_time_step_level();
}

pub fn start_warp(controller: &mut Controller, end_time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Start warp");
    controller.get_model_mut().start_warp(end_time);
}

pub fn create_burn(controller: &mut Controller, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Create burn");
    let model = controller.get_model_mut();
    
    let trajectory_component = model.get_trajectory_component_mut(entity);
    trajectory_component.remove_segments_after(time);
    
    let parent = trajectory_component.get_end_segment().get_parent();
    let tangent = trajectory_component.get_end_segment().get_end_velocity().normalize();
    let start_position = trajectory_component.get_end_segment().get_end_position();
    let start_velocity = trajectory_component.get_end_segment().get_end_velocity();
    
    let rocket_equation_function = match trajectory_component.get_final_burn() {
        Some(burn) => burn.get_rocket_equation_function_at_end_of_burn(),
        None => {
            let vessel_component = model.get_vessel_component(entity);
            let dry_mass_kg = vessel_component.get_dry_mass();
            let initial_fuel_mass_kg = vessel_component.get_fuel_mass();
            let engine = vessel_component.get_slots().get_engine().unwrap();
            let fuel_consumption_kg_per_second = engine.get_type().get_fuel_kg_per_second();
            let specific_impulse = engine.get_type().get_specific_impulse_space();
            RocketEquationFunction::new(dry_mass_kg, initial_fuel_mass_kg, fuel_consumption_kg_per_second, specific_impulse, 0.0)
        }
    };

    let parent_mass = model.get_mass(parent);
    let burn = Burn::new(entity, parent, parent_mass, tangent, vec2(0.0, 0.0), time, rocket_equation_function, start_position, start_velocity);

    let mass_at_burn_end = burn.get_end_point().get_mass();
    let orbit = Orbit::new(parent, mass_at_burn_end, parent_mass, burn.get_end_point().get_position(), burn.get_end_point().get_velocity(), burn.get_end_point().get_time());
    
    model.get_trajectory_component_mut(entity).add_segment(Segment::Burn(burn));
    model.get_trajectory_component_mut(entity).add_segment(Segment::Orbit(orbit));
    model.predict(entity, 1.0e10, SEGMENTS_TO_PREDICT);
}

pub fn delete_burn(controller: &mut Controller, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Delete burn");
    let model = controller.get_model_mut();
    
    let trajectory_component = model.get_trajectory_component_mut(entity);
    trajectory_component.remove_segments_after(time);

    // This is needed to make sure we recompute the conic WITHOUT the burn
    // Add 1 because the final orbit will have duration 0
    let segments_to_predict = SEGMENTS_TO_PREDICT + 1 - trajectory_component.get_remaining_orbits_after_final_burn();
    model.predict(entity, 1.0e10, segments_to_predict);
}

pub fn adjust_burn(controller: &mut Controller, entity: Entity, time: f64, amount: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Adjust burn");
    let model = controller.get_model_mut();
    
    let end_time = model.get_trajectory_component_mut(entity).get_last_segment_at_time(time).get_end_time();
    model.get_trajectory_component_mut(entity).remove_segments_after(end_time);
    model.get_trajectory_component_mut(entity).get_end_segment_mut().as_burn_mut().adjust(amount);

    let parent = model.get_trajectory_component(entity).get_end_segment().get_parent();
    let position = model.get_trajectory_component_mut(entity).get_end_segment().get_end_position();
    let velocity = model.get_trajectory_component_mut(entity).get_end_segment().get_end_velocity();
    let parent_mass = model.get_mass(parent);
    let mass = model.get_mass(entity);

    // Needs to be recalculated after we adjust the burn
    let end_time = model.get_trajectory_component_mut(entity).get_last_segment_at_time(time).get_end_time();

    let orbit = Orbit::new(parent, mass, parent_mass, position, velocity, end_time);

    model.get_trajectory_component_mut(entity).add_segment(Segment::Orbit(orbit));
    model.predict(entity, 1.0e10, SEGMENTS_TO_PREDICT);
}

pub fn destroy(controller: &mut Controller, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set target");
    let model = controller.get_model_mut();

    model.deallocate(entity);
}

pub fn set_target(controller: &mut Controller, entity: Entity, target: Option<Entity>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set target");
    let model = controller.get_model_mut();

    model.get_vessel_component_mut(entity).set_target(target);
}

pub fn set_slot(controller: &mut Controller, entity: Entity, location: SlotLocation, slot: Slot) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set slot");
    let model = controller.get_model_mut();

    model.get_vessel_component_mut(entity).set_slot(location, slot);
}