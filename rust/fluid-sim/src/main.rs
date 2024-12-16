#![allow(clippy::collapsible_match)] // cleaner this way

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use descriptor_helpers::RenderPassBuilder;
use pollster::FutureExt;
use shader_helpers::{
    FragmentStateBuilder, LayoutBuilder, RenderPipelineBuilder, VertexStateBuilder,
};
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

    graphics: Graphics,
    simulation: Simulation,

    target_framerate: u32,
}

struct Graphics {
    shader: wgpu::RenderPipeline,
}

struct Simulation {
    particles: Vec<Particle>,
    last_update: Instant,
}

const GRAVITY: f32 = 9.81;
const PX_PER_M: f32 = 64.0;
const COLLISION_DAMPING: f32 = 0.5;
const PARTICLE_COUNT: usize = 50;
const PARTICLE_SIZE: f32 = 10.0;

#[derive(Default)]
struct Particle {
    position: glam::Vec2,
    velocity: glam::Vec2,
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct PushConstants {
    screen_size: glam::Vec2,
    position: glam::Vec2,
    radius: f32,
}

impl Graphics {
    fn new(wgpu: &Wgpu) -> Self {
        let shader_module = wgpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/particle.wgsl"));
        let primitive = wgpu::PrimitiveState::default();

        let vertex_state = VertexStateBuilder::new(&shader_module);
        let vertex_state = vertex_state.build();

        let fragment_state =
            FragmentStateBuilder::new(&shader_module).with_color_target(wgpu::ColorTargetState {
                blend: None,
                format: wgpu.surface_config.format,
                write_mask: wgpu::ColorWrites::all(),
            });
        let fragment_state = fragment_state.build();

        let layout = LayoutBuilder::new(&wgpu.device)
            .with_push_constant::<PushConstants>(wgpu::ShaderStages::VERTEX_FRAGMENT)
            .build();

        let particle_shader = RenderPipelineBuilder::new(&wgpu.device, vertex_state, primitive)
            .with_fragment_state(fragment_state)
            .with_layout(&layout)
            .build();

        Self {
            shader: particle_shader,
        }
    }

    fn render(&mut self, window: &Window, simulation: &Simulation, wgpu: &Wgpu) {
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
        let mut renderpass = command_encoder.begin_render_pass(&desc);

        let screen_size = glam::vec2(
            wgpu.surface_config.width as f32,
            wgpu.surface_config.height as f32,
        );

        renderpass.set_pipeline(&self.shader);
        for particle in simulation.particles.iter() {
            let push_constants = PushConstants {
                screen_size,
                radius: PARTICLE_SIZE,
                position: particle.position * glam::vec2(1.0, -1.0) * glam::Vec2::splat(PX_PER_M),
            };
            renderpass.set_push_constants(
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                0,
                bytemuck::bytes_of(&push_constants),
            );
            renderpass.draw(0..6, 0..1);
        }

        // finish renderpass
        drop(renderpass);

        let command_buffer = command_encoder.finish();
        wgpu.queue.submit([command_buffer]);

        window.pre_present_notify();
        surface_tex.present();

        window.request_redraw();
    }
}

impl Simulation {
    #[allow(clippy::new_without_default)]
    fn new() -> Self {
        let particles_per_row = (PARTICLE_COUNT as f32).sqrt().floor();
        let particles_per_col = (PARTICLE_COUNT as f32 - 1.0) / particles_per_row + 1.0;
        let spacing = PARTICLE_SIZE * 2.0;

        let particles = (0..PARTICLE_COUNT)
            .map(|i| {
                let x = (i as f32 % particles_per_row - particles_per_row / 2.0 + 0.5) * spacing;
                let y = (i as f32 / particles_per_row - particles_per_col / 2.0 + 0.5) * spacing;
                Particle {
                    position: glam::vec2(x, y) / PX_PER_M,
                    velocity: glam::vec2(0.0, 0.0),
                }
            })
            .collect();

        Self {
            particles,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        let bounding_size = glam::vec2(640.0, 480.0) / PX_PER_M;
        let half_bounding_size = bounding_size / 2.0;

        for particle in self.particles.iter_mut() {
            particle.velocity -= glam::vec2(0.0, 1.0) * GRAVITY * delta;
            particle.position += particle.velocity * delta;

            if particle.position.x.abs() > half_bounding_size.x {
                particle.position.x = half_bounding_size.x * particle.position.x.signum();
                particle.velocity *= -1.0 * COLLISION_DAMPING;
            }
            if particle.position.y.abs() > half_bounding_size.y {
                particle.position.y = half_bounding_size.y * particle.position.y.signum();
                particle.velocity *= -1.0 * COLLISION_DAMPING;
            }
        }
    }
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
