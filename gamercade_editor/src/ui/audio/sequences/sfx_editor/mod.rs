use eframe::egui::Ui;

use crate::{
    editor_data::EditorSoundData,
    ui::{AudioList, AudioSyncHelper},
};

mod sfx_list;
use sfx_list::*;

#[derive(Default)]
pub(crate) struct SfxEditor {
    sfx_list: SfxList,
}

impl SfxEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        self.sfx_list.draw(ui, data, sync);
    }
}
