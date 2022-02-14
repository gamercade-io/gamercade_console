use std::sync::Arc;

use console::{Console, LuaConsole};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod api;
mod console;
mod core;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let code_filename = "test.lua";

    let window = {
        let size = LogicalSize::new(320_f64, 180_f64);
        WindowBuilder::new()
            .with_title(format!(
                "Gamercade Console - {} - {}x{}",
                code_filename, size.width, size.height
            ))
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let rom = core::Rom::default();
    let mut input_manager = core::LocalInputManager::default();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        let (width, height) = (rom.resolution.width(), rom.resolution.height());

        Pixels::new(width, height, surface_texture)?
    };

    //TODO: Load a passed in file or from the rom
    let code = std::fs::read_to_string(code_filename).unwrap();

    let rom = Arc::new(rom);

    let console = LuaConsole::new(rom, 1, &code);

    //TODO: Incorporate Network stuff GGRS
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            console.call_draw();
            console.blit(pixels.get_frame());

            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let next_input_state = input_manager.generate_input_state(&input);

            let button_pressed = input.key_pressed(VirtualKeyCode::Space);

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            console.call_input(button_pressed);
            console.call_update();
            window.request_redraw();
        }
    });
}
