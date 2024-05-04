use crate::{components::ComponentType, Model};

pub fn update_fuel_depletion(model: &mut Model) {
    let end_time = model.time();
    for entity in model.entities(vec![ComponentType::VesselComponent]) {
        let mass = model.mass_at_time(entity, end_time);
        let dry_mass = model.vessel_component(entity).dry_mass();
        model.vessel_component_mut(entity).slots_mut().set_fuel_kg(mass - dry_mass);
    }
}