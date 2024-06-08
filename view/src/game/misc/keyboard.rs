use eframe::egui::Key;

use crate::game::{events::Event, selected::Selected};

use super::View;

pub fn update(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update keyboard");

    view.context.input(|input| {
        if input.key_pressed(Key::Space) {
            view.events.push(Event::TogglePaused);
        }

        if input.key_pressed(Key::Equals) {
            view.events.push(Event::IncreaseTimeStepLevel);
        }

        if input.key_pressed(Key::Minus) {
            view.events.push(Event::DecreaseTimeStepLevel);
        }

        if input.key_pressed(Key::F12) {
            view.debug_window_open = !view.debug_window_open;
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
                    | Selected::Point { .. } => (),
                Selected::Burn { entity, .. }
                    | Selected::FireTorpedo { entity, .. } 
                    | Selected::EnableGuidance { entity, .. } => {
                        view.events.push(Event::CancelLastTimelineEvent { entity });
                        view.selected = Selected::None;
                }
            }
        }

        if input.key_pressed(Key::Escape) {
            if view.vessel_editor.is_some() {
                view.vessel_editor = None;
            } else {
                view.selected = Selected::None;
            }
        }
    });
}