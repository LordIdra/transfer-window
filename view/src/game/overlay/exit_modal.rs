use eframe::{egui::{Align2, CursorIcon, RichText, Window}, epaint};

use crate::{game::{events::ViewEvent, View}, styles};

use super::widgets::custom_image::CustomImage;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update exit modal");

    if !view.exit_modal_open {
        return;
    }

    Window::new("Exit modal")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, epaint::vec2(0.0, 0.0))
        .show(&view.context.clone(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add(CustomImage::new(view, "alert", 60.0));
                ui.label(RichText::new("Exit to main menu? Your progress will be lost!").strong());
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    styles::DialogueContinueButton::apply(ui);

                    ui.add_space(110.0); // ffs egui why isn't this centered

                    let response = ui.button(RichText::new("Cancel").strong().monospace().size(12.0));
                    if response.hovered() {
                        view.context.set_cursor_icon(CursorIcon::PointingHand);
                    }
                    if response.clicked() {
                        view.add_view_event(ViewEvent::ToggleExitModal);
                    }

                    ui.add_space(20.0);

                    let response = ui.button(RichText::new("Exit").strong().monospace().size(12.0));
                    if response.hovered() {
                        view.context.set_cursor_icon(CursorIcon::PointingHand);
                    }
                    if response.clicked() {
                        view.add_view_event(ViewEvent::ExitLevel);
                    }
                })
            })
        });
}