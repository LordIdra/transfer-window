pub mod events;
pub mod game;
pub mod menu;
mod icons;

pub enum View {
    GameScene(game::Scene),
    MenuScene(menu::Scene),
}