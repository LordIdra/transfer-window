use eframe::{egui::{Align2, Grid, RichText, Window}, epaint};
use transfer_window_model::components::orbitable_component::OrbitableComponentPhysics;

use crate::game::{selected::Selected, View};

pub fn update(view: &mut View) {
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
            ui.label(RichText::new(name).size(20.0).strong().monospace());
                
            Grid::new("Orbitable info grid").show(ui, |ui| {
                if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                    ui.label(RichText::new("Orbiting").strong().monospace());
                    ui.label(view.model.name_component(orbit.parent()).name());
                    ui.end_row();
                }

                ui.label(RichText::new("Mass").strong().monospace());
                ui.label(format!("{:.3e} kg", orbitable_component.mass()));
                ui.end_row();

                ui.label(RichText::new("Radius").strong().monospace());
                ui.label(format!("{:.3e} m", orbitable_component.radius()));
                ui.end_row();
            });
        });
}
