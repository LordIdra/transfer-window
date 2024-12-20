use std::fs;

use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::components::vessel_component::timeline::start_turn::StartTurnEvent;
use transfer_window_model::model::story_event::StoryEvent;
use transfer_window_model::storage::entity_builder::VesselBuilder;
use transfer_window_model::{model::time::TimeStep, components::vessel_component::{docking::{DockingPortLocation, ResourceTransferDirection}, timeline::{start_guidance::StartGuidanceEvent, fire_torpedo::FireTorpedoEvent, start_burn::StartBurnEvent, TimelineEvent}}, storage::entity_allocator::Entity};

use crate::game::View;

impl View {
    pub fn save_game(&mut self, name: &str) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Save game");


        let serialized = self.model.serialize();
        let Ok(serialized) = serialized else {
            error!("Failed to handle save_game; error while serializing: {}", serialized.err().unwrap());
            return;
        };

        if let Err(error) = fs::write("data/saves/".to_string() + name + ".json", serialized) {
            error!("Failed to handle save_game; error while saving: {}", error);
        }
    }
    pub fn toggle_paused(&mut self,) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Toggle paused");
        self.model.toggle_paused();
    }

    pub fn increase_time_step_level(&mut self,) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Increase time step level");
        self.model.increase_time_step_level();
    }

    pub fn decrease_time_step_level(&mut self,) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Decrease time step level");
        self.model.decrease_time_step_level();
    }

    pub fn start_warp(&mut self, end_time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Start warp");
        self.model.start_warp(end_time);
    }

    pub fn set_time_step(&mut self, time_step: TimeStep) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Set time step");
        self.model.set_time_step(time_step);
    }

    pub fn force_pause(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Force pause");
        self.model.force_pause();
    }

    pub fn force_unpause(&mut self) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Force unpause");
        self.model.force_unpause();
    }

    pub fn build_vessel(&mut self, vessel_builder: VesselBuilder) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Build vessel");
        vessel_builder.build(&mut self.model);
    }

    pub fn delete_vessel(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Delete vessel");
        self.model.deallocate(entity);
    }

    pub fn cancel_last_event(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Cancel last event");
        self.model.cancel_last_event(entity);
    }

    pub fn create_burn(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Create burn");
        let event = TimelineEvent::StartBurn(StartBurnEvent::new(&mut self.model, entity, time));
        self.model.add_event(entity, event);
    }

    pub fn adjust_burn(&mut self, entity: Entity, time: f64, amount: DVec2) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Adjust burn");
        self.model.start_burn_event_at_time(entity, time)
            .unwrap()
            .adjust(&mut self.model, amount);
    }

    pub fn create_turn(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Create turn");
        let event = TimelineEvent::StartTurn(StartTurnEvent::new(&mut self.model, entity, time));
        self.model.add_event(entity, event);
    }

    pub fn adjust_turn(&mut self, entity: Entity, time: f64, amount: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Adjust turn");
        self.model.start_turn_event_at_time(entity, time)
            .unwrap()
            .adjust(&mut self.model, amount);
    }

    pub fn set_target(&mut self, entity: Entity, target: Option<Entity>) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Set target");
        self.model.vessel_component_mut(entity).set_target(target);
        if let Some(target) = target {
            self.add_story_event(StoryEvent::SetTarget { entity, target });
        }
    }

    pub fn create_fire_torpedo(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Fire torpedo");
        let event = TimelineEvent::FireTorpedo(FireTorpedoEvent::new(&mut self.model, entity, time));
        self.model.add_event(entity, event);
    }

    pub fn adjust_fire_torpedo(&mut self, entity: Entity, time: f64, amount: DVec2) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Adjust fire torpedo");
        self.model.fire_torpedo_event_at_time(entity, time)
            .unwrap()
            .adjust(&mut self.model, amount);
    }

    pub fn enable_torpedo_guidance(&mut self, entity: Entity, time: f64) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Enable torpedo guidance");
        let event = TimelineEvent::StartGuidance(StartGuidanceEvent::new(&mut self.model, entity, time));
        self.model.add_event(entity, event);
    }

    pub fn cancel_current_segment(&mut self, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Cancel current segment");
        self.model.recompute_entire_trajectory(entity);
    }

    pub fn dock(&mut self, station: Entity, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Dock");
        self.model.dock(station, entity);
    }

    pub fn undock(&mut self, station: Entity, entity: Entity) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Undock");
        self.model.undock(station, entity);
    }

    pub fn start_fuel_transfer(&mut self, station: Entity, location: DockingPortLocation, direction: ResourceTransferDirection) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Start fuel transfer");
        self.model.docking_port_mut(station, location).docked_vessel_mut().start_fuel_transfer(direction);
    }

    pub fn stop_fuel_transfer(&mut self, station: Entity, location: DockingPortLocation) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Stop fuel transfer");
        self.model.docking_port_mut(station, location).docked_vessel_mut().stop_fuel_transfer();
    }

    pub fn start_torpedo_transfer(&mut self, station: Entity, location: DockingPortLocation, direction: ResourceTransferDirection) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Start torpedo transfer");
        self.model.docking_port_mut(station, location).docked_vessel_mut().start_torpedo_transfer(direction);
    }

    pub fn stop_torpedo_transfer(&mut self, station: Entity, location: DockingPortLocation) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Stop torpedo transfer");
        self.model.docking_port_mut(station, location).docked_vessel_mut().stop_torpedo_transfer();
    }
}
