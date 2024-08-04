use eframe::epaint::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::components::vessel_component::engine::EngineType;
use transfer_window_model::components::vessel_component::fuel_tank::FuelTankType;
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
            mass: 5.972e24,
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

        let view_config = ViewConfig {
            draw_apsis_icons: false,
            can_select: false,
            draw_explorer: false,
            draw_timeline: false,
        };

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
                    .normal("My name's Jake, and I'll be training you on the basics of using the Interface and orbital mechanics. Please bear with me if you've covered orbital mechanics beforehand; there is still a lot to learn with regards to the Interface. Once you've finished your training, you'll work to solve real-world orbital warfare problems.")
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
                    .normal(" button to focus the camera on it. When an object is focused, the camera will move with it.")
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
                    .normal("Notice how it moves much slower when it gets further from the planet? I started it off at the exact same position, but moving faster than the first spacecraft. And yet, it takes much longer to complete an orbit than the first spacecraft. This is one of the many counterintuitive things about orbital mechanics.")
                    .with_continue()
            ));
            State::new("orbit-apsis", Condition::click_continue())
        });

        story.add("orbit-apsis", move |view| {
            let ship_1 = view.model.entity_by_name("Ship 1").unwrap();
            let ship_2 = view.model.entity_by_name("Ship 2").unwrap();
            view.add_view_event(ViewEvent::SetCameraFocus(centralia));
            view.add_model_event(ModelEvent::DeleteVessel { entity: ship_1 });
            view.add_model_event(ModelEvent::DeleteVessel { entity: ship_2 });
            view.add_model_event(ModelEvent::SetTimeStep { time_step: TimeStep::Level { level: 1, paused: false } });
            view.add_view_event(ViewEvent::SetConfig(ViewConfig { 
                draw_apsis_icons: true, 
                can_select: true, 
                draw_explorer: false, 
                draw_timeline: false 
            }));
            view.add_model_event(ModelEvent::BuildVessel { 
                vessel_builder: VesselBuilder {
                    name: "Ship",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player)
                        .with_fuel_tank(FuelTankType::Tank1)
                        .with_engine(EngineType::Regular),
                    orbit_builder: OrbitBuilder::Freeform {
                        parent: centralia,
                        distance: 1.0e7,
                        speed: 7.0e3,
                        angle: 0.0,
                        direction: OrbitDirection::AntiClockwise, 
                    },
                }
            });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("I've created a new ship, and enabled apoapsis (")
                    .image("apoapsis")
                    .normal(") and periapsis (")
                    .image("periapsis")
                    .normal(") indicators.\n")
                    .normal("- The apoapsis is the")
                    .bold(" highest ")
                    .normal("point in an orbit\n")
                    .normal("- The periapsis is the")
                    .bold(" lowest ")
                    .normal("point in an orbit\n")
                    .normal("Don't worry about too much about remembering which one is which - you can always check on the fly.")
                    .with_continue()
            ));
            State::new("apsis-explanation", Condition::click_continue())
        });

        story.add("apsis-explanation", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("The periapsis and apoapsis give us a useful way to think about orbits. Most orbits you'll be dealing with won't be circular, so it helps to know what the lowest and highest points are. You'll see what I mean when we start constructing orbits ourselves.")
                    .with_continue()
            ));
            State::new("select-vessel", Condition::click_continue())
        });

        story.add("select-vessel", move |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Anyway, let's observe what happens to the ship as it reaches the periapsis and apoapsis. Click the ship to select it, so we can see its altitude and speed.")
            ));
            State::new("warp-one-orbit", Condition::select_vessel(ship).objective("Select the ship"))
        });

        story.add("warp-one-orbit", move |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            let ship_period = view.model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = view.model.time() + ship_period;
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Try speeding up the simulation again and watch how the altitude and speed change as we reach the periapsis and apoapsis.")
            ));
            State::new("select-apoapsis", Condition::time(time).objective("Warp forwards one orbit"))
        });

        story.add("select-apoapsis", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            view.add_model_event(ModelEvent::SetTimeStep { time_step: TimeStep::Level { level: 1, paused: false } });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("We can check what the altitude and speed is at an apsis by selecting it. Try clicking the ")
                    .image("apoapsis")
                    .normal(" symbol to select the apoapsis.")
            ));
            State::new("select-orbit-point", Condition::select_any_apoapsis(ship).objective("Select the apoapsis"))
        });

        story.add("select-orbit-point", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("In fact, we can find out the altitude and speed at any point on the orbit. Try selecting a point on the orbit by clicking somewhere on it.")
            ));
            State::new("warp-to-point", Condition::select_any_orbit_point(ship).objective("Select a point on the orbit"))
        });

        story.add("warp-to-point", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("You can also warp to the point you selected - it's much more precise to do that rather than fiddling around with manual warps. Try clicking the ")
                    .image("warp-here")
                    .normal(" button to warp to your selected point.")
            ));
            State::new("conclusion", Condition::start_any_warp().objective("Warp to the selected point on the orbit"))
        });

        story.add("conclusion", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Great work. That concludes the first training level. Next, we'll look at how to use the ship's engines to perform adjustments and transfers.")
                    .with_continue()
            ));
            State::new("end", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-01".to_string() });
            view.add_controller_event(ControllerEvent::ExitLevel);
            State::default()
        });

        (model, story, view_config, Some(centralia))
    }
}