use std::fs;

use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::vessel_component::{ship::ship_slot::{ShipSlot, ShipSlotLocation}, station::{DockingPortLocation, ResourceTransferDirection}, timeline::{enable_guidance::EnableGuidanceEvent, fire_torpedo::FireTorpedoEvent, start_burn::StartBurnEvent, TimelineEvent}}, storage::entity_allocator::Entity};

use crate::game::View;

pub fn save_game(view: &View, name: &str) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Save game");


    let serialized = view.model.serialize();
    let Ok(serialized) = serialized else {
        error!("Failed to handle save_game; error while serializing: {}", serialized.err().unwrap());
        return;
    };

    if let Err(error) = fs::write("saves/".to_string() + name + ".json", serialized) {
        error!("Failed to handle save_game; error while saving: {}", error);
    }
}
pub fn toggle_paused(view: &mut View,) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Toggle paused");
    view.model.toggle_paused();
}

pub fn increase_time_step_level(view: &mut View,) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Increase time step level");
    view.model.increase_time_step_level();
}

pub fn decrease_time_step_level(view: &mut View,) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Decrease time step level");
    view.model.decrease_time_step_level();
}

pub fn start_warp(view: &mut View, end_time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Start warp");
    view.model.start_warp(end_time);
}

pub fn cancel_last_event(view: &mut View, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Cancel last event");
    view.model.cancel_last_event(entity);
}

pub fn create_burn(view: &mut View, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Create burn");
    let event = TimelineEvent::Burn(StartBurnEvent::new(&mut view.model, entity, time));
    view.model.add_event(entity, event);
}

pub fn adjust_burn(view: &mut View, entity: Entity, time: f64, amount: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Adjust burn");
    view.model.timeline_event_at_time(entity, time)
        .as_start_burn()
        .unwrap()
        .adjust(&mut view.model, amount);
}

pub fn set_target(view: &mut View, entity: Entity, target: Option<Entity>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set target");
    view.model.vessel_component_mut(entity).set_target(target);
}

pub fn set_slot(view: &mut View, entity: Entity, location: ShipSlotLocation, slot: ShipSlot) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Set slot");
    view.model.set_slot(entity, location, slot);
}

pub fn create_fire_torpedo(view: &mut View, entity: Entity, location: ShipSlotLocation, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Fire torpedo");
    let event = TimelineEvent::FireTorpedo(FireTorpedoEvent::new(&mut view.model, entity, time, location));
    view.model.add_event(entity, event);
}

pub fn adjust_fire_torpedo(view: &mut View, entity: Entity, time: f64, amount: DVec2) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Adjust burn");
    view.model.timeline_event_at_time(entity, time)
        .as_fire_torpedo()
        .unwrap()
        .adjust(&mut view.model, amount);
}

pub fn enable_torpedo_guidance(view: &mut View, entity: Entity, time: f64) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Enable torpedo guidance");
    let event = TimelineEvent::EnableGuidance(EnableGuidanceEvent::new(&mut view.model, entity, time));
    view.model.add_event(entity, event);
}

pub fn cancel_current_segment(view: &mut View, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Cancel current segment");
    view.model.recompute_entire_trajectory(entity);
}

pub fn dock(view: &mut View, station: Entity, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Dock");
    view.model.dock(station, entity);
}

pub fn undock(view: &mut View, station: Entity, entity: Entity) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Undock");
    view.model.undock(station, entity);
}

pub fn start_fuel_transfer(view: &mut View, station: Entity, location: DockingPortLocation, direction: ResourceTransferDirection) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Start fuel transfer");
    view.model.get_docking_port_mut(station, location).start_fuel_transfer(direction);
}

pub fn stop_fuel_transfer(view: &mut View, station: Entity, location: DockingPortLocation) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Stop fuel transfer");
    view.model.get_docking_port_mut(station, location).stop_fuel_transfer();
}

pub fn start_torpedo_transfer(view: &mut View, station: Entity, location: DockingPortLocation, direction: ResourceTransferDirection) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Start torpedo transfer");
    view.model.get_docking_port_mut(station, location).start_torpedo_transfer(direction);
}

pub fn stop_torpedo_transfer(view: &mut View, station: Entity, location: DockingPortLocation) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Stop torpedo transfer");
    view.model.get_docking_port_mut(station, location).stop_torpedo_transfer();
}
