use std::sync::Arc;

use eframe::{egui::Rect, glow::{Context, Framebuffer, HasContext, Texture, CLAMP_TO_EDGE, COLOR_ATTACHMENT0, COLOR_BUFFER_BIT, DRAW_FRAMEBUFFER, FRAMEBUFFER, LINEAR, NEAREST, READ_FRAMEBUFFER, RGBA, TEXTURE0, TEXTURE1, TEXTURE_2D, TEXTURE_2D_MULTISAMPLE, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNSIGNED_BYTE}};

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

const SAMPLES: i32 = 16;
const BLOOM_PASSES: i32 = 5;

unsafe fn create_multisample_color_attachment(gl: &Arc<Context>, framebuffer: Framebuffer, screen_rect: Rect) -> Texture {
    unsafe {
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
        gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(texture));
        gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);
        gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D_MULTISAMPLE, Some(texture), 0);
        texture
    }
}

unsafe fn create_normal_color_attachment(gl: &Arc<Context>, framebuffer: Framebuffer, screen_rect: Rect) -> Texture {
    unsafe {
        let texture = gl.create_texture().expect("Failed to create texture");
        gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
        gl.bind_texture(TEXTURE_2D, Some(texture));
        gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
        gl.framebuffer_texture_2d(FRAMEBUFFER, COLOR_ATTACHMENT0, TEXTURE_2D, Some(texture), 0);
        texture
    }
}

unsafe fn clear_framebuffer(gl: &Arc<Context>, framebuffer: Framebuffer) {
    gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(COLOR_BUFFER_BIT);

}

pub struct RenderPipeline {
    gl: Arc<Context>,
    multisample_framebuffer: Framebuffer,
    multisample_texture: Texture,
    intermediate_framebuffer: Framebuffer,
    intermediate_texture: Texture,
    bloom_framebuffer_1: Framebuffer,
    bloom_texture_1: Texture,
    bloom_framebuffer_2: Framebuffer,
    bloom_texture_2: Texture,
    bloom_program: ShaderProgram,
    screen_program: ShaderProgram,
    screen_vao: VertexArrayObject,
}

impl RenderPipeline {
    pub fn new(gl: Arc<Context>, screen_rect: Rect) -> Self {
        unsafe {
            let multisample_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let multisample_texture = create_multisample_color_attachment(&gl, multisample_framebuffer, screen_rect);

            let intermediate_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let intermediate_texture = create_normal_color_attachment(&gl, intermediate_framebuffer, screen_rect);

            let bloom_framebuffer_1 = gl.create_framebuffer().expect("Failed to create framebuffer");
            let bloom_texture_1 = create_normal_color_attachment(&gl, bloom_framebuffer_1, screen_rect);
            
            let bloom_framebuffer_2 = gl.create_framebuffer().expect("Failed to create framebuffer");
            let bloom_texture_2 = create_normal_color_attachment(&gl, bloom_framebuffer_2, screen_rect);

            gl.bind_framebuffer(FRAMEBUFFER, None);

            let bloom_program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/bloom.vert"), include_str!("../../resources/shaders/bloom.frag"));
            let screen_program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/screen.vert"), include_str!("../../resources/shaders/screen.frag"));

            let mut screen_vao = VertexArrayObject::new(gl.clone(), vec![
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
            screen_vao.data(&screen_vertices);

            Self { gl, multisample_framebuffer, multisample_texture, intermediate_framebuffer, intermediate_texture, bloom_framebuffer_1, bloom_texture_1, bloom_framebuffer_2, bloom_texture_2, bloom_program, screen_program, screen_vao, }
        }
    }

    pub fn resize(&self, screen_rect: Rect) {
        unsafe {
            // Resize multisample texture
            self.gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(self.multisample_texture));
            self.gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);

            // Resize intermediate texture
            self.gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);

            // Resize bloom texture
            self.gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_1));
            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
            self.gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_2));
            self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
        }
    }

    pub fn render(&self, render_bloom: impl FnOnce(), render_normal: impl FnOnce(), screen_rect: Rect) {
        let width = screen_rect.width() as i32;
        let height = screen_rect.height() as i32;

        unsafe {
            // Clear multisample framebuffer
            clear_framebuffer(&self.gl, self.multisample_framebuffer);
            clear_framebuffer(&self.gl, self.intermediate_framebuffer);
            clear_framebuffer(&self.gl, self.bloom_framebuffer_1);
            clear_framebuffer(&self.gl, self.bloom_framebuffer_2);

            // Render to multisample framebuffer
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            render_bloom();

            // Blit from multisample framebuffer to bloom framebuffer
            self.gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            self.gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.bloom_framebuffer_1));
            self.gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, NEAREST);

            // Render this ON TOP of the bloom (so both are in the same framebuffer)
            self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            render_normal();

            // Blit from multisample framebuffer to intermediate framebuffer
            self.gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            self.gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.intermediate_framebuffer));
            self.gl.blit_framebuffer(0, 0, width, height, 0, 0, width, height, COLOR_BUFFER_BIT, NEAREST);

            // Bloom passes
            self.bloom_program.use_program();
            for _ in 0..BLOOM_PASSES {
                self.gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_1)); // from
                self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.bloom_framebuffer_2)); // to
                self.bloom_program.uniform_bool("is_horizontal", false);
                self.screen_vao.draw();

                self.gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_2)); // from
                self.gl.bind_framebuffer(FRAMEBUFFER, Some(self.bloom_framebuffer_1)); // to
                self.bloom_program.uniform_bool("is_horizontal", true);
                self.screen_vao.draw();
            }

            // Render final textures to screen
            self.gl.bind_framebuffer(FRAMEBUFFER, None);
            self.gl.active_texture(TEXTURE0);
            self.gl.bind_texture(TEXTURE_2D, Some(self.bloom_texture_1));
            self.gl.active_texture(TEXTURE1);
            self.gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
            self.screen_program.use_program();
            self.screen_program.uniform_int("texture_sampler_lower", 0);
            self.screen_program.uniform_int("texture_sampler_upper", 1);
            self.screen_vao.draw();
        }
    }
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_framebuffer(self.multisample_framebuffer);
            self.gl.delete_texture(self.multisample_texture);
            self.gl.delete_framebuffer(self.intermediate_framebuffer);
            self.gl.delete_texture(self.intermediate_texture);
            self.gl.delete_framebuffer(self.bloom_framebuffer_1);
            self.gl.delete_framebuffer(self.bloom_framebuffer_2);
            self.gl.delete_texture(self.bloom_texture_1);
            self.gl.delete_texture(self.bloom_texture_2);
        }
    }
}