use eframe::egui::{self, Color32, Grid, Image, Pos2, Rect, Response, RichText, Ui, Vec2};
use transfer_window_model::{components::{path_component::burn::rocket_equation_function::RocketEquationFunction, vessel_component::{class::VesselClass, VesselComponent}}, storage::entity_allocator::Entity};

use crate::{game::{events::ViewEvent, overlay::{slot_textures::TexturedSlot, widgets::custom_image_button::CustomCircularImageButton}, View}, styles};

use super::{util::{compute_slot_locations, SLOT_SIZE}, SlotType};

const UNDERLAY_SIZE_PROPORTION: f32 = 0.9;

fn compute_texture_ship_underlay(class: VesselClass) -> &'static str {
    match class {
        VesselClass::Scout1 => "scout-1",
        VesselClass::Scout2 => "scout-2",
        VesselClass::Frigate1 => "frigate-1",
        VesselClass::Frigate2 => "frigate-2",
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
fn draw_slot_from_texture(view: &View, ui: &mut Ui, texture: &str, color: Color32, type_: SlotType, center: Pos2, size: f32, translation: Vec2) {
    let slot_position = center + translation;
    let slot_size = egui::Vec2::splat(size);
    let slot_rect = Rect::from_center_size(slot_position, slot_size);
    
    styles::SlotEditor::apply(ui, size, color);
    ui.allocate_ui_at_rect(slot_rect, |ui| {
        let slot_image = CustomCircularImageButton::new(view, texture, size)
            .with_normal_color(Color32::TRANSPARENT)
            .with_hover_color(Color32::from_rgba_unmultiplied(30, 30, 30, 120));
        if ui.add(slot_image).clicked() {
            on_slot_clicked(view, type_);
        }
});
}

fn draw_slot(view: &View, ui: &mut Ui, vessel_component: &VesselComponent, type_: SlotType, center: Pos2, size: f32, translation: Vec2) {
    let texture = match type_ {
        SlotType::Engine => vessel_component.engine_type().map(|x| x.texture()).unwrap_or("silhouette-engine"),
        SlotType::FuelTank => vessel_component.fuel_tank_type().map(|x| x.texture()).unwrap_or("silhouette-fuel-tank"),
        SlotType::TorpedoStorage => vessel_component.torpedo_storage_type().map(|x| x.texture()).unwrap_or("silhouette-torpedo-storage"),
        SlotType::TorpedoLauncher => vessel_component.torpedo_launcher_type().map(|x| x.texture()).unwrap_or("silhouette-torpedo-launcher"),
    };
    let color = Color32::from_rgb(200, 200, 200);
    draw_slot_from_texture(view, ui, texture, color, type_, center, size, translation);
}

fn draw_ship_underlay(view: &View, ui: &mut Ui, class: VesselClass) -> Response {
    let texture = view.resources.texture_image(compute_texture_ship_underlay(class));
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
        let slot_size = SLOT_SIZE * size.x;
        for (type_, translation) in compute_slot_locations(vessel_component.class()) {
            draw_slot(view, ui, vessel_component, type_, center, slot_size, translation * size.x);
        }
        response.rect
    }).inner
}
