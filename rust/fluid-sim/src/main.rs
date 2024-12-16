#![allow(clippy::collapsible_match)] // cleaner this way

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use graphics::Graphics;
use pollster::FutureExt;
use simulation::Simulation;
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use wgpu_resources::Wgpu;

mod binding_helpers;
mod descriptor_helpers;
mod graphics;
mod shader_helpers;
mod simulation;
mod wgpu_resources;

struct App {
    state: Option<State>,
}

struct State {
    #[allow(dead_code)]
    window: Arc<Window>,
    wgpu: Wgpu,

    graphics: Graphics,
    simulation: Simulation,

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
            .unwrap_or(60 * 1000);

        let wgpu = Wgpu::new(window.clone()).block_on();
        let graphics = Graphics::new(&wgpu);

        let simulation = Simulation::new();

        self.state = Some(State {
            window,
            wgpu,
            graphics,
            simulation,
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
            graphics,
            simulation,
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
                simulation.update();
                graphics.render(window, simulation, wgpu);
                let after = Instant::now();

                let hertz = *target_framerate as f32 / 1000.0;
                let frame_dur = Duration::from_secs_f32(1.0 / hertz);
                let sleep_time = frame_dur.saturating_sub(after - before);
                std::thread::sleep(sleep_time);
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
