use std::{collections::HashMap, fs::{self, DirEntry}, sync::{Arc, Mutex}};

use eframe::{egui::ImageSource, glow};
use glow::Context;
use image::GenericImageView;

use crate::game::rendering::texture;

use super::rendering::{texture_renderer::TextureRenderer, object_renderer::ObjectRenderer};

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
    size: (i32, i32),
    bytes: Vec<u8>,
    image: ImageSource<'static>,
    gl_texture: Option<texture::Texture>,
}

impl Texture {
    fn new(entry: &DirEntry) -> Self {
        let uri = "file://".to_owned() + entry.path().as_path().as_os_str().to_str().unwrap();
        let bytes = fs::read(entry.path()).expect("Failed to load file");
        let image = image::load_from_memory(&bytes).expect("Failed to load image");
        let size = (image.dimensions().0 as i32, image.dimensions().1 as i32);
        let bytes = image.to_rgba8().into_vec();
        let image = ImageSource::Uri(uri.into());
        let gl_texture = None;
        Texture { size, bytes, image, gl_texture }
    }
}

/// Loads all textures in the resources folder upon initialization
pub struct Resources {
    texture_names: Vec<String>,
    object_names: Vec<String>,
    textures: HashMap<String, Texture>,
}

impl Resources {
    pub fn new() -> Self {
        let texture_names = directory_entries("view/resources/textures/icon".to_string())
            .into_iter()
            .map(|entry| entry_name(&entry))
            .collect();
        let object_names = directory_entries("view/resources/textures/planet".to_string())
            .into_iter()
            .map(|entry| entry_name(&entry))
            .collect();
        let textures = directory_entries("view/resources/textures".to_string())
            .into_iter()
            .flat_map(|entry| entry.path().read_dir().unwrap())
            .map(|entry| entry.unwrap())
            .map(|entry| (entry_name(&entry), entry))
            .map(|(name, texture)| (name, Texture::new(&texture)))
            .collect();
        Resources { texture_names, object_names, textures }
    }

    pub fn build_texture_renderers(&mut self, gl: &Arc<Context>) -> HashMap<String, Arc<Mutex<TextureRenderer>>> {
        let mut texture_renderers = HashMap::new();
        for texture_name in &self.texture_names().clone() {
            let texture = self.gl_texture(gl.clone(), texture_name.as_str());
            let texture_renderer = TextureRenderer::new(gl.clone(), texture.clone());
            texture_renderers.insert(texture_name.to_string(), Arc::new(Mutex::new(texture_renderer)));
        }
        texture_renderers
    }
    
    pub fn build_object_renderers(&mut self, gl: &Arc<Context>) -> HashMap<String, Arc<Mutex<ObjectRenderer>>> {
        let mut planet_renderers = HashMap::new();
        for texture_name in &self.texture_names().clone() {
            let texture = self.gl_texture(gl.clone(), texture_name.as_str());
            let planet_renderer = ObjectRenderer::new(gl.clone(), texture.clone());
            planet_renderers.insert(texture_name.to_string(), Arc::new(Mutex::new(planet_renderer)));
        }
        planet_renderers
    }

    pub fn texture_names(&self) -> &Vec<String> {
        &self.texture_names
    }

    #[allow(unused)]
    pub fn texture_image(&self, name: &str) -> ImageSource {
        self.textures.get(name).unwrap_or_else(|| panic!("Texture {name} does not exist")).image.clone()
    }

    pub fn gl_texture(&mut self, gl: Arc<Context>, name: &str) -> &texture::Texture {
        let texture = self.textures.get_mut(name).unwrap_or_else(|| panic!("Texture {name} does not exist"));
        if texture.gl_texture.is_none() {
            texture.gl_texture = Some(texture::Texture::new(gl, texture.size, &texture.bytes));
        }
        texture.gl_texture.as_ref().unwrap()
    }
}