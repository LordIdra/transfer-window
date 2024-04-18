use eframe::{egui::{Align2, Button, Context, Window}, epaint};
use transfer_window_model::{components::trajectory_component::segment::Segment, Model};

use crate::{events::Event, game::{underlay::selected::Selected, util::format_time, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Burn { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Segment::Burn(burn) = model.get_trajectory_component(entity).get_last_segment_at_time(time) else {
        return;
    };
    
    Window::new("Selected burn")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let dv = (burn.get_total_dv() * 10.0).round() / 10.0;
            ui.label(model.get_name_component(entity).get_name());
            ui.label("T-".to_string() + format_time(time - model.get_time()).as_str());
            ui.label(dv.to_string() + " ΔV");

            if ui.button("Warp to burn").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            let can_delete = model.get_trajectory_component(entity).get_final_burn().unwrap().get_start_point().get_time() == time;
            let delete_button = Button::new("Delete burn");
            if ui.add_enabled(can_delete, delete_button).clicked() {
                events.push(Event::DeleteBurn { entity, time });
                if let Selected::Burn { entity: _, time: _, state: _ } = view.selected {
                    view.selected = Selected::None;
                }
            }
        });
}
