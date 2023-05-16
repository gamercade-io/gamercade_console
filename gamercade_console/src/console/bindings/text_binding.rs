use crate::api::{TextApi, TextApiBinding};
use paste::paste;
use std::str;
use wasmtime::{Caller, Extern, Linker};

use crate::console::Contexts;

macro_rules! derive_text_api_binding {
    ($($ident:ident (text_ptr: i32, len: i32, $($name:ident:$args:ty $(,)? )*) $(,)?)*) => {
        paste! {
            impl TextApiBinding for Linker<Contexts> {
                $(
                    fn [<bind_ $ident>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!($ident),
                            |mut caller: Caller<'_, Contexts>, text_ptr: i32, len: i32, $($name: $args,)*| {
                                let mem = match caller.get_export("memory") {
                                    Some(Extern::Memory(mem)) => mem,
                                    _ => return Err(wasmtime::Error::msg("failed to find host memory")),
                                };

                                let data = match mem
                                    .data(&caller)
                                    .get(text_ptr as u32 as usize..)
                                    .and_then(|arr| arr.get(..len as u32 as usize))
                                {
                                    Some(data) => data,
                                    None => return Err(wasmtime::Error::msg("invalid data")),
                                };

                                let text = match str::from_utf8(data) {
                                    Ok(text) => text,
                                    Err(_) => return Err(wasmtime::Error::msg("string is not valid utf-8")),
                                };

                                Ok(caller.data().text_context.$ident(text, $($name,)*))
                        }).unwrap();
                    }

                    fn [<bind_ $ident _utf16>](&mut self) {
                        self.func_wrap(
                            "env",
                            stringify!([<$ident _utf16>]),
                            |mut caller: Caller<'_, Contexts>, text_ptr: i32, len: i32, $($name: $args,)*| {
                                let mem = match caller.get_export("memory") {
                                    Some(Extern::Memory(mem)) => mem,
                                    _ => return Err(wasmtime::Error::msg("failed to find host memory")),
                                };

                                let data = match mem
                                    .data(&caller)
                                    .get(text_ptr as u32 as usize..)
                                    .and_then(|arr| arr.get(..len as u32 as usize))
                                {
                                    Some(data) => data,
                                    None => return Err(wasmtime::Error::msg("invalid data")),
                                };

                                let data = bytemuck::cast_slice(data);

                                let text = match String::from_utf16(data) {
                                    Ok(text) => text,
                                    Err(_) => return Err(wasmtime::Error::msg("string is not valid utf-16")),
                                };

                                Ok(caller.data().text_context.$ident(&text, $($name,)*))
                        }).unwrap();
                    }
                )*
            }
        }
    };
}

derive_text_api_binding! {
    console_log(text_ptr: i32, len: i32,),
}
