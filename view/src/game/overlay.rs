use eframe::egui::{Context, Style};
use transfer_window_model::Model;

use crate::{events::Event, styles};

use super::Scene;

mod fps;
mod right_click_menu;
mod scale;
mod selected;
mod slot_textures;
mod time;
pub mod vessel_editor;
pub mod widgets;

pub fn draw(view: &mut Scene, model: &Model, context: &Context, is_mouse_over_any_icon: bool, events: &mut Vec<Event>) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Draw overlay");

    styles::DefaultWindow::apply(context);
    
    fps::update(view, context);
    scale::update(view, context);
    time::update(view, model, context);
    selected::update(view, model, context, events);
    right_click_menu::update(view, model, context, events, is_mouse_over_any_icon);

    styles::VesselEditor::apply(context);

    vessel_editor::update(view, model, context, events);

    context.set_style(Style::default());
}
