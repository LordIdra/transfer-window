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
    fn new(gl: Arc<glow::Context>, context: &egui::Context, entry: &DirEntry) -> Self {
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

fn build_renderers(textures: &HashMap<String, Texture>, gl: &Arc<glow::Context>) -> HashMap<String, Arc<Mutex<TextureRenderer>>> {
    let mut texture_renderers = HashMap::new();
    for texture_name in textures.keys() {
        let texture = textures.get(texture_name)
            .unwrap_or_else(|| panic!("Texture {texture_name} does not exist"))
            .gl_texture
            .clone();
        let texture_renderer = TextureRenderer::new(gl.clone(), texture);
        texture_renderers.insert(texture_name.to_string(), Arc::new(Mutex::new(texture_renderer)));
    }
    texture_renderers
}

/// Loads all textures in the resources folder upon initialization
pub struct Resources {
    textures: HashMap<String, Texture>,
    renderers: HashMap<String, Arc<Mutex<TextureRenderer>>>,
}

impl Resources {
    pub fn new(gl: &Arc<glow::Context>, context: &egui::Context) -> Self {
        let textures = directory_entries("view/resources/textures".to_string())
            .into_iter()
            .map(|entry| (entry_name(&entry), entry))
            .map(|entry| (entry.0, Texture::new(gl.clone(), context, &entry.1)))
            .collect();
        let renderers = build_renderers(&textures, gl);
        Resources { textures, renderers }
    }

    /// # Panics
    /// Panics if the texture does not exist
    #[allow(unused)]
    pub fn texture_image(&self, name: &str) -> ImageSource {
        self.textures.get(name)
            .unwrap_or_else(|| panic!("Texture {name} does not exist"))
            .image
            .clone()
    }
    
    pub fn renderers(&self) -> &HashMap<String, Arc<Mutex<TextureRenderer>>> {
        &self.renderers
    }

    pub fn renderer(&self, name: &str) -> Option<Arc<Mutex<TextureRenderer>>> {
        self.renderers.get(name).cloned()
    }
}