use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WeaponType {
    Torpedo
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weapon {
    type_: WeaponType,
}

impl Weapon {
    pub fn new(type_: WeaponType) -> Self {
        Weapon { 
            type_: type_, 
        }
    }
    
    pub fn type_(&self) -> &WeaponType {
        &self.type_
    }
}