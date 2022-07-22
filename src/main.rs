mod api;
mod console;
mod gui;

use std::time::{Duration, Instant};

use ggrs::{GGRSError, P2PSession, SessionState};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    console::LocalInputManager,
    gui::{framework::Framework, Gui},
};
use console::{Console, WasmConsole};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();

    let window = init_window(&event_loop);
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;

    //let (num_players, mut session) = init_session_fast(&rom);
    //let (num_players, mut session) = init_session(&rom);
    let mut session: Option<P2PSession<WasmConsole>> = None;
    let mut pixels = init_pixels(&window);

    //let rom = Arc::new(rom);

    //let player_inputs = InputContext::new(num_players);

    //let mut console = WasmConsole::new(rom.clone(), num_players);

    let mut input = WinitInputHelper::new();
    let input_manager = LocalInputManager::default();
    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    let mut framework = Framework::new(
        window_size.width,
        window_size.height,
        scale_factor,
        &pixels,
        Gui::default(),
    );

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            framework.handle_event(event);
        }

        framework.prepare(&mut pixels, &mut session, &window);

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                framework.gui.window_open = !framework.gui.window_open;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            if let Some(console) = &mut framework.gui.wasm_console {
                // Handle GGRS packets
                let session = session.as_mut().unwrap();
                session.poll_remote_clients();

                if session.current_state() == SessionState::Running {
                    // this is to keep ticks between clients synchronized.
                    // if a client is ahead, it will run frames slightly slower to allow catching up
                    let mut fps_delta = 1. / console.rom.frame_rate.frames_per_second() as f64;
                    if session.frames_ahead() > 0 {
                        fps_delta *= 1.1;
                    }

                    // get delta time from last iteration and accumulate it
                    let delta = Instant::now().duration_since(last_update);
                    accumulator = accumulator.saturating_add(delta);
                    last_update = Instant::now();

                    while accumulator.as_secs_f64() > fps_delta {
                        accumulator =
                            accumulator.saturating_sub(Duration::from_secs_f64(fps_delta));

                        // Generate all local inputs
                        // TODO: Refactor this to handle multiple local players correctly
                        for handle in session.local_player_handles() {
                            session
                                .add_local_input(handle, input_manager.generate_input_state(&input))
                                .unwrap();
                        }

                        // Update internal state
                        match session.advance_frame() {
                            Ok(requests) => {
                                console.handle_requests(requests);
                            }
                            Err(GGRSError::PredictionThreshold) => (),
                            Err(e) => panic!("{}", e),
                        }
                    }

                    // Render the game
                    console.call_draw();
                    console.blit(pixels.get_frame());
                };
            };

            let render_result = pixels.render_with(|encoder, render_target, context| {
                //TODO: Handle this correctly
                context.scaling_renderer.render(encoder, render_target);
                framework.render(encoder, render_target, context)?;

                Ok(())
            });

            if render_result.is_err() {
                println!("render_with failed");
                *control_flow = ControlFlow::Exit;
                return;
            }
            window.request_redraw();
        }
    });
}

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(320_f64, 180_f64);
    WindowBuilder::new()
        .with_title("Gamercade Console")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(event_loop)
        .unwrap()
}

fn init_pixels(window: &Window) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    // TODO: Check if this is correct
    //let (width, height) = (rom.resolution.width(), rom.resolution.height());

    Pixels::new(320, 180, surface_texture).unwrap()
}

// TODO: Finish this GGRS Related things:
