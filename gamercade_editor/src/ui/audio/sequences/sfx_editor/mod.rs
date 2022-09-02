use eframe::egui::{Key, Slider, Ui};

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

// TODO:
// This UI is really basic.
// It might be nice to also show other Chains via clickable UI or something
// instead of the slider

impl SfxEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.sfx_list.draw(ui, data, sync);

        if let Some(selected_sfx) = data.sfx.get_mut(self.sfx_list.selected_sfx) {
            ui.label("Chain Index");
            if ui
                .add(Slider::new(
                    &mut selected_sfx.data.chain.0,
                    0..=data.chains.len() - 1,
                ))
                .changed()
            {
                sync.notify_rom_changed()
            }

            ui.label("Bpm: ");
            if ui
                .add(Slider::new(&mut selected_sfx.data.bpm, 0.0..=999.99))
                .changed()
            {
                sync.notify_rom_changed()
            }

            if ui.button("Play").clicked() || ui.input().key_pressed(Key::Space) {
                sync.play_sfx(selected_sfx.data.clone());
            }

            if ui.button("Stop").clicked() {
                sync.stop_sfx();
            }
        } else {
            ui.label("No Sfx exist! Please create one.");
        }
    }
}
