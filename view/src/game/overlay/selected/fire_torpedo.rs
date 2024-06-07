use eframe::{egui::{Align2, Context, RichText, Window}, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{overlay::widgets::custom_image_button::CustomCircularImageButton, selected::Selected, util::format_time, Scene}, styles};

use super::burn::draw_burn_info;

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update torpedo");
    let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Some(fire_torpedo_event) = model.fire_torpedo_event_at_time(entity, time) else {
        return;
    };

    let vessel_component = model.vessel_component(fire_torpedo_event.ghost());
    let burn = model.burn_starting_at_time(fire_torpedo_event.ghost(), fire_torpedo_event.burn_time());
    
    Window::new("Torpedo launch")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(RichText::new("Torpedo Launch").size(20.0).monospace().strong());
            let text = format!("T-{}", format_time(time - model.time()));
            ui.label(RichText::new(text).weak());

            ui.horizontal(|ui| {
                styles::SelectedMenuButton::apply(ui);
                ui.set_height(36.0);

                let enabled = model.can_warp_to(time);
                let button = CustomCircularImageButton::new(view, "warp-here", context.screen_rect(), 36.0)
                    .with_enabled(enabled)
                    .with_padding(8.0);
                if ui.add_enabled(enabled, button).on_hover_text("Warp here").clicked() {
                    events.push(Event::StartWarp { end_time: time });
                }

                let enabled = model.timeline_event_at_time(entity, time).can_delete(model);
                let button = CustomCircularImageButton::new(view, "cancel", context.screen_rect(), 36.0)
                    .with_enabled(enabled)
                    .with_padding(8.0);
                if ui.add_enabled(enabled, button).on_hover_text("Cancel").clicked() {
                    events.push(Event::CancelLastTimelineEvent { entity });
                    view.selected = Selected::None;
                }
            });

            let max_dv = vessel_component.max_dv().unwrap();
            let start_dv = burn.rocket_equation_function().remaining_dv();
            let end_dv = burn.final_rocket_equation_function().remaining_dv();
            let duration = burn.duration();
            draw_burn_info(view, ui, context, max_dv, start_dv, end_dv, duration);
        });
}