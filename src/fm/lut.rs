use std::{
    f32::consts::{PI, TAU},
    mem::MaybeUninit,
};

const LUT_LEN: usize = 64;
const LUT_FULL: usize = LUT_LEN * 4;
static mut LUT: MaybeUninit<[f32; LUT_LEN]> = MaybeUninit::uninit();

pub fn init_lut() {
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

pub fn lut_sin(index: usize) -> f32 {
    // TODO: Consider adding lerping
    lut_lookup(index)
}

fn lut_lookup(index: usize) -> f32 {
    let lut = unsafe { LUT.assume_init_ref() };
    let index_mod = index % LUT_LEN;

    if index < LUT_LEN {
        lut[index_mod]
    } else if index < LUT_LEN * 2 {
        lut[LUT_LEN - index_mod - 1]
    } else if index < LUT_LEN * 3 {
        -lut[index_mod]
    } else if index < LUT_FULL {
        -lut[LUT_LEN - index_mod - 1]
    } else {
        unreachable!()
    }
}
