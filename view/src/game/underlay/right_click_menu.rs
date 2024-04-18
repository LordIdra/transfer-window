use eframe::egui::{Align2, Button, Context, Window};
use nalgebra_glm::vec2;
use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::Scene};

fn draw(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, entity: Entity) {
    let world_position = model.get_absolute_position(entity) + vec2(50.0, 0.0);
    let window_position = view.camera.world_space_to_window_space(model, world_position, context.screen_rect());
    Window::new("Right click menu")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, window_position.to_vec2())
        .show(context, |ui| {
            if let Some(focus) = view.camera.get_focus() {
                if model.try_get_vessel_component(focus).is_some() {
                    let can_target = focus != entity;
                    let target_button = Button::new("Set target");
                    if ui.add_enabled(can_target, target_button).clicked() {
                        events.push(Event::SetTarget { 
                            entity: view.camera.get_focus().unwrap(), 
                            target: Some(entity) 
                        });
                    }
                }
            }
        });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>, is_mouse_over_any_icon: bool) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update right click menu");
    let is_mouse_over_ui_element = context.is_pointer_over_area() || is_mouse_over_any_icon;
    if !is_mouse_over_ui_element {
        let pointer = context.input(|input| {
            input.pointer.clone()
        });
        if view.right_click_menu.is_some() && pointer.primary_clicked() {
            view.right_click_menu = None;
        }
    }

    if let Some(entity) = view.right_click_menu {
        draw(view, model, context, events, entity);
    }
}