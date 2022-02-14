use paste::paste;
use rlua::{Context, UserData};

use super::{LuaConsole, LUA_INPUT_CONTEXT};
use crate::{
    api::{InputApi, InputApiBinding},
    console::InputContext,
};

fn get_input_context(context: &Context) -> InputContext {
    context
        .named_registry_value::<_, InputContext>(LUA_INPUT_CONTEXT)
        .unwrap()
}

impl UserData for InputContext {}

macro_rules! derive_bind_lua_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
    ) => {
        paste! {
            impl InputApiBinding for LuaConsole {
                // BUTTON MACRO
                $(
                    fn [<bind_button_ $btn_name _pressed>](&mut self) {
                        self.lua.context(|ctx| {
                            ctx.globals()
                                .set(
                                    stringify!([<button_ $btn_name _pressed>]),
                                    ctx.create_function(|inner_ctx, player_id: u8| {
                                        Ok(get_input_context(&inner_ctx).[<button_ $btn_name _pressed>](player_id - 1))
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                        });
                    }

                    fn [<bind_button_ $btn_name _released>](&mut self) {
                        self.lua.context(|ctx| {
                            ctx.globals()
                                .set(
                                    stringify!([<button_ $btn_name _released>]),
                                    ctx.create_function(|inner_ctx, player_id: u8| {
                                        Ok(get_input_context(&inner_ctx).[<button_ $btn_name _held>](player_id - 1))
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                        });
                    }

                    fn [<bind_button_ $btn_name _held>](&mut self) {
                        self.lua.context(|ctx| {
                            ctx.globals()
                                .set(
                                    stringify!([<button_ $btn_name _held>]),
                                    ctx.create_function(|inner_ctx, player_id: u8| {
                                        Ok(get_input_context(&inner_ctx).[<button_ $btn_name _held>](player_id - 1))
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                        });
                    }
                )*
                // END BUTTON MACRO

                // ANALOG MACRO
                $(
                    fn [<bind_analog_ $anlg_name _x>](&mut self) {
                        self.lua.context(|ctx| {
                            ctx.globals()
                                .set(
                                    stringify!([<analog_ $anlg_name _x>]),
                                    ctx.create_function(|inner_ctx, player_id: u8| {
                                        Ok(get_input_context(&inner_ctx).[<analog_ $anlg_name _x>](player_id - 1))
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                        });
                    }

                    fn [<bind_analog_ $anlg_name _y>](&mut self) {
                        self.lua.context(|ctx| {
                            ctx.globals()
                                .set(
                                    stringify!([<analog_ $anlg_name _y>]),
                                    ctx.create_function(|inner_ctx, player_id: u8| {
                                        Ok(get_input_context(&inner_ctx).[<analog_ $anlg_name _y>](player_id - 1))
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                        });
                    }
                )*
                // END ANALOG MACRO


                // TRIGGER MACRO
                $(
                    fn [<bind_trigger_ $trg_name>](&mut self) {
                        self.lua.context(|ctx| {
                            ctx.globals()
                                .set(
                                    stringify!([<trigger_ $trg_name>]),
                                    ctx.create_function(|inner_ctx, player_id: u8| {
                                        get_input_context(&inner_ctx).[<trigger_ $trg_name>](player_id - 1);
                                        Ok(())
                                    })
                                    .unwrap(),
                                )
                                .unwrap();
                        });
                    }
                )*
                // END TRIGGER MACRO
            }
        }
    };
}

derive_bind_lua_input_api! {
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
