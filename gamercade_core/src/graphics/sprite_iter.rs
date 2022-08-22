use crate::{ColorIndex, SpriteIndex, SpriteSheet};

pub struct SpriteIter<'a> {
    index: u8,
    sheet: &'a SpriteSheet,
}

impl<'a> SpriteIter<'a> {
    pub fn new(sheet: &'a SpriteSheet) -> Self {
        Self { index: 0, sheet }
    }
}

impl<'a> Iterator for SpriteIter<'a> {
    type Item = &'a [ColorIndex];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.sheet.count {
            let out = &self.sheet[SpriteIndex(self.index)];
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}
