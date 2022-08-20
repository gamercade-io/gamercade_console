use crate::{
    Instrument, InstrumentChannelType, InstrumentKind, PatchInstance, SamplerInstance,
    WavetableInstance,
};

#[derive(Debug, Clone)]
pub enum InstrumentInstance {
    Wavetable(WavetableInstance),
    FMSynth(Box<PatchInstance>),
    Sampler(SamplerInstance),
}

impl InstrumentInstance {
    pub(crate) fn no_sound(output_sample_rate: usize) -> Self {
        Self::Wavetable(WavetableInstance::no_sound(output_sample_rate))
    }

    pub(crate) fn new_from_instrument(source: &Instrument, output_sample_rate: usize) -> Self {
        match source {
            Instrument::Wavetable(wavetable) => InstrumentInstance::Wavetable(
                WavetableInstance::new(wavetable.clone(), output_sample_rate),
            ),
            Instrument::FMSynth(fm_synth) => InstrumentInstance::FMSynth(Box::new(
                PatchInstance::new(fm_synth.clone(), output_sample_rate),
            )),
            Instrument::Sampler(sample) => {
                InstrumentInstance::Sampler(SamplerInstance::new(sample, output_sample_rate))
            }
        }
    }

    pub(crate) fn update_from_instrument(&mut self, instrument: &Instrument) {
        let output_sample_rate = match self {
            InstrumentInstance::Wavetable(wv) => wv.oscillator.output_sample_rate,
            InstrumentInstance::FMSynth(fm) => fm.output_sample_rate(),
            InstrumentInstance::Sampler(sm) => sm.oscillator.output_sample_rate,
        };

        *self = Self::new_from_instrument(instrument, output_sample_rate)
    }

    pub(crate) fn update_from_tracker(&mut self, entry: &InstrumentChannelType) {
        if self.get_type() != entry.instrument.get_type() {
            self.update_from_instrument(&entry.instrument)
        }

        match self {
            InstrumentInstance::Wavetable(wave) => {
                wave.set_frequency(entry.note);
                wave.trigger();
            }
            InstrumentInstance::FMSynth(fm) => {
                fm.set_frequency(entry.note);
                fm.trigger();
            }
            InstrumentInstance::Sampler(sampler) => {
                sampler.set_frequency(entry.note);
                sampler.trigger();
            }
        }
    }

    pub(crate) fn get_type(&self) -> InstrumentKind {
        match self {
            InstrumentInstance::Wavetable(_) => InstrumentKind::Wavetable,
            InstrumentInstance::FMSynth(_) => InstrumentKind::FMSynth,
            InstrumentInstance::Sampler(_) => InstrumentKind::Sampler,
        }
    }

    pub(crate) fn tick(&mut self) -> f32 {
        match self {
            InstrumentInstance::Wavetable(wv) => wv.tick(),
            InstrumentInstance::FMSynth(fm) => fm.tick(),
            InstrumentInstance::Sampler(sm) => sm.tick(),
        }
    }
}
