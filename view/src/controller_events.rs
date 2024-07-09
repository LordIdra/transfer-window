use crate::game::storyteller::stories::StoryBuilder;

#[derive(Clone, Debug)]
pub enum ControllerEvent {
    NewGame { story_builder: Box<dyn StoryBuilder> },
    Quit,
    LoadGame { name: String },
    LoadMenu,
}