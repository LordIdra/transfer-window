use std::sync::Arc;

use eframe::{egui::Rect, glow::{self, Context, Framebuffer, HasContext, Texture, COLOR_BUFFER_BIT, DRAW_FRAMEBUFFER, FRAMEBUFFER, LINEAR, NEAREST, READ_FRAMEBUFFER, RGBA, TEXTURE0, TEXTURE1, TEXTURE2, TEXTURE_2D, TEXTURE_2D_MULTISAMPLE, UNSIGNED_BYTE}};

use super::{shader_program::ShaderProgram, util::{clear_framebuffer, create_multisample_color_attachment, create_normal_color_attachment, SAMPLES}, vertex_array_object::{VertexArrayObject, VertexAttribute}};

const BLOOM_PASSES: i32 = 5;

pub struct RenderPipeline {
    multisample_framebuffer: Framebuffer,
    multisample_texture: Texture,
    intermediate_framebuffer: Framebuffer,
    intermediate_texture: Texture,
    bloom_framebuffer_1: Framebuffer,
    bloom_texture_1: Texture,
    bloom_framebuffer_2: Framebuffer,
    bloom_texture_2: Texture,
    bloom_program: ShaderProgram,
    explosion_framebuffer: Framebuffer,
    explosion_texture: Texture,
    screen_program: ShaderProgram,
    screen_vao: VertexArrayObject,
}

impl RenderPipeline {
    pub fn new(gl: &Arc<Context>, screen_rect: Rect) -> Self {
        unsafe {
            let multisample_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let multisample_texture = create_multisample_color_attachment(gl, multisample_framebuffer, screen_rect);

            let intermediate_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let intermediate_texture = create_normal_color_attachment(gl, intermediate_framebuffer, screen_rect);

            let bloom_framebuffer_1 = gl.create_framebuffer().expect("Failed to create framebuffer");
            let bloom_texture_1 = create_normal_color_attachment(gl, bloom_framebuffer_1, screen_rect);
            
            let bloom_framebuffer_2 = gl.create_framebuffer().expect("Failed to create framebuffer");
            let bloom_texture_2 = create_normal_color_attachment(gl, bloom_framebuffer_2, screen_rect);

            let explosion_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let explosion_texture = create_normal_color_attachment(gl, explosion_framebuffer, screen_rect);

            gl.bind_framebuffer(FRAMEBUFFER, None);

            let bloom_program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/bloom.vert"), include_str!("../../../resources/shaders/bloom.frag"));
            let screen_program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/screen.vert"), include_str!("../../../resources/shaders/screen.frag"));

            let mut screen_vao = VertexArrayObject::new(gl, vec![
                VertexAttribute { index: 0, count: 2 }, // x
                VertexAttribute { index: 1, count: 2 }, // texture coordinates
            ]);
            let screen_vertices = vec![
                -1.0, -1.0, 0.0, 0.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, -1.0, 1.0, 0.0,

                -1.0, -1.0, 0.0, 0.0,
                1.0, 1.0, 1.0, 1.0,
                -1.0, 1.0, 0.0, 1.0,
            ];
            screen_vao.data(gl, &screen_vertices);

            Self { 
                multisample_framebuffer, multisample_texture, 
                intermediate_framebuffer, intermediate_texture, 
                bloom_framebuffer_1, bloom_texture_1, bloom_framebuffer_2, bloom_texture_2, bloom_program, 
                explosion_framebuffer, explosion_texture,
                screen_program, screen_vao, 
            }
        }
    }

    pub fn resize(&self, gl: &Arc<Context>, screen_rect: Rect) {
        unsafe {
            // Resize multisample texture
            gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(self.multisample_texture));
            gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);

            // Resize intermediate texture
            gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);

            // Resize bloom texture
            gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_1));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
            gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_2));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);

            // Resize epxlosion texture
            gl.bind_texture(TEXTURE_2D, Some(self.explosion_texture));
            gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        }
    }

    pub fn render(&self, gl: &Arc<Context>, render_bloom: impl FnOnce(), render_normal: impl FnOnce(), render_explosion: impl FnOnce(), screen_rect: Rect) {
        #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Render pipeline");
        let width = screen_rect.width() as i32;
        let height = screen_rect.height() as i32;

        unsafe {
            // Clear framebuffers
            clear_framebuffer(gl, self.multisample_framebuffer);
            clear_framebuffer(gl, self.intermediate_framebuffer);
            clear_framebuffer(gl, self.bloom_framebuffer_1);
            clear_framebuffer(gl, self.bloom_framebuffer_2);
            clear_framebuffer(gl, self.explosion_framebuffer);

            // Render explosions
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.explosion_framebuffer));
            render_explosion();

            // Render to multisample framebuffer
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            render_bloom();

            // Blit from multisample framebuffer to bloom framebuffer
            gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.bloom_framebuffer_1));
            gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, NEAREST);

            // Render this ON TOP of the bloom (so both are in the same framebuffer)
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            render_normal();

            // Blit from multisample framebuffer to intermediate framebuffer
            gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.intermediate_framebuffer));
            gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, NEAREST);

            {
                #[cfg(feature = "profiling")]
                let _span = tracy_client::span!("Bloom passes");
                // Bloom passes
                self.bloom_program.use_program(gl);
                for _ in 0..BLOOM_PASSES {
                    gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_1)); // from
                    gl.bind_framebuffer(FRAMEBUFFER, Some(self.bloom_framebuffer_2)); // to
                    self.bloom_program.uniform_bool(gl, "is_horizontal", false);
                    self.screen_vao.draw(gl);

                    gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_2)); // from
                    gl.bind_framebuffer(FRAMEBUFFER, Some(self.bloom_framebuffer_1)); // to
                    self.bloom_program.uniform_bool(gl, "is_horizontal", true);
                    self.screen_vao.draw(gl);
                }
            }

            // Render final textures to screen
            gl.bind_framebuffer(FRAMEBUFFER, None);
            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_1));
            gl.active_texture(TEXTURE1);
            gl.bind_texture(TEXTURE_2D, Some(self.explosion_texture));
            gl.active_texture(TEXTURE2);
            gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            self.screen_program.use_program(gl);
            self.screen_program.uniform_int(gl, "texture_sampler_bloom", 0);
            self.screen_program.uniform_int(gl, "texture_sampler_explosion", 1);
            self.screen_program.uniform_int(gl, "texture_sampler_normal", 2);
            self.screen_vao.draw(gl);
        }
    }

    pub fn destroy(&mut self, gl: &Arc<glow::Context>) {
        self.bloom_program.destroy(gl);
        unsafe {
            gl.delete_framebuffer(self.multisample_framebuffer);
            gl.delete_texture(self.multisample_texture);
            gl.delete_framebuffer(self.intermediate_framebuffer);
            gl.delete_texture(self.intermediate_texture);
            gl.delete_framebuffer(self.bloom_framebuffer_1);
            gl.delete_framebuffer(self.bloom_framebuffer_2);
            gl.delete_texture(self.bloom_texture_1);
            gl.delete_texture(self.bloom_texture_2);
        }
    }
}
