#![allow(clippy::collapsible_match)] // cleaner this way

use std::sync::Arc;

use pollster::FutureExt;
use wgpu_resources::Wgpu;
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

mod binding_helpers;
mod shader_helpers;
mod wgpu_resources;

struct App {
    state: Option<State>,
}

struct State {
    #[allow(dead_code)]
    window: Arc<Window>,
    wgpu: Wgpu,
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

        let wgpu = Wgpu::new(window.clone()).block_on();
        self.state = Some(State { window, wgpu });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // State should be Some(_) because in order to get window events we need a window
        let State { wgpu, .. } = self.state.as_mut().unwrap();
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
                let surface_tex = match wgpu.surface.get_current_texture() {
                    Ok(t) => t,
                    // Usually means we need to reconfigure the surface
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        log::warn!("Surface Lost/Outdated, reconfiguring");
                        wgpu.reconfigure();
                        return;
                    }
                    // Took to long to get a surface (Try again next time)
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::error!("Surface timeout!");
                        return;
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory!"),
                };

                let surface_view = surface_tex.texture.create_view(&Default::default());

                let desc = wgpu::CommandEncoderDescriptor {
                    label: Some("main command encoder"),
                };
                let mut command_encoder = wgpu.device.create_command_encoder(&desc);

                let desc = wgpu::RenderPassDescriptor {
                    label: Some("main renderpass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                };
                let renderpass = command_encoder.begin_render_pass(&desc);

                // finish renderpass
                drop(renderpass);

                let command_buffer = command_encoder.finish();
                wgpu.queue.submit([command_buffer]);

                surface_tex.present();
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
