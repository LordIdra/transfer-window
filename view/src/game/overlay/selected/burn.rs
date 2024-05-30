use eframe::{egui::{Align2, Button, Color32, Context, Grid, Ui, Window}, epaint};
use transfer_window_model::{components::path_component::{burn::Burn, segment::Segment}, Model};

use crate::{events::Event, game::{overlay::widgets::{draw_filled_bar, FilledBar}, underlay::selected::Selected, util::format_time, Scene}};

fn draw_dv(ui: &mut Ui, burn: &Burn) {
    let max_dv = burn.rocket_equation_function().start().remaining_dv();
    let start_dv = burn.rocket_equation_function().remaining_dv();
    let end_dv = burn.final_rocket_equation_function().remaining_dv();
    let burnt_dv = start_dv - end_dv;

    let start_dv_proportion = (start_dv / max_dv) as f32;
    let end_dv_proportion = (end_dv / max_dv) as f32;

    let start_bar = FilledBar::new(Color32::RED, start_dv_proportion);
    let end_bar = FilledBar::new(Color32::WHITE, end_dv_proportion);

    ui.horizontal(|ui| {
        draw_filled_bar(ui, 120.0, 10.0, 2.0, 3.0, Color32::DARK_GRAY, vec![start_bar, end_bar]);
    });

    Grid::new("DV grid").show(ui, |ui| {
        ui.label(format!("ΔV Burnt"));
        ui.label(format!("{:.1}", burnt_dv));
        ui.end_row();

        ui.label(format!("ΔV Start"));
        ui.label(format!("{:.1}", start_dv));
        ui.end_row();

        ui.label(format!("ΔV End"));
        ui.label(format!("{:.1}", end_dv));
        ui.end_row();
    });
}

pub fn update(view: &mut Scene, model: &Model, context: &Context, events: &mut Vec<Event>) {
    let Selected::Burn { entity, time, state: _ } = view.selected.clone() else { 
        return
    };

    let Some(Segment::Burn(burn)) = model.path_component(entity).future_segment_starting_at_time(time) else {
        return;
    };
    
    Window::new("Selected burn")
    .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0))
        .show(context, |ui| {
            ui.label(model.name_component(entity).name());
            ui.label("T-".to_string() + format_time(time - model.time()).as_str());
            ui.label(format!("Duration {}", format_time(burn.duration())));

            draw_dv(ui, burn);

            if ui.button("Warp to burn").clicked() {
                events.push(Event::StartWarp { end_time: time });
            }

            let can_delete = model.timeline_event_at_time(entity, time).can_delete(model);
            let delete_button = Button::new("Cancel");
            if ui.add_enabled(can_delete, delete_button).clicked() {
                events.push(Event::CancelLastTimelineEvent { entity });
                view.selected = Selected::None;
            }
        });
}
