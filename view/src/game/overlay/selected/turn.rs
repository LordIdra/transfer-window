use eframe::{egui::{Align2, Grid, Ui, Window}, epaint};
use transfer_window_model::storage::entity_allocator::Entity;

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::draw_select_vessel, custom_image_button::CustomCircularImageButton, labels::{draw_key, draw_subtitle, draw_time_until, draw_title, draw_value}}, selected::Selected, util::format_time, View}, styles};

use super::vessel::visual_timeline::draw_visual_timeline;

pub fn draw_turn_labels(ui: &mut Ui, fuel_burnt: f64, angle: f64, duration: f64) {
    draw_subtitle(ui, "Turn");
    Grid::new("Tun info grid").show(ui, |ui| {
        draw_key(ui, "Duration");
        draw_value(ui, &format_time(duration));
        ui.end_row();

        draw_key(ui, "Turn angle");
        draw_value(ui, &format!("{:.1}°", f64::to_degrees(angle)));
        ui.end_row();

        draw_key(ui, "Fuel usage");
        draw_value(ui, &format!("{fuel_burnt:.1} kg"));
        ui.end_row();
    });
}

fn draw_controls(ui: &mut Ui, view: &View, time: f64, entity: Entity) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);
        ui.set_height(36.0);

        if draw_select_vessel(view, ui, entity) {
            view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
        }

        let enabled = view.model.can_warp_to(time);
        let button = CustomCircularImageButton::new(view, "warp-here", 36)
            .with_enabled(enabled);
        if ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked() {
            view.add_model_event(ModelEvent::StartWarp { end_time: time });
        }

        let enabled = view.model.can_delete_event_at_time(entity, time);
        let button = CustomCircularImageButton::new(view, "cancel", 36)
            .with_enabled(enabled);
        if ui.add_enabled(enabled, button).on_hover_text("Cancel").clicked() {
            view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
            view.add_view_event(ViewEvent::SetSelected(Selected::None));
        }
    });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update turn");
    let Selected::Turn { entity, time, } = view.selected.clone() else { 
        return
    };

    let Some(segment) = view.model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };

    if !segment.is_turn() {
        return;
    }

    let turn = view.model.turn_starting_at_time(entity, time);
    
    Window::new("Selected turn")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Turn");
        draw_time_until(view, ui, time);
        draw_controls(ui, view, time, entity);
        draw_turn_labels(ui, turn.fuel_burnt(), turn.angle(), turn.remaining_time());
        draw_visual_timeline(view, ui, entity, time, false);
    });
}
