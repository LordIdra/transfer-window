use crate::game::View;

use super::{super::util::format_time, widgets::custom_image::CustomImage};

use eframe::{egui::{Align2, RichText, Window}, epaint};

use transfer_window_model::api::time::{TimeStep, TIME_STEP_LEVELS};

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update time");
    Window::new("Time")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_TOP, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            ui.horizontal(|ui| {
                ui.set_height(25.0);
                ui.add(CustomImage::new(view, "time", 25));
                ui.label(RichText::new(format_time(view.model.time())).strong().size(20.0));
            });

            ui.horizontal(|ui| {
                ui.set_height(19.0);
                let paused =  match view.model.time_step() {
                    TimeStep::Level { level: _, paused } | TimeStep::Warp { speed: _, paused } => paused,
                };

                for (i, level) in TIME_STEP_LEVELS.iter().enumerate() {
                    let texture = if *paused {
                        "time-step-paused"
                    } else {
                        match view.model.time_step() {
                            TimeStep::Warp { speed: _, paused: _ } => {
                                "time-step-warp"
                            }
                            TimeStep::Level { level, paused: _ } => {
                                if i < *level as usize {
                                    "time-step-true"
                                } else {
                                    "time-step-false"
                                }
                            },
                        }
                    };
                    ui.add(CustomImage::new(view, texture, 24))
                        .on_hover_text(format!("{}x", level.round()));
                    ui.add_space(-9.0);
                }
            })
        });
}
