use eframe::egui::{self, Color32, Grid, Image, ImageButton, Pos2, Rect, Response, RichText, Ui};
use transfer_window_model::{components::{path_component::burn::rocket_equation_function::RocketEquationFunction, vessel_component::{class::VesselClass, VesselComponent}}, storage::entity_allocator::Entity};

use crate::{game::{events::ViewEvent, overlay::slot_textures::TexturedSlot, View}, styles};

use super::{util::{compute_slot_locations, compute_slot_size}, SlotType};

const UNDERLAY_SIZE_PROPORTION: f32 = 0.7;
const FUEL_TANK_SLOT_COLOR: Color32 = Color32::from_rgb(240, 200, 0);
const ENGINE_SLOT_COLOR: Color32 = Color32::from_rgb(2, 192, 240);
const TORPEDO_STORAGE_SLOT_COLOR: Color32 = Color32::from_rgb(255, 90, 90);
const TORPEDO_LAUNCHER_SLOT_COLOR: Color32 = Color32::from_rgb(212, 11, 24);

fn compute_texture_ship_underlay(class: VesselClass) -> &'static str {
    match class {
        VesselClass::Scout => "ship-scout",
        VesselClass::Frigate => "ship-frigate",
        VesselClass::Torpedo | VesselClass::Hub => unreachable!(),
    }
}

fn on_slot_clicked(view: &View, type_: SlotType) {
    if let Some(vessel_editor) = &view.vessel_editor {
        let slot_editor = if vessel_editor.slot_editor == Some(type_) {
            None
        } else {
            Some(type_)
        };

        let mut vessel_editor = view.vessel_editor.as_ref().unwrap().clone();
        vessel_editor.slot_editor = slot_editor;
        view.add_view_event(ViewEvent::SetVesselEditor(Some(vessel_editor)));
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_slot_from_texture(view: &View, ui: &mut Ui, texture: &str, color: Color32, type_: SlotType, center: Pos2, size: f32, translation: f32) {
    let slot_position = center + egui::vec2(translation, 0.0);
    let slot_size = egui::Vec2::splat(size);
    let slot_rect = Rect::from_center_size(slot_position, slot_size);
    
    styles::SlotEditor::apply(ui, size, color);
    ui.allocate_ui_at_rect(slot_rect, |ui| {
        let slot_image = ImageButton::new(view.resources.icon_image(texture));
        if ui.add(slot_image).clicked() {
            on_slot_clicked(view, type_);
        }
});
}

fn draw_slot(view: &View, ui: &mut Ui, vessel_component: &VesselComponent, type_: SlotType, center: Pos2, size: f32, translation: f32) {
    let texture = match type_ {
        SlotType::FuelTank => vessel_component.fuel_tank_type().map(|x| x.texture()),
        SlotType::Engine => vessel_component.engine_type().map(|x| x.texture()),
        SlotType::TorpedoStorage => vessel_component.torpedo_storage_type().map(|x| x.texture()),
        SlotType::TorpedoLauncher => vessel_component.torpedo_launcher_type().map(|x| x.texture()),
    }.unwrap_or("blank-slot");
    let color = match type_ {
        SlotType::Engine => ENGINE_SLOT_COLOR,
        SlotType::FuelTank => FUEL_TANK_SLOT_COLOR,
        SlotType::TorpedoStorage => TORPEDO_STORAGE_SLOT_COLOR,
        SlotType::TorpedoLauncher => TORPEDO_LAUNCHER_SLOT_COLOR,

    };

    draw_slot_from_texture(view, ui, texture, color, type_, center, size, translation);
}

fn draw_ship_underlay(view: &View, ui: &mut Ui, class: VesselClass) -> Response {
    let texture = view.resources.icon_image(compute_texture_ship_underlay(class));
    let size = view.screen_rect.size() * UNDERLAY_SIZE_PROPORTION;
    ui.add(Image::new(texture).fit_to_exact_size(size))
}

fn draw_ship_stats(ui: &mut Ui, vessel_component: &VesselComponent, vessel_name: &str) {
    ui.vertical(|ui| {
        ui.set_width(200.0);
    
        Grid::new("Ship stats - ".to_string() + vessel_name).show(ui, |ui| {
            ui.label(RichText::new("Class").monospace().strong());
            ui.label(vessel_component.class().name());
            ui.end_row();
        
            ui.label(RichText::new("Dry mass").monospace().strong());
            ui.label(format!("{} kg", vessel_component.dry_mass()));
            ui.end_row();
            
            if !vessel_component.is_fuel_empty() {
                ui.label(RichText::new("Wet mass").monospace().strong());
                ui.label(format!("{} kg", vessel_component.wet_mass().round()));
                ui.end_row();

                ui.label(RichText::new("Fuel capacity").monospace().strong());
                ui.label(format!("{} L", vessel_component.fuel_capacity_litres()));
                ui.end_row();
            

                if vessel_component.has_engine() {
                    let rocket_equation_function = RocketEquationFunction::new(
                        vessel_component.dry_mass(), vessel_component.fuel_capacity_kg(),
                        vessel_component.fuel_kg_per_second(), vessel_component.specific_impulse().unwrap(), 0.0);

                    ui.label(RichText::new("Max ΔV").monospace().strong());
                    ui.label(format!("{} m/s", rocket_equation_function.remaining_dv().round()));
                    ui.end_row();
            
                    ui.label(RichText::new("Acceleration (wet)").monospace().strong());
                    ui.label(format!("{:.2} m/s", rocket_equation_function.acceleration()));
                    ui.end_row();

                    ui.label(RichText::new("Acceleration (dry)").monospace().strong());
                    ui.label(format!("{:.2} m/s", rocket_equation_function.end().acceleration()));
                    ui.end_row();
                }
            }

        });
    });
}

pub fn draw_vessel_editor(view: &View, ui: &mut Ui, vessel_name: &str, entity: Entity) -> Rect {
    ui.horizontal(|ui| {
        let vessel_component = view.model.vessel_component(entity);
        draw_ship_stats(ui, vessel_component, vessel_name);
        let response = draw_ship_underlay(view, ui, vessel_component.class());
        let center = response.rect.center();
        let size = response.rect.size();
        let slot_size = compute_slot_size(vessel_component.class()) * size.x;
        for (type_, translation) in compute_slot_locations(vessel_component.class()) {
            draw_slot(view, ui, vessel_component, type_, center, slot_size, translation * size.x);
        }
        response.rect
    }).inner
}
