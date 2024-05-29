use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::{CentralPanel, Context, PaintCallback, Rect}, egui_glow::CallbackFn, glow};
use log::error;
use transfer_window_model::Model;

use crate::{rendering::{geometry_renderer::GeometryRenderer, render_pipeline::RenderPipeline, texture_renderer::TextureRenderer}, resources::Resources};

use super::Scene;

/// Rendering pipeline overview
/// 1) View adds all necessary vertices to various renderers
/// 2) All renderers are rendered to `multisample_framebuffer` which allows multisampling
/// 3) The resulting texture is resolved onto a texture in `intermediate_framebuffer`
/// 4) The texture is rendered to the default FBO
pub struct Renderers {
    screen_rect: Rect,
    render_pipeline: Arc<Mutex<RenderPipeline>>,
    object_renderer: Arc<Mutex<GeometryRenderer>>,
    segment_renderer: Arc<Mutex<GeometryRenderer>>,
    texture_renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
}

impl Renderers {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(resources: &Resources, gl: Arc<glow::Context>, screen_rect: Rect) -> Self {
        let render_pipeline = Arc::new(Mutex::new(RenderPipeline::new(gl.clone(), screen_rect)));
        let object_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let segment_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let texture_renderers = resources.build_renderers(&gl);
        
        Self { screen_rect, render_pipeline, object_renderer, segment_renderer, texture_renderers }
    }

    pub fn add_object_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.object_renderer.lock().unwrap().add_vertices(vertices);
    }

    pub fn add_segment_vertices(&mut self, vertices: &mut Vec<f32>) {
        self.segment_renderer.lock().unwrap().add_vertices(vertices);
    }

    pub fn add_texture_vertices(&mut self, texture: &str, vertices: &mut Vec<f32>) {
        let Some(renderer) = self.texture_renderers.get(texture) else {
            error!("Texture {} does not exist", texture);
            return;
        };
        renderer.lock().unwrap().add_vertices(vertices);
    }
}

pub fn update(view: &mut Scene, model: &Model, context: &Context) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update rendering");

    let screen_rect = context.screen_rect();
    let render_pipeline = view.renderers.render_pipeline.clone();
    let object_renderer = view.renderers.object_renderer.clone();
    let segment_renderer = view.renderers.segment_renderer.clone();
    let texture_renderers = view.renderers.texture_renderers.clone();

    // Make sure to regenerate framebuffer with new size if window resized
    if screen_rect != view.renderers.screen_rect {
        view.renderers.screen_rect = screen_rect;
        render_pipeline.lock().unwrap().resize(screen_rect);
    }

    // Matrices need model to calculate, which is not send/sync, so we have to calculate matrices *before* constructing a callback
    let zoom_matrix = view.camera.zoom_matrix(screen_rect);
    let translation_matrices = view.camera.translation_matrices(model);

    let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
        let render_bloom = || {
            segment_renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        };

        let render_normal = || {
            object_renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
            for renderer in texture_renderers.values() {
                renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
            }
        };

        render_pipeline.lock().unwrap().render(render_bloom, render_normal, screen_rect);
    }));

    // At time of writing there is no way to do this without providing a callback (which must be send + sync)
    CentralPanel::default().show(context, |ui| {
        ui.painter().add(PaintCallback { rect: screen_rect, callback });
    });
}