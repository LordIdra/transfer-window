use std::fs;

use eframe::egui::{Context, ViewportCommand};
use eframe::epaint::Rgba;
use log::error;
use nalgebra_glm::vec2;
use transfer_window_model::{components::{name_component::NameComponent, orbitable_component::{OrbitableComponent, OrbitableComponentPhysics, OrbitableType}, path_component::{orbit::{orbit_direction::OrbitDirection, Orbit}, segment::Segment, PathComponent}, vessel_component::{faction::Faction, ship::{ship_slot::{engine::EngineType, fuel_tank::FuelTankType, weapon::WeaponType, ShipSlot, ShipSlotLocation}, ShipClass}, station::StationClass, timeline::{enable_guidance::EnableGuidanceEvent, fire_torpedo::FireTorpedoEvent, start_burn::StartBurnEvent, TimelineEvent}, VesselComponent}}, storage::entity_builder::EntityBuilder, Model};
use transfer_window_model::components::atmosphere_component::AtmosphereComponent;
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
        .with_orbitable_component(OrbitableComponent::new(1_988_400e24, 695_700e3, 25.05, 0.0, OrbitableType::Star, OrbitableComponentPhysics::Stationary(vec2(0.0, 0.0)))));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let orbit = Orbit::new(sun, 5.9722e24, 1_988_400e24, vec2(147.095e9, 0.0), vec2(0.0, 30.29e3), 0.0).with_end_at(1.0e10);
    let earth = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Earth".to_string()))
        .with_orbitable_component(OrbitableComponent::new(5.9722e24, 6.371e6, 0.99727, 0.0, OrbitableType::Planet, OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit))))
        .with_atmosphere_component(AtmosphereComponent::new(Rgba::from_rgb(0.529, 0.808, 0.922), 2.0, 0.1)));

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html
    let orbit = Orbit::new(earth, 0.07346e24, 5.9722e24, vec2(0.3633e9, 0.0), vec2(0.0, -1.082e3), 0.0).with_end_at(1.0e10);
    let moon = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Moon".to_string()))
        .with_orbitable_component(OrbitableComponent::new(0.07346e24, 1737.4e3, 27.321667, 0.0, OrbitableType::Moon, OrbitableComponentPhysics::Orbit(Segment::Orbit(orbit)))));

    let orbit = Orbit::circle(earth, ShipClass::Scout.mass(), 5.9722e24, vec2(0.08e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let spacecraft_0 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Scout".to_string()))
        .with_vessel_component(VesselComponent::new_ship(ShipClass::Scout, Faction::Player))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(earth, ShipClass::Frigate.mass(), 5.9722e24, vec2(0.1e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let spacecraft_1 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Frigate".to_string()))
        .with_vessel_component(VesselComponent::new_ship(ShipClass::Frigate, Faction::Player))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(earth, StationClass::Hub.mass(), 5.9722e24, vec2(0.11e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _station = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Hub".to_string()))
        .with_vessel_component(VesselComponent::new_station(StationClass::Hub, Faction::Ally))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(earth, ShipClass::Frigate.mass(), 5.9722e24, vec2(0.2e9, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let spacecraft_2 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Enemy Frigate".to_string()))
        .with_vessel_component(VesselComponent::new_ship(ShipClass::Frigate, Faction::Enemy))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    let orbit = Orbit::circle(moon, ShipClass::Frigate.mass(), 0.07346e24, vec2(0.3e8, 0.0), 0.0, OrbitDirection::AntiClockwise).with_end_at(1.0e10);
    let _spacecraft_3 = model.allocate(EntityBuilder::default()
        .with_name_component(NameComponent::new("Ally Frigate".to_string()))
        .with_vessel_component(VesselComponent::new_ship(ShipClass::Frigate, Faction::Ally))
        .with_path_component(PathComponent::default().with_segment(Segment::Orbit(orbit))));

    model.set_slot(spacecraft_0, ShipSlotLocation::Back, ShipSlot::new_engine(EngineType::Regular));
    model.set_slot(spacecraft_0, ShipSlotLocation::Middle, ShipSlot::new_fuel_tank(FuelTankType::Small));
    model.recompute_entire_trajectory(spacecraft_0);

    model.set_slot(spacecraft_1, ShipSlotLocation::Back, ShipSlot::new_engine(EngineType::Efficient));
    model.set_slot(spacecraft_1, ShipSlotLocation::Middle, ShipSlot::new_fuel_tank(FuelTankType::Medium));
    model.set_slot(spacecraft_1, ShipSlotLocation::Front, ShipSlot::new_weapon(WeaponType::new_enhanced_torpedo()));
    model.recompute_entire_trajectory(spacecraft_1);

    model.set_slot(spacecraft_2, ShipSlotLocation::Back, ShipSlot::new_engine(EngineType::Efficient));
    model.set_slot(spacecraft_2, ShipSlotLocation::Middle, ShipSlot::new_fuel_tank(FuelTankType::Medium));
    model.set_slot(spacecraft_2, ShipSlotLocation::Front, ShipSlot::new_weapon(WeaponType::new_torpedo()));
    model.recompute_entire_trajectory(spacecraft_2);

    let event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, spacecraft_2, 200.0));
    model.vessel_component_mut(spacecraft_2).timeline_mut().add(event);
    model.vessel_component_mut(spacecraft_2).timeline_mut().last_event().unwrap().as_start_burn().unwrap().adjust(&mut model, vec2(300.0, 50.0));

    let fire_torpedo_event = FireTorpedoEvent::new(&mut model, spacecraft_2, 100.0, ShipSlotLocation::Front);
    let torpedo = fire_torpedo_event.ghost();
    let event = TimelineEvent::FireTorpedo(fire_torpedo_event);
    model.vessel_component_mut(spacecraft_2).timeline_mut().add(event);
    
    let event = TimelineEvent::Burn(StartBurnEvent::new(&mut model, torpedo, 130.0));
    model.vessel_component_mut(torpedo).timeline_mut().add(event);
    model.vessel_component_mut(torpedo).timeline_mut().last_event().unwrap().as_start_burn().unwrap().adjust(&mut model, vec2(-353.0, 50.0));

    model.update(0.1);

    model.vessel_component_mut(torpedo).set_target(Some(spacecraft_1));

    let event = TimelineEvent::EnableGuidance(EnableGuidanceEvent::new(&mut model, torpedo, 716_560.0));
    model.vessel_component_mut(torpedo).timeline_mut().add(event);

    let event = TimelineEvent::EnableGuidance(EnableGuidanceEvent::new(&mut model, torpedo, 720_720.0));
    model.vessel_component_mut(torpedo).timeline_mut().add(event);

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
