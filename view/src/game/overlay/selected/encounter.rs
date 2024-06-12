use eframe::{egui::{Align2, Grid, Window}, epaint};

use crate::{game::{events::ModelEvent, overlay::widgets::{buttons::draw_warp_to, labels::{draw_encounter_from, draw_encounter_to, draw_subtitle, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::vessel::visual_timeline::draw_visual_timeline;

fn draw_controls(ui: &mut eframe::egui::Ui, view: &View, time: f64) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if draw_warp_to(view, ui, time) {
            view.add_model_event(ModelEvent::StartWarp { end_time: time });
        }
    });
}

fn draw_info(ui: &mut eframe::egui::Ui, view: &View, from: transfer_window_model::storage::entity_allocator::Entity, to: transfer_window_model::storage::entity_allocator::Entity) {
    draw_subtitle(ui, "Info");
    Grid::new("Selected encounter info").show(ui, |ui| {
        draw_encounter_from(view, ui, from);
        draw_encounter_to(view, ui, to);
    });
}


pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update encounter");
    let Selected::Encounter { type_: _, entity, time, from, to } = view.selected.clone() else {
        return;
    };

    Window::new("Selected encounter")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Encounter");
        draw_time_until(view, ui, time);
        draw_controls(ui, view, time);
        draw_info(ui, view, from, to);
        draw_visual_timeline(view, ui, entity, time, false);
    });
}
