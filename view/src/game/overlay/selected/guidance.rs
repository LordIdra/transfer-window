use eframe::{egui::{Align2, RichText, Window}, epaint};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::custom_image_button::CustomCircularImageButton, selected::Selected, util::format_time, View}, styles};

use super::{burn::draw_burn_info, vessel::visual_timeline};

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update guidance");
    let Selected::EnableGuidance { entity, time } = view.selected.clone() else { 
        return
    };

    let Some(segment) = view.model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };

    if !segment.is_guidance() {
        return;
    }
    
    Window::new("Selected guidance")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        ui.label(RichText::new("Guidance").size(20.0).monospace().strong());
        let text = format!("T-{}", format_time(time - view.model.time()));
        ui.label(RichText::new(text).weak());

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);
            ui.set_height(36.0);

            let enabled = view.model.can_warp_to(time);
            let button = CustomCircularImageButton::new(view, "warp-here", 36.0)
                .with_enabled(enabled)
                .with_padding(8.0);
            if ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked() {
                view.add_model_event(ModelEvent::StartWarp { end_time: time });
            }

            let enabled = view.model.timeline_event_at_time(entity, time).can_delete(&view.model);
            let button = CustomCircularImageButton::new(view, "cancel", 36.0)
                .with_enabled(enabled)
                .with_padding(8.0);
            if ui.add_enabled(enabled, button).on_hover_text("Cancel").clicked() {
                if view.model.vessel_component(entity).timeline().last_event().unwrap().is_intercept() {
                    // also cancel intercept
                    view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
                }
                view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
                view.add_view_event(ViewEvent::SetSelected(Selected::None));
            }
        });

        let guidance = view.model.path_component(entity).future_segment_starting_at_time(time).unwrap().as_guidance().unwrap();
        let max_dv = view.model.vessel_component(entity).max_dv().unwrap();
        let start_dv = guidance.rocket_equation_function().remaining_dv();
        let end_dv = guidance.final_rocket_equation_function().remaining_dv();
        let duration = guidance.duration();
        draw_burn_info(view, ui, max_dv, start_dv, end_dv, duration);

        visual_timeline::draw(view, ui, entity, time, false);
    });
}
