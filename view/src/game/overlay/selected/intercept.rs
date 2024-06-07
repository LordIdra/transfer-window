use eframe::{egui::{Align2, Context, Window}, epaint};
use transfer_window_model::Model;

use crate::{events::Event, game::{overlay::widgets::{buttons::draw_warp_to, labels::{draw_time_until, draw_title}}, selected::Selected, Scene}, styles};

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update intercept");
    let Selected::Intercept { entity: _, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected intercept")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(context, |ui| {
        draw_title(ui, "Intercept");
        draw_time_until(model, ui, time);

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

            if draw_warp_to(view, model, context, ui, time) {
                events.push(Event::StartWarp { end_time: time });
            }
        });
    });
}
