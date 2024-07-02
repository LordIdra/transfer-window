use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Faction {
    Player,
    Ally,
    Enemy,
}

impl Faction {
    pub fn has_intel_for(self, other: Self) -> bool {
        match self {
            Faction::Player | Faction::Ally => match other {
                Faction::Player | Faction::Ally => true,
                Faction::Enemy => false,
            }
            Faction::Enemy => match other {
                Faction::Player | Faction::Ally => false,
                Faction::Enemy => true,
            }
        }
    }

    pub fn can_control(self, other: Self) -> bool {
        self == other
    }
}