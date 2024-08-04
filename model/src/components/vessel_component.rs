use std::collections::BTreeMap;

use class::VesselClass;
use docking::{Docking, DockingPort, DockingPortLocation, DockingType};
use engine::{Engine, EngineType};
use faction::Faction;
use fuel_tank::{FuelTank, FuelTankType};
use log::error;
use rcs::{Rcs, RcsType};
use monopropellant_tank::{MonopropellantTank, MonopropellantTankType};
use serde::{Deserialize, Serialize};
use timeline::Timeline;
use torpedo_launcher::{TorpedoLauncher, TorpedoLauncherType};
use torpedo_storage::{TorpedoStorage, TorpedoStorageType};
use crate::storage::entity_allocator::Entity;

use super::path_component::orbit::scary_math::STANDARD_GRAVITY;

pub mod class;
pub mod docking;
pub mod engine;
pub mod faction;
pub mod fuel_tank;
pub mod monopropellant_tank;
pub mod rcs;
pub mod timeline;
pub mod torpedo_launcher;
pub mod torpedo_storage;


/// Must have `MassComponent` and (if undocked) `PathComponent`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VesselComponent {
    class: VesselClass,
    faction: Faction,
    dry_mass: f64,
    is_ghost: bool,
    can_dock: bool,
    timeline: Timeline,
    target: Option<Entity>,
    fuel_tank: Option<FuelTank>,
    monopropellant_tank: Option<MonopropellantTank>,
    engine: Option<Engine>,
    rcs: Option<Rcs>,
    torpedo_storage: Option<TorpedoStorage>,
    torpedo_launcher: Option<TorpedoLauncher>,
    docking: Option<Docking>,
}

impl VesselComponent {
    pub fn new(class: VesselClass, faction: Faction) -> Self {
        match class {
            VesselClass::Torpedo => Self { 
                class, faction,
                dry_mass: 2.0e3,
                is_ghost: true,
                can_dock: false,
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(FuelTankType::Torpedo)),
                engine: Some(Engine::new(EngineType::Torpedo)),
                monopropellant_tank: Some(MonopropellantTank::new(MonopropellantTankType::Torpedo)),
                rcs: Some(Rcs::new(RcsType::LightRcs)),
                torpedo_storage: None,
                torpedo_launcher: None,
                docking: None,
            },

            VesselClass::Station => Self { 
                class, faction,
                dry_mass: 380.0e3,
                is_ghost: false,
                can_dock: false,
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(FuelTankType::Hub)),
                engine: None,
                monopropellant_tank: None,
                rcs: None,
                torpedo_storage: Some(TorpedoStorage::new(TorpedoStorageType::Hub)),
                torpedo_launcher: None,
                docking: Some(Docking::new(DockingType::Quadruple)),
            },

            VesselClass::Scout1 => Self {
                class, faction,
                dry_mass: 10.0e3,
                is_ghost: false,
                can_dock: true,
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(FuelTankType::Tank1)),
                engine: Some(Engine::new(EngineType::Regular)),
                monopropellant_tank: Some(MonopropellantTank::new(MonopropellantTankType::Tank1)),
                rcs: Some(Rcs::new(RcsType::LightRcs)),
                torpedo_storage: None,
                torpedo_launcher: None,
                docking: None,
            },

            VesselClass::Frigate1 => Self {
                class, faction,
                dry_mass: 30.0e3,
                is_ghost: false,
                can_dock: true,
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(FuelTankType::Tank2)),
                engine: Some(Engine::new(EngineType::Regular)),
                monopropellant_tank: Some(MonopropellantTank::new(MonopropellantTankType::Tank2)),
                rcs: Some(Rcs::new(RcsType::LightRcs)),
                torpedo_storage: Some(TorpedoStorage::new(TorpedoStorageType::TorpedoStorage1)),
                torpedo_launcher: Some(TorpedoLauncher::new(TorpedoLauncherType::TorpedoLauncher1)),
                docking: None,
            },
        }
    }

    // ------------------------
    // Non-component getters
    // ------------------------
    pub fn with_ghost(mut self) -> Self {
        self.is_ghost = true;
        self
    }

    pub fn unset_ghost(&mut self) {
        self.is_ghost = false;
    }

    pub fn class(&self) -> VesselClass {
        self.class
    }

    pub fn faction(&self) -> Faction {
        self.faction
    }

    pub fn is_ghost(&self) -> bool {
        self.is_ghost
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }

    pub(crate) fn should_recompute_trajectory(&self) -> bool {
        !(self.class().is_torpedo() && self.timeline().last_event().is_some_and(|event| event.is_intercept()))
    }
    
    // ------------------------
    // Target
    // ------------------------
    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    pub fn target(&self) -> Option<Entity> {
        self.target
    }

    pub fn set_target(&mut self, target: Option<Entity>) {
        self.target = target;
    }

    // ------------------------
    // Fuel tank
    // ------------------------
    pub fn has_fuel_tank(&self) -> bool {
        self.fuel_tank.is_some()
    }

    pub fn fuel_capacity_litres(&self) -> f64 {
        match &self.fuel_tank {
            Some(fuel_tank) => fuel_tank.capacity_litres(),
            None => 0.0,
        }
    }

    pub fn fuel_capacity_kg(&self) -> f64 {
        match &self.fuel_tank {
            Some(fuel_tank) => fuel_tank.capacity_kg(),
            None => 0.0,
        }
    }

    pub fn fuel_litres(&self) -> f64 {
        match &self.fuel_tank {
            Some(fuel_tank) => fuel_tank.fuel_litres(),
            None => 0.0,
        }
    }

    pub fn fuel_kg(&self) -> f64 {
        match &self.fuel_tank {
            Some(fuel_tank) => fuel_tank.fuel_kg(),
            None => 0.0,
        }
    }

    pub fn is_fuel_empty(&self) -> bool {
        self.fuel_kg() < 1.0e-3
    }

    pub fn is_fuel_full(&self) -> bool {
        (self.fuel_capacity_kg() - self.fuel_kg()) < 1.0e-3
    }

    pub fn set_fuel_kg(&mut self, new_fuel_kg: f64) {
        self.fuel_tank.as_mut()
            .expect("Attempt to set fuel on vessel without fuel tank")
            .set_fuel_kg(new_fuel_kg);
    }

    // ------------------------
    // Monopropellant Tank
    // ------------------------
    pub fn has_monopropellant_tank(&self) -> bool {
        self.monopropellant_tank.is_some()
    }

    pub fn monopropellant_capacity_litres(&self) -> f64 {
        match &self.monopropellant_tank {
            Some(monopropellant_tank) => monopropellant_tank.capacity_litres(),
            None => 0.0,
        }
    }

    pub fn monopropellant_capacity_kg(&self) -> f64 {
        match &self.monopropellant_tank {
            Some(monopropellant_tank) => monopropellant_tank.capacity_kg(),
            None => 0.0,
        }
    }

    pub fn monopropellant_litres(&self) -> f64 {
        match &self.monopropellant_tank {
            Some(monopropellant_tank) => monopropellant_tank.fuel_litres(),
            None => 0.0,
        }
    }

    pub fn monopropellant_kg(&self) -> f64 {
        match &self.monopropellant_tank {
            Some(monopropellant_tank) => monopropellant_tank.fuel_kg(),
            None => 0.0,
        }
    }

    pub fn is_monopropellant_empty(&self) -> bool {
        self.monopropellant_kg() < 1.0e-3
    }

    pub fn is_monopropellant_full(&self) -> bool {
        (self.monopropellant_capacity_kg() - self.monopropellant_kg()) < 1.0e-3
    }

    pub fn set_monopropellant_kg(&mut self, new_monopropellant_kg: f64) {
        self.monopropellant_tank.as_mut()
            .expect("Attempt to set monopropellant on vessel without monopropellant tank")
            .set_fuel_kg(new_monopropellant_kg);
    }

    // ------------------------
    // Mass
    // ------------------------
    pub fn dry_mass(&self) -> f64 {
        self.dry_mass
            + self.docking.as_ref().map_or(0.0, |x| x.type_().mass())
            + self.fuel_tank.as_ref().map_or(0.0, |x| x.type_().mass())
            + self.monopropellant_tank.as_ref().map_or(0.0, |x| x.type_().mass())
            + self.engine.as_ref().map_or(0.0, |x| x.type_().mass())
            + self.rcs.as_ref().map_or(0.0, |x| x.type_().mass())
            + self.torpedo_storage.as_ref().map_or(0.0, |x| x.type_().mass())
            + self.torpedo_launcher.as_ref().map_or(0.0, |x| x.type_().mass())
    }

    pub fn wet_mass(&self) -> f64 {
        self.dry_mass() + self.fuel_capacity_kg() + self.monopropellant_capacity_kg()
    }

    pub fn fuel_wet_mass(&self) -> f64 {
        self.dry_mass() + self.fuel_capacity_kg()
    }

    pub fn monopropellant_wet_mass(&self) -> f64 {
        self.dry_mass() + self.monopropellant_capacity_kg()
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass() + self.fuel_kg() + self.monopropellant_kg()
    }

    // ------------------------
    // Engine
    // ------------------------
    pub fn has_engine(&self) -> bool {
        self.engine.is_some()
    }

    pub fn specific_impulse(&self) -> Option<f64> {
        self.engine.as_ref().map(Engine::specific_impulse)
    }

    pub fn fuel_kg_per_second(&self) -> f64 {
        match &self.engine {
            Some(engine) => engine.fuel_kg_per_second(),
            None => 0.0,
        }
    }

    pub fn max_dv(&self) -> f64 {
        match &self.engine {
            Some(engine) => {
                let initial_mass = self.dry_mass() + self.fuel_capacity_kg();
                let final_mass = self.dry_mass();
                let isp = engine.specific_impulse();
                isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass)
            },
            None => 0.0,
        }
    }

    pub fn dv(&self) -> f64 {
        match &self.engine {
            Some(engine) => {
                let initial_mass = self.mass();
                let final_mass = self.dry_mass();
                let isp = engine.specific_impulse();
                isp * STANDARD_GRAVITY * f64::ln(initial_mass / final_mass)
            },
            None =>0.0,
        }
    }

    // ------------------------
    // RCS
    // ------------------------
    pub fn has_rcs(&self) -> bool {
        self.rcs.is_some()
    }

    // ------------------------
    // Torpedo storage
    // ------------------------
    pub fn has_torpedo_storage(&self) -> bool {
        self.torpedo_storage.is_some()
    }

    pub fn torpedo_capacity(&self) -> usize {
        match &self.torpedo_storage {
            Some(torpedo_storage) => torpedo_storage.capacity(),
            None => 0,
        }
    }

    pub fn torpedoes(&self) -> usize {
        match &self.torpedo_storage {
            Some(torpedo_storage) => torpedo_storage.torpedoes(),
            None => 0,
        }
    }

    pub fn final_torpedoes(&self) -> usize { 
        self.torpedoes() - self.timeline().depleted_torpedoes()
    }

    pub fn is_torpedoes_empty(&self) -> bool {
        self.torpedoes() == 0
    }

    pub fn is_torpedoes_full(&self) -> bool {
        self.torpedoes() == self.torpedo_capacity()
    }

    pub fn increment_torpedoes(&mut self) {
        match &mut self.torpedo_storage {
            Some(torpedo_storage) => torpedo_storage.increment(),
            None => error!("Attempt to increment torpedoes on vessel without a torpedo storage"),
        }
    }

    pub fn decrement_torpedoes(&mut self) {
        match &mut self.torpedo_storage {
            Some(torpedo_storage) => torpedo_storage.decrement(),
            None => error!("Attempt to decrement torpedoes on vessel without a torpedo storage"),
        }
    }

    // ------------------------
    // Torpedo launcher
    // ------------------------
    pub fn has_torpedo_launcher(&self) -> bool {
        self.torpedo_launcher.is_some()
    }

    pub fn step_torpedo_launcher(&mut self, dt: f64) {
        self.torpedo_launcher.as_mut().unwrap().step_time_to_reload(dt);
    }

    pub fn torpedo_launcher_time_to_reload(&self) -> f64 {
        self.torpedo_launcher.as_ref().unwrap().time_to_reload()
    }

    // ------------------------
    // Docking
    // ------------------------
    pub fn has_docking(&self) -> bool {
        self.docking.is_some()
    }

    pub fn docking_ports(&self) -> Option<&BTreeMap<DockingPortLocation, DockingPort>> {
        Some(self.docking.as_ref()?.docking_ports())
    }

    pub fn docking_ports_mut(&mut self) -> Option<&mut BTreeMap<DockingPortLocation, DockingPort>> {
        Some(self.docking.as_mut()?.docking_ports_mut())
    }

    pub fn docking_port(&mut self, location: DockingPortLocation) -> &DockingPort {
        self.docking_ports()
            .expect("Attempt to get docking port on vessel without docking ports")
            .get(&location)
            .expect("No docking port at the requested location")
    }

    pub fn docking_port_mut(&mut self, location: DockingPortLocation) -> &mut DockingPort {
        self.docking_ports_mut()
            .expect("Attempt to get docking port on vessel without docking ports")
            .get_mut(&location)
            .expect("No docking port at the requested location")
    }

    pub fn can_dock(&self) -> bool {
        self.can_dock
    }

    pub fn dock(&mut self, location: DockingPortLocation, entity: Entity) {
        self.docking.as_mut().expect("Attempt to dock to vessel without docking ports").dock(location, entity);
    }

    pub fn undock(&mut self, location: DockingPortLocation) {
        self.docking.as_mut().expect("Attempt to dock to vessel without docking ports").undock(location);
    }
}