use rlua::{Context, Table};
use strum::IntoEnumIterator;

use crate::core::{ButtonCode, Buttons, InputState};

use super::{ToLuaString, ToLuaTable};

impl ToLuaTable for InputState {
    fn to_lua_table<'lua>(&self, ctx: &Context<'lua>) -> Table<'lua> {
        let output = ctx.create_table().unwrap();

        //TODO: Add stuff like .analogs, .triggers etc
        output
            .set("buttons", self.buttons.to_lua_table(ctx))
            .unwrap();

        output
    }
}

impl ToLuaTable for Buttons {
    fn to_lua_table<'lua>(&self, ctx: &Context<'lua>) -> Table<'lua> {
        let output = ctx.create_table().unwrap();

        ButtonCode::iter().for_each(|button| {
            output
                .set(button.to_lua_string(), self.get_button_state(button))
                .unwrap();
        });

        output
    }
}

impl ButtonCode {
    const S_UP: &'static str = "up";
    const S_DOWN: &'static str = "down";
    const S_LEFT: &'static str = "left";
    const S_RIGHT: &'static str = "right";
    const S_A: &'static str = "a";
    const S_B: &'static str = "b";
    const S_C: &'static str = "c";
    const S_D: &'static str = "d";
    const S_START: &'static str = "start";
    const S_SELECT: &'static str = "select";
    const S_LEFT_SHOULDER: &'static str = "lshoulder";
    const S_RIGHT_SHOULDER: &'static str = "rshoulder";
    const S_LEFT_STICK: &'static str = "rstick";
    const S_RIGHT_STICK: &'static str = "lstick";
    const S_LEFT_TRIGGER: &'static str = "ltrigger";
    const S_RIGHT_TRIGGER: &'static str = "rtrigger";
}

impl ToLuaString for ButtonCode {
    fn to_lua_string(&self) -> &str {
        match self {
            Self::Up => Self::S_UP,
            Self::Down => Self::S_DOWN,
            Self::Left => Self::S_LEFT,
            Self::Right => Self::S_RIGHT,
            Self::A => Self::S_A,
            Self::B => Self::S_B,
            Self::C => Self::S_C,
            Self::D => Self::S_D,
            Self::Start => Self::S_START,
            Self::Select => Self::S_SELECT,
            Self::LeftShoulder => Self::S_LEFT_SHOULDER,
            Self::RightShoulder => Self::S_RIGHT_SHOULDER,
            Self::LeftStick => Self::S_LEFT_STICK,
            Self::RightStick => Self::S_RIGHT_STICK,
            Self::LeftTrigger => Self::S_LEFT_TRIGGER,
            Self::RightTrigger => Self::S_RIGHT_TRIGGER,
        }
    }
}
