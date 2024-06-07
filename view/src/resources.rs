use std::{collections::HashMap, fs::{self, DirEntry}, sync::{Arc, Mutex}};

use eframe::{egui::{self, ImageSource}, glow};
use image::GenericImageView;

use crate::rendering::texture;

use super::rendering::texture_renderer::TextureRenderer;

fn directory_entries(directory: String) -> Vec<DirEntry> {
    fs::read_dir(directory)
        .expect("Failed to read directory")
        .map(|entry| entry.expect("Failed to read file"))
        .collect()
}

fn entry_name(entry: &DirEntry) -> String {
    entry.file_name().into_string().unwrap().split('.').next().unwrap().to_string()
}

struct Texture {
    image: ImageSource<'static>,
    gl_texture: texture::Texture,
}

impl Texture {
    fn new(context: &egui::Context, gl: &Arc<glow::Context>, entry: &DirEntry) -> Self {
        let uri = "file://".to_owned() + entry.path().as_path().as_os_str().to_str().unwrap();
        let bytes = fs::read(entry.path()).expect("Failed to load file");
        let image = image::load_from_memory(&bytes).expect("Failed to load image");
        let size = (image.dimensions().0 as i32, image.dimensions().1 as i32);
        let bytes = image.to_rgba8().into_vec();
        let image = ImageSource::Uri(uri.clone().into());
        let gl_texture = texture::Texture::new(gl, size, &bytes);
        context.try_load_bytes(&uri).expect("Failed to load texture");
        Texture { image, gl_texture }
    }
}

/// Loads all textures in the resources folder upon initialization
pub struct Resources {
    textures: HashMap<String, Texture>,
}

impl Resources {
    pub fn new(context: &egui::Context, gl: &Arc<glow::Context>) -> Self {
        let textures = directory_entries("view/resources/textures".to_string())
            .into_iter()
            .map(|entry| (entry_name(&entry), entry))
            .map(|entry| (entry.0, Texture::new(context, gl, &entry.1)))
            .collect();
        Resources { textures }
    }

    /// # Panics
    /// Panics if the texture does not exist
    #[allow(unused)] pub fn texture_image(&self, name: &str) -> ImageSource {
        self.textures.get(name)
            .unwrap_or_else(|| panic!("Texture {name} does not exist"))
            .image
            .clone()
    }

    /// # Panics
    /// Panics if the texture does not exist
    #[allow(unused)]
    pub fn gl_texture(&self, name: &str) -> glow::Texture {
        self.textures.get(name)
            .unwrap_or_else(|| panic!("Texture {name} does not exist"))
            .gl_texture
            .texture()
    }

    pub fn build_renderers(&self, gl: &Arc<glow::Context>) -> HashMap<String, Arc<Mutex<TextureRenderer>>> {
        let mut texture_renderers = HashMap::new();
        for (texture_name, texture) in &self.textures {
            let texture_renderer = TextureRenderer::new(gl, texture.gl_texture.texture());
            texture_renderers.insert(texture_name.to_string(), Arc::new(Mutex::new(texture_renderer)));
        }
        texture_renderers
    }

    pub fn destroy(&self, gl: &Arc<glow::Context>) {
        for texture in self.textures.values() {
            texture.gl_texture.destroy(gl);
        }
    }
}