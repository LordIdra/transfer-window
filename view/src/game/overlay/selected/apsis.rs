use eframe::{egui::{Align2, Context, Grid, Window}, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{overlay::widgets::{buttons::{draw_next, draw_previous, draw_warp_to}, labels::{draw_altitude, draw_orbits, draw_speed, draw_time_until, draw_title}}, selected::Selected, util::ApsisType, Scene}, styles};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update apsis");
    let Selected::Apsis { type_, entity, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected apsis")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(context, |ui| {
        let name = match type_ {
            ApsisType::Periapsis => "Periapsis",
            ApsisType::Apoapsis => "Apoapsis",
        };
        draw_title(ui, name);
        draw_time_until(model, ui, time);

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

            if let Some(time) = draw_previous(view, model, context, ui, time, entity) {
                view.selected = Selected::Apsis { type_, entity, time };
            }

            if let Some(time) = draw_next(view, model, context, ui, time, entity) {
                view.selected = Selected::Apsis { type_, entity, time };
            }

            if draw_warp_to(view, model, context, ui, time) {
                events.push(Event::StartWarp { end_time: time });
            }
        });

        Grid::new("Selected apsis info").show(ui, |ui| {
            draw_altitude(model, ui, entity, time);
            draw_speed(model, ui, entity, time);
            draw_orbits(model, ui, entity, time);
        });
    });
}
