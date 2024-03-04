#![allow(dead_code)]

use state::State;

mod components;
mod constants;
mod debug;
mod state;
mod storage;
mod systems;
mod util;

fn main() {
    State::new();
}
