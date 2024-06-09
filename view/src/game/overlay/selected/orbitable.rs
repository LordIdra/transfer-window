use eframe::{egui::{Align2, Grid, Window}, epaint};
use transfer_window_model::components::orbitable_component::OrbitableComponentPhysics;

use crate::game::{overlay::widgets::labels::{draw_key, draw_subtitle, draw_title, draw_value}, selected::Selected, util::format_distance, View};

use super::point::draw_orbit_labels;

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update orbitable");
    let Selected::Orbitable(entity) = view.selected.clone() else { 
        return
    };

    let orbitable_component = view.model.orbitable_component(entity);
    let name = view.model.name_component(entity).name().to_uppercase();

    Window::new("Selected orbitable ".to_string() + name.as_str())
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
            .show(&view.context.clone(), |ui| {
        draw_title(ui, &name);
        
        draw_subtitle(ui, "Info");
        Grid::new("Orbitable info grid").show(ui, |ui| {
            draw_key(ui, "Mass");
            draw_value(ui, &format!("{:.3e} kg", orbitable_component.mass()));
            ui.end_row();

            draw_key(ui, "Radius");
            draw_value(ui, &format_distance(orbitable_component.radius()));
            ui.end_row();
        });

        draw_subtitle(ui, "Orbit");
        Grid::new("Orbitable orbit info grid").show(ui, |ui| {
            if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                draw_orbit_labels(view, ui, orbit);
                draw_key(ui, "Sphere of influence");
                draw_value(ui, &format_distance(orbit.sphere_of_influence()));
                ui.end_row();
            }
        });
    });
}
