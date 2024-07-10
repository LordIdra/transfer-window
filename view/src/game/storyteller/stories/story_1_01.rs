use eframe::epaint::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::{api::{builder::{OrbitBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, VesselBuilder}, time::TimeStep}, components::{orbitable_component::OrbitableType, path_component::orbit::orbit_direction::OrbitDirection, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity, Model};
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;
use crate::game::{overlay::dialogue::Dialogue, storyteller::story::{action::{create_vessel_action::CreateVesselAction, finish_level_action::FinishLevelAction, set_focus_action::SetFocusAction, set_time_step_action::SetTimeStepAction, show_dialogue_action::ShowDialogueAction}, condition::Condition, state::State, transition::Transition, Story}, ViewConfig};

use super::StoryBuilder;

#[derive(Debug, Clone, Default)]
pub struct Story1_01;

impl StoryBuilder for Story1_01 {
    fn prerequisite(&self) -> Option<String> {
        None
    }

    fn build(&self) -> (Model, Story, ViewConfig, Option<Entity>) {
        let mut model = Model::default();

        let centralia = OrbitableBuilder {
            name: "Centralia",
            mass: 6.0 * 10.0e23,
            radius: 6.371e6,
            rotation_period: 1.0,
            rotation_angle: 100.0,
            type_: OrbitableType::Planet,
            physics: OrbitablePhysicsBuilder::Stationary(vec2(0.0, 0.0)),
            atmosphere: Atmosphere::new_some(
                Color32::from_hex("#3558A5").unwrap(),
                0.9,
                0.15,
                4.0
            )
        }.build(&mut model);

        let mut story = Story::new("intro-1");

        story.add("intro-1", |_| State::default()
            .transition(Transition::new("intro-2", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Hello. Welcome to the Transfer Window Interface.")
                    .with_continue()
                )
            )
        );

        story.add("intro-2", |_| State::default()
            .transition(Transition::new("intro-3", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("My name's Jake, and I'll be training you on the basics of using the Interface. Once you've finished your training, you'll work to solve real-world tactical and strategic problems.")
                    .with_continue()
                )
            )
        );

        story.add("intro-3", |_| State::default()
            .transition(Transition::new("camera-movement", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Please keep in mind that for now, this is a fully simulated environment, and your actions will have no real-world consequences. You have access to a limited subset of the Interface for now so as not to overwhelm you, but I'll activate new sections as you learn. Now, let's begin.")
                    .with_continue()
                )
            )
        );

        story.add("camera-movement", |_| State::default()
            .transition(Transition::new("created-ship", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Let's start with camera movement.\n")
                    .normal("- ").bold("Right click and drag ").normal("to move the camera\n")
                    .normal("- ").bold("Scroll ").normal("to zoom in and out\n")
                    .normal("Try it now.")
                    .with_continue()
                )
            )
        );

        story.add("created-ship", move |_| State::default()
            .transition(Transition::new("warp", Condition::click_continue()))
            .action(CreateVesselAction::new(VesselBuilder {
                name: "Ship 1",
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

        story.add("warp", |model| {
            let ship = model.entity_by_name("Ship 1").unwrap();
            let ship_period = model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = model.time() + ship_period;
            State::default()
                .transition(Transition::new("pause", Condition::time(time).objective("Warp forwards one orbit")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Try speeding up and slowing down time using the ")
                        .bold("+\n").normal("and ").bold("- ")
                        .normal("keys. Warp forward until the ship has completed at least one orbit.")
                    )
                )
            }
        );

        story.add("pause", |_| {
            State::default()
                .transition(Transition::new("change-focus", Condition::pause().objective("Pause the simulation")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Great. Now try tapping ")
                        .bold("spacebar\n")
                        .normal("to pause the simulation.")
                    )
                )
            }
        );

        story.add("change-focus", |model| {
            let ship = model.entity_by_name("Ship 1").unwrap();
            State::default()
                .transition(Transition::new("basic-controls-end", Condition::focus(ship).objective("Focus the ship")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Now let's change the camera focus. ")
                        .bold("Right click ")
                        .normal("the ship and click the ")
                        .image("focus")
                        .normal(" button to focus the camera on it.")
                    )
                )
                }
        );

        story.add("basic-controls-end", |_| State::default()
            .transition(Transition::new("orbit-intro", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Excellent. That's basic controls down.")
                    .with_continue()
                )
            )
        );

        story.add("orbit-intro", |_| State::default()
            .transition(Transition::new("orbit-definition", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Now let's take a look at how orbits work.")
                    .with_continue()
                )
            )
        );

        story.add("orbit-definition", |_| State::default()
            .transition(Transition::new("orbit-shapes", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("This ship is in orbit around Centralia. That means it's constantly falling towards Centralia due to gravity, but never hits the surface because it's travelling too fast. It's a bit like falling sideways so fast that you never hit the ground.")
                    .with_continue()
                )
            )
        );

        story.add("orbit-shapes", |_| State::default()
            .transition(Transition::new("orbit-ellipse", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("The ship I've created is in a circular orbit, but orbits can take other shapes.")
                    .with_continue()
                )
            )
        );

        story.add("orbit-ellipse", move |_| {
            State::default()
                .transition(Transition::new("orbit-ellipse-warp", Condition::click_continue()))
                .action(SetFocusAction::new(centralia))
                .action(CreateVesselAction::new(VesselBuilder {
                    name: "Ship 2",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                    orbit_builder: OrbitBuilder::Freeform { 
                        parent: centralia,
                        distance: 1.0e7,
                        speed: 8.0e3,
                        angle: 0.0,
                        direction: OrbitDirection::AntiClockwise, 
                    },
                }))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Here's another ship in an elliptical orbit.")
                        .with_continue()
                    )
                )
            }
        );

        story.add("orbit-ellipse-warp", move |model| {
            let ship = model.entity_by_name("Ship 2").unwrap();
            let ship_period = model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = model.time() + ship_period;
            State::default()
                .transition(Transition::new("orbit-ellipse-explanation", Condition::time(time).objective("Warp forwards one orbit")))
                .action(SetTimeStepAction::new(TimeStep::Level { level: 1, paused: false }))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("I've reset the simulation speed for you, so try speeding it up again and watch how the new ship orbits.")
                    )
                )
            }
        );

        story.add("orbit-ellipse-explanation", |_| State::default()
            .transition(Transition::new("orbit-conclusion", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Notice how it moves much slower when it gets further from the planet? I started it off at the exact same position, but moving faster than the first spacecraft. And yet, it takes much longer to complete an orbit than the first spacecraft. Take a moment to think about that.")
                    .with_continue()
                )
            )
        );

        story.add("orbit-conclusion", |_| State::default()
            .transition(Transition::new("end", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("There are lots of counterintuitive things about orbits, and you might find it hard to wrap your head around some of them. The rest of this training will help you learn how to reason about orbits, and soon you'll be using them to your advantage on the battlefield.")
                    .with_continue()
                )
            )
        );

        story.add("end", |_model: &Model| State::default()
            .action(FinishLevelAction::new("1-01".to_string())));

        let view_config = ViewConfig {
            draw_apsis_icons: false,
            can_select: false,
            draw_explorer: false,
            draw_timeline: false,
        };

        (model, story, view_config, Some(centralia))
    }
}