use std::collections::BTreeMap;

use class::VesselClass;
use docking::{Docking, DockingPort, DockingPortLocation};
use engine::Engine;
use faction::Faction;
use fuel_tank::FuelTank;
use log::error;
use nalgebra_glm::{vec2, DVec2};
use rcs::{Rcs, RcsControlScheme, RcsThruster};
use serde::{Deserialize, Serialize};
use timeline::Timeline;
use torpedo_launcher::TorpedoLauncher;
use torpedo_storage::TorpedoStorage;
use crate::storage::entity_allocator::Entity;

use super::path_component::orbit::scary_math::STANDARD_GRAVITY;

pub mod class;
pub mod docking;
pub mod engine;
pub mod faction;
pub mod fuel_tank;
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
    dimensions: DVec2,
    timeline: Timeline,
    target: Option<Entity>,
    fuel_tank: Option<FuelTank>,
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
                dimensions: vec2(0.0, 0.0),
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(10_000.0)),
                engine: Some(Engine::new(7.0, 15_000.0)),
                rcs: None,
                torpedo_storage: None,
                torpedo_launcher: None,
                docking: None,
            },

            VesselClass::Station => Self { 
                class, faction,
                dry_mass: 380.0e3,
                is_ghost: false,
                can_dock: false,
                dimensions: vec2(2680.0, 2000.0),
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(140_000.0)),
                engine: None,
                rcs: None,
                torpedo_storage: Some(TorpedoStorage::new(5)),
                torpedo_launcher: None,
                docking: Some(Docking::new(vec![DockingPortLocation::West, DockingPortLocation::East])),
            },

            VesselClass::Scout1 => Self {
                class, faction,
                dry_mass: 10.0e3,
                is_ghost: false,
                can_dock: true,
                dimensions: vec2(540.0, 200.0),
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(30_000.0)),
                engine: Some(Engine::new(30.0, 70_000.0)),
                rcs: Some(Rcs::new(5.0, 
                    vec![
                        RcsThruster::new(1.0, 0.1, vec2(10.0, 78.0), vec2(0.0, -1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, 78.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, 78.0), vec2(-1.0, 0.0)),

                        RcsThruster::new(1.0, 0.1, vec2(10.0, -78.0), vec2(0.0, 1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, -78.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, -78.0), vec2(-1.0, 0.0)),

                        RcsThruster::new(1.0, 0.1, vec2(-199.0, 88.0), vec2(0.0, -1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, 88.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, 88.0), vec2(-1.0, 0.0)),

                        RcsThruster::new(1.0, 0.1, vec2(-199.0, -88.0), vec2(0.0, 1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, -88.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, -88.0), vec2(-1.0, 0.0)),
                    ],
                    RcsControlScheme::new(None, Some(541.0), 
                        vec![
                            0.0, 0.0, 1.0,
                            1.0, 1.0, 0.0,
                            1.0, 0.0, 1.0,
                            0.0, 1.0, 0.0
                        ],
                        vec![
                            1.0, 1.0, 0.0,
                            0.0, 0.0, 1.0,
                            0.0, 1.0, 0.0,
                            1.0, 0.0, 1.0
                        ])
                    )
                ),
                torpedo_storage: None,
                torpedo_launcher: None,
                docking: None,
            },

            VesselClass::Frigate1 => Self {
                class, faction,
                dry_mass: 30.0e3,
                is_ghost: false,
                can_dock: true,
                dimensions: vec2(0.0, 0.0),
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(60_000.0)),
                engine: Some(Engine::new(30.0, 70_000.0)),
                rcs: None,
                torpedo_storage: Some(TorpedoStorage::new(2)),
                torpedo_launcher: Some(TorpedoLauncher::new(2.0 * 60.0 * 60.0)),
                docking: None,
            },

            VesselClass::TestShip => Self {
                class, faction,
                dry_mass: 50.0e3,
                is_ghost: false,
                can_dock: true,
                dimensions: vec2(0.0, 0.0),
                timeline: Timeline::default(),
                target: None,
                fuel_tank: Some(FuelTank::new(60_000.0)),
                engine: Some(Engine::new(10000.0, 10_000_000.0)),
                rcs: Some(Rcs::new(5.0, 
                    vec![
                        RcsThruster::new(1.0, 0.1, vec2(10.0, 78.0), vec2(0.0, -1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, 78.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, 78.0), vec2(-1.0, 0.0)),

                        RcsThruster::new(1.0, 0.1, vec2(10.0, -78.0), vec2(0.0, 1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, -78.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(10.0, -78.0), vec2(-1.0, 0.0)),

                        RcsThruster::new(1.0, 0.1, vec2(-199.0, 88.0), vec2(0.0, -1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, 88.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, 88.0), vec2(-1.0, 0.0)),

                        RcsThruster::new(1.0, 0.1, vec2(-199.0, -88.0), vec2(0.0, 1.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, -88.0), vec2(1.0, 0.0)),
                        RcsThruster::new(1.0, 0.1, vec2(-199.0, -88.0), vec2(-1.0, 0.0)),
                    ],
                    RcsControlScheme::new(None, Some(541.0), 
                        vec![
                            0.0, 0.0, 1.0,
                            1.0, 1.0, 0.0,
                            1.0, 0.0, 1.0,
                            0.0, 1.0, 0.0
                        ],
                        vec![
                            1.0, 1.0, 0.0,
                            0.0, 0.0, 1.0,
                            0.0, 1.0, 0.0,
                            1.0, 0.0, 1.0
                        ])
                    )
                ),
                torpedo_storage: Some(TorpedoStorage::new(8)),
                torpedo_launcher: Some(TorpedoLauncher::new(60.0 * 60.0)),
                docking: None,
            }
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
    // Mass
    // ------------------------
    pub fn dry_mass(&self) -> f64 {
        self.dry_mass
    }

    pub fn wet_mass(&self) -> f64 {
        self.dry_mass() + self.fuel_capacity_kg()
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass() + self.fuel_kg()
    }

    // ------------------------
    // Engine
    // ------------------------
    pub fn has_engine(&self) -> bool {
        self.engine.is_some()
    }

    pub fn engine(&self) -> Option<&Engine> {
        self.engine.as_ref()
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

    pub fn rcs(&self) -> Option<&Rcs> {
        self.rcs.as_ref()
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

    pub fn dimensions(&self) -> DVec2 {
        self.dimensions
    }
}
