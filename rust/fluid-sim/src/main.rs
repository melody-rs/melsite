#![allow(clippy::collapsible_match)] // cleaner this way

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use descriptor_helpers::RenderPassBuilder;
use pollster::FutureExt;
use wgpu_resources::Wgpu;
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

mod binding_helpers;
mod descriptor_helpers;
mod shader_helpers;
mod wgpu_resources;

struct App {
    state: Option<State>,
}

struct State {
    #[allow(dead_code)]
    window: Arc<Window>,
    wgpu: Wgpu,

    target_framerate: u32,
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let window_attrs = Window::default_attributes()
            .with_resizable(true)
            .with_title("fluid sim");
        let window = event_loop
            .create_window(window_attrs)
            .expect("failed to create window");
        let window = Arc::new(window);

        let target_framerate = window
            .current_monitor()
            .and_then(|m| m.refresh_rate_millihertz())
            .unwrap_or(60);

        let wgpu = Wgpu::new(window.clone()).block_on();
        self.state = Some(State {
            window,
            wgpu,
            target_framerate,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // State should be Some(_) because in order to get window events we need a window
        let State {
            wgpu,
            window,
            target_framerate,
        } = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                } = event
                {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(new_size) => {
                wgpu.resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                let before = Instant::now();
                let Some(surface_tex) = wgpu.get_surface_texture() else {
                    window.request_redraw();
                    return;
                };

                let surface_view = surface_tex.texture.create_view(&Default::default());

                let desc = wgpu::CommandEncoderDescriptor {
                    label: Some("main command encoder"),
                };
                let mut command_encoder = wgpu.device.create_command_encoder(&desc);

                let desc = RenderPassBuilder::new()
                    .with_label("main renderpass")
                    .with_color_attachment(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    });
                let desc = desc.build();
                let renderpass = command_encoder.begin_render_pass(&desc);

                // finish renderpass
                drop(renderpass);

                let command_buffer = command_encoder.finish();
                wgpu.queue.submit([command_buffer]);

                window.pre_present_notify();
                surface_tex.present();
                let after = Instant::now();

                let hertz = *target_framerate as f32 / 1000.0;
                let frame_dur = Duration::from_secs_f32(1.0 / hertz);
                let sleep_time = frame_dur.saturating_sub(after - before);
                std::thread::sleep(sleep_time);

                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new().expect("failed to create event loop");

    let mut app = App { state: None };
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Event Loop Error: {e}")
    }
}
