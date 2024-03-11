mod fast_solver;
mod numerical_methods;
#[cfg(test)]
mod test_cases;
mod encounter;


// TODO - move these comments

// Incremental prediction is used at the start of the simulation and must only be run once
// This needs to be done for all orbitable entities
// Spacecraft do not need incremental prediction as they are not orbitable, so no other entities depend on them
// Besides, we might need to recalculate spacecraft trajectories when eg a burn is adjusted
// The way incremental prediction works is by predicting the trajectory of all orbitable entities at once

// Used to calculate the trajectory of one object (ie a spacecraft)
// Expects that all orbitables have had their trajectory until end_time computed