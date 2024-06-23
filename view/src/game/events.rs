#[allow(clippy::wildcard_imports)]
use model::*;
use log::debug;
use nalgebra_glm::DVec2;
use transfer_window_model::{components::vessel_component::ship::ship_slot::{ShipSlot, ShipSlotLocation}, storage::entity_allocator::Entity};

use super::{debug::DebugWindowTab, overlay::vessel_editor::VesselEditor, selected::Selected, View};

mod model;
mod view;

#[derive(Debug)]
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
    SetSlot { entity: Entity, slot_location: ShipSlotLocation, slot: ShipSlot },
    CreateFireTorpedo { entity: Entity, slot_location: ShipSlotLocation, time: f64 },
    AdjustFireTorpedo { entity: Entity, time: f64, amount: DVec2 },
    CreateGuidance { entity: Entity, time: f64, },
    CancelLastTimelineEvent { entity: Entity },
    CancelCurrentSegment { entity: Entity },
    Dock { station: Entity, entity: Entity },
    Undock { station: Entity, entity: Entity },
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
}

impl View {
    pub(crate) fn handle_events(&mut self) {
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
                ModelEvent::SetSlot { entity, slot_location, slot } => set_slot(self, entity, slot_location, slot),
                ModelEvent::CreateFireTorpedo { entity, slot_location, time } => create_fire_torpedo(self, entity, slot_location, time),
                ModelEvent::AdjustFireTorpedo { entity, time, amount } => adjust_fire_torpedo(self, entity, time, amount),
                ModelEvent::CancelLastTimelineEvent { entity } => cancel_last_event(self, entity),
                ModelEvent::CreateGuidance { entity, time } => enable_torpedo_guidance(self, entity, time),
                ModelEvent::CancelCurrentSegment { entity } => cancel_current_segment(self, entity),
                ModelEvent::Dock { station, entity } => dock(self, station, entity),
                ModelEvent::Undock { station, entity } => undock(self, station, entity),
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
            }
        }
    }
}