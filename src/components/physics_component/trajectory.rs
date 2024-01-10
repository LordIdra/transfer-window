use std::collections::HashMap;

use self::segment::Segment;

#[cfg(test)]
mod brute_force_tester;
mod burn;
mod segment;

pub struct Trajectory {
    index: usize,
    segments: HashMap<usize, Segment>
}