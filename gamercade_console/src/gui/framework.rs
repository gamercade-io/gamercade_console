use std::{
    collections::VecDeque,
    f32::{INFINITY, NEG_INFINITY},
};

use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use ggrs::P2PSession;
use gilrs::Gilrs;
use pixels::{wgpu, Pixels, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::console::{LocalInputManager, WasmConsole};

use super::Gui;

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // Our stuff
    pub gui: Gui,

    pub perf_tracker: PerformanceTracker,
}

#[derive(Default)]
pub struct PerformanceTracker {
    render_times_ms: VecDeque<f32>,
    update_times_ms: VecDeque<f32>,
    pub frames_per_second: usize,
    pub memory_usage: usize,
}

pub struct PerformanceResult {
    pub max_render_time_ms: f32,
    pub min_render_time_ms: f32,

    pub max_update_time_ms: f32,
    pub min_update_time_ms: f32,

    pub average_render_time_ms: f32,
    pub average_update_time_ms: f32,
}

impl PerformanceTracker {
    pub fn push_times(&mut self, render_time_ms: f32, update_time_ms: f32) {
        if self.render_times_ms.len() >= self.frames_per_second {
            self.render_times_ms.pop_back();
        }

        if self.update_times_ms.len() >= self.frames_per_second {
            self.update_times_ms.pop_back();
        }

        self.render_times_ms.push_front(render_time_ms);
        self.update_times_ms.push_front(update_time_ms);
    }

    fn calc_min_max_avg<'a>(data: &'a mut impl Iterator<Item = &'a f32>) -> (f32, f32, f32) {
        let mut min = INFINITY;
        let mut max = NEG_INFINITY;
        let mut sum = 0.0;
        let mut count = 0;

        for point in data.by_ref() {
            sum += point;
            max = max.max(*point);
            min = min.min(*point);
            count += 1;
        }

        let avg = sum / count as f32;

        (min, max, avg)
    }

    pub fn calculate_frame_times(&self) -> PerformanceResult {
        let (min_render_time_ms, max_render_time_ms, average_render_time_ms) =
            Self::calc_min_max_avg(&mut self.render_times_ms.iter());
        let (min_update_time_ms, max_update_time_ms, average_update_time_ms) =
            Self::calc_min_max_avg(&mut self.update_times_ms.iter());

        PerformanceResult {
            max_render_time_ms,
            min_render_time_ms,
            max_update_time_ms,
            min_update_time_ms,
            average_render_time_ms,
            average_update_time_ms,
        }
    }
}

impl Framework {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
        gui: Gui,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
            perf_tracker: PerformanceTracker::default(),
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(
        &mut self,
        pixels: &mut Pixels,
        session: &mut Option<P2PSession<WasmConsole>>,
        window: &Window,
        input: &mut LocalInputManager,
        gilrs: &mut Gilrs,
    ) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the application.
            self.gui.ui(
                pixels,
                window,
                session,
                egui_ctx,
                input,
                gilrs,
                &self.perf_tracker,
            );
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}
