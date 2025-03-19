use macroquad::prelude::*;
use std::collections::HashMap;

pub struct GameTextures {
    textures: HashMap<String, Texture2D>,
}

impl GameTextures {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Texture2D> {
        self.textures.get(key)
    }

    pub async fn load_menu_assets(&mut self) -> bool {
        match load_texture("assets/Background/warchild_full.png").await {
            Ok(texture) => {
                self.textures.insert("menu_background".to_string(), texture);
                true
            }
            Err(err) => {
                eprintln!("Error: Failed to load menu background: {}", err);
                false
            }
        }
    }

    pub async fn add_texture(&mut self, key: &str, path: &str) -> bool {
        match load_texture(path).await {
            Ok(texture) => {
                self.textures.insert(key.to_string(), texture);
                true
            }
            Err(err) => {
                eprintln!("Error: Failed to load texture {}: {}", key, err);
                false
            }
        }
    }
}
