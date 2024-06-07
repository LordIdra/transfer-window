use crate::game::Scene;

use super::{super::util::format_time, widgets::custom_image::CustomImage};

use eframe::{egui::{Align2, Context, RichText, Window}, epaint};

use transfer_window_model::{api::time::{TimeStep, TIME_STEP_LEVELS}, Model};

pub fn update(view: &Scene, model: &Model, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update time");
    Window::new("Time")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.horizontal(|ui| {
                ui.set_height(25.0);
                ui.add(CustomImage::new(view, "time", context.screen_rect(), 25.0)
                    .with_padding(2.0));
                ui.label(RichText::new(format_time(model.time())).strong().size(20.0));
            });

            ui.horizontal(|ui| {
                ui.set_height(19.0);
                let paused =  match model.time_step() {
                    TimeStep::Level { level: _, paused } | TimeStep::Warp { speed: _, paused } => paused,
                };

                for i in 0..TIME_STEP_LEVELS.len() {
                    let texture = if *paused {
                        "time-step-paused"
                    } else {
                        match model.time_step() {
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
                    ui.add(CustomImage::new(view, texture, context.screen_rect(), 24.0)
                        .with_padding(2.0));
                    ui.add_space(-9.0);
                }
            })
        });
}
