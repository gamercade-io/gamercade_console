mod api;
mod console;
mod core;

use crate::core::Rom;
use std::sync::Arc;

use console::{Console, LuaConsole};
use parking_lot::Mutex;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let code_filename = "test.lua";
    let window = init_window(&event_loop, code_filename);

    let rom = core::Rom::default();

    let mut pixels = init_pixels(&window, &rom);

    //TODO: Load a passed in file or from the rom
    let code = std::fs::read_to_string(code_filename).unwrap();

    // Prepare a frame buffer
    let frame_buffer = Arc::new(Mutex::new(init_frame_buffer(&rom)));

    let rom = Arc::new(rom);

    //TODO: For more players add stuff here
    let player_inputs = Arc::new(Mutex::new(
        vec![core::PlayerInputEntry::default()].into_boxed_slice(),
    ));

    let console = LuaConsole::new(rom, &code, frame_buffer, player_inputs.clone());
    console.call_init();

    //TODO: Incorporate Network stuff GGRS
    let mut input = WinitInputHelper::new();
    let input_manager = core::LocalInputManager::default();
    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state
            let next_input_state = input_manager.generate_input_state(&input);
            player_inputs.lock()[0].push_input_state(next_input_state);
            console.call_update();

            // Render the game
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

            window.request_redraw();
        }
    });
}

fn init_window(event_loop: &EventLoop<()>, code_filename: &str) -> Window {
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
}

fn init_pixels(window: &Window, rom: &Rom) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let (width, height) = (rom.resolution.width(), rom.resolution.height());

    Pixels::new(width, height, surface_texture).unwrap()
}

fn init_frame_buffer(rom: &Rom) -> Box<[u8]> {
    (0..rom.resolution.total_pixels() * 4)
        .map(|_| 0)
        .collect::<Vec<u8>>()
        .into_boxed_slice()
}
