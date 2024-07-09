use std::fs;

use eframe::egui::{Context, ViewportCommand};
use log::error;
use transfer_window_model::Model;
use transfer_window_view::{game::{self, storyteller::{stories::StoryBuilder, story::Story}}, Scene};

use crate::Controller;

pub fn quit(context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Quit");

    context.send_viewport_cmd(ViewportCommand::Close);
}

pub fn new_game(controller: &mut Controller, context: &Context, story_builder: &dyn StoryBuilder) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("New game");

    let (model, story, focus) = story_builder.build();
    controller.scene = Scene::Game(game::View::new(controller.gl.clone(), model, story, context.clone(), controller.resources.clone(), focus));
}

pub fn load_game(controller: &mut Controller, context: &Context, name: &str) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Load game");
    let serialized = fs::read_to_string("saves/".to_string() + name + ".json");
    let Ok(serialized) = serialized else {
        error!("Failed to handle load game; error while loading file: {}", serialized.err().unwrap());
        return;
    };

    let model = Model::deserialize(serialized.as_str());
    let Ok(model) = model else {
    error!("Failed to handle load game; error while deseraizing: {}", model.err().unwrap());
    return;
    };

    controller.scene = Scene::Game(game::View::new(controller.gl.clone(), model, Story::empty(), context.clone(), controller.resources.clone(), None));
}

pub fn load_menu(controller: &mut Controller) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Load menu");

    controller.load_menu = true;
}
