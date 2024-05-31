use eframe::{egui::{Align2, Context, ImageButton, RichText, Window}, epaint};
use transfer_window_model::{components::path_component::segment::Segment, Model};

use crate::{events::Event, game::{underlay::selected::Selected, util::format_time, Scene}, styles};

use super::burn::draw_burn_info;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::EnableGuidance { entity, time } = view.selected.clone() else { 
        return
    };

    let Some(Segment::Guidance(guidance)) = model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };
    
    Window::new("Selected guidance")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(RichText::new("Guidance").size(20.0).monospace().strong());
            let text = format!("T-{}", format_time(time - model.time()));
            ui.label(RichText::new(text).weak());

            ui.horizontal(|ui| {
                styles::SelectedMenuButton::apply(ui);
                ui.set_height(36.0);

                let button = ImageButton::new(view.resources.texture_image("warp-here"));
                let can_warp = model.can_warp_to(time);
                if ui.add_enabled(can_warp, button).on_hover_text("Warp here").clicked() {
                    events.push(Event::StartWarp { end_time: time });
                }

                let can_delete = model.timeline_event_at_time(entity, time).can_delete(model);
                let button = ImageButton::new(view.resources.texture_image("cancel"));
                if ui.add_enabled(can_delete, button).on_hover_text("Cancel").clicked() {
                    if model.vessel_component(entity).timeline().last_event().unwrap().is_intercept() {
                        // also cancel intercept
                        events.push(Event::CancelLastTimelineEvent { entity });
                    }
                    events.push(Event::CancelLastTimelineEvent { entity });
                    view.selected = Selected::None;
                }
            });

            let max_dv = model.vessel_component(entity).max_dv().unwrap();
            let start_dv = guidance.rocket_equation_function().remaining_dv();
            let end_dv = guidance.final_rocket_equation_function().remaining_dv();
            let duration = guidance.duration();
            draw_burn_info(view, ui, max_dv, start_dv, end_dv, duration);
        });
}
