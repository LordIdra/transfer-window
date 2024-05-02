use serde::{Deserialize, Serialize};

use crate::components::trajectory_component::orbit::scary_math::STANDARD_GRAVITY;

/// Helper struct for computing vessel mass after N seconds of
/// firing its engine using the rocket equation
/// See <https://en.wikipedia.org/wiki/Tsiolkovsky_rocket_equation>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RocketEquationFunction {
    dry_mass_kg: f64,
    /// Fuel mass at burn_time = 0
    initial_fuel_mass_kg: f64,
    fuel_consumption_kg_per_second: f64,
    specific_impulse: f64,
    burn_time: f64,
}

impl RocketEquationFunction {
    pub fn new(dry_mass_kg: f64, initial_fuel_mass_kg: f64, fuel_consumption_kg_per_second: f64, specific_impulse: f64, burn_time: f64) -> Self {
        Self { dry_mass_kg, initial_fuel_mass_kg, fuel_consumption_kg_per_second, specific_impulse, burn_time }
    }

    pub fn start(&self) -> Self {
        Self::new(self.dry_mass_kg, self.initial_fuel_mass_kg, self.fuel_consumption_kg_per_second, self.specific_impulse, 0.0)
    }

    pub fn end(&self) -> Self {
        let end_time = self.initial_fuel_mass_kg / self.fuel_consumption_kg_per_second;
        Self::new(self.dry_mass_kg, self.initial_fuel_mass_kg, self.fuel_consumption_kg_per_second, self.specific_impulse, end_time)
    }

    pub fn step_by_time(&self, time_to_step: f64) -> Option<Self> {
        if time_to_step == 0.0 {
            return Some(self.clone());
        }
        let new_burn_time = self.burn_time + time_to_step;
        if new_burn_time >= 0.0 && new_burn_time <= self.end().get_burn_time() {
            Some(Self::new(self.dry_mass_kg, self.initial_fuel_mass_kg, self.fuel_consumption_kg_per_second, self.specific_impulse, new_burn_time))
        } else {
            None
        }
    }

    pub fn step_by_dv(&self, dv: f64) -> Option<Self> {
        if dv == 0.0 {
            return Some(self.clone());
        }
        let extra_burn_time = (self.get_mass() / self.fuel_consumption_kg_per_second) * (1.0 - f64::exp(-dv / (self.specific_impulse * STANDARD_GRAVITY)));
        let new_burn_time = self.burn_time + extra_burn_time;
        if new_burn_time >= 0.0 && new_burn_time <= self.end().get_burn_time() {
            Some(Self::new(self.dry_mass_kg, self.initial_fuel_mass_kg, self.fuel_consumption_kg_per_second, self.specific_impulse, new_burn_time))
        } else {
            None
        }
    }

    pub fn get_burn_time(&self) -> f64 {
        self.burn_time
    }

    pub fn get_used_dv(&self) -> f64 {
        let start_mass = self.dry_mass_kg + self.initial_fuel_mass_kg;
        let end_mass = self.get_mass();
        STANDARD_GRAVITY * self.specific_impulse * f64::ln(start_mass / end_mass)
    }

    pub fn get_mass(&self) -> f64 {
        let start_mass = self.dry_mass_kg + self.initial_fuel_mass_kg;
        let burnt_mass = self.fuel_consumption_kg_per_second * self.burn_time;
        start_mass - burnt_mass
    }

    pub fn get_fuel_kg_burnt(&self) -> f64 {
        self.fuel_consumption_kg_per_second * self.burn_time
    }

    pub fn get_acceleration(&self) -> f64 {
        let force = STANDARD_GRAVITY * self.specific_impulse * self.fuel_consumption_kg_per_second;
        force / self.get_mass()
    }
}

#[cfg(test)]
mod test {
    use super::RocketEquationFunction;

    #[test]
    fn test_basic() {
        let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 1.0, 0.0);
        assert_eq!(rocket_equation_function.end().get_burn_time(), 100.0);
        assert_eq!(rocket_equation_function.end().start().get_mass(), 200.0);
    }

    #[test]
    fn test_step() {
        let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 1.0, 0.0);
        let step_end_mass = rocket_equation_function.step_by_time(99.9999).unwrap().get_mass();
        let actual_end_mass = rocket_equation_function.end().get_mass();
        assert!((step_end_mass - actual_end_mass).abs() < 1.0e-3);

        let step_end_time = rocket_equation_function.step_by_dv(rocket_equation_function.end().get_used_dv()).unwrap().get_burn_time();
        let actual_end_time = rocket_equation_function.end().get_burn_time();
        assert!((step_end_time - actual_end_time).abs() < 1.0e-3)
    }
}