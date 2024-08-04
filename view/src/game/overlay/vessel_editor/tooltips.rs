use eframe::egui::{Grid, RichText, Ui};
use transfer_window_model::components::vessel_component::{engine::EngineType, fuel_tank::FuelTankType, torpedo_launcher::TorpedoLauncherType, torpedo_storage::TorpedoStorageType};

use crate::game::View;

pub type TooltipFn = Box<dyn Fn(&View, &mut Ui)>;

pub fn show_tooltip_engine(type_: Option<EngineType>) -> TooltipFn {
    Box::new(move |view: &View, ui: &mut Ui| {
        let Some(type_) = type_ else {
            ui.label(RichText::new("None").strong().monospace().size(20.0));
            return;
        };

        let name = match type_ {
            EngineType::Regular => "Regular Engine",
            EngineType::Efficient => "Efficient Engine",
            EngineType::Booster => "Booster Engine",
            EngineType::Torpedo => unreachable!(),
        };


        ui.label(RichText::new(name).strong().monospace().size(20.0));

        Grid::new("Tooltip for ".to_string() + name).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.image(view.resources.texture_image("fuel"));
                ui.label(RichText::new("Fuel Consumption").monospace().strong());
            });
            ui.label(format!("{} L/s", type_.fuel_kg_per_second()));
            ui.end_row();

            ui.horizontal(|ui| {
                ui.image(view.resources.texture_image("thrust"));
                ui.label(RichText::new("Thrust").monospace().strong());
            });
            ui.label(format!("{} N", type_.thrust_newtons()));
            ui.end_row();

            ui.horizontal(|ui| {
                ui.image(view.resources.texture_image("isp"));
                ui.label(RichText::new("Specific Impulse").monospace().strong());
            });
            ui.label(format!("{} s", type_.specific_impulse().round()));
            ui.end_row();
        });
    })
}

pub fn show_tooltip_fuel_tank(type_: Option<FuelTankType>) -> TooltipFn {
    Box::new(move |view: &View, ui: &mut Ui| {
        let Some(type_) = type_ else {
            ui.label(RichText::new("None").strong().monospace().size(20.0));
            return;
        };

        let name = match type_ {
            FuelTankType::Tank1 => "Fuel Tank I",
            FuelTankType::Tank2 => "Fuel Tank II",
            FuelTankType::Tank3 => "Fuel Tank III",
            FuelTankType::Tank4 => "Fuel Tank IV",
            FuelTankType::Torpedo | FuelTankType::Hub => unreachable!(),
        };

        ui.label(RichText::new(name).strong().monospace().size(20.0));
        ui.horizontal(|ui| {
            ui.image(view.resources.texture_image("fuel"));
            ui.label(RichText::new("Capacity").monospace().strong());
            ui.label(format!("{} L", type_.capacity_litres()));
        });
    })
}

pub fn show_tooltip_torpedo_storage(type_: Option<TorpedoStorageType>) -> TooltipFn {
    Box::new(move |view: &View, ui: &mut Ui| {
        let Some(type_) = type_ else {
            ui.label(RichText::new("None").strong().monospace().size(20.0));
            return;
        };

        let name = match type_ {
            TorpedoStorageType::TorpedoStorage1 => "Tiny Torpedo Storage",
            TorpedoStorageType::TorpedoStorage2 => "Small Torpedo Storage",
            TorpedoStorageType::Hub => unreachable!(),
        };

        ui.label(RichText::new(name).strong().monospace().size(20.0));
        ui.horizontal(|ui| {
            ui.image(view.resources.texture_image("torpedo"));
            ui.label(RichText::new("Capacity").monospace().strong());
            ui.label(format!("{}", type_.capacity()));
        });

        ui.label(RichText::new(name).strong().monospace().size(20.0));
    })
}

pub fn show_tooltip_torpedo_launcher(type_: Option<TorpedoLauncherType>) -> TooltipFn {
    Box::new(move |_view: &View, ui: &mut Ui| {
        let Some(type_) = type_ else {
            ui.label(RichText::new("None").strong().monospace().size(20.0));
            return;
        };

        let name = match type_ {
            TorpedoLauncherType::TorpedoLauncher1 => "Torpedo Launcher I",
            TorpedoLauncherType::TorpedoLauncher2 => "Torpedo Launcher II",
        };

        ui.label(RichText::new(name).strong().monospace().size(20.0));
    })
}