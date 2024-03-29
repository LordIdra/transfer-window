pub mod events;
pub mod game;
pub mod menu;

pub enum View {
    GameScene(game::Scene),
    MenuScene(menu::Scene),
}