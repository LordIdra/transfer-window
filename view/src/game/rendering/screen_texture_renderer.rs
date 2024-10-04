use std::sync::Arc;

use eframe::{egui::{Pos2, Rect}, glow::{self, Framebuffer, COLOR_BUFFER_BIT, DRAW_FRAMEBUFFER, FRAMEBUFFER, NEAREST, READ_FRAMEBUFFER, RGBA, TEXTURE_2D, TEXTURE_2D_MULTISAMPLE, UNSIGNED_BYTE}};
use glow::{Context, HasContext};
use nalgebra_glm::IVec2;

use super::{shader_program::ShaderProgram, util::{clear_framebuffer, create_multisample_color_attachment, create_normal_color_attachment, SAMPLES}, vertex_array_object::{VertexArrayObject, VertexAttribute}};

/// Implement our own MSAA as egui does not provide its own.
/// Some textures (ie pixel perfect ones) are not affected by this.
pub struct ScreenTextureRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    multisample_framebuffer: Framebuffer,
    multisample_texture: glow::Texture,
    intermediate_framebuffer: Framebuffer,
    intermediate_texture: glow::Texture,
}

impl ScreenTextureRenderer {
    pub fn new(gl: &Arc<Context>, screen_rect: Rect) -> Self {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Screen texture renderer initialisation");
        let program = ShaderProgram::new(gl, include_str!("../../../resources/shaders/screen_texture.vert"), include_str!("../../../resources/shaders/screen_texture.frag"));
        let vertex_array_object = VertexArrayObject::new(gl, vec![
            VertexAttribute { index: 0, count: 2 }, // position
            VertexAttribute { index: 1, count: 2 }, // texture coordinates
        ]);
        unsafe {
            #[cfg(feature = "profiling")]
            let _span = tracy_client::span!("Framebuffer creation");

            let multisample_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let multisample_texture = create_multisample_color_attachment(gl, multisample_framebuffer, screen_rect);

            let intermediate_framebuffer = gl.create_framebuffer().expect("Failed to create framebuffer");
            let intermediate_texture = create_normal_color_attachment(gl, intermediate_framebuffer, screen_rect);

            Self { program, vertex_array_object, multisample_framebuffer, multisample_texture, intermediate_framebuffer, intermediate_texture }
        }
    }

    pub fn resize(&self, gl: &Arc<Context>, screen_rect: Rect) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Screen texture renderer resizing");
        unsafe {
            {
                #[cfg(feature = "profiling")]
                let _span = tracy_client::span!("Resize multisample buffer");
                gl.bind_texture(TEXTURE_2D_MULTISAMPLE, Some(self.multisample_texture));
                gl.tex_image_2d_multisample(TEXTURE_2D_MULTISAMPLE, SAMPLES, RGBA as i32, screen_rect.width() as i32, screen_rect.height() as i32, true);
            }

            {
                #[cfg(feature = "profiling")]
                let _span = tracy_client::span!("Resize intermediate buffer");
                gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
                gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32,  screen_rect.width() as i32, screen_rect.height() as i32, 0, RGBA, UNSIGNED_BYTE, None);
            }
        }
    }

    fn screen_to_window(screen_rect: Rect, coordinate: Pos2) -> IVec2 {
        IVec2::new(
            (((coordinate.x + 1.0) / 2.0) * screen_rect.width()) as i32,
            (((coordinate.y + 1.0) / 2.0) * screen_rect.height()) as i32
        )
    }

    fn window_to_screen(screen_rect: Rect, coordinate: IVec2) -> Pos2 {
        Pos2::new(
            coordinate.x as f32 * 2.0 / screen_rect.width() - 1.0,
            coordinate.y as f32 * 2.0 / screen_rect.height() - 1.0,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(&mut self, gl: &Arc<Context>, texture: glow::Texture, screen_rect: Rect, corner: Pos2, width: i32, height: i32, alpha: f32) {
        #[cfg(feature = "profiling")]
        let _span = tracy_client::span!("Screen texture render");
        
        unsafe {
            clear_framebuffer(gl, self.multisample_framebuffer);
            clear_framebuffer(gl, self.intermediate_framebuffer);
        }

        let pixels_from = Self::screen_to_window(screen_rect, corner);
        let pixels_to = pixels_from + IVec2::new(width, -height);

        let from = Self::window_to_screen(screen_rect, pixels_from);
        let to = Self::window_to_screen(screen_rect, pixels_to);

        let vertices = vec![
            from.x, from.y, 0.0, 0.0,
            to.x, to.y, 1.0, 1.0,
            to.x, from.y, 1.0, 0.0,

            from.x, from.y, 0.0, 0.0,
            to.x, to.y, 1.0, 1.0,
            from.x, to.y, 0.0, 1.0,
        ];
        self.vertex_array_object.data(gl, &vertices);

        unsafe {
            gl.bind_framebuffer(FRAMEBUFFER, Some(self.multisample_framebuffer));
            gl.bind_texture(TEXTURE_2D, Some(texture));
        }

        self.program.use_program(gl);
        self.program.uniform_float(gl, "alpha", alpha);
        self.vertex_array_object.draw(gl);

        unsafe {
            gl.bind_framebuffer(READ_FRAMEBUFFER, Some(self.multisample_framebuffer));
            gl.bind_framebuffer(DRAW_FRAMEBUFFER, Some(self.intermediate_framebuffer));
            gl.blit_framebuffer(pixels_from.x, pixels_from.y, pixels_to.x, pixels_to.y, pixels_from.x, pixels_from.y, pixels_to.x, pixels_to.y, COLOR_BUFFER_BIT, NEAREST);
            gl.bind_framebuffer(FRAMEBUFFER, None);
            gl.bind_texture(TEXTURE_2D, Some(self.intermediate_texture));
        }

        let vertices = vec![
            -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, -1.0, 1.0, 0.0,

            -1.0, -1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 0.0, 1.0,
        ];
        self.vertex_array_object.data(gl, &vertices);

        self.program.use_program(gl);
        self.program.uniform_float(gl, "alpha", 1.0);
        self.vertex_array_object.draw(gl);
    }

    pub fn destroy(&self, gl: &Arc<Context>) {
        unsafe {
            gl.delete_framebuffer(self.multisample_framebuffer);
            gl.delete_texture(self.multisample_texture);
            gl.delete_framebuffer(self.intermediate_framebuffer);
            gl.delete_texture(self.intermediate_texture);
        }
    }
}
