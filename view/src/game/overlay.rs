use eframe::{egui::{Align2, Context, Window}, epaint};
use transfer_window_model::Model;

use crate::events::Event;

use super::{underlay::selected::{burn::BurnState, Selected}, util::format_time, Scene};

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");
    if model.get_time_step().is_paused() {
        Window::new("Paused")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, -30.0))
            .show(context, |ui| {
                ui.label("SIMULATION PAUSED")
        });
    }

    Window::new("Time")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_TOP, epaint::vec2(0.0, 30.0))
        .show(context, |ui| {
            let time_step = f64::round(model.get_time_step().get_time_step());
            ui.label("Time: ".to_string() + format_time(model.get_time()).as_str());
            ui.label("Time step: ".to_string() + time_step.to_string().as_str() + "s");
    });

    if let Selected::Point { entity, time, state } = view.selected.clone() {
        if state.is_selected() {
            Window::new("Selected point")
                .title_bar(false)
                .resizable(false)
                .anchor(Align2::LEFT_TOP, epaint::vec2(30.0, 30.0))
                .show(context, |ui| {
                    ui.label(model.get_name_component(entity).get_name());
                    ui.label("T-".to_string() + format_time(time - model.get_time()).as_str());
                    if ui.button("Warp here").clicked() {
                        events.push(Event::StartWarp { end_time: time });
                    }
                    if ui.button("Create burn").clicked() {
                        events.push(Event::CreateBurn { entity, time });
                        view.selected = Selected::Burn { entity, time, state: BurnState::Selected }
                    }
            });
        }
    }
}