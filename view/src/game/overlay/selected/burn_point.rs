use eframe::{egui::{Align2, Grid, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::Faction, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_select_vessel, draw_warp_to}, labels::{draw_altitude, draw_speed, draw_subtitle, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::{burn::draw_burn_labels, vessel::visual_timeline::draw_visual_timeline};

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

fn draw_info(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    draw_subtitle(ui, "Info");
    Grid::new("Selected point info").show(ui, |ui| {
        draw_altitude(view, ui, entity, time);
        draw_speed(view, ui, entity, time);
    });
}

fn draw_burn(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let burn = view.model.burn_at_time(entity, time, Some(Faction::Player));
    let max_dv = view.model.vessel_component(entity).max_dv().unwrap();
    let start_dv = burn.rocket_equation_function().remaining_dv();
    let end_dv = burn.final_rocket_equation_function().remaining_dv();
    let duration = burn.duration();
    draw_subtitle(ui, "Burn");
    draw_burn_labels(view, ui, max_dv, start_dv, end_dv, duration);
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update burn point");
    let Selected::BurnPoint { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Burn");
        draw_time_until(view, ui, time);
        draw_controls(view, entity, ui, time);
        draw_info(view, ui, entity, time);
        draw_burn(view, ui, entity, time);
        draw_visual_timeline(view, ui, entity, time, true);
    });
}