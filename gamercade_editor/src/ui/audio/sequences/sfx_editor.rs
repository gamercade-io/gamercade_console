use eframe::egui::Ui;
use gamercade_audio::Sfx;

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Default)]
pub(crate) struct SfxList {
    selected_sfx: usize,
}

impl AudioList<Sfx> for SfxList {
    fn target_data(data: &EditorSoundData) -> &Vec<EditorAudioDataEntry<Sfx>> {
        &data.sfx
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_sfx
    }

    fn name() -> &'static str {
        "Sfx List"
    }

    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        _data: &mut EditorSoundData,
        _sync: &mut AudioSyncHelper,
    ) {
        //TODO: This
        ui.label("TODO: Draw Buttons - Sfx List");
    }
}

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
