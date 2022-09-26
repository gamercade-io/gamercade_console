mod api;
mod console;
mod gui;
mod pixel_buffer;

use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::Parser;
use gamercade_core::Resolution;
use ggrs::{GGRSError, P2PSession, SessionState};
use gilrs::Gilrs;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, Event, MouseScrollDelta, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    console::LocalInputManager,
    gui::{framework::Framework, Gui},
};
use console::{Console, InputMode, MouseEventCollector, WasmConsole};

#[derive(Parser, Debug)]
struct Cli {
    /// Path to .gcrom to load.
    #[clap(short, long, value_parser)]
    game: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let event_loop = EventLoop::new();

    let window = init_window(&event_loop);
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;

    let mut session: Option<P2PSession<WasmConsole>> = None;
    let mut pixels = init_pixels(&window);

    let mut gilrs = Gilrs::new().unwrap();

    let mut input = WinitInputHelper::new();
    let mut input_manager = LocalInputManager::new(InputMode::default());
    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    let mut framework = Framework::new(
        window_size.width,
        window_size.height,
        scale_factor,
        &pixels,
        Gui::default(),
    );

    if let Some(game_path) = &cli.game {
        let seed = fastrand::u64(0..u64::MAX);
        framework
            .gui
            .fast_launch_game(game_path.clone(), seed, &mut pixels, &window, &mut session);
    }

    let mut mouse_events = MouseEventCollector::default();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event, .. } = &event {
            framework.handle_event(event);
        }

        if session.is_some() {
            if let Event::DeviceEvent { event, .. } = &event {
                if let DeviceEvent::MouseMotion { delta } = event {
                    mouse_events.delta_x += delta.0 as i16;
                    mouse_events.delta_y += delta.1 as i16;
                }

                if let DeviceEvent::MouseWheel { delta } = event {
                    let mut out_x = 0.0;
                    let mut out_y = 0.0;

                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            out_x += x;
                            out_y += y;
                        }
                        MouseScrollDelta::PixelDelta(d) => {
                            out_x += d.x as f32;
                            out_y += d.y as f32
                        }
                    }

                    if out_y > 0.0 {
                        mouse_events.wheel_down = true
                    } else if out_y < 0.0 {
                        mouse_events.wheel_up = true
                    }

                    if out_x > 0.0 {
                        mouse_events.wheel_right = true
                    } else if out_x < 0.0 {
                        mouse_events.wheel_left = true
                    }
                }
            }
        }

        framework.prepare(
            &mut pixels,
            &mut session,
            &window,
            &mut input_manager,
            &mut gilrs,
        );

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

                        // Process all the gamepad events
                        while gilrs.next_event().is_some() {}

                        let shared_mouse = std::mem::take(&mut mouse_events);

                        // Generate all local inputs
                        // TODO: Refactor this to handle multiple local players correctly
                        for handle in session.local_player_handles() {
                            session
                                .add_local_input(
                                    handle,
                                    input_manager.generate_input_state(
                                        &pixels,
                                        &shared_mouse,
                                        &input,
                                        &gilrs,
                                    ),
                                )
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

                    // If sound changed, update the output
                    console.sync_audio();

                    // Sync the mouse lock state
                    console.sync_mouse(&window);

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

const DEFAULT_WINDOW_RESOLUTION: Resolution = Resolution::High;

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(
        DEFAULT_WINDOW_RESOLUTION.width() as f64,
        DEFAULT_WINDOW_RESOLUTION.height() as f64,
    );
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

    Pixels::new(320, 180, surface_texture).unwrap()
}
