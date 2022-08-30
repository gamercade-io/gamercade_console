use eframe::egui::Ui;
use gamercade_audio::InstrumentDataDefinition;

use crate::{editor_data::EditorAudioDataEntry, ui::AudioSyncHelper};

use super::KeyboardMode;

#[derive(Clone, Default)]
pub(crate) struct InstrumentTopPanel {}

impl InstrumentTopPanel {
    pub(crate) fn draw(
        &self,
        ui: &mut Ui,
        instrument: &mut EditorAudioDataEntry<InstrumentDataDefinition>,
        _sync: &mut AudioSyncHelper,
        keyboard_mode: &mut KeyboardMode,
    ) {
        ui.group(|ui| {
            ui.label("Instrument Top Panel");
            ui.horizontal(|ui| {
                ui.label("Instrument Name: ");
                if ui.text_edit_singleline(&mut instrument.name).has_focus() {
                    *keyboard_mode = KeyboardMode::Normal
                } else {
                    *keyboard_mode = KeyboardMode::PianoRoll
                };
            });

            // TODO: Add something about selecting/change instrument types
        });
    }
}
