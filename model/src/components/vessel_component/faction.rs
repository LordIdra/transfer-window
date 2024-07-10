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
            Faction::Player | Faction::Ally => !matches!(other, Faction::Enemy),
            Faction::Enemy => matches!(other, Faction::Enemy),
        }
    }

    pub fn can_control(self, other: Self) -> bool {
        self == other
    }
}
