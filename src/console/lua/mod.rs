mod graphics;
mod input;
mod lua_console;

pub use lua_console::LuaConsole;

use rlua::{Context, Table};

use super::Console;

pub(crate) static LUA_RENDER_CONTEXT: &str = "__LUA_RENDER_CONTEXT__";

pub(crate) trait ToLuaTable {
    fn to_lua_table<'lua>(&self, ctx: &Context<'lua>) -> Table<'lua>;
}

pub(crate) trait ToLuaString: Sized {
    fn to_lua_string(&self) -> &str;
}
