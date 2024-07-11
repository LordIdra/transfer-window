#[allow(clippy::wildcard_imports)]
use log::debug;
use log::error;
use nalgebra_glm::DVec2;
use transfer_window_model::{api::{builder::VesselBuilder, time::TimeStep}, components::vessel_component::{docking::{DockingPortLocation, ResourceTransferDirection}, engine::EngineType, fuel_tank::FuelTankType, torpedo_launcher::TorpedoLauncherType, torpedo_storage::TorpedoStorageType}, storage::entity_allocator::Entity, story_event::StoryEvent};

use crate::game::selected::util::BurnState;
use crate::game::{overlay::{dialogue::Dialogue, objectives::Objective}, util::ApsisType};

use super::ViewConfig;
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
    SetTimeStep { time_step: TimeStep },
    BuildVessel { vessel_builder: VesselBuilder },
    DeleteVessel { entity: Entity },
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

#[derive(Debug, Clone)]
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
    StartObjective(&'static str),
    FinishObjective(&'static str),
    ToggleExitModal,
    SetConfig(ViewConfig),
}

impl View {
    pub(crate) fn handle_events(&mut self) {
        self.story.update(&self);
        self.story_events.lock().unwrap().clear();

        let model_events = self.model_events.clone();
        let mut model_events = model_events.lock().unwrap();
        model_events.reverse(); // process in the order they were added
        while let Some(event) = model_events.pop() {
            debug!("Handling model event {:?}", event);
            match event {
                ModelEvent::SaveGame { name } => self.save_game(name.as_str()),
                ModelEvent::TogglePaused => self.toggle_paused(),
                ModelEvent::IncreaseTimeStepLevel => self.increase_time_step_level(),
                ModelEvent::DecreaseTimeStepLevel => self.decrease_time_step_level(),
                ModelEvent::StartWarp { end_time } => self.start_warp(end_time),
                ModelEvent::SetTimeStep { time_step } => self.set_time_step(time_step),
                ModelEvent::BuildVessel { vessel_builder } => self.build_vessel(vessel_builder),
                ModelEvent::DeleteVessel { entity } => self.delete_vessel(entity),
                ModelEvent::CreateBurn { entity, time } => self.create_burn(entity, time),
                ModelEvent::AdjustBurn { entity, time, amount } => self.adjust_burn(entity, time, amount),
                ModelEvent::SetTarget { entity, target } => self.set_target(entity, target),
                ModelEvent::SetFuelTank { entity, type_ } => self.set_fuel_tank(entity, type_),
                ModelEvent::SetEngine { entity, type_ } => self.set_engine(entity, type_),
                ModelEvent::SetTorpedoStorage { entity, type_ } => self.set_torpedo_storage(entity, type_),
                ModelEvent::SetTorpedoLauncher { entity, type_ } => self.set_torpedo_launcher(entity, type_),
                ModelEvent::CreateFireTorpedo { entity, time } => self.create_fire_torpedo(entity, time),
                ModelEvent::AdjustFireTorpedo { entity, time, amount } => self.adjust_fire_torpedo(entity, time, amount),
                ModelEvent::CancelLastTimelineEvent { entity } => self.cancel_last_event(entity),
                ModelEvent::CreateGuidance { entity, time } => self.enable_torpedo_guidance(entity, time),
                ModelEvent::CancelCurrentSegment { entity } => self.cancel_current_segment(entity),
                ModelEvent::Dock { station, entity } => self.dock(station, entity),
                ModelEvent::Undock { station, entity } => self.undock(station, entity),
                ModelEvent::StartFuelTransfer { station, location, direction } => self.start_fuel_transfer(station, location, direction),
                ModelEvent::StopFuelTransfer { station, location } => self.stop_fuel_transfer(station, location),
                ModelEvent::StartTorpedoTransfer { station, location, direction } => self.start_torpedo_transfer(station, location, direction),
                ModelEvent::StopTorpedoTransfer { station, location } => self.stop_torpedo_transfer(station, location),
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
                ViewEvent::SetCameraFocus(focus) => {
                    self.add_story_event(StoryEvent::ChangeFocus(focus));
                    self.camera.set_focus(focus, self.model.absolute_position(focus));
                },
                ViewEvent::SetSelected(selected) => {
                    if self.config.can_select {
                        if matches!(selected, Selected::Apsis { type_: ApsisType::Apoapsis, .. }) {
                            self.add_story_event(StoryEvent::AnyApoapsisSelected);
                        }
                        if matches!(selected, Selected::OrbitPoint { .. }) {
                            self.add_story_event(StoryEvent::AnyOrbitPointSelected);
                        }
                        if let Selected::Vessel(entity) = selected {
                            self.add_story_event(StoryEvent::VesselSelected(entity));
                        }
                        if let Selected::Burn { state, .. } = &selected {
                            if matches!(state, BurnState::Adjusting) {
                                self.add_story_event(StoryEvent::StartBurnAdjust);
                            }
                        }
                        self.selected = selected;
                    }
                }
                ViewEvent::SetVesselEditor(vessel_editor) => self.vessel_editor = vessel_editor,
                ViewEvent::SetDebugWindowOpen(debug_window_open) => self.debug_window_open = debug_window_open,
                ViewEvent::SetDebugWindowTab(debug_window_tab) => self.debug_window_tab = debug_window_tab,
                ViewEvent::IconHovered => self.pointer_over_icon = true,
                ViewEvent::ToggleRightClickMenu(entity) => self.toggle_right_click_menu(entity),
                ViewEvent::HideRightClickMenu => self.right_click_menu = None,
                ViewEvent::ShowDialogue(dialogue) => self.dialogue = Some(dialogue),
                ViewEvent::CloseDialogue => self.dialogue = None,
                ViewEvent::StartObjective(objective) => self.objectives.push(Objective::new(objective)),
                ViewEvent::FinishObjective(objective) => {
                    self.objectives.iter_mut()
                        .find(|x| x.objective() == objective)
                        .map_or_else(|| error!("Attempt to complete nonexistent objective {}", objective), Objective::set_complete);
                },
                ViewEvent::ToggleExitModal => self.exit_modal_open = !self.exit_modal_open,
                ViewEvent::SetConfig(config) => self.config = config,
            }
        }
    }
}