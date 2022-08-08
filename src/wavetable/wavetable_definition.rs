use super::WavetableBitDepth;

pub struct WavetableDefinition {
    pub(crate) data: Box<[WavetableBitDepth]>,
}

impl WavetableDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
