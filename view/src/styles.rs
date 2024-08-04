use eframe::{egui::{style::{Interaction, Spacing, WidgetVisuals}, Color32, Context, Margin, Rounding, Stroke, Style, Ui, Visuals}, epaint::Shadow};

pub struct DefaultWindow;

impl DefaultWindow {
    pub fn apply(context: &Context) {
        context.set_style(Style {
            visuals: Visuals {
                window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 100),
                window_stroke: Stroke::NONE,
                window_shadow: Shadow::NONE,
                window_rounding: Rounding::ZERO,
                ..Default::default()
            },
            interaction: Interaction {
                selectable_labels: false,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub struct VesselEditor;

impl VesselEditor {
    pub fn apply(context: &Context) {
        context.set_visuals(Visuals {
            window_fill: Color32::from_rgba_unmultiplied(0, 0, 0, 200),
            window_stroke: Stroke::NONE,
            window_shadow: Shadow::NONE,
            window_rounding: Rounding::ZERO,
            ..Visuals::default()
        });

        context.set_style(Style {
            interaction: Interaction {
                selectable_labels: false,
                ..Interaction::default()
            },
            spacing: Spacing {
                window_margin: Margin::same(20.0),
                ..Spacing::default()
            },
            ..Style::default()
        });
    }
}

pub struct SelectedMenuButton;

impl SelectedMenuButton {
    pub fn apply(ui: &mut Ui) {
        let default_color = Color32::from_rgba_unmultiplied(0, 0, 0, 0);
        let hovered_color = Color32::from_rgba_unmultiplied(40, 40, 40, 220);
        let selected_color = Color32::from_rgba_unmultiplied(60, 60, 60, 220);

        let bg_stroke = Stroke::NONE;
        let rounding = Rounding::same(20.0);
        let fg_stroke = Stroke::NONE;
        let expansion = 2.0;

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

pub struct DialogueWindow;

impl DialogueWindow {
    pub fn apply(context: &Context) {
        context.set_style(Style {
            visuals: Visuals {
                window_fill: Color32::from_rgba_unmultiplied(30, 30, 40, 255),
                window_stroke: Stroke::NONE,
                window_shadow: Shadow::NONE,
                window_rounding: Rounding::same(15.0),
                ..Default::default()
            },
            spacing: Spacing {
                window_margin: Margin::same(10.0),
                ..Spacing::default()
            },
            interaction: Interaction {
                selectable_labels: false,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub struct DialogueContinueButton;

impl DialogueContinueButton {
    pub fn apply(ui: &mut Ui) {
        let default_color = Color32::from_rgba_unmultiplied(60, 60, 100, 255);
        let hovered_color = Color32::from_rgba_unmultiplied(80, 80, 120, 255);
        let selected_color = Color32::from_rgba_unmultiplied(100, 100, 140, 255);

        let bg_stroke = Stroke::NONE;
        let rounding = Rounding::same(5.0);
        let fg_stroke = Stroke::new(4.0, Color32::WHITE);
        let expansion = 5.0;

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

pub struct ExitModal;

impl ExitModal {
    pub fn apply(context: &Context) {
        context.set_style(Style {
            visuals: Visuals {
                window_fill: Color32::from_rgba_unmultiplied(20, 20, 20, 255),
                window_stroke: Stroke::NONE,
                window_shadow: Shadow::NONE,
                window_rounding: Rounding::same(40.0),
                ..Default::default()
            },
            interaction: Interaction {
                selectable_labels: false,
                ..Default::default()
            },
            spacing: Spacing {
                window_margin: Margin::same(30.0),
                ..Spacing::default()
            },
            ..Default::default()
        });
    }
}