use eframe::{egui::{Align2, Button, Context, Window}, epaint};
use transfer_window_model::{components::path_component::segment::Segment, Model};

use crate::{events::Event, game::{underlay::selected::Selected, util::format_time, Scene}};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::EnableGuidance { entity, time } = view.selected.clone() else { 
        return
    };

    let Some(Segment::Guidance(_)) = model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };
    
    Window::new("Selected guidance")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(model.name_component(entity).name());
            ui.label("T-".to_string() + format_time(time - model.time()).as_str());

            if ui.button("Warp here").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            let can_delete = model.event_at_time(entity, time).can_delete(model);
            let delete_button = Button::new("Cancel");
            if ui.add_enabled(can_delete, delete_button).clicked() {
                if model.vessel_component(entity).timeline().last_event().unwrap().is_intercept() {
                    // also cancel intercept
                    events.push(Event::CancelLastTimelineEvent { entity });
                }
                events.push(Event::CancelLastTimelineEvent { entity });
                view.selected = Selected::None;
            }
        });
}
