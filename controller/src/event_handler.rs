use std::fs;

use eframe::egui::{Context, ViewportCommand};
use log::error;
use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}, vessel_component::{Faction, VesselClass, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};
use transfer_window_view::{game, Scene};

use crate::Controller;

pub fn quit(context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Quit");

    context.send_viewport_cmd(ViewportCommand::Close);
}

pub fn new_game(controller: &mut Controller, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("New game");

    let mut model = Model::default();

    let sun = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Sun".to_string()))
        .with_orbitable_component(OrbitableComponent::new(1_988_400e24, 695_700e3, OrbitableType::Star, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let orbit = Orbit::new(sun, 5.9722e24, 1_988_400e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0).with_end_at(1.0e10);
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(5.9722e24, 6.371e6, OrbitableType::Planet, OrbitableComponentPhysics::Orbit(orbit))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let orbit = Orbit::new(earth, 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, -1.082e3), 0.0).with_end_at(1.0e10);
    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(0.07346e24, 1737.4e3, OrbitableType::Moon, OrbitableComponentPhysics::Orbit(orbit))));

    let orbit = Orbit::circle(earth, VesselClass::Light.mass(), 5.9722e24, vec2(0.1e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let spacecraft_1 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Spacecraft 1".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light, Faction::Player))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(earth, VesselClass::Light.mass(), 5.9722e24, vec2(0.2e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _spacecraft_2 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Spacecraft 2".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light, Faction::Enemy))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(moon, VesselClass::Light.mass(), 0.07346e24, vec2(0.3e8, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _spacecraft_3 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Spacecraft 3".to_string()))
        .with_vessel_component(VesselComponent::new(VesselClass::Light, Faction::Ally))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    controller.scene = Scene::Game(game::View::new(controller.gl.clone(), model, context.clone(), controller.resources.clone(), Some(spacecraft_1)));
}

pub fn load_game(controller: &mut Controller, context: &Context, name: &str) {
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

    controller.scene = Scene::Game(game::View::new(controller.gl.clone(), model, context.clone(), controller.resources.clone(), None));
}
