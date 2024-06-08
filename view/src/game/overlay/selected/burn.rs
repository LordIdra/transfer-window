use eframe::{egui::{Align2, Color32, Grid, RichText, Ui, Window}, epaint};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{bars::{draw_filled_bar, FilledBar}, custom_image::CustomImage, custom_image_button::CustomCircularImageButton}, selected::Selected, util::format_time, View}, styles};

pub fn draw_burn_info(view: &View, ui: &mut Ui, max_dv: f64, start_dv: f64, end_dv: f64, duration: f64) {
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
            let image = CustomImage::new(view, "duration", 20.0);
            ui.add(image);
            ui.label(RichText::new("Duration").strong().monospace());
        });
        ui.label(format_time(duration));
        ui.end_row();

        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "burn-start", 20.0);
            ui.add(image);
            ui.label(RichText::new("ΔV start").strong().monospace());
        });
        ui.label(format!("{start_dv:.1}"));
        ui.end_row();

        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "burn-burnt", 20.0);
            ui.add(image);
            ui.label(RichText::new("ΔV burnt").strong().monospace());
        });
        ui.label(format!("{burnt_dv:.1}"));
        ui.end_row();

        ui.horizontal(|ui| {
            let image = CustomImage::new(view, "burn-end", 20.0);
            ui.add(image);
            ui.label(RichText::new("ΔV end").strong().monospace());
        });
        ui.label(format!("{end_dv:.1}"));
        ui.end_row();
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
    
    Window::new("Selected burn")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        ui.label(RichText::new("Burn").size(20.0).monospace().strong());
        let text = format!("T-{}", format_time(time - view.model.time()));
        ui.label(RichText::new(text).weak());

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);
            ui.set_height(36.0);

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
                view.add_model_event(ModelEvent::CancelLastTimelineEvent { entity });
                view.add_view_event(ViewEvent::SetSelected(Selected::None));
            }
        });

        let vessel_component = view.model.vessel_component(entity);
        let burn = view.model.path_component(entity).future_segment_starting_at_time(time).unwrap().as_burn().unwrap();
        let max_dv = vessel_component.max_dv().unwrap();
        let start_dv = burn.rocket_equation_function().remaining_dv();
        let end_dv = burn.final_rocket_equation_function().remaining_dv();
        let duration = burn.duration();
        draw_burn_info(view, ui, max_dv, start_dv, end_dv, duration);
    });
}
