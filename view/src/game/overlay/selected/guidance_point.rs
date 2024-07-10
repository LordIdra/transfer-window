use eframe::egui::{Align2, Ui, Window};
use eframe::epaint;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::storage::entity_allocator::Entity;

use super::burn::draw_burn_labels;
use super::vessel::visual_timeline::draw_visual_timeline;
use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::widgets::buttons::{draw_select_vessel, draw_warp_to};
use crate::game::overlay::widgets::labels::{
    draw_info_at_time, draw_subtitle, draw_time_until, draw_title,
};
use crate::game::selected::Selected;
use crate::game::View;
use crate::styles;

fn draw_controls(view: &View, entity: Entity, ui: &mut Ui, time: f64) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if draw_select_vessel(view, ui, entity) {
            view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
        }

        if draw_warp_to(view, ui, time) {
            view.add_model_event(ModelEvent::StartWarp { end_time: time });
        }
    });
}

fn draw_guidance(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let guidance = view
        .model
        .guidance_at_time(entity, time, Some(Faction::Player));
    let max_dv = view.model.vessel_component(entity).max_dv();
    let start_dv = guidance.rocket_equation_function().remaining_dv();
    let end_dv = guidance.final_rocket_equation_function().remaining_dv();
    let duration = guidance.duration();
    draw_subtitle(ui, "Guidance");
    draw_burn_labels(view, ui, max_dv, start_dv, end_dv, duration);
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update guidance point");
    let Selected::GuidancePoint { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            draw_title(ui, "Guidance");
            draw_time_until(view, ui, time);
            draw_controls(view, entity, ui, time);
            draw_info_at_time(view, ui, entity, time);
            draw_guidance(view, ui, entity, time);
            draw_visual_timeline(view, ui, entity, time, true);
        });
}
