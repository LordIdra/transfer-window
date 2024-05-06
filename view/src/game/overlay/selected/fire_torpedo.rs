use eframe::{egui::{Align2, Button, Context, Window}, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{underlay::selected::Selected, util::format_time, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Some(fire_torpedo_event) = model.fire_torpedo_event_at_time(entity, time) else {
        return;
    };
    
    Window::new("Torpedo launch")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            let burn = model.burn_starting_at_time(fire_torpedo_event.ghost(), fire_torpedo_event.burn_time());
            let dv = (burn.total_dv() * 10.0).round() / 10.0;
            ui.label(model.name_component(entity).name());
            ui.label("T-".to_string() + format_time(time - model.time()).as_str());
            ui.label(dv.to_string() + " ΔV");

            if ui.button("Warp to burn").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            let can_delete = model.can_modify_timeline_event(entity, time);
            let delete_button = Button::new("Cancel");
            if ui.add_enabled(can_delete, delete_button).clicked() {
                events.push(Event::CancelLastTimelineEvent { entity });
                view.selected = Selected::None;
            }
        });
}