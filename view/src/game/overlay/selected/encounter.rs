use eframe::{egui::{Align2, Grid, Window}, epaint};

use crate::{game::{events::ModelEvent, overlay::widgets::{buttons::draw_warp_to, labels::{draw_encounter_from, draw_encounter_to, draw_time_until, draw_title}}, selected::Selected, View}, styles};

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update encounter");
    let Selected::Encounter { type_: _, entity: _, time, from, to } = view.selected.clone() else {
        return;
    };

    Window::new("Selected encounter")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, "Encounter");
        draw_time_until(view, ui, time);

        ui.horizontal(|ui| {
            styles::SelectedMenuButton::apply(ui);

            if draw_warp_to(view, ui, time) {
                view.add_model_event(ModelEvent::StartWarp { end_time: time });
            }
        });

        Grid::new("Selected encounter info").show(ui, |ui| {
            draw_encounter_from(view, ui, from);
            draw_encounter_to(view, ui, to);
        });
    });
}
