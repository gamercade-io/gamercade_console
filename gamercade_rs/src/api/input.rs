use super::{f32_to_option, i32_bool_to_option, i32_u32_to_option};
use crate::raw;

use paste::paste;

#[derive(Clone, Copy)]
/// A Raw Input State. Contains buttons and analog stick data.
pub struct RawInputState(pub i64);

impl RawInputState {
    fn is_valid(self) -> bool {
        self.0 & 1 << 63 == 0
    }
}

#[derive(Clone, Copy)]
/// A Raw Mouse state. Contains X/Y positions, deltas, and button states.
pub struct RawMouseState(pub i64);

impl RawMouseState {
    fn is_valid(self) -> bool {
        self.0 & 1 << 31 == 0
    }
}

macro_rules! derive_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse {
            Buttons { $($mbtn_name:ident,)* },
            Axis { $($maxis_name:ident,)* },
            Wheel { $($mwheel_name:ident,)* },
         },
    ) => {
        paste! {
            // BUTTON MACRO
            $(
                /// Returns true if $btn_name was just pressed this frame.
                /// Returns None is player_id is invalid.
                pub fn [<button_ $btn_name _pressed>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<button_ $btn_name _pressed>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                /// Returns true if $btn_name was just released this frame.
                /// Returns None is player_id is invalid.
                pub fn [<button_ $btn_name _released>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<button_ $btn_name _released>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                /// Returns true if $btn_name is held this frame.
                /// Returns None is player_id is invalid.
                pub fn [<button_ $btn_name _held>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<button_ $btn_name _held>](player_id as i32) };
                    i32_bool_to_option(val)
                }
            )*
            // END BUTTON MACRO

            // ANALOG MACRO
            $(
                /// Returns the $anlg_name stick's X value.
                /// Returns None is player_id is invalid.
                pub fn [<analog_ $anlg_name _x>](player_id: usize) -> Option<f32> {
                    let val = unsafe { raw::[<analog_ $anlg_name _x>](player_id as i32) };
                    f32_to_option(val)
                }

                /// Returns the $anlg_name stick's Y value.
                /// Returns None is player_id is invalid.
                pub fn [<analog_ $anlg_name _y>](player_id: usize) -> Option<f32> {
                    let val = unsafe { raw::[<analog_ $anlg_name _y>](player_id as i32) };
                    f32_to_option(val)
                }
            )*
            // END ANALOG MACRO

            // MOUSE MACRO
            $(
                /// Returns true if $mbtn_name was just pressed this frame.
                /// Returns None is player_id is invalid.
                pub fn [<mouse_ $mbtn_name _pressed>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_ $mbtn_name _pressed>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                /// Returns true if $mbtn_name was just released this frame.
                /// Returns None is player_id is invalid.
                pub fn [<mouse_ $mbtn_name _released>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_ $mbtn_name _released>](player_id as i32) };
                    i32_bool_to_option(val)
                }

                /// Returns true if $mbtn_name is held this frame.
                /// Returns None is player_id is invalid.
                pub fn [<mouse_ $mbtn_name _held>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_ $mbtn_name _held>](player_id as i32) };
                    i32_bool_to_option(val)
                }
            )*

            $(
                /// Returns the mouse's $maxis_name position in pixel coordinates.
                /// Returns None is player_id is invalid.
                pub fn [<mouse_ $maxis_name _pos>](player_id: usize) -> Option<u32> {
                    let val = unsafe { raw::[<mouse_ $maxis_name _pos>](player_id as i32) };
                    i32_u32_to_option(val)
                }

                /// Returns the mouse's $maxis_name delta movement.
                /// Returns None is player_id is invalid.
                pub fn [<mouse_ $maxis_name _delta>](player_id: usize) -> Option<i32> {
                    let val = unsafe { raw::[<mouse_ $maxis_name _delta>](player_id as i32) };
                    if val == i32::MIN {
                        None
                    } else {
                        Some(val)
                    }
                }
            )*

            $(
                /// Returns true if the mouse wheels $maxis_name was moved this frame.
                /// Returns None is player_id is invalid.
                pub fn [<mouse_wheel_ $mwheel_name>](player_id: usize) -> Option<bool> {
                    let val = unsafe { raw::[<mouse_wheel_ $mwheel_name>](player_id as i32) };
                    i32_bool_to_option(val)
                }
            )*

            // END MOUSE MACRO

            // TRIGGER MACRO
            $(
                //Returns the $trg_name trigger's value.
                /// Returns None is player_id is invalid.
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

/// Locks the local mouse and hides it
pub fn lock_mouse(locked: bool) {
    unsafe { raw::lock_mouse(locked as i32) }
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
        Buttons {
            left,
            right,
            middle,
        },
        Axis {
            x,
            y,
        },
        Wheel {
            up,
            down,
            left,
            right,
        },
    },
}
