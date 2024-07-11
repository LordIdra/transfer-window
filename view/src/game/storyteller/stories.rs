use std::fmt::Debug;

use transfer_window_model::{storage::entity_allocator::Entity, Model};

use crate::game::ViewConfig;

use super::story::Story;

pub mod story_1_01;
pub mod story_1_02;
pub mod story_1_03;

// wtf?
// https://stackoverflow.com/questions/50017987/cant-clone-vecboxtrait-because-trait-cannot-be-made-into-an-object

pub trait StoryBuilder: StoryBuilderClone + Debug {
    fn prerequisite(&self) -> Option<String>;
    fn build(&self) -> (Model, Story, ViewConfig, Option<Entity>);
}

pub trait StoryBuilderClone {
    fn clone_box(&self) -> Box<dyn StoryBuilder>;
}

impl<T: 'static + StoryBuilder + Clone> StoryBuilderClone for T {
    fn clone_box(&self) -> Box<dyn StoryBuilder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn StoryBuilder> {
    fn clone(&self) -> Box<dyn StoryBuilder> {
        self.clone_box()
    }
}