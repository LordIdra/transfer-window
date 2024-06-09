use eframe::{egui::{Align2, Window}, epaint};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{custom_image_button::CustomCircularImageButton, labels::{draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::{burn::draw_burn_info, vessel::visual_timeline};

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update torpedo");
    let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    if view.model.fire_torpedo_event_at_time(entity, time).is_none() {
        return;
    };
    
    Window::new("Torpedo launch")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            draw_title(ui, "Torpedo Launch");
            draw_time_until(view, ui, time);

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

            let fire_torpedo_event = view.model.fire_torpedo_event_at_time(entity, time).unwrap();
            let vessel_component = view.model.vessel_component(fire_torpedo_event.ghost());
            let burn = view.model.burn_starting_at_time(fire_torpedo_event.ghost(), fire_torpedo_event.burn_time());
            let max_dv = vessel_component.max_dv().unwrap();
            let start_dv = burn.rocket_equation_function().remaining_dv();
            let end_dv = burn.final_rocket_equation_function().remaining_dv();
            let duration = burn.duration();
            draw_burn_info(view, ui, max_dv, start_dv, end_dv, duration);
            
            visual_timeline::draw(view, ui, entity, time, false);
        });
}