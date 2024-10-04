use std::{collections::HashSet, fs::{self}, path::Path, process::Command, thread};

use serde::{Deserialize, Serialize};
use sha256::digest;

const RESOURCE_CACHE_LOCATION: &str = "resources/cache.json";
const INPUT_DIRECTORY: &str = "resources/textures/";
const OUTPUT_DIRECTORY: &str = "resources/final_textures/";

#[derive(Serialize, Deserialize, Default)]
struct ResourceCache {
    hashes: HashSet<String>,
}

impl ResourceCache {
    pub fn load() -> Self {
        let file = fs::read_to_string(RESOURCE_CACHE_LOCATION).expect("Failed to load cache");
        serde_json::from_str(file.as_str()).expect("Failed to deserialize cache")
    }

    pub fn save(&self) {
        let serialized = serde_json::to_string(self).expect("Failed to serialize cache");
        fs::write(RESOURCE_CACHE_LOCATION, serialized).expect("Failed to save cache");
    }

    pub fn has(&self, filepath: &str) -> bool {
        if !Path::new(filepath).exists() {
            return false;
        }

        let bytes = fs::read(filepath).expect("Failed to read file for hashing");
        let hash = digest(bytes);
        self.hashes.contains(&hash)
    }

    pub fn add(&mut self, filepath: &str) {
        let bytes = fs::read(filepath).expect("Failed to read file for hashing");
        let hash = digest(bytes);
        self.hashes.insert(hash);
    }
}

fn read_input(path: &str) -> Vec<String> {
    let directory = INPUT_DIRECTORY.to_string() + path;
    let files = fs::read_dir(&directory).unwrap_or_else(|_| panic!("Failed to read directory {directory}"));
    let mut names = vec![];
    for file in files {
        let name = file.expect("Failed to read file").file_name().to_str().expect("Failed to get OS string as string").to_string();
        names.push(name);
    }
    names
}

fn copy(from: &str, to: &str) {
    Command::new("cp")
        .arg(from)
        .arg(to)
        .output()
        .unwrap();
}

fn export_drawio(from: &str, to: &str, scale: f64) {
    Command::new("drawio")
        .arg("--export")
        .arg("--scale")
        .arg(format!("{scale:.1}"))
        .arg("--transparent")
        .arg("--output")
        .arg(to)
        .arg(from)
        .output()
        .unwrap();
}

fn bloom(path: &str, amount: usize) {
    let blur_output = path.to_string() + ".blur";

    Command::new("convert")
        .arg(path)
        .arg("-channel")
        .arg("RBGA")
        .arg("-gaussian-blur")
        .arg(format!("0x{amount}"))
        .arg(&blur_output)
        .output()
        .unwrap();

    Command::new("convert")
        .arg(path)
        .arg(&blur_output)
        .arg("-compose")
        .arg("screen")
        .arg("-composite")
        .arg(path)
        .output()
        .unwrap();

    Command::new("rm")
        .arg(&blur_output)
        .output()
        .unwrap();
}

fn celestial_object(cache: &ResourceCache, new_cache: &mut ResourceCache) {
    for name in read_input("celestial_object") {
        let from = INPUT_DIRECTORY.to_string() + "celestial_object/" + &name;
        let to = OUTPUT_DIRECTORY.to_string() + &name;

        new_cache.add(&from);

        if cache.has(&from) {
            copy(&from, &to);
        }
    }
}

fn character(cache: &ResourceCache, new_cache: &mut ResourceCache) {
    let mut handles = vec![];

    for name in read_input("character") {
        assert!(name.contains("drawio"));

        let from = INPUT_DIRECTORY.to_string() + "character/" + &name;
        let to = OUTPUT_DIRECTORY.to_string() + &name.replace("drawio", "png");

        new_cache.add(&from);

        if !cache.has(&from) {
            let from_clone = from.clone();
            handles.push(thread::spawn(move || {
                export_drawio(&from_clone, &to, 2.0);
                bloom(&to, 8);
            }));
        }
    }

    for handle in handles {
        handle.join().expect("Failed to join handle");
    }
}

fn icon(cache: &ResourceCache, new_cache: &mut ResourceCache) {
    for name in read_input("icon") {
        let from = INPUT_DIRECTORY.to_string() + "icon/" + &name;
        let to = OUTPUT_DIRECTORY.to_string() + &name;

        new_cache.add(&from);

        if !cache.has(&from) {
            copy(&from, &to);
        }
    }
}

fn menu(cache: &ResourceCache, new_cache: &mut ResourceCache) {
    let mut handles = vec![];

    for name in read_input("menu") {
        assert!(name.contains("drawio"));

        let from = INPUT_DIRECTORY.to_string() + "menu/" + &name;
        let to = OUTPUT_DIRECTORY.to_string() + &name.replace("drawio", "png");

        new_cache.add(&from);

        if !cache.has(&from) {
            handles.push(thread::spawn(move || {
                export_drawio(&from, &to, 2.0);
                bloom(&to, 8);
            }));
        }
    }

    for handle in handles {
        handle.join().expect("Failed to join handle");
    }
}

fn ship(cache: &ResourceCache, new_cache: &mut ResourceCache) {
    let mut handles = vec![];

    for name in read_input("ship") {
        assert!(name.contains("drawio"));

        let from = INPUT_DIRECTORY.to_string() + "ship/" + &name;
        let to = OUTPUT_DIRECTORY.to_string() + &name.replace("drawio", "png");

        new_cache.add(&from);

        if !cache.has(&from) {
            handles.push(thread::spawn(move || {
                export_drawio(&from, &to, 2.0);
                bloom(&to, 8);
            }));
        }
    }

    for handle in handles {
        handle.join().expect("Failed to join handle");
    }
}

pub fn main() {
    let cache = ResourceCache::load();
    let mut new_cache = ResourceCache::default();

    icon(&cache, &mut new_cache);
    celestial_object(&cache, &mut new_cache);
    character(&cache, &mut new_cache);
    menu(&cache, &mut new_cache);
    ship(&cache, &mut new_cache);

    new_cache.save();
}
