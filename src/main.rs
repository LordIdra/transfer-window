#![allow(dead_code)]

use state::State;

mod components;
mod constants;
mod state;
mod storage;
mod util;

fn main() {
    State::new();
}
