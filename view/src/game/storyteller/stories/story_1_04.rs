use eframe::egui::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;
use transfer_window_model::components::orbitable_component::builder::OrbitablePhysicsBuilder;
use transfer_window_model::components::orbitable_component::OrbitableType;
use transfer_window_model::components::path_component::orbit::builder::InitialOrbitBuilder;
use transfer_window_model::components::path_component::orbit::orbit_direction::OrbitDirection;
use transfer_window_model::components::vessel_component::class::VesselClass;
use transfer_window_model::components::vessel_component::faction::Faction;
use transfer_window_model::components::vessel_component::VesselComponent;
use transfer_window_model::model::Model;
use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::storage::entity_builder::{OrbitableBuilder, VesselBuilder};

use crate::controller_events::ControllerEvent;
use crate::game::events::ViewEvent;
use crate::game::overlay::dialogue::Dialogue;
use crate::game::storyteller::story::condition::Condition;
use crate::game::storyteller::story::state::State;
use crate::game::storyteller::story::Story;
use crate::game::ViewConfig;

use super::StoryBuilder;

#[derive(Debug, Clone, Default)]
pub struct Story1_04;

impl StoryBuilder for Story1_04 {
    fn prerequisite(&self) -> Option<String> {
        Some("1-01".to_string())
    }

    fn build(&self) -> (Model, Story, ViewConfig, Option<Entity>) {
        let mut model = Model::default();

        let centralia = OrbitableBuilder {
            name: "Centralia",
            mass: 5.972e24,
            radius: 6.371e6,
            rotation_period: 24.0 * 60.0 * 60.0,
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

        let _helios = OrbitableBuilder {
            name: "Helios",
            mass: 0.07346e24,
            radius: 1737.4e3,
            rotation_period: 29.5 * 24.0 * 60.0 * 60.0,
            rotation_angle: 100.0,
            type_: OrbitableType::Moon,
            physics: OrbitablePhysicsBuilder::Orbit(InitialOrbitBuilder::Freeform { 
                parent: centralia, 
                distance: 0.3633e9,
                angle: 0.0, 
                direction: OrbitDirection::AntiClockwise,
                speed: 1.082e3,
            }),
            atmosphere: None,
        }.build(&mut model);

        let _player_ship = VesselBuilder {
            name: "Ship",
            vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
            orbit_builder: InitialOrbitBuilder::Circular {
                parent: centralia,
                distance: 9.371e6,
                angle: 0.0,
                direction: OrbitDirection::AntiClockwise, 
            },
        }.build(&mut model);

        let mut story = Story::new("intro");

        story.add("intro", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("I've enabled the explorer in the top left of your HUD, which shows all the objects in the Interface.")
                    .with_continue()
            ));
            State::new("helios", Condition::click_continue())
        });

        story.add("helios", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("As you can see, I've added a moon - Helios to Centralia. This time, we're going to take our ship from Centralia orbit to Helios orbit.")
                    .with_continue()
            ));
            State::new("soi-1", Condition::click_continue())
        });

        story.add("soi-1", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now, let me introduce a concept called a 'sphere of influence.' This is the approximate region around an object where its gravitational pull is greater than the gravitational pull of its parent.")
                    .with_continue()
            ));
            State::new("soi-2", Condition::click_continue())
        });

        story.add("soi-2", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("If that's confusing, imagine moving the ship on a line from Centralia to Helios. When the attraction from Helios is stronger, we're inside the sphere of influence.")
                    .with_continue()
            ));
            State::new("soi-3", Condition::click_continue())
        });

        story.add("soi-3", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Things aren't that simple really, the sphere of influence is actually a constant distance, and just taken to be the average distance over the orbit, and of course the Interface just models it like this to abstract away from the actual complex N-body interactions, and... ah I'm rambling a bit, don't worry about that for now.")
                    .with_continue()
            ));
            State::new("encounter-explanation", Condition::click_continue())
        });

        story.add("encounter-explanation", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Anyway, the first thing we need to do is get our ship within Helios's sphere of influence.")
                    .with_continue()
            ));
            State::new("any-moon-encounter", Condition::click_continue())
        });

        story.add("any-moon-encounter", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Try creating a burn that gets you close to Helios. You'll know that you're on a trajectory to enter Helios' sphere of influence when you see the ")
                    .image("encounter-entrance")
                    .normal(" icon. It'll be similar to last time - remember you can set a ship's target to Helios to see the closest approaches")
            ));
            State::new("moon-encounter-next-orbit", Condition::click_continue())
        });

        story.add("moon-encounter-next-orbit", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Good job! However, we're aiming to get to Helios on the next orbit. The burn you've created will take multiple orbits until Helios is in the correct position. Try")
            ));
            State::new("encounter-explanation", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-03".to_string() });
            view.add_controller_event(ControllerEvent::ExitLevel);
            State::default()
        });

        let view_config = ViewConfig {
            draw_apsis_icons: true,
            can_select: true,
            draw_explorer: true,
            draw_timeline: true,
        };

        (model, story, view_config, Some(centralia))
    }
}
