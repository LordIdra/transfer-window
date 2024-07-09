pub mod explosion_renderer;
pub mod geometry_renderer;
pub mod render_pipeline;
pub mod screen_texture_renderer;
pub mod celestial_object_renderer;
pub mod atmosphere_renderer;
mod shader_program;
pub mod texture_renderer;
pub mod texture;
mod util;
mod vertex_array_object;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::{CentralPanel, PaintCallback, Rect}, egui_glow::CallbackFn, glow::{self}};
use log::error;

use crate::{game::rendering::{explosion_renderer::ExplosionRenderer, geometry_renderer::GeometryRenderer, render_pipeline::RenderPipeline, screen_texture_renderer::ScreenTextureRenderer, texture_renderer::TextureRenderer, celestial_object_renderer::CelestialObjectRenderer, atmosphere_renderer::AtmosphereRenderer}, resources::Resources};
use transfer_window_model::components::ComponentType;
use transfer_window_model::Model;

use super::View;

/// Rendering pipeline overview
/// 1) View adds all necessary vertices to various renderers
/// 2) All renderers are rendered to `multisample_framebuffer` which allows multisampling
/// 3) The resulting texture is resolved onto a texture in `intermediate_framebuffer`
/// 4) The texture is rendered to the default FBO
pub struct Renderers {
    render_pipeline: Arc<Mutex<RenderPipeline>>,
    celestial_object_renderers: HashMap<String, Arc<Mutex<CelestialObjectRenderer>>>,
    atmosphere_renderers: HashMap<String, Arc<Mutex<AtmosphereRenderer>>>,
    segment_renderer: Arc<Mutex<GeometryRenderer>>,
    texture_renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
    screen_texture_renderer: Arc<Mutex<ScreenTextureRenderer>>,
    explosion_renderers: Arc<Mutex<Vec<ExplosionRenderer>>>,
}

impl Renderers {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(resources: &Resources, gl: &Arc<glow::Context>, model: &Model, screen_rect: Rect) -> Self {
        let render_pipeline = Arc::new(Mutex::new(RenderPipeline::new(gl, screen_rect)));
        let celestial_object_renderers = resources.build_celestial_object_renderers(gl);
        let mut atmosphere_renderers = HashMap::new();
        for entity in model.entities(vec![ComponentType::AtmosphereComponent]) {
            let name = model.name_component(entity).name().to_lowercase();
            let atmosphere = model.atmosphere_component(entity);
            let renderer = Arc::new(Mutex::new(AtmosphereRenderer::new(gl, atmosphere)));
            atmosphere_renderers.insert(name, renderer);
        }
        let segment_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl)));
        let texture_renderers = resources.build_texture_renderers(gl);
        let screen_texture_renderer = Arc::new(Mutex::new(ScreenTextureRenderer::new(gl, screen_rect)));
        let explosion_renderers = Arc::new(Mutex::new(vec![]));
        
        Self { render_pipeline, celestial_object_renderers, atmosphere_renderers, segment_renderer, texture_renderers, screen_texture_renderer, explosion_renderers }
    }

    pub fn add_celestial_object_vertices(&self, name: &str, vertices: &mut Vec<f32>) {
        self.celestial_object_renderers[name].lock().unwrap().add_vertices(vertices);
    }
    
    pub fn set_object_rotation(&self, name: &str, rotation: f32) {
        self.celestial_object_renderers[name].lock().unwrap().set_rotation(rotation);
    }

    pub fn add_atmosphere_vertices(&self, name: &str, vertices: &mut Vec<f32>) {
        self.atmosphere_renderers[name].lock().unwrap().add_vertices(vertices);
    }

    pub fn add_segment_vertices(&self, vertices: &mut Vec<f32>) {
        self.segment_renderer.lock().unwrap().add_vertices(vertices);
    }

    pub fn add_texture_vertices(&self, texture: &str, vertices: &mut Vec<f32>) {
        let Some(renderer) = self.texture_renderers.get(texture) else {
            error!("Texture {} does not exist", texture);
            return;
        };
        renderer.lock().unwrap().add_vertices(vertices);
    }

    pub fn screen_texture_renderer(&self) -> Arc<Mutex<ScreenTextureRenderer>> {
        self.screen_texture_renderer.clone()
    }

    pub fn destroy(&mut self, gl: &Arc<glow::Context>) {
        self.render_pipeline.lock().unwrap().destroy(gl);
        self.segment_renderer.lock().unwrap().destroy(gl);
        for renderer in self.texture_renderers.values() {
            renderer.lock().unwrap().destroy(gl);
        }
        for renderer in self.celestial_object_renderers.values() {
            renderer.lock().unwrap().destroy(gl);
        }
        for renderer in self.atmosphere_renderers.values() {
            renderer.lock().unwrap().destroy(gl);
        }
        for renderer in self.explosion_renderers.lock().unwrap().iter_mut() {
            renderer.destroy(gl);
        }
        self.screen_texture_renderer.lock().unwrap().destroy(gl);
    }
}

pub fn update(view: &View) {
    #[cfg(feature = "profiling")]
    let _span = tracy_client::span!("Update rendering");

    let screen_rect = view.screen_rect;
    let render_pipeline = view.renderers.render_pipeline.clone();
    let object_renderers = view.renderers.celestial_object_renderers.clone();
    let atmosphere_renderers = view.renderers.atmosphere_renderers.clone();
    let segment_renderer = view.renderers.segment_renderer.clone();
    let texture_renderers = view.renderers.texture_renderers.clone();
    let explosion_renderers = view.renderers.explosion_renderers.clone();
    let time = view.model.time();
    let zoom = view.camera.zoom();

    // Matrices need model to calculate, which is not send/sync, so we have to calculate matrices *before* constructing a callback
    let zoom_matrix = view.camera.zoom_matrix(screen_rect);
    let translation_matrices = view.camera.translation_matrices();

    // Start new explosion renderers
    for explosion in view.model.explosions_started_this_frame() {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Start explosion renderer");
        let renderer = ExplosionRenderer::new(&view.gl, view.model.time(), explosion.parent(), explosion.offset(), explosion.combined_mass());
        view.renderers.explosion_renderers.lock().unwrap().push(renderer);
    }

    // Delete expired explosion renderers
    view.renderers.explosion_renderers.lock().unwrap()
        .retain(|renderer| !renderer.is_finished(view.model.time()));

    // Update explosion renderers
    view.renderers.explosion_renderers.clone().lock().unwrap()
        .iter_mut().for_each(|renderer| renderer.update_position(view));

    // Make sure to regenerate framebuffer with new size if window resized
    if screen_rect != view.previous_screen_rect {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Resize buffers");
        render_pipeline.lock().unwrap().resize(&view.gl, screen_rect);
        view.renderers.screen_texture_renderer.lock().unwrap().resize(&view.gl, screen_rect);
    }

    let callback = Arc::new(CallbackFn::new(move |_info, painter| {
        let render_bloom = || {
            #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Render bloom");
            segment_renderer.lock().unwrap().render_lines(painter.gl(), zoom_matrix, translation_matrices);
        };

        let render_normal = || {
            #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Render normal");
            for renderer in object_renderers.values() {
                renderer.lock().unwrap().render(painter.gl(), zoom_matrix, translation_matrices);
            }
            for renderer in atmosphere_renderers.values() {
                renderer.lock().unwrap().render(painter.gl(), zoom_matrix, translation_matrices);
            }
            for renderer in texture_renderers.values() {
                renderer.lock().unwrap().render(painter.gl(),zoom_matrix, translation_matrices);
            }
        };

        let render_explosion = || {
            #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Render explosion");
            for renderer in explosion_renderers.lock().unwrap().iter() {
                renderer.render(painter.gl(), time, screen_rect, zoom);
            }
        };

        render_pipeline.lock().unwrap().render(painter.gl(), render_bloom, render_normal, render_explosion, screen_rect);
    }));

    // At time of writing there is no way to do this without providing a callback (which must be send + sync)
    CentralPanel::default().show(&view.context.clone(), |ui| {
        ui.painter().add(PaintCallback { rect: screen_rect, callback });
    });
}