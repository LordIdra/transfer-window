use nalgebra_glm::{vec2, DVec2};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BurnAdjustDirection {
    Prograde,
    Retrograde,
    Normal,
    Antinormal,
}

impl BurnAdjustDirection {
    pub fn vector(self) -> DVec2 {
        match self {
            BurnAdjustDirection::Prograde => vec2(1.0, 0.0),
            BurnAdjustDirection::Retrograde => vec2(-1.0, 0.0),
            BurnAdjustDirection::Normal => vec2(0.0, 1.0),
            BurnAdjustDirection::Antinormal => vec2(0.0, -1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BurnState {
    Selected,
    Adjusting,
    Dragging(BurnAdjustDirection),
}

impl BurnState {
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected)
    }

    pub fn is_adjusting(&self) -> bool {
        matches!(self, Self::Adjusting)
    }

    pub fn is_dragging(&self) -> bool {
        matches!(self, Self::Dragging(_))
    }
}

pub fn burn_adjustment_amount(amount: f64) -> f64 {
    if amount.is_sign_positive() {
        1.0e-8 * amount.powf(2.5)
    } else {
        20.0 * amount
    }
}