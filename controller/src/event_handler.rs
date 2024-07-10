use std::fs;

use eframe::egui::{Context, ViewportCommand};
use log::error;
use transfer_window_model::Model;
use transfer_window_view::{game::{self, storyteller::{stories::StoryBuilder, story::Story}, ViewConfig}, Scene};

use crate::Controller;

pub fn quit(context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Quit");

    context.send_viewport_cmd(ViewportCommand::Close);
}

pub fn new_game(controller: &mut Controller, context: &Context, story_builder: &dyn StoryBuilder) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("New game");

    let (model, story, view_config, focus) = story_builder.build();
    controller.scene = Scene::Game(game::View::new(controller.gl.clone(), model, story, context.clone(), controller.resources.clone(), view_config, focus));
}

pub fn load_game(controller: &mut Controller, context: &Context, name: &str) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Load game");
    let serialized = fs::read_to_string("data/saves/".to_string() + name + ".json");
    let Ok(serialized) = serialized else {
        error!("Failed to handle load game; error while loading file: {}", serialized.err().unwrap());
        return;
    };

    let model = Model::deserialize(serialized.as_str());
    let Ok(model) = model else {
    error!("Failed to handle load game; error while deserializing: {}", model.err().unwrap());
    return;
    };

    let view_config = ViewConfig::default();

    controller.scene = Scene::Game(game::View::new(controller.gl.clone(), model, Story::empty(), context.clone(), controller.resources.clone(), view_config, None));
}

pub fn finish_level(controller: &mut Controller, level: String) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Finish level");

    controller.load_menu = true;
    controller.completed_levels.add(level);
    controller.completed_levels.save();
}

pub fn exit_level(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Exit level");

    controller.load_menu = true;
}
