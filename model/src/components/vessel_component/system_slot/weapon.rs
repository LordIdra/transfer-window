use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum WeaponType {
    Torpedo
}

impl WeaponType {
    pub const TYPES: [WeaponType; 1] = [
        WeaponType::Torpedo,
    ];
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    type_: WeaponType,
}

impl Weapon {
    pub fn new(type_: WeaponType) -> Self {
        Weapon { 
            type_, 
        }
    }
    
    pub fn type_(&self) -> &WeaponType {
        &self.type_
    }
}