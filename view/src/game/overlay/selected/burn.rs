use eframe::{egui::{Align2, Color32, Context, Grid, ImageButton, RichText, Ui, Window}, epaint};
use transfer_window_model::{components::path_component::segment::Segment, Model};

use crate::{events::Event, game::{overlay::widgets::{draw_filled_bar, FilledBar}, underlay::selected::Selected, util::format_time, Scene}, styles};

pub fn draw_burn_info(view: &Scene, ui: &mut Ui, max_dv: f64, start_dv: f64, end_dv: f64, duration: f64) {
    let burnt_dv = start_dv - end_dv;

    let start_dv_proportion = (start_dv / max_dv) as f32;
    let end_dv_proportion = (end_dv / max_dv) as f32;

    let start_bar = FilledBar::new(Color32::DARK_RED, start_dv_proportion);
    let end_bar = FilledBar::new(Color32::WHITE, end_dv_proportion);

    ui.horizontal(|ui| {
        draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![start_bar, end_bar]);
    });

    Grid::new("DV grid").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.image(view.resources.texture_image("duration"));
            ui.label(RichText::new("Duration").strong().monospace());
        });
        ui.label(format_time(duration));
        ui.end_row();

        ui.horizontal(|ui| {
            ui.image(view.resources.texture_image("burn-start"));
            ui.label(RichText::new("ΔV start").strong().monospace());
        });
        ui.label(format!("{start_dv:.1}"));
        ui.end_row();

        ui.horizontal(|ui| {
            ui.image(view.resources.texture_image("burn-burnt"));
            ui.label(RichText::new("ΔV burnt").strong().monospace());
        });
        ui.label(format!("{burnt_dv:.1}"));
        ui.end_row();

        ui.horizontal(|ui| {
            ui.image(view.resources.texture_image("burn-end"));
            ui.label(RichText::new("ΔV end").strong().monospace());
        });
        ui.label(format!("{end_dv:.1}"));
        ui.end_row();
    });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Burn { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Some(Segment::Burn(burn)) = model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };

    let vessel_component = model.vessel_component(entity);
    
    Window::new("Selected burn")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(RichText::new("Burn").size(20.0).monospace().strong());
            let text = format!("T-{}", format_time(time - model.time()));
            ui.label(RichText::new(text).weak());

            ui.horizontal(|ui| {
                styles::SelectedMenuButton::apply(ui);
                ui.set_height(36.0);

                let button = ImageButton::new(view.resources.texture_image("warp-here"));
                let can_warp = model.can_warp_to(time);
                if ui.add_enabled(can_warp, button).on_hover_text("Warp here").clicked() {
                    events.push(Event::StartWarp { end_time: time });
                }

                let can_delete = model.timeline_event_at_time(entity, time).can_delete(model);
                let button = ImageButton::new(view.resources.texture_image("cancel"));
                if ui.add_enabled(can_delete, button).on_hover_text("Cancel").clicked() {
                    events.push(Event::CancelLastTimelineEvent { entity });
                    view.selected = Selected::None;
                }
            });

            let max_dv = vessel_component.max_dv().unwrap();
            let start_dv = burn.rocket_equation_function().remaining_dv();
            let end_dv = burn.final_rocket_equation_function().remaining_dv();
            let duration = burn.duration();
            draw_burn_info(view, ui, max_dv, start_dv, end_dv, duration);
        });
}
