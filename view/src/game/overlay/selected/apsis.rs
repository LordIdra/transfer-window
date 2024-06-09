use eframe::{egui::{Align2, Grid, Window}, epaint};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::widgets::{buttons::{draw_next, draw_previous, draw_warp_to}, labels::{draw_altitude, draw_orbits, draw_speed, draw_subtitle, draw_time_until, draw_title}}, selected::Selected, util::ApsisType, View}, styles};

use super::vessel::visual_timeline;

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

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

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

        draw_subtitle(ui, "Info");
        Grid::new("Selected apsis info").show(ui, |ui| {
            draw_altitude(view, ui, entity, time);
            draw_speed(view, ui, entity, time);
            draw_orbits(view, ui, entity, time);
        });

        visual_timeline::draw(view, ui, entity, time, false);
    });
}
