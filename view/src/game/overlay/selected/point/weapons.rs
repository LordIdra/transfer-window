use eframe::egui::{vec2, Align, Color32, Frame, Image, ImageButton, Layout, Margin, Pos2, Rect, Stroke, Ui, Vec2};
use transfer_window_model::{components::vessel_component::{system_slot::{weapon::Weapon, SlotLocation, System}, timeline::fire_torpedo::FireTorpedoEvent}, storage::entity_allocator::Entity, Model};

use crate::{events::Event, game::{overlay::slot_textures::TexturedSlot, underlay::selected::{util::BurnState, Selected}, Scene}, styles};

#[allow(clippy::too_many_arguments)]
fn draw_fire(view: &mut Scene, model: &Model, ui: &mut Ui, center: Pos2, entity: Entity, slot_location: SlotLocation, time: f64, events: &mut Vec<Event>) {
    let lock_centre = center + vec2(-27.0, -27.0);
    let lock_size = Vec2::splat(20.0);
    let lock_rect = Rect::from_center_size(lock_centre, lock_size);
    ui.allocate_ui_at_rect(lock_rect, |ui| {
        let can_create = FireTorpedoEvent::can_create(model, entity, time, slot_location);
        let texture = "fire-possible";
        let image_button = ImageButton::new(view.resources.texture_image(texture));
        if ui.add_enabled(can_create, image_button).clicked() {
            let event = Event::CreateFireTorpedo { entity, slot_location, time };
            events.push(event);
            view.selected = Selected::FireTorpedo { entity, time, state: BurnState::Selected }
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_weapon(view: &mut Scene, model: &Model, ui: &mut Ui, entity: Entity, location: SlotLocation, slot: Option<&Weapon>, time: f64, events: &mut Vec<Event>) {
    let texture_name = match slot {
        Some(weapon) => weapon.type_().texture(),
        None => "blank-slot",
    };

    Frame::none()
        .stroke(Stroke::new(2.0, Color32::GRAY))
        .outer_margin(Margin::same(3.0))
        .show(ui, |ui| {
            let image = Image::new(view.resources.texture_image(texture_name))
                .fit_to_exact_size(vec2(80.0, 80.0));
            let center = ui.add(image).rect.center();

            styles::WeaponSlotButton::apply(ui);
            if slot.is_some() {
                draw_fire(view, model, ui, center, entity, location, time, events);
            }
        });
}

pub fn draw_weapons(view: &mut Scene, model: &Model, ui: &mut Ui, entity: Entity, weapon_slots: &[(SlotLocation, Option<&Weapon>)], time: f64, events: &mut Vec<Event>) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        for pair in weapon_slots {
            draw_weapon(view, model, ui, entity, pair.0, pair.1, time, events);
        }
    });
}