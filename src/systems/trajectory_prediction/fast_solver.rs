// ALRIGHT, HERE'S THE PLAN FOR ELLIPSE-ELLIPSE
// 1) Choose enttity and other_entity where we are finding encounters of entity with other_entity
// 2) Calculate SOI of other_entity
// 2) Construct a signed distance function (SDF) in terms of theta where the SDF goes negative when entity is further out than other_entity
// Note that the SDF is not the actual minimum distance between the objects
// Rather, it's just a similar (but much easier to evaluate) function that has roots in the same position and one minimum and one maximum
// 3) Find the minimum and maximum of the SDF
// 4) Construct a new function (F) representing the minimum distance between the two ellipses at a given theta for entity's ellipse
// Note that this function will need to be solved numerically using newton-raphson
// Also note this function is unsigned
// When we say min F, this is not actually the minimum of F, this is the value of F when SDF is a minimum
// 5) Start at theta = minimum SDF and use newton-raphson to find the minimum of F
// 6) Do the same for maximum of F
// 7) If min SDF > 0 and min F > SOI, it is impossible for an encounter to happen; return no encounters
// 8) If min SDF < 0 and min F > SOI, there are 2 possible encounters during one period; go to TWO ENCOUNTERS
// 9) Otherwise, there is 1 possible encounter; go to ONE ENCOUNTER

// ONE ENCOUNTER
// 1) Construct a function G = F - SOI
// We now want to solve for G = 0  and know there will be exactly 2 solutions
// We also know that at G = 0, G is at a minimum, therefore dG/dtheta = 0
// We also know that the solutions lie on either side of min F and max F (these are the two maximums of G)
// 2) Use binary search to find estimates to the solutions of dG/dtheta = 0 in the intervals min G to max G and max G to min G
// 3) Use newton-raphson to refine the estimates
// 4) Construct a range from one solution to the other that includings min G
// This range is the range of thetas in which an encounter is possible
// 5) Return SOLVE WINDOW with the range

// TWO ENCOUNTERS
// 1) Construct a function G = F - SOI
// We now want to solve for G = 0  and know there will be exactly 4 solutions
// We also know that at G = 0, G is at a minimum, therefore dG/dtheta = 0
// 2) Solve for F = 0
// We know F = 0 is at a minimum, so just solve for dF/dtheta = 0 using binary search + newton refinement
// 3) Order min F, max F, and the two solutions to F = 0
// 4) Pair each solution in the order they appear to create 4 pairs (last will need to be paired with first)
// We know that each quadrant contains exactly one solution to G = 0
// 5) Solve for G = 0 in each quadrant by solving dG/dtheta = 0 with binary search and newton refinement
// 6) Pair the solutions up into 2 pairs so that each pair contains one of the two solutions to F = 0
// Do this by ordering the solutions, then pairing the first two and last two. Do the same but pair the first and last and middle two
// Then check which set of pairs contains both solutions to F = 0
// Angles might be difficult to deal with here
// We now have two ranges of thetas where an encounter is possible
// Return the lowest of SOLVE WINDOW with first range and SOLVE WINDOW with second range

// test cases to add:
// moon orbiting very close to earth with various eccentricities of spacecraft
// Encounters with two moons

mod bounding;

#[cfg(test)]
mod test {
    use crate::{debug::get_entity_by_name, systems::trajectory_prediction::test_cases::load_case};

    use super::bounding::find_encounter_bounds;

    #[test]
    fn temp() {
        let (mut state, mut encounters, _, end_time, time_step) = load_case("insanity-3");
        let moon = get_entity_by_name(&state, "moon");
        let spacecraft = get_entity_by_name(&state, "spacecraft");
        let moon_orbit = state.get_trajectory_component(moon).get_current_segment().as_orbit();
        let spacecraft_orbit = state.get_trajectory_component(spacecraft).get_current_segment().as_orbit();
        println!("{:?}", find_encounter_bounds(spacecraft_orbit, moon_orbit));
    }
}