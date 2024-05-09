use serde::{Deserialize, Serialize};

use self::torpedo::Torpedo;

use super::{System, SystemType};

mod torpedo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WeaponType {
    Torpedo(Torpedo),
}

impl SystemType for WeaponType {}

impl WeaponType {
    pub fn types() -> [Self; 1] {
        [
            WeaponType::Torpedo(Torpedo::new())
        ]
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_torpedo(&self) -> &Torpedo {
        match self {
            WeaponType::Torpedo(torpedo) => torpedo,
            _ => panic!("Attempt to get non-torpedo weapon as torpedo"),
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_torpedo_mut(&mut self) -> &mut Torpedo {
        match self {
            WeaponType::Torpedo(torpedo) => torpedo,
            _ => panic!("Attempt to get non-torpedo weapon as torpedo"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weapon {
    type_: WeaponType,
}

impl System for Weapon {
    type Type = WeaponType;
    
    fn type_(&self) -> &Self::Type {
        &self.type_
    }

    fn type_mut(&mut self) -> &mut Self::Type {
        &mut self.type_
    }
}

impl Weapon {
    pub fn new(type_: WeaponType) -> Self {
        Weapon { 
            type_, 
        }
    }
}