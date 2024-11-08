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
use transfer_window_model::model::state_query::StateQuery;
use transfer_window_model::model::Model;
use transfer_window_model::storage::entity_allocator::Entity;
use transfer_window_model::storage::entity_builder::{OrbitableBuilder, VesselBuilder};

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

        let view_config = ViewConfig {
            draw_apsis_icons: true,
            can_select: true,
            draw_explorer: false,
            draw_timeline: false,
        };

        let ship = VesselBuilder {
            name: "Ship",
            vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
            orbit_builder: InitialOrbitBuilder::Freeform { 
                parent: centralia,
                distance: 9.371e6,
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

        story.add("select-point-for-burn", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("First, select a new point where you wish to create the burn.")
            ));
            State::new("create-burn", Condition::select_any_orbit_point(ship).objective("Select a point to create the burn"))
        });

        story.add("create-burn", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now, click the ")
                    .image("create-burn")
                    .normal(" button to create a burn.")
            ));
            State::new("start-burn-adjustment", Condition::create_burn(ship).objective("Create a burn at your selected point"))
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
                    .normal("Now, you can drag the arrows to change the direction and length of the burn. Try adjusting the burn a bit to see how the orbit changes.\n\n")
                    .normal("When you're ready to move on, press continue.")
                    .with_continue()
            ));
            State::new("hohmann-1", Condition::click_continue())
        });

        story.add("hohmann-1", move |view| {
            view.add_model_event(ModelEvent::ForcePause);
            view.add_model_event(ModelEvent::DeleteVessel { entity: ship });
            view.add_model_event(ModelEvent::BuildVessel { 
                vessel_builder: VesselBuilder {
                    name: "Ship",
                    vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Player),
                    orbit_builder: InitialOrbitBuilder::Circular { 
                        parent: centralia,
                        distance: 9.371e6,
                        angle: 0.0,
                        direction: OrbitDirection::AntiClockwise, 
                    },
                }
            });
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Let's now try our first orbital transfer. I've recreated the ship on a circular orbit at about ")
                    .bold("3,000km.")
                    .normal("We're going to create a transfer to get to a circular orbit at about ")
                    .bold("6,000km.")
                    .with_continue()
            ));
            State::new("hohmann-2", Condition::click_continue())
        });

        story.add("hohmann-2", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("We'll need to do this in two stages. Why? Well, if we fire our engines at any given point on the orbit, the resulting orbit will always include the point where we turn off our engines. So when we turn off our engines, we'd have to already be on the circular orbit!")
                    .with_continue()
            ));
            State::new("hohmann-3", Condition::click_continue())
        });

        story.add("hohmann-3", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("In the first stage, we'll have to raise our apoapsis to 6,000km. In the second stage, we'll raise our periapsis to 6,000km. If both our periapsis and apoapsis are at the same height, the orbit will be circular.")
                    .with_continue()
            ));
            State::new("hohmann-4", Condition::click_continue())
        });

        story.add("hohmann-4", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Create a burn that injects us onto an orbit with an apoapsis at roughly 6,000km.")
            ));
            State::new("hohmann-5", Condition::last_orbit_apoapsis(ship, 5.8e6, 6.2e6)
                .objective("Create a burn that injects the ship onto an orbit with an apoapsis between 5,800km and 6,200km"))
        });

        story.add("hohmann-5", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Great work! Now, we'll want to raise our periapsis to roughly 6,000km while keeping our apoapsis roughly the same to circularise our orbit.")
                    .with_continue()
            ));
            State::new("hohmann-6", Condition::click_continue())
        });

        story.add("hohmann-6", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Create a burn near the apoapsis that raises the periapsis to about 6,000km.")
            ));
            State::new("hohmann-7", Condition::last_orbit_circular(ship, 5.5e6, 6.5e6)
                .objective("Create a burn that injects the ship onto an orbit with both periapsis and apoapsis between 5,500km and 6,500km"))
        });

        story.add("hohmann-7", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("And done! The transfer we just architected is called a 'Hohmann transfer.'")
                    .with_continue()
            ));
            State::new("dv-1", Condition::click_continue())
        });

        story.add("dv-1", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Oh, I guess I should also explain Delta-V at this point.")
                    .with_continue()
            ));
            State::new("dv-2", Condition::click_continue())
        });

        story.add("dv-2", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("When we perform a burn, we change the ship's velocity. That change in velocity is called the Delta-V, or ΔV, of the burn, and gives us a way to quantify how 'expensive' a burn is.")
                    .with_continue()
            ));
            State::new("dv-3", Condition::click_continue())
        });

        story.add("dv-3", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("We could try and quantify burns by how much fuel is burnt, but as we burn fuel, the mass of the ship is reduced. That means we need to burn less fuel to achieve the same change in velocity, so the amount of fuel burnt for the same change in velocity depends on your starting fuel.")
                    .with_continue()
            )); 
            State::new("warp-first-burn", Condition::click_continue())
        });

        story.add("warp-first-burn", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            let time = view.model.snapshot_now_observe(Faction::Player).future_burns(ship).first().unwrap().start_point().time() - 10.0;
            view.add_model_event(ModelEvent::ForceUnpause);
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Anyway, enough of that. Let's watch the transfer happen. Go ahead and warp to the first burn. Careful not to overshoot if you're doing it manually - but I recommend using the ")
                    .image("warp-here")
                    .normal(" button when you have the burn selected.")
            ));
            State::new("select-ship", Condition::time(time))
        });

        story.add("select-ship", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            let time = view.model.snapshot_now_observe(Faction::Player)
                .future_burns(ship)
                .first()
                .map_or_else(|| view.model.time(), |burn| burn.end_point().time());
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now, select the ship, and watch the fuel tanks deplete as it executes this first burn.")
            ));
            State::new("warp-to-circle", Condition::time(time))
        });

        story.add("warp-to-circle", |view| {
            let ship = view.model.entity_by_name("Ship").unwrap();
            let time = view.model.snapshot_now_observe(Faction::Player).future_segments(ship).last().unwrap().start_time();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now warp until the ship is on its final orbit.")
            ));
            State::new("conclusion-1", Condition::time(time))
        });

        story.add("conclusion-1", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Fantastic! That concludes this training level. You've done well.")
                    .with_continue()
            ));
            State::new("conclusion-2", Condition::click_continue())
        });

        story.add("conclusion-2", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Next, we'll use what you've learnt so far to intercept an enemy spacecraft.")
                    .with_continue()
            ));
            State::new("end", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-02".to_string() });
            view.add_controller_event(ControllerEvent::ExitLevel);
            State::default()
        });

        (model, story, view_config, Some(centralia))
    }
}
