use serde::{Deserialize, Serialize};

use super::{System, SystemType};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum WeaponType {
    Torpedo
}

impl SystemType for WeaponType {}

impl WeaponType {
    pub const TYPES: [WeaponType; 1] = [
        WeaponType::Torpedo,
    ];
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weapon {
    type_: WeaponType,
}

impl System for Weapon {
    type Type = WeaponType;
    
    fn get_type(&self) -> &Self::Type {
        &self.type_
    }
}

impl Weapon {
    pub fn new(type_: WeaponType) -> Self {
        Weapon { 
            type_, 
        }
    }
}