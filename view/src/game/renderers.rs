use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::{CentralPanel, Context, PaintCallback, Rect}, egui_glow::CallbackFn, glow::{self}};
use log::error;
use transfer_window_model::Model;

use crate::{rendering::{explosion_renderer::ExplosionRenderer, geometry_renderer::GeometryRenderer, render_pipeline::RenderPipeline, screen_texture_renderer::ScreenTextureRenderer, texture_renderer::TextureRenderer}, resources::Resources};

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
    screen_texture_renderers: HashMap<String, Arc<Mutex<ScreenTextureRenderer>>>,
    explosion_renderers: Arc<Mutex<Vec<ExplosionRenderer>>>,
}

impl Renderers {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(resources: &Resources, gl: &Arc<glow::Context>, screen_rect: Rect) -> Self {
        let render_pipeline = Arc::new(Mutex::new(RenderPipeline::new(gl, screen_rect)));
        let object_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl)));
        let segment_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl)));
        let texture_renderers = resources.build_renderers(&gl);
        let screen_texture_renderers = resources.build_screen_renderers(&gl, screen_rect);
        let explosion_renderers = Arc::new(Mutex::new(vec![]));
        
        Self { screen_rect, render_pipeline, object_renderer, segment_renderer, texture_renderers, screen_texture_renderers, explosion_renderers }
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

    pub fn get_screen_texture_renderer(&self, name: &str) -> Arc<Mutex<ScreenTextureRenderer>> {
        self.screen_texture_renderers.get(name).unwrap().clone()
    }

    pub fn destroy(&mut self, gl: &Arc<glow::Context>) {
        self.render_pipeline.lock().unwrap().destroy(gl);
        self.object_renderer.lock().unwrap().destroy(gl);
        self.segment_renderer.lock().unwrap().destroy(gl);
        for renderer in self.texture_renderers.values() {
            renderer.lock().unwrap().destroy(gl);
        }
        for renderer in self.explosion_renderers.lock().unwrap().iter_mut() {
            renderer.destroy(gl);
        }
        for renderer in self.screen_texture_renderers.values() {
            renderer.lock().unwrap().destroy(gl);
        }
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
    let explosion_renderers = view.renderers.explosion_renderers.clone();
    let time = model.time();
    let zoom = view.camera.zoom();

    // Matrices need model to calculate, which is not send/sync, so we have to calculate matrices *before* constructing a callback
    let zoom_matrix = view.camera.zoom_matrix(screen_rect);
    let translation_matrices = view.camera.translation_matrices(model);

    // Start new explosion renderers
    for explosion in model.explosions_started_this_frame() {
        let renderer = ExplosionRenderer::new(&view.gl, model.time(), explosion.parent(), explosion.offset(), explosion.combined_mass());
        view.renderers.explosion_renderers.lock().unwrap().push(renderer);
    }

    // Delete expired explosion renderers
    view.renderers.explosion_renderers.lock().unwrap()
        .retain(|renderer| !renderer.is_finished(model.time()));

    // Update explosion renderers
    view.renderers.explosion_renderers.lock().unwrap()
        .iter_mut().for_each(|renderer| renderer.update_position(&mut view.camera, model, context.screen_rect()));

    // Make sure to regenerate framebuffer with new size if window resized
    if screen_rect != view.renderers.screen_rect {
        view.renderers.screen_rect = screen_rect;
        render_pipeline.lock().unwrap().resize(&view.gl, screen_rect);
        for renderer in view.renderers.screen_texture_renderers.values() {
            renderer.lock().unwrap().resize(&view.gl, screen_rect);
        }
    }

    let callback = Arc::new(CallbackFn::new(move |_info, painter| {
        let render_bloom = || {
            segment_renderer.lock().unwrap().render(painter.gl(), zoom_matrix, translation_matrices);
        };

        let render_normal = || {
            object_renderer.lock().unwrap().render(painter.gl(), zoom_matrix, translation_matrices);
            for renderer in texture_renderers.values() {
                renderer.lock().unwrap().render(painter.gl(),zoom_matrix, translation_matrices);
            }
        };

        let render_explosion = || {
            for renderer in explosion_renderers.lock().unwrap().iter() {
                renderer.render(painter.gl(), time, screen_rect, zoom);
            }
        };

        render_pipeline.lock().unwrap().render(painter.gl(), render_bloom, render_normal, render_explosion, screen_rect);
    }));

    // At time of writing there is no way to do this without providing a callback (which must be send + sync)
    CentralPanel::default().show(context, |ui| {
        ui.painter().add(PaintCallback { rect: screen_rect, callback });
    });
}