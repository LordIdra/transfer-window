use eframe::{egui::Pos2, epaint::Rect};
use log::{error, trace};
use nalgebra_glm::{scale2d, translate2d, vec2, DMat3, DVec2, Mat3, Vec2};
use transfer_window_model::{storage::entity_allocator::Entity, Model};

pub fn f64_to_f32_pair(v: f64) -> (f32, f32) {
    let upper = v as f32;
    let lower = (v - upper as f64) as f32;
    (upper, lower)
}

pub struct Camera {
    focus: Option<Entity>,
    focus_position: DVec2,
    panning: DVec2,
    zoom: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            focus: None,
            focus_position: vec2(0.0, 0.0),
            panning: vec2(0.0, 0.0),
            zoom: 0.00001,
        }
    }

    pub fn pan(&mut self, amount: DVec2) {
        self.panning += amount / self.zoom;
    }

    pub fn reset_panning(&mut self) {
        self.panning = vec2(0.0, 0.0);
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom;
    }

    pub fn set_focus(&mut self, focus: Option<Entity>) {
        trace!("Camera focus switched to {:?}", focus);
        self.focus = focus;
    }

    pub fn get_zoom(&self) -> f64 {
        self.zoom
    }

    pub fn get_translation(&self, model: &Model) -> DVec2 {
        let focus_position = if let Some(focus) = self.focus {
            if let Some(component) = model.try_get_stationary_component(focus) {
                component.get_position()
            } else if let Some(component) = model.try_get_trajectory_component(focus) {
                component.get_current_segment().get_current_position()
            } else {
                error!("Focus has neither stationary nor trajectory component");
                error!("Defaulting back to focus_position");
                self.focus_position
            }
        } else {
            self.focus_position
        };
        focus_position + self.panning
    }

    pub fn get_zoom_matrix(&self, screen_size: Rect) -> Mat3 {
        let mut mat = DMat3::identity();
        // Scale to width and height so we don't end up stretching shapes
        mat = scale2d(&mat, &DVec2::new(2.0 / screen_size.width() as f64, 2.0 / screen_size.height() as f64));
        mat = scale2d(&mat, &DVec2::new(self.zoom, self.zoom));
        Mat3::new(
            mat.m11 as f32, mat.m12 as f32, mat.m13 as f32,
            mat.m21 as f32, mat.m22 as f32, mat.m23 as f32,
            mat.m31 as f32, mat.m32 as f32, mat.m33 as f32,
        )
    }

    pub fn get_translation_matrices(&self, model: &Model) -> (Mat3, Mat3) {
        let translation = self.get_translation(model);
        let translation_pair_x = f64_to_f32_pair(translation.x);
        let translation_pair_y = f64_to_f32_pair(translation.y);
        let mat1 = translate2d(&Mat3::identity(), &Vec2::new(-translation_pair_x.0, -translation_pair_y.0));
        let mat2 = translate2d(&Mat3::identity(), &Vec2::new(-translation_pair_x.1, -translation_pair_y.1));
        (mat1, mat2)
    }

    pub fn window_space_to_world_space(&self, model: &Model, window_coords: Pos2, screen_size: Rect) -> DVec2 {
        let offset_x = f64::from(window_coords.x - (screen_size.width() / 2.0)) / self.zoom;
        let offset_y = f64::from((screen_size.height() / 2.0) - window_coords.y) / self.zoom;
        self.get_translation(model) + DVec2::new(offset_x, offset_y)
    }
}