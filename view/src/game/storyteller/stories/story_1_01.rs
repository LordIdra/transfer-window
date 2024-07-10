use eframe::epaint::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::{api::{builder::{OrbitBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, VesselBuilder}, time::TimeStep}, components::{orbitable_component::OrbitableType, path_component::orbit::orbit_direction::OrbitDirection, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity, Model};
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;

use crate::controller_events::ControllerEvent;
use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::dialogue::Dialogue;
use crate::game::storyteller::story::condition::Condition;
use crate::game::storyteller::story::state::State;
use crate::game::storyteller::story::Story;
use crate::game::ViewConfig;

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
                Color32::from_hex("#a0b5ff").unwrap(),
                0.95,
                0.3,
                4.0,
                vec![0.7]
            )
        }.build(&mut model);

        let mut story = Story::new("intro-1");

        story.add("intro-1", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Hello. Welcome to the Transfer Window Interface.")
                    .with_continue()
            ));
            State::new("intro-2", Condition::click_continue())
        });

        story.add("intro-2", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("My name's Jake, and I'll be training you on the basics of using the Interface. Once you've finished your training, you'll work to solve real-world orbital warfare problems.")
                    .with_continue()
            ));
            State::new("intro-3", Condition::click_continue())
        });

        story.add("intro-3", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Please keep in mind that for now, this is a fully simulated environment, and your actions will have no real-world consequences. You have access to a limited subset of the Interface for now so as not to overwhelm you, but I'll activate new sections as you learn. Now, let's begin.")
                    .with_continue()
            ));
            State::new("camera-movement", Condition::click_continue())
        });

        story.add("camera-movement", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Let's start with camera movement.\n")
                    .normal("- ").bold("Right click and drag ").normal("to move the camera\n")
                    .normal("- ").bold("Scroll ").normal("to zoom in and out\n")
                    .normal("Try it now.")
                    .with_continue()
            ));
            State::new("created-ship", Condition::click_continue())
        });

        story.add("created-ship", move |view| {
            view.add_model_event(ModelEvent::BuildVessel { 
                vessel_builder: VesselBuilder {
                    name: "Ship 1",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                    orbit_builder: OrbitBuilder::Circular { 
                        parent: centralia,
                        distance: 1.0e7,
                        angle: 0.0, 
                        direction: OrbitDirection::AntiClockwise, 
                    },
                }
            });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("I've created a ship in orbit of Centralia.")
                    .with_continue()
            ));
            State::new("warp", Condition::click_continue())
        });

        story.add("warp", |view| {
            let ship = view.model.entity_by_name("Ship 1").unwrap();
            let ship_period = view.model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = view.model.time() + ship_period;
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Try speeding up and slowing down time using the ")
                    .bold("+\n").normal("and ").bold("- ")
                    .normal("keys. Warp forward until the ship has completed at least one orbit.")
            ));
            State::new("pause", Condition::time(time).objective("Warp forwards one orbit"))
        });

        story.add("pause", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Great. Now try tapping ")
                    .bold("spacebar\n")
                    .normal("to pause the simulation.")
            ));
            State::new("change-focus", Condition::pause().objective("Pause the simulation"))
        });

        story.add("change-focus", |view| {
            let ship = view.model.entity_by_name("Ship 1").unwrap();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now let's change the camera focus. ")
                    .bold("Right click ")
                    .normal("the ship and click the ")
                    .image("focus")
                    .normal(" button to focus the camera on it.")
            ));
            State::new("basic-controls-end", Condition::focus(ship).objective("Focus the ship"))
        });

        story.add("basic-controls-end", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Excellent. That's basic controls down.")
                    .with_continue()
            ));
            State::new("orbit-intro", Condition::click_continue())
        });

        story.add("orbit-intro", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now let's take a look at how orbits work.")
                    .with_continue()
            ));
            State::new("orbit-definition", Condition::click_continue())
        });

        story.add("orbit-definition", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("This ship is in orbit around Centralia. That means it's constantly falling towards Centralia due to gravity, but never hits the surface because it's travelling too fast. It's a bit like falling sideways so fast that you never hit the ground.")
                    .with_continue()
            ));
            State::new("orbit-shapes", Condition::click_continue())
        });

        story.add("orbit-shapes", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("The ship I've created is in a circular orbit, but orbits can take other shapes.")
                    .with_continue()
            ));
            State::new("orbit-ellipse", Condition::click_continue())
        });

        story.add("orbit-ellipse", move |view| {
            view.add_view_event(ViewEvent::SetCameraFocus(centralia));
            view.add_model_event(ModelEvent::BuildVessel { 
                vessel_builder: VesselBuilder {
                    name: "Ship 2",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                    orbit_builder: OrbitBuilder::Freeform { 
                        parent: centralia,
                        distance: 1.0e7,
                        speed: 8.0e3,
                        angle: 0.0,
                        direction: OrbitDirection::AntiClockwise, 
                    },
                }
            });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Here's another ship in an elliptical orbit.")
                    .with_continue()
            ));
            State::new("orbit-ellipse-warp", Condition::click_continue())
            }
        );

        story.add("orbit-ellipse-warp", |view| {
            let ship = view.model.entity_by_name("Ship 2").unwrap();
            let ship_period = view.model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = view.model.time() + ship_period;
            view.add_model_event(ModelEvent::SetTimeStep { 
                time_step: TimeStep::Level { level: 1, paused: false }
            });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("I've reset the simulation speed for you, so try speeding it up again and watch how the new ship orbits.")
            ));
            State::new("orbit-ellipse-explanation", Condition::time(time).objective("Warp forwards one orbit"))
        });

        story.add("orbit-ellipse-explanation", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Notice how it moves much slower when it gets further from the planet? I started it off at the exact same position, but moving faster than the first spacecraft. And yet, it takes much longer to complete an orbit than the first spacecraft. Take a moment to think about that.")
                    .with_continue()
            ));
            State::new("orbit-conclusion", Condition::click_continue())
        });

        story.add("orbit-conclusion", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("There are lots of counterintuitive things about orbits, and you might find it hard to wrap your head around some of them. The rest of this training will help you learn how to reason about orbits, and soon you'll be using them to your advantage on the battlefield.")
                    .with_continue()
            ));
            State::new("end", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-01".to_string() });
            view.add_controller_event(ControllerEvent::ExitLevel);
            State::default()
        });

        let view_config = ViewConfig {
            draw_apsis_icons: false,
            can_select: false,
            draw_explorer: false,
            draw_timeline: false,
        };

        (model, story, view_config, Some(centralia))
    }
}