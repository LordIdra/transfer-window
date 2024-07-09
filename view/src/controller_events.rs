#[derive(Debug)]
pub enum ControllerEvent {
    NewGame,
    Quit,
    LoadGame { name: String },
}