use eframe::egui::{vec2, Align, Color32, Frame, Image, ImageButton, Layout, Margin, Pos2, Rect, Stroke, Ui, Vec2};
use transfer_window_model::{components::vessel_component::{system_slot::{weapon::Weapon, SlotLocation, System}, VesselComponent}, storage::entity_allocator::Entity};

use crate::{events::Event, game::{overlay::slot_textures::TexturedSlot, Scene}, styles};

fn draw_fire(view: &mut Scene, ui: &mut Ui, center: Pos2, vessel_component: &VesselComponent, entity: Entity, location: SlotLocation, events: &mut Vec<Event>) {
    let lock_centre = center + vec2(-27.0, -27.0);
    let lock_size = Vec2::splat(20.0);
    let lock_rect = Rect::from_center_size(lock_centre, lock_size);
    ui.allocate_ui_at_rect(lock_rect, |ui| {
        let texture = if vessel_component.has_target() {
            "fire-possible"
        } else {
            "fire-disabled"
        };
        let image_button = ImageButton::new(view.resources.texture_image(texture));
        if ui.add_enabled(vessel_component.has_target(), image_button).clicked() {
            let event = Event::FireTorpedo { entity, slot_location: location, target: vessel_component.target().unwrap() };
            events.push(event);
        }
    });
}

fn draw_weapon(view: &mut Scene, ui: &mut Ui, vessel_component: &VesselComponent, entity: Entity, location: SlotLocation, slot: Option<&Weapon>, events: &mut Vec<Event>) {
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
                draw_fire(view, ui, center, vessel_component, entity, location, events);
            }
        });
}

pub fn draw_weapons(view: &mut Scene, ui: &mut Ui, vessel_component: &VesselComponent, entity: Entity, weapon_slots: &[(SlotLocation, Option<&Weapon>)], events: &mut Vec<Event>) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        for pair in weapon_slots {
            draw_weapon(view, ui, vessel_component, entity, pair.0, pair.1, events);
        }
    });
}