use eframe::{egui::{Align2, Grid, Ui, Window}, epaint};
use transfer_window_model::{api::encounters::EncounterType, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_select_vessel, draw_warp_to}, labels::{draw_encounter_from, draw_encounter_to, draw_subtitle, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::vessel::visual_timeline::draw_visual_timeline;

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

fn draw_info(ui: &mut Ui, view: &View, from: transfer_window_model::storage::entity_allocator::Entity, to: transfer_window_model::storage::entity_allocator::Entity) {
    draw_subtitle(ui, "Info");
    Grid::new("Selected encounter info").show(ui, |ui| {
        draw_encounter_from(view, ui, from);
        draw_encounter_to(view, ui, to);
    });
}


pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update encounter");
    let Selected::Encounter { type_, entity, time, from, to } = view.selected.clone() else {
        return;
    };

    Window::new("Selected encounter")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        let title = match type_ {
            EncounterType::Entrance =>  view.model.name_component(to).name() + "Entrance",
            EncounterType::Exit => view.model.name_component(from).name() + "Exit",
        };
        draw_title(ui, &title);
        draw_time_until(view, ui, time);
        draw_controls(ui, view, entity, time);
        draw_info(ui, view, from, to);
        draw_visual_timeline(view, ui, entity, time, false);
    });
}
