use std::f64::consts::PI;

use eframe::egui::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::api::time::TimeStep;
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;
use transfer_window_model::{api::builder::{OrbitBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, VesselBuilder}, components::{orbitable_component::OrbitableType, path_component::orbit::orbit_direction::OrbitDirection, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity, Model};

use crate::controller_events::ControllerEvent;
use crate::game::events::{ModelEvent, ViewEvent};
use crate::game::overlay::dialogue::Dialogue;
use crate::game::storyteller::story::condition::Condition;
use crate::game::storyteller::story::state::State;
use crate::game::storyteller::story::Story;
use crate::game::ViewConfig;

use super::StoryBuilder;

#[derive(Debug, Clone, Default)]
pub struct Story1_02;

impl StoryBuilder for Story1_02 {
    fn prerequisite(&self) -> Option<String> {
        Some("1-01".to_string())
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

        let ship = VesselBuilder {
            name: "Ship",
            vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
            orbit_builder: OrbitBuilder::Freeform { 
                parent: centralia,
                distance: 1.0e7,
                speed: 7.0e3,
                angle: 0.0,
                direction: OrbitDirection::AntiClockwise, 
            },
        }.build(&mut model);

        let mut story = Story::new("intro");

        story.add("intro", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Welcome back. Let's discuss some more about orbits.")
                    .with_continue()
            ));
            State::new("apsis-icons", Condition::click_continue())
        });

        story.add("apsis-icons", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("I've enabled apoapsis (")
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
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Anyway, let's observe what happens to the ship as it reaches the periapsis and apoapsis. Click the ship to select it, so we can see its altitude and speed.")
            ));
            State::new("warp-one-orbit", Condition::select_vessel(ship).objective("Select the ship"))
        });

        story.add("warp-one-orbit", move |view| {
            let ship_period = view.model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = view.model.time() + ship_period;
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Try speeding up the simulation again and watch how the altitude and speed change as we reach the periapsis and apoapsis.")
            ));
            State::new("select-apoapsis", Condition::time(time).objective("Warp forwards one orbit"))
        });

        story.add("select-apoapsis", |view| {
            view.add_model_event(ModelEvent::SetTimeStep { time_step: TimeStep::Level { level: 1, paused: false } });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("We can check what the altitude and speed is at an apsis by selecting it. Try clicking the ")
                    .image("apoapsis")
                    .normal(" symbol to select the apoapsis.")
            ));
            State::new("select-orbit-point", Condition::select_any_apoapsis().objective("Select the apoapsis"))
        });

        story.add("select-orbit-point", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("In fact, we can find out the altitude and speed at any point on the orbit. Try selecting a point on the orbit by clicking somewhere on it.")
            ));
            State::new("warp-to-point", Condition::select_any_orbit_point().objective("Select a point on the orbit"))
        });

        story.add("warp-to-point", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("You can also warp to the point you selected - it's much more precise to do that rather than fiddling around with manual warps. Try clicking the ")
                    .image("warp-here")
                    .normal(" button to warp to your selected point.")
            ));
            State::new("burn-explanation", Condition::start_any_warp().objective("Warp to the selected point on the orbit"))
        });

        story.add("burn-explanation", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Great. Now, let's try firing the ship's engines to adjust our orbit. We call this a 'burn' since we're burning fuel.")
                    .with_continue()
            ));
            State::new("select-point-for-burn", Condition::click_continue())
        });

        story.add("select-point-for-burn", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("First, select a new point where you wish to create the burn.")
                    .with_continue()
            ));
            State::new("create-burn", Condition::select_any_orbit_point().objective("Select a point to create the burn"))
        });

        story.add("create-burn-circularise", move |view| {
            view.add_model_event(ModelEvent::BuildVessel { 
                vessel_builder: VesselBuilder {
                    name: "Demo Ship",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                    orbit_builder: OrbitBuilder::Circular {
                        parent: centralia,
                        distance: 1.576e7,
                        angle: PI,
                        direction: OrbitDirection::AntiClockwise,
                    },
                }
            });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Great. Now, let's say we want to take this elliptical orbit and make it circular at the apoapsis. I've created a temporary vessel on the orbit we're aiming for to show you what I mean. How do we get to that orbit?")
                    .with_continue()
            ));
            State::new("create-burn-engines", Condition::click_continue())
        });

        story.add("create-burn-engines", |view| {
            let demo_ship = view.model.entity_by_name("Demo Ship").unwrap();
            view.add_model_event(ModelEvent::DeleteVessel { entity: demo_ship });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Well, we'll need to fire the ship's engine at the apoapsis.")
                    .with_continue()
            ));
            State::new("end", Condition::click_continue())
        });

        story.add("create-burn", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("We'll need to fire the ship's engines somewhere. But where?")
                    .with_continue()
            ));
            State::new("end", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-02".to_string() });
            State::default()
        });

        let view_config = ViewConfig {
            draw_apsis_icons: true,
            can_select: true,
            draw_explorer: false,
            draw_timeline: false,
        };

        (model, story, view_config, Some(centralia))
    }
}