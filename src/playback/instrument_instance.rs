use rodio::Source;
use rtrb::{Consumer, PopError};

use crate::{
    Instrument, InstrumentChannelType, InstrumentKind, PatchInstance, SamplerInstance,
    WavetableInstance,
};

pub struct InstrumentInstance {
    pub consumer: Consumer<InstrumentChannelType>,
    pub output_sample_rate: usize,
    pub instance_type: InstrumentInstanceType,
}

impl InstrumentInstance {
    pub fn no_sound(consumer: Consumer<InstrumentChannelType>, output_sample_rate: usize) -> Self {
        Self {
            consumer,
            instance_type: InstrumentInstanceType::Wavetable(WavetableInstance::no_sound(
                output_sample_rate,
            )),
            output_sample_rate,
        }
    }
}

impl Iterator for InstrumentInstance {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.consumer.pop() {
                Err(PopError::Empty) => break,
                Ok(next) => {
                    self.instance_type
                        .update_from_channel(next, self.output_sample_rate);
                }
            }
        }

        Iterator::next(&mut self.instance_type)
    }
}

impl Source for InstrumentInstance {
    fn current_frame_len(&self) -> Option<usize> {
        Some(1)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.output_sample_rate as u32
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum InstrumentInstanceType {
    Wavetable(WavetableInstance),
    FMSynth(Box<PatchInstance>),
    Sampler(SamplerInstance),
}

impl InstrumentInstanceType {
    pub(crate) fn from_instrument(source: &Instrument, output_sample_rate: usize) -> Self {
        match source {
            Instrument::Wavetable(wavetable) => InstrumentInstanceType::Wavetable(
                WavetableInstance::new(wavetable.clone(), output_sample_rate),
            ),
            Instrument::FMSynth(fm_synth) => InstrumentInstanceType::FMSynth(Box::new(
                PatchInstance::new(fm_synth.clone(), output_sample_rate),
            )),
            Instrument::Sampler(sample) => {
                InstrumentInstanceType::Sampler(SamplerInstance::new(sample, output_sample_rate))
            }
        }
    }

    pub(crate) fn update_from_channel(
        &mut self,
        entry: InstrumentChannelType,
        output_sample_rate: usize,
    ) {
        if self.get_type() != entry.instrument.get_type() {
            *self = Self::from_instrument(&entry.instrument, output_sample_rate)
        }

        match self {
            InstrumentInstanceType::Wavetable(wave) => {
                wave.set_frequency(entry.note);
                wave.trigger();
            }
            InstrumentInstanceType::FMSynth(fm) => {
                fm.set_frequency(entry.note);
                fm.trigger();
            }
            InstrumentInstanceType::Sampler(sampler) => {
                sampler.set_frequency(entry.note);
                sampler.trigger();
            }
        }
    }

    pub(crate) fn get_type(&self) -> InstrumentKind {
        match self {
            InstrumentInstanceType::Wavetable(_) => InstrumentKind::Wavetable,
            InstrumentInstanceType::FMSynth(_) => InstrumentKind::FMSynth,
            InstrumentInstanceType::Sampler(_) => InstrumentKind::Sampler,
        }
    }
}

// impl From<&Instrument> for InstrumentInstanceType {
//     fn from(source: &Instrument) -> Self {
//         match source {
//             Instrument::Wavetable(wavetable) => {
//                 InstrumentInstanceType::Wavetable(WavetableOscilator::new(wavetable.clone()))
//             }
//             Instrument::FMSynth(fm_synth) => {
//                 InstrumentInstanceType::FMSynth(Box::new(PatchInstance::new(fm_synth.clone())))
//             }
//         }
//     }
// }

impl Iterator for InstrumentInstanceType {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            match self {
                InstrumentInstanceType::Wavetable(wave) => wave.tick(),
                InstrumentInstanceType::FMSynth(fm) => fm.tick(),
                InstrumentInstanceType::Sampler(sampler) => sampler.tick(),
            } * 0.15,
        )
    }
}

impl Source for InstrumentInstanceType {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        match self {
            InstrumentInstanceType::Wavetable(wave) => wave.oscillator.output_sample_rate as u32,
            InstrumentInstanceType::FMSynth(fm) => fm.output_sample_rate() as u32,
            InstrumentInstanceType::Sampler(sample) => sample.oscillator.output_sample_rate as u32,
        }
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
