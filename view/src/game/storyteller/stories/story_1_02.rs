use eframe::egui::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;
use transfer_window_model::components::vessel_component::engine::EngineType;
use transfer_window_model::components::vessel_component::fuel_tank::FuelTankType;
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
                    .normal("Let's fire the ship's engines to adjust an orbit. We often call this adjustment a 'burn' since, well, we're burning fuel to do it.")
                    .with_continue()
            ));
            State::new("select-point-for-burn", Condition::click_continue())
        });

        story.add("select-point-for-burn", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("First, select a new point where you wish to create the burn.")
            ));
            State::new("create-burn", Condition::select_any_orbit_point().objective("Select a point to create the burn"))
        });

        story.add("create-burn", move |view| {
            view.add_model_event(ModelEvent::SetEngine { entity: ship, type_: Some(EngineType::Regular) });
            view.add_model_event(ModelEvent::SetFuelTank { entity: ship, type_: Some(FuelTankType::FuelTank1) });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now, click the ")
                    .image("create-burn")
                    .normal(" button to create a burn.")
            ));
            State::new("start-burn-adjustment", Condition::create_burn_condition(ship).objective("Create a burn at your selected point"))
        });

        story.add("start-burn-adjustment", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("You'll notice the ")
                    .image("burn")
                    .normal(" icon that's now appeared at the selected point. Click that to start adjusting the burn.")
            ));
            State::new("adjust-burn", Condition::start_burn_adjust().objective("Click the burn icon to start adjusting the burn"))
        });

        story.add("adjust-burn", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now, you can drag the arrows to change the direction and length of the burn. Try adjusting the burn and see how the orbit will change when the burn's executed. When you're starting to get the hang of it, press continue and we'll move on.")
                    .with_continue()
            ));
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-02".to_string() });
            State::new("hohmann-explanation", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_view_event(ViewEvent::CloseDialogue);
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