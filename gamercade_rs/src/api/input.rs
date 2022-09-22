#![allow(missing_docs)]
use super::{f32_to_option, i32_bool_to_option};
use crate::raw;

use paste::paste;

#[derive(Clone, Copy)]
pub struct RawInputState(pub i64);

impl RawInputState {
    pub fn is_valid(self) -> bool {
        self.0 & 1 << 63 == 0
    }
}

#[derive(Clone, Copy)]
pub struct RawMouseState(pub i32);

impl RawMouseState {
    pub fn is_valid(self) -> bool {
        self.0 >= 0
    }
}

macro_rules! derive_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse { $($mouse_name:ident,)* },
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

            // MOUSE MACRO
            $(
                pub fn [<mouse_ $mouse_name _pressed>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_ $mouse_name _pressed>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                pub fn [<mouse_ $mouse_name _released>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_ $mouse_name _released>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                pub fn [<mouse_ $mouse_name _held>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_ $mouse_name _held>](player_id as i32) };
                    i32_bool_to_option(val)
                }
            )*
            // END MOUSE MACRO

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

/// Returns a raw input state. If the player_id is invalid, returns None.
pub fn raw_input_state(player_id: usize) -> Option<RawInputState> {
    unsafe {
        let raw: RawInputState = std::mem::transmute(raw::raw_input_state(player_id as i32));
        if raw.is_valid() {
            Some(raw)
        } else {
            None
        }
    }
}

/// Returns a raw mouse state. If the player_id is invalid, returns None.
pub fn raw_mouse_state(player_id: usize) -> Option<RawMouseState> {
    unsafe {
        let raw: RawMouseState = std::mem::transmute(raw::raw_mouse_state(player_id as i32));
        if raw.is_valid() {
            Some(raw)
        } else {
            None
        }
    }
}

/// Returns the mouse's x coordinate. If the player_id is invalid, returns None.
pub fn mouse_x(player_id: usize) -> Option<usize> {
    let val = unsafe { raw::mouse_x(player_id as i32) };
    val.try_into().ok()
}

/// Returns the mouse's y coordinate. If the player_id is invalid, returns None.
pub fn mouse_y(player_id: usize) -> Option<usize> {
    let val = unsafe { raw::mouse_y(player_id as i32) };
    val.try_into().ok()
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
    Mouse {
        left,
        right,
        middle,
    },
}
