use eframe::egui::{vec2, Align, Color32, Frame, Layout, Margin, Pos2, Rect, Rounding, Stroke, Ui, Vec2};
use transfer_window_model::{components::vessel_component::{timeline::fire_torpedo::FireTorpedoEvent, torpedo_launcher::TorpedoLauncherType}, storage::entity_allocator::Entity};

use crate::{game::{events::{ModelEvent, ViewEvent}, overlay::{slot_textures::TexturedSlot, widgets::{custom_image::CustomImage, custom_image_button::CustomCircularImageButton}}, selected::{util::BurnState, Selected}, View}, styles};

#[allow(clippy::too_many_arguments)]
fn draw_fire(view: &View, ui: &mut Ui, center: Pos2, entity: Entity, time: f64) {
    let lock_centre = center + vec2(-24.0, -24.0);
    let lock_size = Vec2::splat(23.0);
    let lock_rect = Rect::from_center_size(lock_centre, lock_size);
    ui.allocate_ui_at_rect(lock_rect, |ui| {
        let enabled = FireTorpedoEvent::can_create(&view.model, entity, time);
        let button = CustomCircularImageButton::new(view, "fire-possible", 23.0)
            .with_enabled(enabled)
            .with_padding(3.0);
        if ui.add_enabled(enabled, button).clicked() {
            let event = ModelEvent::CreateFireTorpedo { entity, time };
            view.add_model_event(event);
            let selected = Selected::FireTorpedo { entity, time, state: BurnState::Selected };
            view.add_view_event(ViewEvent::SetSelected(selected));
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_weapon(view: &View, ui: &mut Ui, entity: Entity, slot: Option<TorpedoLauncherType>, time: f64) {
    let Some(weapon) = slot else {
        return;
    };

    let texture = weapon.type_().texture();

    Frame::none()
        .stroke(Stroke::new(2.0, Color32::DARK_GRAY))
        .rounding(Rounding::same(6.0))
        .outer_margin(Margin::same(3.0))
        .show(ui, |ui| {
            let image = CustomImage::new(view, texture, 80.0)
                .with_padding(8.0);
            let center = ui.add(image).rect.center();

            styles::WeaponSlotButton::apply(ui);
            draw_fire(view, ui, center, entity, time);
        });
}

#[allow(clippy::too_many_arguments)]
pub fn draw_weapons(view: &View, ui: &mut Ui, entity: Entity, weapon_slots: &[(ShipSlotLocation, Option<&Weapon>)], time: f64) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        for pair in weapon_slots {
            draw_weapon(view, ui, entity, pair.0, pair.1, time);
        }
    });
}