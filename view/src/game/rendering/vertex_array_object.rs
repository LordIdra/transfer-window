use std::sync::Arc;

use eframe::glow::{self, LINES};
use glow::{VertexArray, Buffer, Context, HasContext, ARRAY_BUFFER, FLOAT, DYNAMIC_DRAW, TRIANGLES};

pub struct VertexAttribute {
    pub index: u32,
    pub count: i32,
}

impl VertexAttribute {
    pub fn size(&self) -> i32 {
        self.count * std::mem::size_of::<f32>() as i32
    }
}

pub struct VertexArrayObject {
    vertices: i32,
    attributes_per_vertex: i32,
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
}

impl VertexArrayObject {
    pub fn new(gl: &Arc<Context>, vertex_attributes: Vec<VertexAttribute>) -> Self {
        let vertex_array: VertexArray;
        let vertex_buffer: Buffer;
        let attributes_per_vertex = vertex_attributes.iter().map(|attribute| attribute.count).sum();
        let stride = vertex_attributes.iter().map(VertexAttribute::size).sum();

        unsafe { 
            vertex_array = gl.create_vertex_array().expect("Cannot create vertex array");
            vertex_buffer = gl.create_buffer().expect("Cannot create vertex buffer");
            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer));
        }

        let mut offset = 0;
        for attribute in vertex_attributes {
            unsafe { 
                gl.vertex_attrib_pointer_f32(attribute.index, attribute.count, FLOAT, false, stride, offset);
                gl.enable_vertex_attrib_array(attribute.index);
            };
            offset += attribute.size();
        }

        VertexArrayObject { vertices: 0, attributes_per_vertex, vertex_array, vertex_buffer }
    }

    fn bind(&self, gl: &Arc<Context>) {
        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.bind_buffer(ARRAY_BUFFER, Some(self.vertex_buffer));
        }
    }

    pub fn data(&mut self, gl: &Arc<Context>, data: &[f32]) {
        let byte_count = std::mem::size_of_val(data);
        self.vertices = data.len() as i32;
        unsafe {
            let bytes = std::slice::from_raw_parts(data.as_ptr().cast(), byte_count);
            self.bind(gl);
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, DYNAMIC_DRAW);
        }
    }

    pub fn draw(&self, gl: &Arc<Context>) {
        unsafe {
            self.bind(gl);
            gl.draw_arrays(TRIANGLES, 0, self.vertices / self.attributes_per_vertex);
        }
    }

    pub fn draw_lines(&self, gl: &Arc<Context>) {
        unsafe {
            self.bind(gl);
            gl.draw_arrays(LINES, 0, self.vertices / self.attributes_per_vertex);
        }
    }

    pub fn destroy(&mut self, gl: &Arc<Context>) {
        unsafe {
            gl.delete_vertex_array(self.vertex_array);
            gl.delete_buffer(self.vertex_buffer);
        }
    }
}