use super::burn::Burn;

pub enum Segment {
    Orbit(),
    Burn(Burn),
}