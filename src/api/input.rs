use crate::{f32_to_option, i32_bool_to_option, raw};

use paste::paste;

macro_rules! derive_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
    ) => {
        paste! {
            // BUTTON MACRO
            $(
                pub fn [<button_ $btn_name _pressed>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<button_ $btn_name _pressed>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                pub fn [<button_ $btn_name _released>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<button_ $btn_name _released>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                pub fn [<button_ $btn_name _held>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<button_ $btn_name _held>](player_id as i32) };
                    i32_bool_to_option(val)
                }
            )*
            // END BUTTON MACRO

            // ANALOG MACRO
            $(
                pub fn [<analog_ $anlg_name _x>](player_id: usize) -> Option<f32> {
                    let val = unsafe { raw::[<analog_ $anlg_name _x>](player_id as i32) };
                    f32_to_option(val)
                }

                pub fn [<analog_ $anlg_name _y>](player_id: usize) -> Option<f32> {
                    let val = unsafe { raw::[<analog_ $anlg_name _y>](player_id as i32) };
                    f32_to_option(val)
                }
            )*
            // END ANALOG MACRO

            // TRIGGER MACRO
            $(
                pub fn [<trigger_ $trg_name>](player_id: usize) -> Option<f32>{
                    let val = unsafe { raw::[<trigger_ $trg_name>](player_id as i32) };
                    f32_to_option(val)
                }
            )*
            // END TRIGGER MACRO
        }
    };
}

/// TODO: Write the docs
pub fn raw_input_state(player_id: usize) -> i64 {
    // TODO: This could check for an invalid state, like UP + DOWN + LEFT + RIGHT all pressed
    // at once, which is not possible
    unsafe { raw::raw_input_state(player_id as i32) }
}

derive_input_api! {
    Buttons {
        a,
        b,
        c,
        d,
        up,
        down,
        left,
        right,
        start,
        select,
        left_shoulder,
        right_shoulder,
        left_stick,
        right_stick,
        left_trigger,
        right_trigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
}
