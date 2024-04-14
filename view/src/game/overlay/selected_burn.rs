use eframe::{egui::{Align2, Context, Window}, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{underlay::selected::Selected, util::format_time, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Burn { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    Window::new("Selected burn")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(30.0, 30.0))
        .show(context, |ui| {
            ui.label(model.get_name_component(entity).get_name());
            ui.label("T-".to_string() + format_time(time - model.get_time()).as_str());
            if ui.button("Warp to burn").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }
        });
}
