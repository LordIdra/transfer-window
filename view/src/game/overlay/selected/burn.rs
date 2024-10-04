use eframe::{egui::{Align2, Color32, Grid, Ui, Window}, epaint};
use transfer_window_model::storage::entity_allocator::Entity;

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{bars::{draw_filled_bar, FilledBar}, buttons::draw_select_vessel, custom_image::CustomImage, custom_image_button::CustomCircularImageButton, labels::{draw_key, draw_time_until, draw_title, draw_value}}, selected::Selected, util::format_time_with_millis, View}, styles};

use super::vessel::visual_timeline::draw_visual_timeline;

pub fn draw_burn_labels(view: &View, ui: &mut Ui, max_dv: f64, start_dv: f64, end_dv: f64, duration: f64) {
    let burnt_dv = start_dv - end_dv;

    let start_dv_proportion = (start_dv / max_dv) as f32;
    let end_dv_proportion = (end_dv / max_dv) as f32;

    let start_bar = FilledBar::new(Color32::DARK_RED, start_dv_proportion);
    let end_bar = FilledBar::new(Color32::WHITE, end_dv_proportion);

    ui.horizontal(|ui| {
        draw_key(ui, "ΔV");
        draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![start_bar, end_bar]);
    });

    Grid::new("DV grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "duration", 20);
            ui.add(image);
            draw_key(ui, "Duration");
        });
        draw_value(ui, &format_time_with_millis(duration));
        ui.end_row();

        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "burn-start", 20);
            ui.add(image);
            draw_key(ui, "ΔV start");
        });
        draw_value(ui, &format!("{start_dv:.1}"));
        ui.end_row();

        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "burn-burnt", 20);
            ui.add(image);
            draw_key(ui, "ΔV burnt");
        });
        draw_value(ui, &format!("{burnt_dv:.1}"));
        ui.end_row();

        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "burn-end", 20);
            ui.add(image);
            draw_key(ui, "ΔV end");
        });
        draw_value(ui, &format!("{end_dv:.1}"));
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
    let _span = tracy_client::span!("Update burn");
    let Selected::Burn { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Some(segment) = view.model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };

    if !segment.is_burn() {
        return;
    }

    let vessel_component = view.model.vessel_component(entity);
    let burn = view.model.burn_starting_at_time(entity, time);
    let max_dv = vessel_component.max_dv();
    let start_dv = burn.start_remaining_dv();
    let end_dv = burn.end_remaining_dv();
    let duration = burn.duration();
    
    Window::new("Selected burn")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Burn");
        draw_time_until(view, ui, time);
        draw_controls(ui, view, time, entity);
        draw_burn_labels(view, ui, max_dv, start_dv, end_dv, duration);
        draw_visual_timeline(view, ui, entity, time, false);
    });
}
