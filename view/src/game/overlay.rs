use eframe::{egui::{Align2, Button, Context, Window}, epaint};
use transfer_window_model::Model;

use crate::events::Event;

use super::{underlay::selected::{burn::BurnState, Selected}, util::format_time, Scene};

pub fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    Window::new("Save")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            if ui.button("Save").clicked() {
                    events.push(Event::SaveGame { name: "test".to_owned() });
            }
        });
    
    Window::new("FPS")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label("FPS: ".to_string() + view.frame_history.fps().to_string().as_str());
        });

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
                    if model.try_get_vessel_component(entity).is_some() {
                        if ui.button("Create burn").clicked() {
                            events.push(Event::CreateBurn { entity, time });
                            view.selected = Selected::Burn { entity, time, state: BurnState::Selected }
                        }
                        let segment = model.get_trajectory_component(entity).get_first_segment_at_time(time).as_orbit();
                        if let Some(period) = segment.get_period() {
                            {
                                let time = time - period;
                                let state = state.clone();
                                let button = Button::new("Previous orbit");
                                let enabled = time > segment.get_current_point().get_time();
                                if ui.add_enabled(enabled, button).clicked() {
                                    view.selected = Selected::Point { entity, time, state };
                                }
                            }
                            {
                                let time = time + period;
                                let button = Button::new("Next orbit");
                                let enabled = time < segment.get_end_point().get_time();
                                if ui.add_enabled(enabled, button).clicked() {
                                    view.selected = Selected::Point { entity, time, state };
                                }
                            }
                            let orbits = ((time - model.get_time()) / period) as usize;
                            if orbits != 0 {
                                ui.label("Orbits: ".to_string() + orbits.to_string().as_str());
                            }
                        }
                    }
                });
        }
    }

    if let Selected::Burn { entity, time, state: _ } = view.selected.clone() {
        Window::new("Selected burn")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(30.0, 30.0))
            .show(context, |ui| {
                ui.label(model.get_name_component(entity).get_name());
                ui.label("T-".to_string() + format_time(time - model.get_time()).as_str());
                if ui.button("Warp to burn").clicked() {
                    events.push(Event::StartWarp { end_time: time });
                }
            });
    }
}