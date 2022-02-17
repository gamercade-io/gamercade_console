use rlua::UserData;

use super::LuaConsole;
use crate::{
    api::{GraphicsApi, GraphicsApiBinding},
    console::GraphicsContext,
};

impl GraphicsApiBinding for LuaConsole {
    fn bind_clear_screen(&mut self) {
        self.lua.context(|ctx| {
            let gfx = self.registers.get_graphics_context(&ctx);
            ctx.globals()
                .set(
                    "clear_screen",
                    ctx.create_function(move |_, args: (i32, i32)| {
                        gfx.clear_screen(args.0, args.1);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
        });
    }

    fn bind_set_pixel(&mut self) {
        self.lua.context(|ctx| {
            let gfx = self.registers.get_graphics_context(&ctx);
            ctx.globals()
                .set(
                    "set_pixel",
                    ctx.create_function(move |_, args: (i32, i32, i32, i32)| {
                        gfx.set_pixel(args.0, args.1, args.2, args.3);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
        })
    }

    fn bind_height(&mut self) {
        self.lua.context(|ctx| {
            let gfx = self.registers.get_graphics_context(&ctx);
            ctx.globals()
                .set(
                    "height",
                    ctx.create_function(move |_, ()| Ok(gfx.height())).unwrap(),
                )
                .unwrap()
        })
    }

    fn bind_width(&mut self) {
        self.lua.context(|ctx| {
            let gfx = self.registers.get_graphics_context(&ctx);
            ctx.globals()
                .set(
                    "width",
                    ctx.create_function(move |_, ()| Ok(gfx.width())).unwrap(),
                )
                .unwrap()
        })
    }

    fn bind_line(&mut self) {
        self.lua.context(|ctx| {
            let gfx = self.registers.get_graphics_context(&ctx);
            ctx.globals()
                .set(
                    "line",
                    ctx.create_function(move |_, args: (i32, i32, i32, i32, i32, i32)| {
                        gfx.line(args.0, args.1, args.2, args.3, args.4, args.5);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap()
        })
    }

    fn bind_rect(&mut self) {
        self.lua.context(|ctx| {
            let gfx = self.registers.get_graphics_context(&ctx);
            ctx.globals()
                .set(
                    "rect",
                    ctx.create_function(move |_, args: (i32, i32, i32, i32, i32, i32)| {
                        gfx.rect(args.0, args.1, args.2, args.3, args.4, args.5);
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap()
        })
    }
}

impl UserData for GraphicsContext {}
