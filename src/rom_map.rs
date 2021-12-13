use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::core::{ColorIndex, PaletteIndex, SpriteIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RomMap<K, V> {
    data: Box<[V]>,
    _key_type: PhantomData<K>,
}

impl<K: RomKey, V: Copy> RomMap<K, V> {
    pub fn get(&self, index: K) -> V {
        *self.data.get(index.index()).unwrap()
    }
}

pub trait RomKey {
    fn index(&self) -> usize;
}

macro_rules! derive_rom_map {
    ($($name:ty,)*) => {
        $(
            impl RomKey for $name {
            fn index(&self) -> usize {
                self.0 as usize
            }
        })*
    }
}

derive_rom_map! {
    PaletteIndex,
    ColorIndex,
    SpriteIndex,
}
