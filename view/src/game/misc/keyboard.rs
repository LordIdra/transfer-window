use eframe::egui::Key;

use crate::game::{events::{ModelEvent, ViewEvent}, selected::Selected};

use super::View;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update keyboard");

    view.context.input(|input| {
        if input.key_pressed(Key::Space) {
            view.add_model_event(ModelEvent::TogglePaused);
        }

        if input.key_pressed(Key::Equals) {
            view.add_model_event(ModelEvent::IncreaseTimeStepLevel);
        }

        if input.key_pressed(Key::Minus) {
            view.add_model_event(ModelEvent::DecreaseTimeStepLevel);
        }

        if input.key_pressed(Key::F12) {
            view.add_view_event(ViewEvent::SetDebugWindowOpen(!view.debug_window_open));
        }

        if input.key_pressed(Key::Delete) {
            match view.selected {
                Selected::None 
                    | Selected::Orbitable(_)
                    | Selected::Vessel(_) 
                    | Selected::Apsis { .. }
                    | Selected::Approach { .. }
                    | Selected::Encounter { .. }
                    | Selected::Intercept { .. }
                    | Selected::BurnPoint { .. }
                    | Selected::GuidancePoint { .. }
                    | Selected::OrbitPoint { .. } => (),
                Selected::Burn { entity, .. }
                    | Selected::FireTorpedo { entity, .. } 
                    | Selected::EnableGuidance { entity, .. } => {
                        view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
                        view.add_view_event(ViewEvent::SetSelected(Selected::None));
                }
            }
        }

        if input.key_pressed(Key::Escape) {
            if view.vessel_editor.is_some() {
                view.add_view_event(ViewEvent::SetVesselEditor(None));
            } else {
                if matches!(view.selected, Selected::None) {
                    view.add_view_event(ViewEvent::ToggleExitModal);
                } else {
                    view.add_view_event(ViewEvent::SetSelected(Selected::None));
                }
            }
        }
    });
}