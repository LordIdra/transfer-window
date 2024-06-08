use eframe::{egui::{Align2, Window}, epaint};

use crate::{game::{events::Event, overlay::widgets::{buttons::draw_warp_to, labels::{draw_time_until, draw_title}}, selected::Selected, View}, styles};

pub fn update(view: &mut View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update intercept");
    let Selected::Intercept { entity: _, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected intercept")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Intercept");
        draw_time_until(view, ui, time);

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

            if draw_warp_to(view, ui, time) {
                view.events.push(Event::StartWarp { end_time: time });
            }
        });
    });
}
