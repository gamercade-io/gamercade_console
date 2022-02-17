use paste::paste;
use std::sync::Arc;
use wasmer::WasmerEnv;

use parking_lot::Mutex;

use crate::{
    api::InputApi,
    core::{ButtonCode, PlayerInputEntry},
};

#[derive(WasmerEnv, Clone)]
pub struct InputContext {
    pub(crate) input_entries: Arc<Mutex<Box<[PlayerInputEntry]>>>,
}

macro_rules! derive_generate_input_api {
    (
        Buttons { $($btn_name:ident: $btn_code:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
    ) => {
        paste! {
            impl InputApi for InputContext {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32 {
                        let player_input = &self.input_entries.lock()[player_id as usize];
                        let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                        let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                        (prev == false && curr == true) as i32
                    }

                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32 {
                        let player_input = &self.input_entries.lock()[player_id as usize];
                        let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                        let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                        (prev == true && curr == false) as i32
                    }

                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32 {
                        let player_input = &self.input_entries.lock()[player_id as usize];
                        player_input.current.buttons.get_button_state(ButtonCode::$btn_code) as i32
                    }
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32 {
                        let player_input = &self.input_entries.lock()[player_id as usize];
                        player_input.current.[<$anlg_name _stick>].get_x_axis()
                    }

                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32 {
                        let player_input = &self.input_entries.lock()[player_id as usize];
                        player_input.current.[<$anlg_name _stick>].get_y_axis()
                    }
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32 {
                        let player_input = &self.input_entries.lock()[player_id as usize];
                        player_input.current.[<$trg_name _trigger>].get_value()
                    }
                )*
            }
        }
    }
}

derive_generate_input_api! {
    Buttons {
        a: A,
        b: B,
        c: C,
        d: D,
        up: Up,
        down: Down,
        left: Left,
        right: Right,
        start: Start,
        select: Select,
        left_shoulder: LeftShoulder,
        right_shoulder: RightShoulder,
        left_stick: LeftStick,
        right_stick: RightStick,
        left_trigger: LeftTrigger,
        right_trigger: RightTrigger,
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
