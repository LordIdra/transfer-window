use crate::{components::ComponentType, Model};

pub fn update_fuel_depletion(model: &mut Model) {
    let end_time = model.get_time();
    for entity in model.get_entities(vec![ComponentType::VesselComponent]) {
        let mass = model.get_mass_at_time(entity, end_time);
        let dry_mass = model.get_vessel_component(entity).get_dry_mass();
        model.get_vessel_component_mut(entity).get_slots_mut().set_fuel_kg(mass - dry_mass);
    }
}