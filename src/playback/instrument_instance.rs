use rodio::Source;
use rtrb::{Consumer, PopError};

use crate::{Instrument, InstrumentChannelType, InstrumentKind, PatchInstance, WavetableOscilator};

pub struct InstrumentInstance {
    pub consumer: Consumer<InstrumentChannelType>,
    pub instance_type: InstrumentInstanceType,
}

impl InstrumentInstance {
    pub fn no_sound(consumer: Consumer<InstrumentChannelType>) -> Self {
        Self {
            consumer,
            instance_type: InstrumentInstanceType::Wavetable(WavetableOscilator::no_sound()),
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
                    self.instance_type.update_from_channel(next);
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
        self.instance_type.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum InstrumentInstanceType {
    Wavetable(WavetableOscilator),
    FMSynth(Box<PatchInstance>),
}

impl InstrumentInstanceType {
    pub(crate) fn update_from_channel(&mut self, entry: InstrumentChannelType) {
        if self.get_type() != entry.instrument.get_type() {
            *self = Self::from(&entry.instrument)
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
        }
    }

    pub(crate) fn get_type(&self) -> InstrumentKind {
        match self {
            InstrumentInstanceType::Wavetable(_) => InstrumentKind::Wavetable,
            InstrumentInstanceType::FMSynth(_) => InstrumentKind::FMSynth,
        }
    }
}

impl From<&Instrument> for InstrumentInstanceType {
    fn from(source: &Instrument) -> Self {
        match source {
            Instrument::Wavetable(wavetable) => {
                InstrumentInstanceType::Wavetable(WavetableOscilator::new(wavetable.clone()))
            }
            Instrument::FMSynth(fm_synth) => {
                InstrumentInstanceType::FMSynth(Box::new(PatchInstance::new(fm_synth.clone())))
            }
        }
    }
}

impl Iterator for InstrumentInstanceType {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            match self {
                InstrumentInstanceType::Wavetable(wave) => wave.tick(),
                InstrumentInstanceType::FMSynth(fm) => fm.tick(),
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
            InstrumentInstanceType::Wavetable(wave) => wave.sample_rate(),
            InstrumentInstanceType::FMSynth(fm) => fm.sample_rate(),
        }
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
