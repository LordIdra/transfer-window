use crate::{components::{vessel_component::docking::{DockedVessel, DockingPortLocation, ResourceTransferDirection}, ComponentType}, storage::entity_allocator::Entity, Model};

fn update_fuel_transfer(model: &mut Model, docked_vessel: &DockedVessel, docked_entity: Entity, station_entity: Entity, location: DockingPortLocation, dt: f64) {
    let Some(transfer) = docked_vessel.fuel_transfer() else {
         return 
    };

    let (from, to) = match transfer.direction() {
        ResourceTransferDirection::FromDocked => (docked_entity, station_entity),
        ResourceTransferDirection::ToDocked => (station_entity, docked_entity),
    };

    let from_fuel = model.vessel_component(from).fuel_kg();
    let to_fuel = model.vessel_component(to).fuel_kg();
    let remaining_space = model.vessel_component(to).fuel_capacity_kg() - to_fuel;
    let amount = f64::min(f64::min(dt * model.time_step().time_step() * transfer.rate(), from_fuel), remaining_space);

    if amount.abs() < 1.0e-3 {
        model.vessel_component_mut(station_entity).docking_port_mut(location).docked_vessel_mut().stop_fuel_transfer();
    } else {
        model.vessel_component_mut(from).set_fuel_kg(from_fuel - amount);
        model.vessel_component_mut(to).set_fuel_kg(to_fuel + amount);
    }
}

fn update_torpedo_transfer(model: &mut Model, docked_vessel: &DockedVessel, docked_entity: Entity, station_entity: Entity, location: DockingPortLocation, dt: f64) {
    let Some(mut transfer) = docked_vessel.torpedo_transfer().cloned() else { 
        return 
    };

    let (from, to) = match transfer.direction() {
        ResourceTransferDirection::FromDocked => (docked_entity, station_entity),
        ResourceTransferDirection::ToDocked => (station_entity, docked_entity),
    };

    let simulation_dt = model.time_step().time_step() * dt;
    model.docking_port_mut(station_entity, location).docked_vessel_mut().step_torpedo_transfer(simulation_dt);
    transfer = transfer.step(simulation_dt);

    loop {
        if model.vessel_component(from).is_torpedoes_empty() || model.vessel_component(to).is_torpedoes_full() {
            model.docking_port_mut(station_entity, location).docked_vessel_mut().stop_torpedo_transfer();
            break;
        }

        if transfer.time_to_next() > 0.0 {
            break;
        }

        model.vessel_component_mut(from).decrement_torpedoes();
        model.vessel_component_mut(to).increment_torpedoes();

        model.docking_port_mut(station_entity, location).docked_vessel_mut().step_torpedo_transfer(-transfer.interval());
        transfer = transfer.step(-transfer.interval());
    }
}

fn update_docking_port(model: &mut Model, station_entity: Entity, location: DockingPortLocation, docked_vessel: &DockedVessel, dt: f64) {
    let docked_entity = docked_vessel.entity();
    update_fuel_transfer(model, docked_vessel, docked_entity, station_entity, location, dt);
    update_torpedo_transfer(model, docked_vessel, docked_entity, station_entity, location, dt);
}

impl Model {
    pub(crate) fn update_stations(&mut self, dt: f64) {
        for entity in self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            if let Some(docking_ports) = self.vessel_component(entity).docking_ports() {
                for (location, docking_port) in docking_ports.clone() {
                    if docking_port.has_docked_vessel() {
                        update_docking_port(self, entity, location, docking_port.docked_vessel(), dt);
                    }
                }
            }
        }
    }
}