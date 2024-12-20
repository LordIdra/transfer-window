use eframe::{egui::{Align2, Ui, Window}, epaint};
use transfer_window_model::{model::state_query::StateQuery, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::draw_select_vessel, custom_image_button::CustomCircularImageButton, labels::{draw_subtitle, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::{burn::draw_burn_labels, vessel::visual_timeline::draw_visual_timeline};

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

        let enabled = view.model.fire_torpedo_event_at_time(entity, time).unwrap().can_remove();
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
    let _span = tracy_client::span!("Update torpedo");
    let Selected::FireTorpedo { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Some(fire_torpedo_event) = view.model.fire_torpedo_event_at_time(entity, time) else {
        return;
    };

    let vessel_component = view.model.vessel_component(fire_torpedo_event.ghost());
    let snapshot = view.model.snapshot_at(fire_torpedo_event.burn_time());
    let burn = snapshot.burn(fire_torpedo_event.ghost());
    let max_dv = vessel_component.max_dv();
    let start_dv = burn.start_remaining_dv();
    let end_dv = burn.end_dv();
    let duration = burn.duration();
    
    Window::new("Torpedo launch")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            draw_title(ui, "Torpedo Launch");
            draw_time_until(view, ui, time);
            draw_controls(ui, view, time, entity);
            draw_subtitle(ui, "Burn");
            draw_burn_labels(view, ui, max_dv, start_dv, end_dv, duration);
            draw_visual_timeline(view, ui, fire_torpedo_event.ghost(), time, false);
        });
}
