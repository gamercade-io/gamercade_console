use rodio::Source;

use crate::{notes, Instrument, PatchInstance, PhraseEntry, WavetableOscilator};

pub enum InstrumentInstance {
    Wavetable(WavetableOscilator),
    FMSynth(PatchInstance),
}

impl InstrumentInstance {
    pub(crate) fn update_from_phrase_entry(&mut self, entry: &PhraseEntry) {
        // TODO: Set volume
        // TODO: Set effects
        match self {
            InstrumentInstance::Wavetable(wave) => {
                wave.set_frequency(notes::get_note(entry.note).frequency);
                wave.trigger();
            }
            InstrumentInstance::FMSynth(fm) => {
                fm.set_frequency(notes::get_note(entry.note).frequency);
                fm.trigger();
            }
        }
    }
}

impl From<&Instrument> for InstrumentInstance {
    fn from(source: &Instrument) -> Self {
        match source {
            Instrument::Wavetable(wavetable) => {
                InstrumentInstance::Wavetable(WavetableOscilator::new(wavetable.clone()))
            }
            Instrument::FMSynth(fm_synth) => {
                InstrumentInstance::FMSynth(PatchInstance::new(fm_synth.clone()))
            }
        }
    }
}

impl Iterator for InstrumentInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            match self {
                InstrumentInstance::Wavetable(wave) => wave.tick(),
                InstrumentInstance::FMSynth(fm) => fm.tick(),
            } * 0.15,
        )
    }
}

impl Source for InstrumentInstance {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        match self {
            InstrumentInstance::Wavetable(wave) => wave.sample_rate(),
            InstrumentInstance::FMSynth(fm) => fm.sample_rate(),
        }
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
