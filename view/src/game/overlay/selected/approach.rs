use eframe::{egui::{Align2, Grid, Window}, epaint};

use crate::{game::{events::ModelEvent, overlay::widgets::{buttons::draw_warp_to, labels::{draw_altitude, draw_distance, draw_speed, draw_time_until, draw_title}}, selected::Selected, View}, styles};

use super::vessel::visual_timeline;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update approach");
    let Selected::Approach { type_: _, entity, target: _, time } = view.selected.clone() else {
        return;
    };

    Window::new("Selected apsis")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Approach");
        draw_time_until(view, ui, time);

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

            if draw_warp_to(view, ui, time) {
                view.add_model_event(ModelEvent::StartWarp { end_time: time });
            }
        });

        Grid::new("Selected approach info").show(ui, |ui| {
            draw_altitude(view, ui, entity, time);
            draw_speed(view, ui, entity, time);
            draw_distance(view, ui, entity, time);
        });

        visual_timeline::draw(view, ui, entity, time, false);
    });
}
