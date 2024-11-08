use eframe::{egui::{Align2, Ui, Window}, epaint};
use transfer_window_model::{components::vessel_component::faction::Faction, model::state_query::StateQuery, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_select_vessel, draw_warp_to}, labels::{draw_info_at_time, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::{turn::draw_turn_labels, vessel::visual_timeline::draw_visual_timeline};

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

fn draw_turn(view: &View, ui: &mut Ui, entity: Entity, time: f64) {
    let snapshot = view.model.snapshot_at_observe(time, Faction::Player);
    let turn = snapshot.turn(entity);
    draw_turn_labels(ui, turn.fuel_burnt(), turn.angle(), turn.duration());
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update turn point");
    let Selected::TurnPoint { entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected point")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Turn");
        draw_time_until(view, ui, time);
        draw_controls(view, entity, ui, time);
        draw_info_at_time(view, ui, entity, time);
        draw_turn(view, ui, entity, time);
        draw_visual_timeline(view, ui, entity, time, true);
    });
}
