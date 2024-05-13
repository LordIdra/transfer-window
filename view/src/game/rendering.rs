use std::sync::Arc;

use eframe::{egui::{CentralPanel, Context, PaintCallback}, egui_glow::CallbackFn};
use transfer_window_model::Model;

use super::Scene;

pub mod geometry_renderer;
mod shader_program;
pub mod texture_renderer;
pub mod texture;
mod vertex_array_object;
pub mod object_renderer;

pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update rendering");
    
    let rect = context.screen_rect();
    let object_renderers = view.object_renderers.clone();
    let segment_renderer = view.segment_renderer.clone();
    let texture_renderers = view.texture_renderers.clone();

    // Matrices need model to calculate, which is not send/sync, so we have to calculate matrices *before* constructing a callback
    let zoom_matrix = view.camera.zoom_matrix(rect);
    let translation_matrices = view.camera.translation_matrices(model);

    let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
        for renderer in object_renderers.values() {
            renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        }
        segment_renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        for renderer in texture_renderers.values() {
            renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        }
    }));

    // At time of writing there is no way to do this without providing a callback (which must be send + sync)
    CentralPanel::default().show(context, |ui| {
        ui.painter().add(PaintCallback { rect, callback });
    });
}