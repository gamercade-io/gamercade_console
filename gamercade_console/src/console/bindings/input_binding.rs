use crate::api::{InputApi, InputApiBinding};
use crate::console::Contexts;
use paste::paste;
use wasmtime::{Caller, Linker};

macro_rules! derive_bind_wasm_input_api {
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
            impl InputApiBinding for Linker<Contexts> {
                // BUTTON MACRO
                $(
                    fn [<bind_button_ $btn_name _pressed>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<button_ $btn_name _pressed>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<button_ $btn_name _pressed>](id)
                        }).unwrap();
                    }

                    fn [<bind_button_ $btn_name _released>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<button_ $btn_name _released>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<button_ $btn_name _released>](id)
                        }).unwrap();
                    }

                    fn [<bind_button_ $btn_name _held>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<button_ $btn_name _held>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<button_ $btn_name _held>](id)
                        }).unwrap();
                    }
                )*
                // END BUTTON MACRO

                // ANALOG MACRO
                $(
                    fn [<bind_analog_ $anlg_name _x>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<analog_ $anlg_name _x>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<analog_ $anlg_name _x>](id)
                        }).unwrap();
                    }

                    fn [<bind_analog_ $anlg_name _y>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<analog_ $anlg_name _y>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<analog_ $anlg_name _y>](id)
                        }).unwrap();
                    }
                )*
                // END ANALOG MACRO

                // TRIGGER MACRO
                $(
                    fn [<bind_trigger_ $trg_name>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<trigger_ $trg_name>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<trigger_ $trg_name>](id)
                        }).unwrap();
                    }
                )*
                // END TRIGGER MACRO

                // MOUSE MACRO
                $(
                    fn [<bind_mouse_ $mbtn_name _pressed>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<mouse_ $mbtn_name _pressed>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<mouse_ $mbtn_name _pressed>](id)
                        }).unwrap();
                    }

                    fn [<bind_mouse_ $mbtn_name _released>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<mouse_ $mbtn_name _released>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<mouse_ $mbtn_name _released>](id)
                        }).unwrap();
                    }

                    fn [<bind_mouse_ $mbtn_name _held>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<mouse_ $mbtn_name _held>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<mouse_ $mbtn_name _held>](id)
                        }).unwrap();
                    }
                )*

                $(
                    fn [<bind_mouse_ $maxis_name _pos>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<mouse_ $maxis_name _pos>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<mouse_ $maxis_name _pos>](id)
                        }).unwrap();
                    }

                    fn [<bind_mouse_ $maxis_name _delta>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<mouse_ $maxis_name _delta>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<mouse_ $maxis_name _delta>](id)
                        }).unwrap();
                    }
                )*

                $(
                    fn [<bind_mouse_wheel_ $mwheel_name>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<mouse_wheel_ $mwheel_name>]),
                            |caller: Caller<'_, Contexts>, id: i32| {
                                caller.data().input_context.[<mouse_wheel_ $mwheel_name>](id)
                        }).unwrap();
                    }
                )*
                // END MOUSE MACRO

                fn bind_raw_mouse_state(&mut self) {
                    self.func_wrap("env", "raw_mouse_state", |caller: Caller<'_, Contexts>, id: i32| {
                        caller.data().input_context.raw_mouse_state(id)
                    }).unwrap();
                }

                fn bind_raw_input_state(&mut self) {
                    self.func_wrap("env", "raw_input_state", |caller: Caller<'_, Contexts>, id: i32| {
                        caller.data().input_context.raw_input_state(id)
                    }).unwrap();
                }

                fn bind_lock_mouse(&mut self) {
                    self.func_wrap("env", "lock_mouse", |mut caller: Caller<'_, Contexts>, locked: i32| {
                        caller.data_mut().input_context.lock_mouse(locked)
                    }).unwrap();
                }
            }
        }
    };
}

derive_bind_wasm_input_api! {
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
