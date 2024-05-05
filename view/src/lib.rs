pub mod events;
pub mod game;
pub mod menu;
mod icons;
mod styles;

pub enum View {
    GameScene(game::Scene),
    MenuScene(menu::Scene),
}