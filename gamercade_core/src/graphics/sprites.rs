use std::ops::Index;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{ColorIndex, SpriteIter, PALETTE_COLORS};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpriteSheetIndex(pub u8);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpriteIndex(pub u8);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpriteSheet {
    pub height: usize,
    pub width: usize,

    #[serde(serialize_with = "ser_sprites", deserialize_with = "de_sprites")]
    pub sprites: Box<[ColorIndex]>,

    pub count: u8,
}

fn de_sprites<'de, D>(deserializer: D) -> Result<Box<[ColorIndex]>, D::Error>
where
    D: Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        let text: String = Deserialize::deserialize(deserializer)?;
        let bytes = base64::decode(&text).map_err(serde::de::Error::custom)?;
        let bytes: Vec<ColorIndex> = unsafe { std::mem::transmute(bytes) };
        Ok(bytes.into_boxed_slice())
    } else {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let bytes: Vec<ColorIndex> = unsafe { std::mem::transmute(bytes) };
        Ok(bytes.into_boxed_slice())
    }
}

fn ser_sprites<S>(sprites: &[ColorIndex], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let sprites: &[u8] = unsafe { std::mem::transmute(sprites) };
    if serializer.is_human_readable() {
        let sprites = base64::encode(sprites);
        serializer.serialize_str(&sprites)
    } else {
        serializer.serialize_bytes(sprites)
    }
}

impl Default for SpriteSheet {
    fn default() -> Self {
        let sprites = (0..PALETTE_COLORS)
            .map(|i| ColorIndex(i as u8))
            .collect::<Vec<ColorIndex>>()
            .into_boxed_slice();

        let dimension = (PALETTE_COLORS as f32).sqrt() as usize;

        Self {
            height: dimension,
            width: dimension,
            sprites,
            count: 1,
        }
    }
}

impl SpriteSheet {
    /// Resizes a sprite setting the new width and height. Will clip
    /// or pad any excess size, using the default 0 index color.
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let total_entries = new_width * new_height * self.count as usize;
        let mut new_sprites = Vec::with_capacity(total_entries);
        let width = self.width;

        self.iter_sprites().for_each(|sprite| {
            let new_sprite = (0..new_height).flat_map(|y| {
                (0..new_width).map(move |x| {
                    if x >= width {
                        Default::default()
                    } else {
                        sprite.get(x + (y * width)).copied().unwrap_or_default()
                    }
                })
            });

            new_sprites.extend(new_sprite);
        });

        self.sprites = new_sprites.into_boxed_slice();
        self.width = new_width;
        self.height = new_height;
    }

    pub(crate) fn step(&self) -> usize {
        self.width * self.height
    }

    pub fn iter_sprites(&self) -> SpriteIter {
        SpriteIter::new(self)
    }

    fn add_sprite(&mut self, index: SpriteIndex, kind: AddKind) {
        let step = self.step();
        let pixel_index = step * index.0 as usize;
        let start = &self.sprites[..(pixel_index + step)];
        let copy = &self.sprites[pixel_index..pixel_index + step];
        let end = &self.sprites[(pixel_index + step)..];

        let mut new_sprites = Vec::with_capacity(step * (self.count as usize + 1));

        // Copy the data before this sprite
        new_sprites.extend(start);

        // Add the copy or empty one
        match kind {
            AddKind::Empty => new_sprites.extend((0..step).map(|_| ColorIndex::default())),
            AddKind::Copy => new_sprites.extend(copy),
            AddKind::Import(source) => new_sprites.extend(source),
        };

        // Copy the remaining data
        new_sprites.extend(end);

        self.sprites = new_sprites.into_boxed_slice();
        self.count += 1;
    }

    /// Inserts a new empty sprite at the given index
    pub fn new_empty(&mut self, index: SpriteIndex) {
        self.add_sprite(index, AddKind::Empty);
    }

    /// Duplicates a sprite at the given index
    pub fn duplicate(&mut self, index: SpriteIndex) {
        self.add_sprite(index, AddKind::Copy);
    }

    pub fn add_new_sprite(&mut self, index: SpriteIndex, sprite: &[ColorIndex]) {
        self.add_sprite(index, AddKind::Import(sprite))
    }

    pub fn delete_sprite(&mut self, sprite_index: SpriteIndex) {
        let step = self.step();
        let mut new_sprites = Vec::with_capacity(step * (self.count as usize + 1));

        self.sprites
            .chunks_exact(step)
            .enumerate()
            .for_each(|(index, slice)| {
                if index != sprite_index.0 as usize {
                    new_sprites.extend_from_slice(slice)
                }
            });

        self.sprites = new_sprites.into_boxed_slice();
        self.count -= 1;
    }
}

impl Index<SpriteIndex> for SpriteSheet {
    type Output = [ColorIndex];

    fn index(&self, index: SpriteIndex) -> &Self::Output {
        let step = self.step();
        let index = index.0 as usize;
        &self.sprites[step * index..step * (index + 1)]
    }
}

enum AddKind<'a> {
    Empty,
    Copy,
    Import(&'a [ColorIndex]),
}
