use eframe::{egui::{Align2, Context, Grid, RichText, Window}, epaint};
use transfer_window_model::{components::orbitable_component::OrbitableComponentPhysics, Model};

use crate::game::{selected::Selected, Scene};

pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    let Selected::Orbitable(entity) = view.selected.clone() else { 
        return
    };

    let orbitable_component = model.orbitable_component(entity);
    let name = model.name_component(entity).name().to_uppercase();

    Window::new("Selected orbitable ".to_string() + name.as_str())
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(RichText::new(name).size(20.0).strong().monospace());
                
            Grid::new("Orbitable info grid").show(ui, |ui| {
                if let OrbitableComponentPhysics::Orbit(orbit) = orbitable_component.physics() {
                    ui.label(RichText::new("Orbiting").strong().monospace());
                    ui.label(model.name_component(orbit.parent()).name());
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
