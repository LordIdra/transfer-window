pub mod controller_events;
pub mod game;
pub mod menu;
pub mod resources;
mod styles;

pub enum Scene {
    Game(game::View),
    Menu(menu::View),
}
