use nalgebra_glm::vec2;
use transfer_window_model::{api::builder::{OrbitBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, VesselBuilder}, components::{orbitable_component::OrbitableType, path_component::orbit::orbit_direction::OrbitDirection, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity, Model};

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

        let mut story = Story::new("intro-1");

        story.add("intro-1", |_| State::default()
            .transition(Transition::new("intro-2", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("Welcome back. Let's discuss some more about orbits.")
                    .with_continue()
                )
            )
        );

        story.add("intro-2", |_| State::default()
            .transition(Transition::new("intro-3", Condition::click_continue()))
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

        story.add("intro-3", |_| State::default()
            .transition(Transition::new("intro-4", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("The periapsis and apoapsis give us a useful way to think about orbits. Most orbits you'll be dealing with won't be circular, so it helps to know what the lowest and highest points are. You'll see what I mean in the next level, where we'll start constructing orbits ourselves.")
                    .with_continue()
                )
            )
        );

        story.add("intro-4", |_| State::default()
            .transition(Transition::new("intro-5", Condition::click_continue()))
            .action(ShowDialogueAction::new(
                Dialogue::new("jake")
                    .normal("You can select an apsis to see more information about it. Try selecting the")
                    .bold(" periapsis ")
                    .normal(".")
                    .with_continue()
                )
            )
        );

        story.add("end", |_model: &Model| State::default()
            .action(FinishLevelAction::new("1-02".to_string())));

        let view_config = ViewConfig {
            draw_apsis_icons: true,
            can_select: true,
            draw_explorer: false,
            draw_timeline: false
        };

        (model, story, view_config, Some(centralia))
    }
}