use serde::{Deserialize, Serialize};

use crate::components::{path_component::orbit::scary_math::STANDARD_GRAVITY, vessel_component::VesselComponent};

/// Helper struct for computing vessel mass after N seconds of
/// firing its engine using the rocket equation.
/// See <https://en.wikipedia.org/wiki/Tsiolkovsky_rocket_equation>.
/// Something to note that might be confusing: `burn_time` is a
/// measure of the time spent burning *at full thrust.* For example,
/// if we burnt for 10 seconds at full thrust, 10 seconds of burn
/// time would be added. If we burnt at half thrust, around (but
/// not exactly, due to rocket equation) 5 seconds would be added
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RocketEquationFunction {
    dry_mass_kg: f64,
    fuel_mass_kg: f64,
    fuel_consumption_kg_per_second: f64,
    specific_impulse: f64,
}

impl RocketEquationFunction {
    pub fn new(dry_mass_kg: f64, fuel_mass_kg: f64, fuel_consumption_kg_per_second: f64, specific_impulse: f64) -> Self {
        Self { dry_mass_kg, fuel_mass_kg, fuel_consumption_kg_per_second, specific_impulse}
    }

    /// # Panics
    /// Panics if the vessel component has no engine installed
    pub fn fuel_from_vessel_component(vessel_component: &VesselComponent) -> Self {
        let dry_mass_kg = vessel_component.dry_mass();
        let initial_fuel_mass_kg = vessel_component.fuel_kg();
        let fuel_consumption_kg_per_second = vessel_component.fuel_kg_per_second();
        let specific_impulse = vessel_component.specific_impulse().unwrap();
        RocketEquationFunction::new(dry_mass_kg, initial_fuel_mass_kg, fuel_consumption_kg_per_second, specific_impulse)
    }

    pub fn force(&self) -> f64 {
        STANDARD_GRAVITY * self.specific_impulse * self.fuel_consumption_kg_per_second
    }

    pub fn acceleration(&self) -> f64 {
        self.force() / self.mass()
    }

    pub fn mass(&self) -> f64 {
        self.dry_mass_kg + self.fuel_mass_kg
    }

    pub fn fuel_kg(&self) -> f64 {
        self.fuel_mass_kg
    }

    pub fn remaining_time(&self) -> f64 {
        self.fuel_mass_kg / self.fuel_consumption_kg_per_second
    }

    pub fn remaining_dv(&self) -> f64 {
        let start_mass = self.mass();
        let end_mass = self.dry_mass_kg;
        STANDARD_GRAVITY * self.specific_impulse * f64::ln(start_mass / end_mass)
    }

    pub fn remaining_fuel_kg(&self) -> f64 {
        self.fuel_mass_kg
    }

    pub fn time_to_step_dv(&self, dv: f64) -> Option<f64> {
        let remaining_time_after_step = self.step_by_dv(dv)?.remaining_time();
        Some(self.remaining_time() - remaining_time_after_step)
    }

    pub fn step_by_time(&self, time: f64) -> Option<Self> {
        if time == 0.0 {
            return Some(self.clone());
        }
        if time >= 0.0 && time <= self.remaining_time() {
            let fuel_mass_kg = self.fuel_mass_kg - self.fuel_consumption_kg_per_second * time;
            Some(Self::new(self.dry_mass_kg, fuel_mass_kg, self.fuel_consumption_kg_per_second, self.specific_impulse))
        } else {
            None
        }
    }

    pub fn step_by_dv(&self, dv: f64) -> Option<Self> {
        if dv == 0.0 {
            return Some(self.clone());
        }
        let time = (self.mass() / self.fuel_consumption_kg_per_second) * (1.0 - f64::exp(-dv / (self.specific_impulse * STANDARD_GRAVITY)));
        if time >= 0.0 && time <= self.remaining_time() {
            let fuel_mass_kg= self.fuel_mass_kg - self.fuel_consumption_kg_per_second * time;
            Some(Self::new(self.dry_mass_kg, fuel_mass_kg, self.fuel_consumption_kg_per_second, self.specific_impulse))
        } else {
            None
        }
    }

    pub fn end(&self) -> Self {
        Self::new(self.dry_mass_kg, 0.0, self.fuel_consumption_kg_per_second, self.specific_impulse)
    }
}

#[cfg(test)]
mod test {
    use super::RocketEquationFunction;

    #[test]
    fn test_basic() {
        let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 1.0);
        assert_eq!(rocket_equation_function.remaining_time(), 100.0);
    }

    #[test]
    fn test_step() {
        let rocket_equation_function = RocketEquationFunction::new(100.0, 100.0, 1.0, 1.0);
        let step_end_mass = rocket_equation_function.step_by_time(99.9999).unwrap().mass();
        let actual_end_mass = rocket_equation_function.end().mass();
        assert!((step_end_mass - actual_end_mass).abs() < 1.0e-3);
    }
}
