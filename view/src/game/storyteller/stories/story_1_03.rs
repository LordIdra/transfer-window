use eframe::egui::Color32;
use nalgebra_glm::vec2;
use transfer_window_model::components::orbitable_component::atmosphere::Atmosphere;
use transfer_window_model::{api::builder::{OrbitBuilder, OrbitableBuilder, OrbitablePhysicsBuilder, VesselBuilder}, components::{orbitable_component::OrbitableType, path_component::orbit::orbit_direction::OrbitDirection, vessel_component::{class::VesselClass, faction::Faction, VesselComponent}}, storage::entity_allocator::Entity, Model};

use crate::controller_events::ControllerEvent;
use crate::game::events::ViewEvent;
use crate::game::overlay::dialogue::Dialogue;
use crate::game::storyteller::story::condition::Condition;
use crate::game::storyteller::story::state::State;
use crate::game::storyteller::story::Story;
use crate::game::ViewConfig;

use super::StoryBuilder;

#[derive(Debug, Clone, Default)]
pub struct Story1_03;

impl StoryBuilder for Story1_03 {
    fn prerequisite(&self) -> Option<String> {
        Some("1-01".to_string())
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

        let player_ship = VesselBuilder {
            name: "Ship",
            vessel_component: VesselComponent::new(VesselClass::Frigate1, Faction::Player),
            orbit_builder: OrbitBuilder::Circular {
                parent: centralia,
                distance: 9.371e6,
                angle: 0.0,
                direction: OrbitDirection::AntiClockwise, 
            },
        }.build(&mut model);

        let enemy_ship = VesselBuilder {
            name: "Ship",
            vessel_component: VesselComponent::new(VesselClass::Scout1, Faction::Enemy),
            orbit_builder: OrbitBuilder::Circular {
                parent: centralia,
                distance: 10.371e6,
                angle: 0.2,
                direction: OrbitDirection::AntiClockwise, 
            },
        }.build(&mut model);

        let mut story = Story::new("intro");

        story.add("intro", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("We're currently in an orbit at around 3,000km. A dummy enemy ship is in orbit at about 4,000km. We want to 'negotiate' with the enemy ship, and we think that a torpedo might help us make our point a bit more saliently.")
                    .with_continue()
            ));
            State::new("timeline-reason", Condition::click_continue())
        });

        story.add("timeline-reason", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("This mission will be a bit more complex than the last few, so I've enabled timelines on your Interface. Select your ship to see its timeline.")
            ));
            State::new("timeline-explanation", Condition::select_vessel(player_ship).objective("Select your ship"))
        });

        story.add("timeline-explanation", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("For now we've only got the periapsis and apoapsis, but as you plan the mission, more events will appear - things like burns, torpedo launches, intercepts, and so on.")
                    .with_continue()
            ));
            State::new("timeline-utility", Condition::click_continue())
        });

        story.add("timeline-utility", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Oh, you can also see things like the altitude at the apoapsis and periapsis, which helps a lot when planning burn transfers and such. And you can click on items on the timeline to select them.")
                    .with_continue()
            ));
            
            State::new("torpedo-transition", Condition::click_continue())
        });

        story.add("torpedo-transition", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("That's enough about the timeline. Let's talk about torpedoes.")
                    .with_continue()
            ));
            State::new("torpedo-equipment", Condition::click_continue())
        });

        story.add("torpedo-equipment", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Your ship has a torpedo storage and a torpedo launcher. Torpedoes are heavy and expensive, so we've only got one.")
                    .with_continue()
            ));
            State::new("intercept-explanation", Condition::click_continue())
        });

        story.add("intercept-explanation", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Intercepting a ship comes in four stages.\n")
                    .normal("1) Select a target\n")
                    .normal("2) Launch the torpedo\n")
                    .normal("3) Fire the torpedo's engines to get onto a trajectory that will bring us within a few km of the target\n")
                    .normal("4) Enable terminal guidance to guide the torpedo to the target")
                    .with_continue()
            ));
            State::new("select-target", Condition::click_continue())
        });

        story.add("select-target", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Keeping youe ship selected, right click the enemy ship and click the ")
                    .image("set-target")
                    .normal(" button to set your ship's target to the enemy ship.")
            ));
            State::new("select-point", Condition::set_target(player_ship, enemy_ship).objective("Target the enemy ship"))
        });

        story.add("select-point", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("You'll notice two new items on your timeline; closest approaches. We'll get to that later. Now select a point to launch the torpedo, preferably sooner rather than later.")
            ));
            State::new("launch-torpedo", Condition::select_any_orbit_point(player_ship).objective("Select a point to launch the torpedo"))
        });

        story.add("launch-torpedo", move |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now click the ")
                    .image("fire-torpedo")
                    .normal(" button to launch a torpedo at that point.")
            ));
            State::new("start-adjust-torpedo", Condition::fire_torpedo(player_ship).objective("Schedule a torpedo launch"))
        });

        story.add("start-adjust-torpedo", move |view| {
            let ghost = view.model.vessel_component(player_ship)
                .timeline()
                .last_event()
                .unwrap()
                .as_fire_torpedo()
                .unwrap()
                .ghost();
            view.add_view_event(ViewEvent::SetPersistentData("torpedo", ghost));
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("When a torpedo is launched, it'll immediately start a burn. You can adjust this initial burn by clicking the ")
                    .image("fire-torpedo")
                    .normal(" icon and then dragging the arrows, as with normal burns. Try it now.")
            ));
            State::new("adjust-torpedo", Condition::fire_torpedo_adjust().objective("Start adjusting the torpedo's initial burn"))
        });

        story.add("adjust-torpedo", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now notice the ")
                    .image("closest-approach-1")
                    .normal(" and ")
                    .image("closest-approach-2")
                    .normal(" icons. These icons indicate the next two 'approaches', or points where you'll be closest to your target.")
                    .with_continue()
            ));
            State::new("initial-intercept", Condition::click_continue())
        });

        story.add("initial-intercept", |view| {
            let torpedo = view.story.get_persistent_data("torpedo");
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now, adjust the torpedo's initial burn to get a first approach (")
                    .image("closest-approach-1")
                    .normal(") within 20km or so of your target. You might find it helpful to fire the torpedo earlier or later than you're firing it currently. Don't worry if you find this difficult - it'll take some fiddling if you're not used to dealing with orbital mechanics.")
            ));
            State::new("activate-guidance", Condition::first_closest_approach(torpedo, 20.0e3).objective("Get a first approach within 20km of the target."))
        });

        story.add("activate-guidance", |view| {
            let torpedo = view.story.get_persistent_data("torpedo");
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Now let's activate terminal guidance. Select a point somewhere close to the approach (but not too close), then use the ")
                    .image("enable-guidance")
                    .normal(" button to enable guidance at that point. You should see a ")
                    .image("intercept")
                    .normal(" icon appear, indicating a predicted intercept. If you don't, try creating the guidance earlier or later.")
            ));
            State::new("warp-to-intercept", Condition::get_intercept(torpedo).objective("Activate guidance and acquire an intercept"))
        });

        story.add("warp-to-intercept", |view| {
            let torpedo = view.story.get_persistent_data("torpedo");
            let time = view.model.vessel_component(torpedo)
                .timeline()
                .last_event()
                .unwrap()
                .time();
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Great, we have an intercept! Go ahead and watch your mission plan play out. You might want to cover your eyes for the intercept...")
            ));
            State::new("conclusion", Condition::time(time).objective("Watch the torpedo intercept the target"))
        });

        story.add("conclusion", |view| {
            view.add_view_event(ViewEvent::ShowDialogue(
                Dialogue::new("jake")
                    .normal("Good work! You can now intercept enemy ships. Next mission, we'll attempt to fly to Centralia's moon.")
                    .with_continue()
            ));
            State::new("end", Condition::click_continue())
        });

        story.add("end", |view| {
            view.add_controller_event(ControllerEvent::FinishLevel { level: "1-03".to_string() });
            view.add_controller_event(ControllerEvent::ExitLevel);
            State::default()
        });

        let view_config = ViewConfig {
            draw_apsis_icons: true,
            can_select: true,
            draw_explorer: false,
            draw_timeline: true,
        };

        (model, story, view_config, Some(centralia))
    }
}
