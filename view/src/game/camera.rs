use eframe::{egui::Pos2, epaint::Rect};

use nalgebra_glm::{scale2d, translate2d, vec2, DMat3, DVec2, Mat3, Vec2};
use transfer_window_model::storage::entity_allocator::Entity;

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
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            focus: None,
            focus_position: vec2(0.0, 0.0),
            panning: vec2(0.0, 0.0),
            zoom: 0.00003,
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

    pub fn set_focus(&mut self, focus: Entity, focus_position: DVec2) {
        self.focus = Some(focus);
        self.focus_position = focus_position;
    }

    pub fn unset_focus(&mut self,) {
        self.focus = None;
    }

    pub fn focus(&self) -> Option<Entity> {
        self.focus
    }

    pub fn zoom(&self) -> f64 {
        self.zoom
    }

    pub fn translation(&self) -> DVec2 {
        self.focus_position + self.panning
    }

    pub fn zoom_matrix(&self, screen_rect: Rect) -> Mat3 {
        let mut mat = DMat3::identity();
        // Scale to width and height so we don't end up stretching shapes
        mat = scale2d(&mat, &DVec2::new(2.0 / screen_rect.width() as f64, 2.0 / screen_rect.height() as f64));
        mat = scale2d(&mat, &DVec2::new(self.zoom, self.zoom));
        Mat3::new(
            mat.m11 as f32, mat.m12 as f32, mat.m13 as f32,
            mat.m21 as f32, mat.m22 as f32, mat.m23 as f32,
            mat.m31 as f32, mat.m32 as f32, mat.m33 as f32,
        )
    }

    pub fn translation_matrices(&self) -> (Mat3, Mat3) {
        let translation = self.translation();
        let translation_pair_x = f64_to_f32_pair(translation.x);
        let translation_pair_y = f64_to_f32_pair(translation.y);
        let mat1 = translate2d(&Mat3::identity(), &Vec2::new(-translation_pair_x.0, -translation_pair_y.0));
        let mat2 = translate2d(&Mat3::identity(), &Vec2::new(-translation_pair_x.1, -translation_pair_y.1));
        (mat1, mat2)
    }

    pub fn window_space_to_screen_space(screen_rect: Rect, window_coords: Pos2) -> Pos2 {
        Pos2::new(
            (window_coords.x * 2.0) / screen_rect.width() - 1.0, 
            -(window_coords.y * 2.0) / screen_rect.height() + 1.0, )
    }
}