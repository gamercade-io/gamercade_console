mod graphics;
mod input;
mod lua_console;

pub use lua_console::LuaConsole;

use super::Console;

pub(crate) static LUA_RENDER_CONTEXT: &str = "__LUA_RENDER_CONTEXT__";
pub(crate) static LUA_INPUT_CONTEXT: &str = "__LUA_INPUT_CONTEXT__";
