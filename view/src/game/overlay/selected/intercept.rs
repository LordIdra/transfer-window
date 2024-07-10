use eframe::egui::{Align2, Ui, Window};
use eframe::epaint;
use transfer_window_model::storage::entity_allocator::Entity;

use super::vessel::visual_timeline::draw_visual_timeline;
use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::widgets::buttons::{draw_select_vessel, draw_warp_to};
use crate::game::overlay::widgets::labels::{draw_time_until, draw_title};
use crate::game::selected::Selected;
use crate::game::View;
use crate::styles;

fn draw_controls(ui: &mut Ui, view: &View, entity: Entity, time: f64) {
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

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update intercept");
    let Selected::Intercept { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected intercept")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            draw_title(ui, "Intercept");
            draw_time_until(view, ui, time);
            draw_controls(ui, view, entity, time);
            draw_visual_timeline(view, ui, entity, time, false);
        });
}
