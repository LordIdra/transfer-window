use std::sync::Arc;

use eframe::glow;
use glow::{Context, Program, HasContext, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation};

struct Shader {
    shader: glow::Shader,
}

impl Shader {
    fn new(gl: &Arc<Context>, shader_source: &str, shader_type: u32) -> Self {
        unsafe {
            let shader = gl.create_shader(shader_type).expect("Failed to create shader");
            // gl.render
            gl.shader_source(shader, shader_source);
            gl.compile_shader(shader);
            assert!(gl.get_shader_compile_status(shader), "Failed to compile shader:\n{}", gl.get_shader_info_log(shader));
            Shader { shader }
        }
    }

    fn attach(&self, gl: &Arc<Context>, program: Program) {
        unsafe {
            gl.attach_shader(program, self.shader);
        }
    }

    fn destroy(&mut self, gl: &Arc<Context>,) {
        unsafe { 
            gl.delete_shader(self.shader);
        };
    }
}

pub struct ShaderProgram {
    program: Program,
}

impl ShaderProgram {
    pub fn new(gl: &Arc<Context>, vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
        let mut vertex_shader = Shader::new(gl, vertex_shader_source, VERTEX_SHADER);
        let mut fragment_shader = Shader::new(gl, fragment_shader_source, FRAGMENT_SHADER);
        let program = unsafe { gl.create_program().expect("Failed to create shader program") };
        vertex_shader.attach(gl, program);
        fragment_shader.attach(gl, program);

        vertex_shader.destroy(gl);
        fragment_shader.destroy(gl);
        
        unsafe {
            gl.link_program(program);
            assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program));
        }

        ShaderProgram { program }
    }

    pub fn use_program(&self, gl: &Arc<Context>) {
        unsafe { gl.use_program(Some(self.program)) };
    }

    fn location(&self, gl: &Arc<Context>, name: &str) -> NativeUniformLocation {
        unsafe { gl.get_uniform_location(self.program, name).unwrap_or_else(|| panic!("Failed to find uniform location '{name}'")) }
    }

    pub fn uniform_bool(&self, gl: &Arc<Context>, name: &str, v: bool) {
        self.use_program(gl);
        unsafe { gl.uniform_1_i32(Some(&Self::location(self, gl, name)), v as i32); } 
    }

    pub fn uniform_int(&self, gl: &Arc<Context>,name: &str, v: i32) {
        self.use_program(gl);
        unsafe { gl.uniform_1_i32(Some(&Self::location(self, gl, name)), v); } 
    }

    pub fn uniform_float(&self, gl: &Arc<Context>,name: &str, v: f32) {
        self.use_program(gl);
        unsafe { gl.uniform_1_f32(Some(&Self::location(self, gl, name)), v); } 
    }

    pub fn uniform_vec2(&self, gl: &Arc<Context>,name: &str, x: f32, y: f32) {
        self.use_program(gl);
        unsafe { gl.uniform_2_f32(Some(&Self::location(self, gl, name)), x, y); } 
    }

    pub fn uniform_mat3(&self, gl: &Arc<Context>,name: &str, v: &[f32]) {
        self.use_program(gl);
        unsafe { gl.uniform_matrix_3_f32_slice(Some(&Self::location(self, gl, name)), false, v); }
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        unsafe { 
            gl.delete_program(self.program);
        };
    }
}