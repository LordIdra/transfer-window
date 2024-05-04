use eframe::{egui::{Align2, Button, Context, Window}, epaint};
use transfer_window_model::{components::path_component::segment::Segment, Model};

use crate::{events::Event, game::{underlay::selected::Selected, util::format_time, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Burn { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Segment::Burn(burn) = model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };
    
    Window::new("Selected burn")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let dv = (burn.total_dv() * 10.0).round() / 10.0;
            ui.label(model.name_component(entity).name());
            ui.label("T-".to_string() + format_time(time - model.time()).as_str());
            ui.label(dv.to_string() + " ΔV");

            if ui.button("Warp to burn").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            let can_delete = model.path_component(entity).final_burn().unwrap().start_point().time() == time;
            let delete_button = Button::new("Delete burn");
            if ui.add_enabled(can_delete, delete_button).clicked() {
                events.push(Event::DeleteBurn { entity, time });
                if let Selected::Burn { entity: _, time: _, state: _ } = view.selected {
                    view.selected = Selected::None;
                }
            }
        });
}
