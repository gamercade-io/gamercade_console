use paste::paste;
use wasmer::Function;

use super::WasmConsoleBuilder;
use crate::{
    api::{InputApi, InputApiBinding},
    console::InputContext,
};

macro_rules! derive_bind_wasm_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
    ) => {
        paste! {
            impl InputApiBinding for WasmConsoleBuilder<'_> {
                // BUTTON MACRO
                $(
                    fn [<bind_button_ $btn_name _pressed>](&mut self) {
                        self.imports.push((
                            stringify!([<button_ $btn_name _pressed>]),
                            Function::new_native_with_env(
                                self.store,
                                self.input_context.clone(),
                                InputContext::[<button_ $btn_name _pressed>])
                            ));
                    }

                    fn [<bind_button_ $btn_name _released>](&mut self) {
                        self.imports.push((
                            stringify!([<button_ $btn_name _released>]),
                            Function::new_native_with_env(
                                self.store,
                                self.input_context.clone(),
                                InputContext::[<button_ $btn_name _released>])
                            ));
                    }

                    fn [<bind_button_ $btn_name _held>](&mut self) {
                        self.imports.push((
                            stringify!([<button_ $btn_name _held>]),
                            Function::new_native_with_env(
                                self.store,
                                self.input_context.clone(),
                                InputContext::[<button_ $btn_name _held>])
                            ));
                    }
                )*
                // END BUTTON MACRO

                // ANALOG MACRO
                $(
                    fn [<bind_analog_ $anlg_name _x>](&mut self) {
                        self.imports.push((
                            stringify!([<analog_ $anlg_name _x>]),
                            Function::new_native_with_env(
                                self.store,
                                self.input_context.clone(),
                                InputContext::[<analog_ $anlg_name _x>])
                            ));
                    }

                    fn [<bind_analog_ $anlg_name _y>](&mut self) {
                        self.imports.push((
                            stringify!([<analog_ $anlg_name _y>]),
                            Function::new_native_with_env(
                                self.store,
                                self.input_context.clone(),
                                InputContext::[<analog_ $anlg_name _y>])
                            ));
                    }
                )*
                // END ANALOG MACRO


                // TRIGGER MACRO
                $(
                    fn [<bind_trigger_ $trg_name>](&mut self) {
                        self.imports.push((
                            stringify!([<trigger_ $trg_name>]),
                            Function::new_native_with_env(
                                self.store,
                                self.input_context.clone(),
                                InputContext::[<trigger_ $trg_name>])
                            ));
                    }
                )*
                // END TRIGGER MACRO
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
}
