use eframe::{egui::{Align2, Context, Grid, Window}, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{overlay::widgets::{buttons::draw_warp_to, labels::{draw_altitude, draw_distance, draw_speed, draw_time_until, draw_title}}, selected::Selected, Scene}, styles};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update approach");
    let Selected::Approach { type_: _, entity, target: _, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected apsis")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(context, |ui| {
        draw_title(ui, "Closest Approach");
        draw_time_until(model, ui, time);

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

            if draw_warp_to(view, model, context, ui, time) {
                events.push(Event::StartWarp { end_time: time });
            }
        });

        Grid::new("Selected approach info").show(ui, |ui| {
            draw_altitude(model, ui, entity, time);
            draw_speed(model, ui, entity, time);
            draw_distance(model, ui, entity, time);
        });
    });
}
