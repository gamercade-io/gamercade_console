use eframe::egui::Ui;
use gamercade_audio::InstrumentDataDefinition;

use crate::editor_data::EditorSoundData;

use super::AudioSyncHelper;

mod envelope_widget;
mod fm_editor;
mod instrument_list;
mod piano_roll;
mod sampler_editor;
mod wavetable_editor;

use fm_editor::*;
use instrument_list::*;
use piano_roll::*;
use sampler_editor::*;
use wavetable_editor::*;

#[derive(Clone, Default)]
pub struct InstrumentEditor {
    fm_editor: FMEditor,
    wavetable_editor: WavetableEditor,
    sampler_editor: SamplerEditor,

    instrument_list: InstrumentList,
    piano_roll: PianoRoll,
}

impl InstrumentEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        self.instrument_list.draw(ui, data, sync);

        // Now we need to determine which instrument kind we are currenty editing
        let index = self.instrument_list.selected_instrument;

        if let Some(instrument) = data.instruments.get_mut(index) {
            match &mut instrument.data {
                InstrumentDataDefinition::Wavetable(wv) => self.wavetable_editor.draw(ui, wv, sync),
                InstrumentDataDefinition::FMSynth(fm) => self.fm_editor.draw(ui, fm, sync),
                InstrumentDataDefinition::Sampler(sm) => self.sampler_editor.draw(ui, sm, sync),
            }
        } else {
            println!("InstrumentEditor: selected_index is invalid")
        }

        self.piano_roll
            .draw(ui, sync, self.instrument_list.selected_instrument);
    }
}
