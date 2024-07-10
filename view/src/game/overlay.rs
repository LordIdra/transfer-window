use eframe::egui::Style;

use crate::styles;

use super::View;

pub mod dialogue;
pub mod exit_modal;
mod explorer;
mod fps;
pub mod objectives;
mod right_click_menu;
mod scale;
mod selected;
mod slot_textures;
mod time;
pub mod vessel_editor;
pub mod widgets;

pub fn draw(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    styles::DefaultWindow::apply(&view.context);

    if view.config.draw_explorer {
        explorer::update(view);
    }
    fps::update(view);
    objectives::update(view);
    scale::update(view);
    time::update(view);
    selected::update(view);
    right_click_menu::update(view);

    styles::ExitModal::apply(&view.context);
    exit_modal::update(view);
    view.context.set_style(Style::default());

    styles::DialogueWindow::apply(&view.context);
    dialogue::update(view);
    view.context.set_style(Style::default());

    styles::VesselEditor::apply(&view.context);
    vessel_editor::update(view);
    view.context.set_style(Style::default());
}
