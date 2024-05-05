use eframe::{egui::{style::{Spacing, WidgetVisuals}, Color32, Context, Margin, Rounding, Stroke, Style, Ui, Visuals}, epaint::Shadow};

#[derive(Default)]
pub struct Styles {
    pub default_window_visuals: DefaultWindowVisuals,
    pub vessel_editor_visuals: VesselEditorVisuals,
    pub slot_visuals: SlotEditorStyle,
    pub slot_editor_widgets: SlotSelectorStyle,
}

#[derive(Default)]
pub struct DefaultWindowVisuals {}

impl DefaultWindowVisuals {
    pub fn apply(&self, context: &Context) {
        context.set_visuals(Visuals {
            window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 100),
            window_stroke: Stroke::NONE,
            window_shadow: Shadow::NONE,
            window_rounding: Rounding::ZERO,
            ..Visuals::default()
        });
    }
}

#[derive(Default)]
pub struct VesselEditorVisuals {}

impl VesselEditorVisuals {
    pub fn apply(&self, context: &Context) {
        context.set_visuals(Visuals {
            window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 200),
            window_stroke: Stroke::NONE,
            window_shadow: Shadow::NONE,
            window_rounding: Rounding::ZERO,
            ..Visuals::default()
        });

        context.set_style(Style {
            spacing: Spacing {
                window_margin: Margin::same(20.0),
                ..Spacing::default()
            },
            ..Style::default()
        });
    }
}

#[derive(Default)]
pub struct SlotEditorStyle {}

impl SlotEditorStyle {
    pub fn apply(&self, ui: &mut Ui) {
        let default_color = Color32::from_rgba_unmultiplied(40, 40, 40, 220);
        let hovered_color = Color32::from_rgba_unmultiplied(60, 60, 60, 220);
        let selected_color = Color32::from_rgba_unmultiplied(80, 80, 80, 220);

        let bg_stroke = Stroke::NONE;
        let rounding = Rounding::ZERO;
        let fg_stroke = Stroke::NONE;
        let expansion = 0.0;

        ui.visuals_mut().widgets.inactive = WidgetVisuals {
            bg_fill: default_color,
            weak_bg_fill: default_color,
            bg_stroke, rounding, fg_stroke, expansion,
        };

        ui.visuals_mut().widgets.hovered = WidgetVisuals {
            bg_fill: hovered_color,
            weak_bg_fill: hovered_color,
            bg_stroke, rounding, fg_stroke, expansion,
        };

        ui.visuals_mut().widgets.active = WidgetVisuals {
            bg_fill: selected_color,
            weak_bg_fill: selected_color,
            bg_stroke, rounding, fg_stroke, expansion,
        };
    }
}

#[derive(Default)]
pub struct SlotSelectorStyle {}

impl SlotSelectorStyle {
    pub fn apply(&self, ui: &mut Ui, size: f32, color: Color32) {
        let default_color = Color32::from_white_alpha(0);
        let hovered_color = Color32::from_white_alpha(10);
        let selected_color = Color32::from_white_alpha(20);

        let bg_stroke = Stroke::new(0.08 * size, color);
        let rounding = Rounding::ZERO;
        let fg_stroke = Stroke::NONE;
        let expansion = 0.0;

        ui.visuals_mut().widgets.inactive = WidgetVisuals {
            bg_fill: default_color,
            weak_bg_fill: default_color,
            bg_stroke, rounding, fg_stroke, expansion,
        };

        ui.visuals_mut().widgets.hovered = WidgetVisuals {
            bg_fill: hovered_color,
            weak_bg_fill: hovered_color,
            bg_stroke, rounding, fg_stroke, expansion,
        };

        ui.visuals_mut().widgets.active = WidgetVisuals {
            bg_fill: selected_color,
            weak_bg_fill: selected_color,
            bg_stroke, rounding, fg_stroke, expansion,
        };
    }
}