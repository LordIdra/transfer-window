use eframe::{egui::{Align2, Ui, Window}, epaint};
use transfer_window_model::storage::entity_allocator::Entity;

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::draw_select_vessel, custom_image_button::CustomCircularImageButton, labels::{draw_subtitle, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::{burn::draw_burn_labels, vessel::visual_timeline::draw_visual_timeline};

fn draw_controls(view: &View, ui: &mut Ui, time: f64, entity: Entity) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);
        ui.set_height(36.0);

        if draw_select_vessel(view, ui, entity) {
            view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
        }

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
}


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

    let guidance = view.model.path_component(entity).future_segment_starting_at_time(time).unwrap().as_guidance().unwrap();
    let max_dv = view.model.vessel_component(entity).max_dv();
    let start_dv = guidance.start_rocket_equation_function().remaining_dv();
    let end_dv = guidance.final_rocket_equation_function().remaining_dv();
    let duration = guidance.duration();
    
    Window::new("Selected guidance")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Guidance");
        draw_time_until(view, ui, time);
        draw_controls(view, ui, time, entity);
        draw_subtitle(ui, "Guidance");
        draw_burn_labels(view, ui, max_dv, start_dv, end_dv, duration);
        draw_visual_timeline(view, ui, entity, time, false);
    });
}
