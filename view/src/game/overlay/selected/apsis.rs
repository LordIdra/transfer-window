use eframe::{egui::{Align2, Ui, Window}, epaint};
use transfer_window_model::storage::entity_allocator::Entity;

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_next, draw_previous, draw_select_orbitable, draw_select_vessel, draw_warp_to}, labels::{draw_info_at_time, draw_time_until, draw_title}}, selected::Selected, util::ApsisType, View}, styles};

use super::vessel::visual_timeline::draw_visual_timeline;

fn draw_controls(ui: &mut Ui, view: &View, time: f64, entity: Entity, type_: ApsisType) {
    ui.horizontal(|ui| {
        styles::SelectedMenuButton::apply(ui);

        if view.model.try_vessel_component(entity).is_some() {
            if draw_select_vessel(view, ui, entity) {
                view.add_view_event(ViewEvent::SetSelected(Selected::Vessel(entity)));
            }
        } else if draw_select_orbitable(view, ui, entity) {
            view.add_view_event(ViewEvent::SetSelected(Selected::Orbitable(entity)));
        }

        if let Some(time) = draw_previous(view, ui, time, entity) {
            let selected = Selected::Apsis { type_, entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if let Some(time) = draw_next(view, ui, time, entity) {
            let selected = Selected::Apsis { type_, entity, time };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }

        if draw_warp_to(view, ui, time) {
            view.add_model_event(ModelEvent::StartWarp { end_time: time });
        }
    });
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update apsis");
    let Selected::Apsis { type_, entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected apsis")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        let name = match type_ {
            ApsisType::Periapsis => "Periapsis",
            ApsisType::Apoapsis => "Apoapsis",
        };
        draw_title(ui, name);
        draw_time_until(view, ui, time);
        draw_controls(ui, view, time, entity, type_);
        draw_info_at_time(view, ui, entity, time);
        if view.model.try_vessel_component(entity).is_some() {
            draw_visual_timeline(view, ui, entity, time, false);
        }
    });
}
