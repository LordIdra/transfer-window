use std::fmt::Debug;

use transfer_window_model::{storage::entity_allocator::Entity, Model};

use super::story::Story;

pub mod story_01_welcome;

// wtf?
// https://stackoverflow.com/questions/50017987/cant-clone-vecboxtrait-because-trait-cannot-be-made-into-an-object

pub trait StoryBuilder: StoryBuilderClone + Debug {
    fn build(&self) -> (Model, Story, Option<Entity>);
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