use eframe::egui::{Context, Key};

use crate::events::Event;
use crate::game::underlay::selected::Selected;

use super::Scene;

pub fn update(view: &mut Scene, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update keyboard");

    context.input(|input| {
        if input.key_pressed(Key::R) {
            view.camera.reset_panning();
        }

        if input.key_pressed(Key::Space) {
            events.push(Event::TogglePaused);
        }

        if input.key_pressed(Key::PlusEquals) {
            events.push(Event::IncreaseTimeStepLevel);
        }

        if input.key_pressed(Key::Minus) {
            events.push(Event::DecreaseTimeStepLevel);
        }

        if input.key_pressed(Key::F12) {
            view.debug_window_open = !view.debug_window_open;
        }

        if input.key_pressed(Key::Delete) {
            match view.selected {
                Selected::Burn { entity, time: _, state: _ }
                    | Selected::FireTorpedo { entity, time: _, state: _ } => events.push(Event::CancelLastTimelineEvent { entity }),
                Selected::None 
                    | Selected::Orbitable(orbitable)
                    | Selected::Vessel(vessel) 
                    | Selected::Point { entity, time } => (),
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