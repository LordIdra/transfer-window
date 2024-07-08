use serde::{Deserialize, Serialize};

use self::{engine::{Engine, EngineType}, fuel_tank::{FuelTank, FuelTankType}, weapon::{Weapon, WeaponType}};

pub mod engine;
pub mod fuel_tank;
pub mod weapon;

pub trait System {
    type Type: SystemType;
    fn type_(&self) -> &Self::Type;
    fn type_mut(&mut self) -> &mut Self::Type;
}

pub trait SystemType {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShipSlotLocation {
    Front,
    Middle,
    Back,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShipSlot {
    Weapon(Option<Weapon>),
    FuelTank(Option<FuelTank>),
    Engine(Option<Engine>),
}

impl ShipSlot {
    pub fn is_weapon(&self) -> bool {
        matches!(self, Self::Weapon(_))
    }

    pub fn is_fuel_tank(&self) -> bool {
        matches!(self, Self::FuelTank(_))
    }

    pub fn is_engine(&self) -> bool {
        matches!(self, Self::Engine(_))
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_weapon(&self) -> Option<&Weapon> {
        match self {
            ShipSlot::Weapon(weapon) => weapon.as_ref(),
            _ => panic!("Attempt to get non-weapon slot as weapon slot"),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_weapon_mut(&mut self) -> Option<&mut Weapon> {
        match self {
            ShipSlot::Weapon(weapon) => weapon.as_mut(),
            _ => panic!("Attempt to get non-weapon slot as weapon slot"),
        }
    }
    
    #[allow(clippy::missing_panics_doc)]
    pub fn as_fuel_tank(&self) -> Option<&FuelTank> {
        match self {
            ShipSlot::FuelTank(fuel_tank) => fuel_tank.as_ref(),
            _ => panic!("Attempt to get non-fuel-tank slot as fuel-tank slot"),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_engine(&self) -> Option<&Engine> {
        match self {
            ShipSlot::Engine(engine) => engine.as_ref(),
            _ => panic!("Attempt to get non-engine slot as engine slot"),
        }
    }

    pub fn new_weapon(type_: WeaponType) -> Self {
        Self::Weapon(Some(Weapon::new(type_)))
    }

    pub fn new_fuel_tank(type_: FuelTankType) -> Self {
        Self::FuelTank(Some(FuelTank::new(type_)))
    }

    pub fn new_engine(type_: EngineType) -> Self {
        Self::Engine(Some(Engine::new(type_)))
    }
}
