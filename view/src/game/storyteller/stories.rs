use std::fmt::Debug;

use transfer_window_model::{storage::entity_allocator::Entity, Model};

use super::story::Story;

pub mod story_01_welcome;

pub trait StoryBuilder: Debug {
    fn build(&self) -> (Model, Story, Option<Entity>);
}
