use eframe::egui::Style;

use crate::styles;

use super::View;

mod fps;
mod right_click_menu;
mod scale;
mod selected;
mod slot_textures;
mod time;
pub mod vessel_editor;
pub mod widgets;

pub fn draw(view: &mut View, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    styles::DefaultWindow::apply(&view.context);

    fps::update(view);
    scale::update(view);
    time::update(view);
    selected::update(view);
    right_click_menu::update(view, is_mouse_over_any_icon);

    styles::VesselEditor::apply(&view.context);

    vessel_editor::update(view);

    view.context.set_style(Style::default());
}
