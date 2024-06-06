use eframe::egui::{vec2, Align, Color32, Context, Frame, Layout, Margin, Pos2, Rect, Rounding, Stroke, Ui, Vec2};
use transfer_window_model::{components::vessel_component::{system_slot::{weapon::Weapon, SlotLocation, System}, timeline::fire_torpedo::FireTorpedoEvent}, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{overlay::{slot_textures::TexturedSlot, widgets::{custom_image::CustomImage, custom_image_button::CustomCircularImageButton}}, selected::{util::BurnState, Selected}, Scene}, styles};

#[allow(clippy::too_many_arguments)]
fn draw_fire(view: &mut Scene, model: &Model, ui: &mut Ui, context: &Context, center: Pos2, entity: Entity, slot_location: SlotLocation, time: f64, events: &mut Vec<Event>) {
    let lock_centre = center + vec2(-24.0, -24.0);
    let lock_size = Vec2::splat(23.0);
    let lock_rect = Rect::from_center_size(lock_centre, lock_size);
    ui.allocate_ui_at_rect(lock_rect, |ui| {
        let enabled = FireTorpedoEvent::can_create(model, entity, time, slot_location);
        let button = CustomCircularImageButton::new(view.renderers.get_screen_texture_renderer("fire-possible"), context.screen_rect(), 23.0)
            .with_enabled(enabled)
            .with_padding(3.0);
        if ui.add_enabled(enabled, button).clicked() {
            let event = Event::CreateFireTorpedo { entity, slot_location, time };
            events.push(event);
            view.selected = Selected::FireTorpedo { entity, time, state: BurnState::Selected }
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_weapon(view: &mut Scene, model: &Model, ui: &mut Ui, context: &Context, entity: Entity, location: SlotLocation, slot: Option<&Weapon>, time: f64, events: &mut Vec<Event>) {
    let Some(weapon) = slot else {
        return;
    };

    let texture = weapon.type_().texture();

    Frame::none()
        .stroke(Stroke::new(2.0, Color32::DARK_GRAY))
        .rounding(Rounding::same(6.0))
        .outer_margin(Margin::same(3.0))
        .show(ui, |ui| {
            let image = CustomImage::new(view.renderers.get_screen_texture_renderer(texture), context.screen_rect(), 80.0)
                .with_padding(8.0);
            let center = ui.add(image).rect.center();

            styles::WeaponSlotButton::apply(ui);
            if slot.is_some() {
                draw_fire(view, model, ui, context, center, entity, location, time, events);
            }
        });
}

#[allow(clippy::too_many_arguments)]
pub fn draw_weapons(view: &mut Scene, model: &Model, ui: &mut Ui, context: &Context, entity: Entity, weapon_slots: &[(SlotLocation, Option<&Weapon>)], time: f64, events: &mut Vec<Event>) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        for pair in weapon_slots {
            draw_weapon(view, model, ui, context, entity, pair.0, pair.1, time, events);
        }
    });
}