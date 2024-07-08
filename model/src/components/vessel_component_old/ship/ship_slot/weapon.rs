use serde::{Deserialize, Serialize};

use self::torpedo::Torpedo;

use super::{System, SystemType};

pub mod torpedo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WeaponType {
    Torpedo(Torpedo),
    EnhancedTorpedo(Torpedo),
}

impl SystemType for WeaponType {}

impl WeaponType {
    pub fn new_torpedo() -> Self {
        Self::Torpedo(Torpedo::new(2))
    }

    pub fn new_enhanced_torpedo() -> Self {
        Self::EnhancedTorpedo(Torpedo::new(4))
    }

    pub fn types() -> [Self; 2] {
        [
            Self::new_torpedo(),
            Self::new_enhanced_torpedo(),
        ]
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_torpedo(&self) -> &Torpedo {
        match self {
            WeaponType::EnhancedTorpedo(torpedo) | WeaponType::Torpedo(torpedo) => torpedo,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn as_torpedo_mut(&mut self) -> &mut Torpedo {
        match self {
            WeaponType::EnhancedTorpedo(torpedo) | WeaponType::Torpedo(torpedo) => torpedo,
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