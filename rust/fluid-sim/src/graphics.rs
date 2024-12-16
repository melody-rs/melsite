use winit::window::Window;

use crate::descriptor_helpers::RenderPassBuilder;
use crate::shader_helpers::{
    FragmentStateBuilder, LayoutBuilder, RenderPipelineBuilder, VertexStateBuilder,
};
use crate::simulation::{Simulation, PARTICLE_SIZE, PX_PER_M};
use crate::wgpu_resources::Wgpu;

pub struct Graphics {
    shader: wgpu::RenderPipeline,
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct PushConstants {
    screen_size: glam::Vec2,
    position: glam::Vec2,
    radius: f32,
}

impl Graphics {
    pub fn new(wgpu: &Wgpu) -> Self {
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

    pub fn render(&mut self, window: &Window, simulation: &Simulation, wgpu: &Wgpu) {
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
