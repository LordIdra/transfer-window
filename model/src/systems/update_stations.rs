use crate::{components::{vessel_component::station::{DockingPort, DockingPortLocation, ResourceTransferDirection}, ComponentType}, storage::entity_allocator::Entity, Model};

fn update_fuel_transfer(model: &mut Model, docking_port: DockingPort, docked_entity: Entity, station_entity: Entity, location: DockingPortLocation, dt: f64) {
    let Some(transfer) = docking_port.fuel_transfer() else {
         return 
    };

    let (from, to) = match transfer.direction() {
        ResourceTransferDirection::FromDocked => (docked_entity, station_entity),
        ResourceTransferDirection::ToDocked => (station_entity, docked_entity),
    };

    let from_fuel = model.vessel_component(from).fuel_kg();
    let to_fuel = model.vessel_component(to).fuel_kg();
    let remaining_space = model.vessel_component(to).max_fuel_kg() - to_fuel;
    let amount = f64::min(f64::min(dt * model.time_step().time_step() * transfer.rate(), from_fuel), remaining_space);

    if amount.abs() < 1.0e-3 {
        model.get_docking_port_mut(station_entity, location).stop_fuel_transfer();
    } else {
        model.vessel_component_mut(from).set_fuel_kg(from_fuel - amount);
        model.vessel_component_mut(to).set_fuel_kg(to_fuel + amount);
    }
}

fn update_torpedo_transfer(model: &mut Model, docking_port: DockingPort, docked_entity: Entity, station_entity: Entity, location: DockingPortLocation, dt: f64) {
    let Some(mut transfer) = docking_port.torpedo_transfer() else { 
        return 
    };

    let (from, to) = match transfer.direction() {
        ResourceTransferDirection::FromDocked => (docked_entity, station_entity),
        ResourceTransferDirection::ToDocked => (station_entity, docked_entity),
    };

    let simulation_dt = model.time_step().time_step() * dt;
    model.get_docking_port_mut(station_entity, location).step_torpedo_transfer(simulation_dt);
    transfer = transfer.step(simulation_dt);

    loop {
        if model.vessel_component(from).is_torpedoes_empty() || model.vessel_component(to).is_torpedoes_full() {
            model.get_docking_port_mut(station_entity, location).stop_torpedo_transfer();
            break;
        }

        if transfer.time_to_next() > 0.0 {
            break;
        }

        model.vessel_component_mut(from).decrement_torpedoes();
        model.vessel_component_mut(to).increment_torpedoes();

        model.get_docking_port_mut(station_entity, location).step_torpedo_transfer(-transfer.interval());
        transfer = transfer.step(-transfer.interval());
    }
}

fn update_docking_port(model: &mut Model, station_entity: Entity, location: DockingPortLocation, docking_port: DockingPort, dt: f64) {
    let docked_entity = docking_port.docked_entity();
    update_fuel_transfer(model, docking_port, docked_entity, station_entity, location, dt);
    update_torpedo_transfer(model, docking_port, docked_entity, station_entity, location, dt);
}

impl Model {
    pub(crate) fn update_stations(&mut self, dt: f64) {
        for entity in self.entities(vec![ComponentType::VesselComponent, ComponentType::PathComponent]) {
            if let Some(station) = self.vessel_component(entity).as_station() {
                for (location, docking_port) in station.docking_ports().clone() {
                    if let Some(docking_port) = docking_port {
                        update_docking_port(self, entity, location, docking_port, dt);
                    }
                }
            }
        }
    }
}