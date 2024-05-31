pub mod events;
pub mod game;
pub mod menu;
mod rendering;
pub mod resources;
mod styles;

pub enum View {
    GameScene(game::Scene),
    MenuScene(menu::Scene),
}