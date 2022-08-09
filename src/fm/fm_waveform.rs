use std::{
    f32::consts::{PI, TAU},
    mem::MaybeUninit,
};

const LUT_LEN: usize = 64;
const LUT_FULL: usize = LUT_LEN * 4;
static mut LUT: MaybeUninit<[f32; LUT_LEN]> = MaybeUninit::uninit();

pub fn init_fm_lut() {
    unsafe {
        LUT.write(
            (0..LUT_LEN)
                .map(|index| {
                    let phase = (TAU * index as f32) / LUT_FULL as f32;
                    let phase = phase + (PI / LUT_FULL as f32); //Offset it slightly to break symmetry

                    phase.sin()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
    }
}

#[derive(Clone, Copy)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl Quadrant {
    pub fn from_index(index: usize) -> Self {
        if index < LUT_LEN {
            Quadrant::First
        } else if index < LUT_LEN * 2 {
            Quadrant::Second
        } else if index < LUT_LEN * 3 {
            Quadrant::Third
        } else if index < LUT_FULL {
            Quadrant::Fourth
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
pub enum FMWaveform {
    Sine,
    InverseSine,
    HalfSine,
    InverseHalfSine,
    AlternatingSine,
    InverseAlternatingSine,
    CamelSine,
    InveseCamelSine,
}

impl FMWaveform {
    pub fn lookup(self, index: usize) -> f32 {
        match self {
            FMWaveform::Sine => sine_lut(index),
            FMWaveform::InverseSine => inverse_sine_lut(index),
            FMWaveform::HalfSine => half_sine(index),
            FMWaveform::InverseHalfSine => inverse_half_sine(index),
            FMWaveform::AlternatingSine => alternating_sine(index),
            FMWaveform::InverseAlternatingSine => inverse_alternating_sine(index),
            FMWaveform::CamelSine => camel_sine(index),
            FMWaveform::InveseCamelSine => invese_camel_sine(index),
        }
    }
}

fn sine_lut(index: usize) -> f32 {
    let lut = unsafe { LUT.assume_init_ref() };
    let index_mod = index % LUT_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => lut[index_mod],
        Quadrant::Second => lut[LUT_LEN - index_mod - 1],
        Quadrant::Third => -lut[index_mod],
        Quadrant::Fourth => -lut[LUT_LEN - index_mod - 1],
    }
}

fn inverse_sine_lut(index: usize) -> f32 {
    let lut = unsafe { LUT.assume_init_ref() };
    let index_mod = index % LUT_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => 1.0 - lut[LUT_LEN - index_mod - 1],
        Quadrant::Second => 1.0 - lut[index_mod],
        Quadrant::Third => -1.0 + lut[LUT_LEN - index_mod - 1],
        Quadrant::Fourth => -1.0 + lut[index_mod],
    }
}

fn half_sine(index: usize) -> f32 {
    let lut = unsafe { LUT.assume_init_ref() };
    let index_mod = index % LUT_LEN;

    match Quadrant::from_index(index) {
        Quadrant::First => 1.0 - lut[LUT_LEN - index_mod - 1],
        Quadrant::Second => 1.0 - lut[LUT_LEN],
        Quadrant::Third | Quadrant::Fourth => 0.0,
    }
}

fn inverse_half_sine(index: usize) -> f32 {
    todo!()
}
fn alternating_sine(index: usize) -> f32 {
    todo!()
}
fn inverse_alternating_sine(index: usize) -> f32 {
    todo!()
}
fn camel_sine(index: usize) -> f32 {
    todo!()
}
fn invese_camel_sine(index: usize) -> f32 {
    todo!()
}
