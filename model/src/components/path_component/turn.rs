use serde::{Deserialize, Serialize};

use crate::storage::entity_allocator::Entity;

use super::burn::rocket_equation_function::RocketEquationFunction;

const BURN_TIME_STEP: f64 = 0.001;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Burn {
//     parent: Entity,
//     rocket_equation_function: RocketEquationFunction, // For monopropellant
//     current_point: TurnPoint,
//     points: Vec<TurnPoint>,
// }
