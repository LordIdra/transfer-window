use eframe::epaint::Color32;
use nalgebra_glm::vec2;

use transfer_window_model::api::builder::{AtmosphereBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, OrbitBuilder, VesselBuilder};
use transfer_window_model::components::orbitable_component::OrbitableType;
use transfer_window_model::components::path_component::orbit::orbit_direction::OrbitDirection;
use transfer_window_model::components::vessel_component::class::VesselClass;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::components::vessel_component::VesselComponent;
use transfer_window_model::Model;
use transfer_window_model::storage::entity_allocator::Entity;

use crate::game::overlay::dialogue::Dialogue;
use crate::game::storyteller::story::action::close_dialogue_action::CloseDialogueAction;
use crate::game::storyteller::story::action::create_vessel_action::CreateVesselAction;
use crate::game::storyteller::story::action::show_dialogue_action::ShowDialogueAction;
use crate::game::storyteller::story::condition::Condition;
use crate::game::storyteller::story::state::State;
use crate::game::storyteller::story::Story;
use crate::game::storyteller::story::transition::Transition;
use super::StoryBuilder;

#[derive(Debug, Default)]
pub struct Story01Welcome;

impl StoryBuilder for Story01Welcome {
    fn build(&self) -> (Model, Story, Option<Entity>) {
        let mut model = Model::default();

        let centralia = OrbitableBuilder {
            name: "Centralia",
            mass: 6.0 * 10.0e23,
            radius: 6.371e6,
            rotation_period: 1.0,
            rotation_angle: 100.0,
            type_: OrbitableType::Planet,
            physics: OrbitablePhysicsBuilder::Stationary(vec2(0.0, 0.0)),
            atmosphere: AtmosphereBuilder {
                color: Color32::from_hex("#3558A5").unwrap(),
                density: 0.9,
                height: 0.13,
                falloff: 2.0,
            }.build_some()
        }.build(&mut model);

        let mut story = Story::new("1");

        story.add("1", |_| State::default()
            .transition(Transition::new("2", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Hello. Welcome to the Transfer Window command interface.")
                    .with_continue()
                )
            )
        );

        story.add("2", |_| State::default()
            .transition(Transition::new("3", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("My name's Jake, and I'll be training you on the basics of using the interface. Once you've finished your training, you'll work to solve real-world tactical and strategic problems.")
                    .with_continue()
                )
            )
        );

        story.add("3", |_| State::default()
            .transition(Transition::new("4", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Please keep in mind that for now, this is a fully simulated environment, and your actions will have no real-world consequences. Now, let's begin.")
                    .with_continue()
                )
            )
        );

        story.add("4", |_| State::default()
            .transition(Transition::new("5", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Let's start with camera movement.\n")
                    .normal("- ").bold("Right cick and drag ").normal("to move the camera\n")
                    .normal("- ").bold("Scroll ").normal("to zoom in and out\n")
                    .normal("Try it now.")
                    .with_continue()
                )
            )
        );

        story.add("5", move |_| State::default()
            .transition(Transition::new("6", Condition::click_continue()))
            .action(CreateVesselAction::new(VesselBuilder {
                name: "Ship",
                vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                orbit_builder: OrbitBuilder::Circular { 
                    parent: centralia,
                    distance: 1.0e7,
                    angle: 0.0, 
                    direction: OrbitDirection::AntiClockwise, 
                },
            }))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("I've created a ship in orbit of Centralia.")
                    .with_continue()
                )
            )
        );

        story.add("6", |model: &Model| {
            let ship = model.entity_by_name("Ship");
            let ship_period = model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            State::default()
                .transition(Transition::new("7", Condition::time(ship_period).objective("Warp one orbit forward")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Try speeding up and slowing up time with the ")
                        .bold("+").normal(" and ").bold("= ")
                        .normal("keys to watch the ship orbit.")
                    )
                )
            }
        );

        story.add("7", |model: &Model| State::default()
            .transition(Transition::new("8", Condition::focus(model.entity_by_name("Ship")).objective("Focus the ship")))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Great. Now ")
                    .bold("right click ")
                    .normal("the ship and click the ")
                    .image("focus")
                    .normal("button to focus the camera on it.")
                )
            )
        );

        story.add("8", |_| State::default()
            .transition(Transition::new("9", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Vote reform UK!")
                    .with_continue()
                )
            )
        );

        story.add("9", |_model: &Model| State::default()
            .action(CloseDialogueAction::new()));

        (model, story, Some(centralia))
    }
}