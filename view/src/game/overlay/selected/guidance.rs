use eframe::{egui::{Align2, Context, RichText, Window}, epaint};
use transfer_window_model::{components::path_component::segment::Segment, Model};

use crate::{events::Event, game::{overlay::widgets::custom_image_button::CustomCircularImageButton, selected::Selected, util::format_time, Scene}, styles};

use super::burn::draw_burn_info;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update guidance");
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

                let enabled = model.can_warp_to(time);
                let button = CustomCircularImageButton::new(view.renderers.get_screen_texture_renderer("warp-here"), context.screen_rect(), 36.0)
                    .with_enabled(enabled)
                    .with_padding(8.0);
                if ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked() {
                    events.push(Event::StartWarp { end_time: time });
                }

                let enabled = model.timeline_event_at_time(entity, time).can_delete(model);
                let button = CustomCircularImageButton::new(view.renderers.get_screen_texture_renderer("cancel"), context.screen_rect(), 36.0)
                    .with_enabled(enabled)
                    .with_padding(8.0);
                if ui.add_enabled(enabled, button).on_hover_text("Cancel").clicked() {
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
            draw_burn_info(view, ui, context, max_dv, start_dv, end_dv, duration);
        });
}
