use std::f64::consts::PI;

use eframe::egui::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::api::time::TimeStep;
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;
use transfer_window_model::{api::builder::{OrbitBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, VesselBuilder}, components::{orbitable_component::OrbitableType, path_component::orbit::orbit_direction::OrbitDirection, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity, Model};

use crate::game::storyteller::story::action::create_vessel_action::CreateVesselAction;
use crate::game::storyteller::story::action::delete_vessel_action::DeleteVesselAction;
use crate::game::storyteller::story::action::set_time_step_action::SetTimeStepAction;
use crate::game::{overlay::dialogue::Dialogue, storyteller::story::{action::{finish_level_action::FinishLevelAction, show_dialogue_action::ShowDialogueAction}, condition::Condition, state::State, transition::Transition, Story}, ViewConfig};

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
                vec![]
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

        story.add("intro", |_| State::default()
            .transition(Transition::new("apsis-icons", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Welcome back. Let's discuss some more about orbits.")
                    .with_continue()
                )
            )
        );

        story.add("apsis-icons", |_| State::default()
            .transition(Transition::new("apsis-explanation", Condition::click_continue()))
            .action(ShowDialogueAction::new(
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
                )
            )
        );

        story.add("apsis-explanation", |_| State::default()
            .transition(Transition::new("select-vessel", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("The periapsis and apoapsis give us a useful way to think about orbits. Most orbits you'll be dealing with won't be circular, so it helps to know what the lowest and highest points are. You'll see what I mean when we start constructing orbits ourselves.")
                    .with_continue()
                )
            )
        );

        story.add("select-vessel", move |_| State::default()
            .transition(Transition::new("warp-one-orbit", Condition::select_vessel(ship).objective("Select the ship")))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Anyway, let's observe what happens to the ship as it reaches the periapsis and apoapsis. Click the ship to select it, so we can see its altitude and speed.")
                )
            )
        );

        story.add("warp-one-orbit", move |model| {
            let ship_period = model.current_segment(ship).as_orbit().unwrap().period().unwrap();
            let time = model.time() + ship_period;
            State::default()
                .transition(Transition::new("select-apoapsis", Condition::time(time).objective("Warp forwards one orbit")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Try speeding up the simulation again and watch how the altitude and speed change as we reach the periapsis and apoapsis.")
                    )
                )
            }
        );

        story.add("select-apoapsis", |_| {
            State::default()
                .transition(Transition::new("select-orbit-point", Condition::select_any_apoapsis().objective("Select the apoapsis")))
                .action(SetTimeStepAction::new(TimeStep::Level { level: 1, paused: false }))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("We can check what the altitude and speed is at an apsis by selecting it. Try clicking the ")
                        .image("apoapsis")
                        .normal(" symbol to select the apoapsis.")
                    )
                )
            }
        );

        story.add("select-orbit-point", |_| {
            State::default()
                .transition(Transition::new("warp-to-point", Condition::select_any_orbit_point().objective("Select a point on the orbit")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("In fact, we can find out the altitude and speed at any point on the orbit. Try selecting a point on the orbit by clicking somewhere on it.")
                    )
                )
            }
        );

        story.add("warp-to-point", |_| {
            State::default()
                .transition(Transition::new("create-burn-circularise", Condition::start_any_warp().objective("Warp to the selected point on the orbit")))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("You can also warp to the point you selected - it's much more precise to do that rather than fiddling around with manual warps. Try clicking the ")
                        .image("warp-here")
                        .normal(" button to warp to your selected point.")
                    )
                )
            }
        );

        story.add("create-burn-circularise", move |_| {
            State::default()
                .transition(Transition::new("create-burn-engines", Condition::click_continue()))
                .action(CreateVesselAction::new(VesselBuilder {
                    name: "Demo Ship",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                    orbit_builder: OrbitBuilder::Circular {
                        parent: centralia,
                        distance: 1.576e7,
                        angle: PI,
                        direction: OrbitDirection::AntiClockwise,
                    },
                }))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Great. Now, let's say we want to take this elliptical orbit and make it circular at the apoapsis. I've created a temporary vessel on the orbit we're aiming for to show you what I mean. How do we get to that orbit?")
                        .with_continue()
                    )
                )
            }
        );

        story.add("create-burn-engines", |model| {
            let demo_ship = model.entity_by_name("Demo Ship").unwrap();
            State::default()
                .transition(Transition::new("end", Condition::click_continue()))
                .action(DeleteVesselAction::new(demo_ship))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("Well, we'll need to fire the ship's engine at the apoapsis.")
                        .with_continue()
                    )
                )
            }
        );

        story.add("create-burn", |_| {
            State::default()
                .transition(Transition::new("end", Condition::click_continue()))
                .action(ShowDialogueAction::new(
                    Dialogue::new("jake")
                        .normal("We'll need to fire the ship's engines somewhere. But where?")
                        .with_continue()
                    )
                )
            }
        );

        story.add("end", |_model: &Model| State::default()
            .action(FinishLevelAction::new("1-02".to_string())));

        let view_config = ViewConfig {
            draw_apsis_icons: true,
            can_select: true,
            draw_explorer: false,
            draw_timeline: false,
        };

        (model, story, view_config, Some(centralia))
    }
}