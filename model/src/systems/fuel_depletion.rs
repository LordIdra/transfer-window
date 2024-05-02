use crate::{components::{trajectory_component::segment::Segment, ComponentType}, storage::entity_allocator::Entity, Model};

fn get_fuel_depleted_in_time_from_now(model: &Model, entity: Entity, time: f64) -> f64 {
    let initial_fuel_mass = model.get_vessel_component(entity).get_fuel_mass();
    let mut final_fuel_mass = initial_fuel_mass;
    for segment in model.get_trajectory_component(entity).get_segments().iter().flatten() {
        if segment.get_start_time() > time {
            break;
        }

        if let Segment::Burn(burn) = segment {
            if burn.get_end_point().get_time() > time {
                final_fuel_mass = burn.get_end_point().get_mass() - model.get_vessel_component(entity).get_dry_mass();
                break;
            }
        }
    }
    final_fuel_mass - initial_fuel_mass
}

pub fn update_fuel_depletion(model: &mut Model, dt: f64) {
    let simulation_dt = model.get_time_step().get_time_step() * dt;
    for entity in model.get_entities(vec![ComponentType::VesselComponent]) {
        let fuel_depleted = get_fuel_depleted_in_time_from_now(model, entity, simulation_dt);
        model.get_vessel_component_mut(entity).get_slots_mut().deplete_fuel(fuel_depleted);
    }
}