use std::fs;

use eframe::egui::{self, ColorImage, Context, ImageSource, TextureHandle, TextureOptions};
use image::GenericImageView;
use nohash_hasher::IntMap;

use crate::GAME_DIR;

pub struct ImageCache {
    pub games: IntMap<i64, TextureHandle>,
}

impl ImageCache {
    pub const fn default_game_image() -> &'static ImageSource<'static> {
        &egui::include_image!("../../default-logo.png")
    }

    pub fn new(ctx: &Context) -> Self {
        // Create the dir if it doesn't exist
        match fs::metadata(GAME_DIR).map(|meta| meta.is_dir()) {
            Ok(true) => (),
            _ => fs::create_dir(GAME_DIR).unwrap(),
        }
        let dir = fs::read_dir(GAME_DIR).unwrap();

        let mut games = IntMap::default();

        // Parses all of the images on disk into memory/gpu
        for file in dir.into_iter() {
            if let Ok(path) = file.map(|file| file.path()) {
                if let Some(extension) = path.extension() {
                    let game_id = path.file_name().and_then(|filename| {
                        filename
                            .to_str()
                            .and_then(|filename| filename.parse::<i64>().ok())
                    });

                    if extension == "png" && game_id.is_some() {
                        if let Ok(bytes) = fs::read(path.clone()) {
                            if let Ok(image) = image::load_from_memory(&bytes) {
                                let dimensions =
                                    [image.dimensions().0 as usize, image.dimensions().1 as usize];

                                let handle = ctx.load_texture(
                                    path.to_str().unwrap(),
                                    ColorImage::from_rgba_unmultiplied(
                                        dimensions,
                                        image.as_bytes(),
                                    ),
                                    TextureOptions::default(),
                                );
                                games.insert(game_id.unwrap(), handle);
                            }
                        }
                    }
                }
            }
        }

        Self { games }
    }
}
