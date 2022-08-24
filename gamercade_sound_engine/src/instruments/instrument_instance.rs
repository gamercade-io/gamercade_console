use gamercade_audio::{get_note, PhraseEntry, PhraseStorageType};

use crate::{
    InstrumentDefinition, InstrumentDefinitionKind, PatchInstance, SamplerInstance,
    SoundRomInstance, WavetableInstance,
};

#[derive(Debug, Clone)]
pub struct InstrumentInstance {
    id: usize,
    kind: InstrumentInstanceKind,
}

#[derive(Debug, Clone)]
pub enum InstrumentInstanceKind {
    Wavetable(WavetableInstance),
    FMSynth(Box<PatchInstance>),
    Sampler(SamplerInstance),
}

pub type InstrumentChannelType = PhraseEntry<f32, InstrumentDefinition>;

pub fn new_instrument_channel_message(
    entry: &PhraseStorageType,
    rom: &SoundRomInstance,
) -> InstrumentChannelType {
    let note = get_note(entry.note).frequency;
    let instrument = rom[entry.instrument].clone();

    InstrumentChannelType {
        note,
        volume: entry.volume,
        instrument,
        effects: entry.effects.clone(),
    }
}

impl InstrumentInstance {
    pub(crate) fn no_sound(output_sample_rate: usize) -> Self {
        Self {
            id: usize::MAX,
            kind: InstrumentInstanceKind::Wavetable(WavetableInstance::no_sound(
                output_sample_rate,
            )),
        }
    }

    pub(crate) fn new_from_instrument(
        source: &InstrumentDefinition,
        output_sample_rate: usize,
    ) -> Self {
        let kind = match &source.kind {
            InstrumentDefinitionKind::Wavetable(wavetable) => InstrumentInstanceKind::Wavetable(
                WavetableInstance::new(wavetable.clone(), output_sample_rate),
            ),
            InstrumentDefinitionKind::FMSynth(fm_synth) => InstrumentInstanceKind::FMSynth(
                Box::new(PatchInstance::new(fm_synth.clone(), output_sample_rate)),
            ),
            InstrumentDefinitionKind::Sampler(sample) => {
                InstrumentInstanceKind::Sampler(SamplerInstance::new(sample, output_sample_rate))
            }
        };

        Self {
            id: source.id,
            kind,
        }
    }

    pub(crate) fn update_from_instrument(&mut self, instrument: &InstrumentDefinition) {
        let output_sample_rate = match &self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.oscillator.output_sample_rate,
            InstrumentInstanceKind::FMSynth(fm) => fm.output_sample_rate(),
            InstrumentInstanceKind::Sampler(sm) => sm.oscillator.output_sample_rate,
        };

        *self = Self::new_from_instrument(instrument, output_sample_rate)
    }

    pub(crate) fn update_from_tracker(&mut self, entry: &InstrumentChannelType) {
        if self.id != entry.instrument.id {
            self.update_from_instrument(&entry.instrument)
        }

        match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wave) => {
                wave.set_frequency(entry.note);
                wave.trigger();
            }
            InstrumentInstanceKind::FMSynth(fm) => {
                fm.set_frequency(entry.note);
                fm.trigger();
            }
            InstrumentInstanceKind::Sampler(sampler) => {
                sampler.set_frequency(entry.note);
                sampler.trigger();
            }
        }
    }

    pub(crate) fn tick(&mut self) -> f32 {
        match &mut self.kind {
            InstrumentInstanceKind::Wavetable(wv) => wv.tick(),
            InstrumentInstanceKind::FMSynth(fm) => fm.tick(),
            InstrumentInstanceKind::Sampler(sm) => sm.tick(),
        }
    }
}
