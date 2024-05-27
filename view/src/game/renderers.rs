use std::{collections::HashMap, sync::{Arc, Mutex}};

use eframe::{egui::{CentralPanel, Context, PaintCallback, Rect}, egui_glow::CallbackFn, glow};
use log::error;
use transfer_window_model::Model;

use crate::{rendering::{color_buffer_multisample::ColorBufferMultisample, color_buffer_normal::ColorBufferNormal, framebuffer::Framebuffer, geometry_renderer::GeometryRenderer, intermediate_renderer::IntermediateRenderer, screen_renderer::ScreenRenderer, texture_renderer::TextureRenderer}, resources::Resources};

use super::Scene;

/// Rendering pipeline overview
/// 1) View adds all necessary vertices to various renderers
/// 2) All renderers are rendered to `multisample_framebuffer` which allows multisampling
/// 3) The resulting texture is resolved onto a texture in `intermediate_framebuffer`
/// 4) The texture is rendered to the default FBO
pub struct Renderers {
    screen_rect: Rect,
    intermediate_renderer: Arc<Mutex<IntermediateRenderer>>,
    screen_renderer: Arc<Mutex<ScreenRenderer>>,
    object_renderer: Arc<Mutex<GeometryRenderer>>,
    segment_renderer: Arc<Mutex<GeometryRenderer>>,
    texture_renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
}

impl Renderers {
    pub fn new(resources: &Resources, gl: &Arc<glow::Context>, screen_rect: Rect) -> Self {
        let framebuffer = Framebuffer::new(gl.clone());
        let color_buffer = ColorBufferMultisample::new(gl.clone(), screen_rect);
        color_buffer.attach_to_framebuffer(&framebuffer);
        let intermediate_renderer = Arc::new(Mutex::new(IntermediateRenderer::new(framebuffer, color_buffer)));

        let framebuffer = Framebuffer::new(gl.clone());
        let color_buffer = ColorBufferNormal::new(gl.clone(), screen_rect);
        color_buffer.attach_to_framebuffer(&framebuffer);
        let screen_renderer = Arc::new(Mutex::new(ScreenRenderer::new(gl.clone(), framebuffer, color_buffer)));

        let object_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let segment_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let texture_renderers = resources.build_renderers(gl);
        
        Self { screen_rect, intermediate_renderer, screen_renderer, object_renderer, segment_renderer, texture_renderers }
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
    let intermediate_renderer = view.renderers.intermediate_renderer.clone();
    let screen_renderer = view.renderers.screen_renderer.clone();
    let object_renderer = view.renderers.object_renderer.clone();
    let segment_renderer = view.renderers.segment_renderer.clone();
    let texture_renderers = view.renderers.texture_renderers.clone();

    // Make sure to regenerate framebuffer with new size if window resized
    if screen_rect != view.renderers.screen_rect {
        view.renderers.screen_rect = screen_rect;
        screen_renderer.lock().unwrap().resize(screen_rect);
        intermediate_renderer.lock().unwrap().resize(screen_rect);
    }

    // Matrices need model to calculate, which is not send/sync, so we have to calculate matrices *before* constructing a callback
    let zoom_matrix = view.camera.zoom_matrix(screen_rect);
    let translation_matrices = view.camera.translation_matrices(model);

    let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
        screen_renderer.lock().unwrap().clear();
        intermediate_renderer.lock().unwrap().clear();

        // Draw scene to intermediate renderer
        intermediate_renderer.lock().unwrap().set_as_draw_target();
        object_renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        segment_renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        for renderer in texture_renderers.values() {
            renderer.lock().unwrap().render(zoom_matrix, translation_matrices);
        }

        // Blit intermediate renderer to screen renderer and draw to screen
        screen_renderer.lock().unwrap().blit_from(&intermediate_renderer.lock().unwrap(), screen_rect);
        screen_renderer.lock().unwrap().set_screen_as_draw_target();
        screen_renderer.lock().unwrap().render();
    }));

    // At time of writing there is no way to do this without providing a callback (which must be send + sync)
    CentralPanel::default().show(context, |ui| {
        ui.painter().add(PaintCallback { rect: screen_rect, callback });
    });
}