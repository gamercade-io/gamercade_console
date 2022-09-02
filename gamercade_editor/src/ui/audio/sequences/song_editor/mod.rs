use eframe::egui::Ui;

mod song_list;
use song_list::*;

use crate::{
    editor_data::EditorSoundData,
    ui::{AudioList, AudioSyncHelper},
};

#[derive(Default)]
pub(crate) struct SongEditor {
    song_list: SongList,
}

impl SongEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        self.song_list.draw(ui, data, sync);
    }
}
