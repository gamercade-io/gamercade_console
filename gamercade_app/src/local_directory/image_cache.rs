use eframe::egui::{self, ImageSource, TextureHandle};
use nohash_hasher::IntMap;

use crate::GAME_DIR;

pub struct ImageCache {
    pub games: IntMap<i64, TextureHandle>,
}

impl ImageCache {
    pub const fn default_game_image() -> &'static ImageSource<'static> {
        &egui::include_image!("../../default-logo.png")
    }

    pub fn new() -> Self {
        std::fs::create_dir(GAME_DIR).unwrap();
        let dir = std::fs::read_dir(GAME_DIR).unwrap();

        let mut games = IntMap::default();

        for file in dir.into_iter() {
            if let Ok(path) = file.map(|file| file.path()) {
                if let Some(extension) = path.extension() {
                    if extension == "png" {
                        // TODO: Load the image and push it into the map
                    }
                }
            }
        }

        Self { games }
    }
}
