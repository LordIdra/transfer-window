use eframe::{egui::{Align2, Context, Window}, epaint};

use crate::{events::Event, game::{underlay::selected::Selected, Scene}};

pub fn update(view: &mut Scene, context: &Context, events: &mut Vec<Event>) {
    let Selected::Vessel(entity) = view.selected.clone() else { 
        return
    };

    Window::new("Selected vessel")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            if ui.button("Edit").clicked() {
                view.vessel_editor = Some(entity);
            }
            if ui.button("Yeet").clicked() {
                events.push(Event::Destroy { entity });
            }
        });
}
