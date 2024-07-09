#[allow(clippy::wildcard_imports)]
use model::*;
use log::debug;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::vessel_component::{docking::{DockingPortLocation, ResourceTransferDirection}, engine::EngineType, fuel_tank::FuelTankType, torpedo_launcher::TorpedoLauncherType, torpedo_storage::TorpedoStorageType}, storage::entity_allocator::Entity};

use crate::game::overlay::dialogue::Dialogue;

use super::{debug::DebugWindowTab, overlay::vessel_editor::VesselEditor, selected::Selected, View};

mod model;
mod view;

#[derive(Debug, Clone)]
pub enum ModelEvent {
    #[allow(unused)]
    SaveGame { name: String },
    TogglePaused,
    IncreaseTimeStepLevel,
    DecreaseTimeStepLevel,
    StartWarp { end_time: f64 },
    CreateBurn { entity: Entity, time: f64 },
    AdjustBurn { entity: Entity, time: f64, amount: DVec2 },
    SetTarget { entity: Entity, target: Option<Entity> },
    SetFuelTank { entity: Entity, type_: Option<FuelTankType> },
    SetEngine { entity: Entity, type_: Option<EngineType> },
    SetTorpedoStorage { entity: Entity, type_: Option<TorpedoStorageType> },
    SetTorpedoLauncher { entity: Entity, type_: Option<TorpedoLauncherType> },
    CreateFireTorpedo { entity: Entity, time: f64 },
    AdjustFireTorpedo { entity: Entity, time: f64, amount: DVec2 },
    CreateGuidance { entity: Entity, time: f64, },
    CancelLastTimelineEvent { entity: Entity },
    CancelCurrentSegment { entity: Entity },
    Dock { station: Entity, entity: Entity },
    Undock { station: Entity, entity: Entity },
    StartFuelTransfer { station: Entity, location: DockingPortLocation, direction: ResourceTransferDirection },
    StopFuelTransfer { station: Entity, location: DockingPortLocation },
    StartTorpedoTransfer { station: Entity, location: DockingPortLocation, direction: ResourceTransferDirection },
    StopTorpedoTransfer { station: Entity, location: DockingPortLocation },
}

#[derive(Debug)]
pub enum ViewEvent {
    ResetCameraPanning,
    PanCamera(DVec2),
    SetCameraZoom(f64),
    SetCameraFocus(Entity),
    SetSelected(Selected),
    SetVesselEditor(Option<VesselEditor>),
    SetDebugWindowOpen(bool),
    SetDebugWindowTab(DebugWindowTab),
    IconHovered,
    ToggleRightClickMenu(Entity),
    HideRightClickMenu,
    ShowDialogue(Dialogue),
    CloseDialogue,
}

impl View {
    pub(crate) fn handle_events(&mut self) {
        let (new_model_events, new_view_events) = self.story.update(&self.story_events.lock().unwrap());
        self.model_events.lock().unwrap().extend(new_model_events);
        self.view_events.lock().unwrap().extend(new_view_events);
        self.story_events.lock().unwrap().clear();

        let model_events = self.model_events.clone();
        let mut model_events = model_events.lock().unwrap();
        model_events.reverse(); // process in the order they were added
        while let Some(event) = model_events.pop() {
            debug!("Handling model event {:?}", event);
            match event {
                ModelEvent::SaveGame { name } => save_game(self, name.as_str()),
                ModelEvent::TogglePaused => toggle_paused(self),
                ModelEvent::IncreaseTimeStepLevel => increase_time_step_level(self),
                ModelEvent::DecreaseTimeStepLevel => decrease_time_step_level(self),
                ModelEvent::StartWarp { end_time } => start_warp(self, end_time),
                ModelEvent::CreateBurn { entity, time } => create_burn(self, entity, time),
                ModelEvent::AdjustBurn { entity, time, amount } => adjust_burn(self, entity, time, amount),
                ModelEvent::SetTarget { entity, target } => set_target(self, entity, target),
                ModelEvent::SetFuelTank { entity, type_ } => set_fuel_tank(self, entity, type_),
                ModelEvent::SetEngine { entity, type_ } => set_engine(self, entity, type_),
                ModelEvent::SetTorpedoStorage { entity, type_ } => set_torpedo_storage(self, entity, type_),
                ModelEvent::SetTorpedoLauncher { entity, type_ } => set_torpedo_launcher(self, entity, type_),
                ModelEvent::CreateFireTorpedo { entity, time } => create_fire_torpedo(self, entity, time),
                ModelEvent::AdjustFireTorpedo { entity, time, amount } => adjust_fire_torpedo(self, entity, time, amount),
                ModelEvent::CancelLastTimelineEvent { entity } => cancel_last_event(self, entity),
                ModelEvent::CreateGuidance { entity, time } => enable_torpedo_guidance(self, entity, time),
                ModelEvent::CancelCurrentSegment { entity } => cancel_current_segment(self, entity),
                ModelEvent::Dock { station, entity } => dock(self, station, entity),
                ModelEvent::Undock { station, entity } => undock(self, station, entity),
                ModelEvent::StartFuelTransfer { station, location, direction } => start_fuel_transfer(self, station, location, direction),
                ModelEvent::StopFuelTransfer { station, location } => stop_fuel_transfer(self, station, location),
                ModelEvent::StartTorpedoTransfer { station, location, direction } => start_torpedo_transfer(self, station, location, direction),
                ModelEvent::StopTorpedoTransfer { station, location } => stop_torpedo_transfer(self, station, location),
            }
        }

        let view_events = self.view_events.clone();
        let mut view_events = view_events.lock().unwrap();
        view_events.reverse(); // process in the order they were added
        while let Some(event) = view_events.pop() {
            debug!("Handling view event {:?}", event);
            match event {
                ViewEvent::ResetCameraPanning => self.camera.reset_panning(),
                ViewEvent::PanCamera(amount) => self.camera.pan(amount),
                ViewEvent::SetCameraZoom(zoom) => self.camera.set_zoom(zoom),
                ViewEvent::SetCameraFocus(focus) => self.camera.set_focus(focus, self.model.absolute_position(focus)),
                ViewEvent::SetSelected(selected) => self.selected = selected,
                ViewEvent::SetVesselEditor(vessel_editor) => self.vessel_editor = vessel_editor,
                ViewEvent::SetDebugWindowOpen(debug_window_open) => self.debug_window_open = debug_window_open,
                ViewEvent::SetDebugWindowTab(debug_window_tab) => self.debug_window_tab = debug_window_tab,
                ViewEvent::IconHovered => self.pointer_over_icon = true,
                ViewEvent::ToggleRightClickMenu(entity) => self.toggle_right_click_menu(entity),
                ViewEvent::HideRightClickMenu => self.right_click_menu = None,
                ViewEvent::ShowDialogue(dialogue) => self.dialogue = Some(dialogue),
                ViewEvent::CloseDialogue => self.dialogue = None,
            }
        }
    }
}