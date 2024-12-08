use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

#[allow(dead_code)]
pub struct Wgpu {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl Wgpu {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance_desc = wgpu::InstanceDescriptor {
            flags: wgpu::InstanceFlags::debugging(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_desc);

        let PhysicalSize { width, height } = window.inner_size();
        let surface = instance.create_surface(window).unwrap();

        let adapter_opts = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };
        let adapter = instance.request_adapter(&adapter_opts).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let device_desc = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::PUSH_CONSTANTS
                | wgpu::Features::MULTI_DRAW_INDIRECT
                | wgpu::Features::MULTI_DRAW_INDIRECT_COUNT,
            required_limits: wgpu::Limits {
                max_push_constant_size: 128,
                ..Default::default()
            },
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_desc, None).await.unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            width,
            height,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_config,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let PhysicalSize { width, height } = size;
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.reconfigure();
    }

    pub fn reconfigure(&self) {
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn get_surface_texture(&self) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            Ok(t) => Some(t),
            // Usually means we need to reconfigure the surface
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                log::warn!("Surface Lost/Outdated, reconfiguring");
                self.reconfigure();
                None
            }
            // Took to long to get a surface (Try again next time)
            Err(wgpu::SurfaceError::Timeout) => {
                log::error!("Surface timeout!");
                None
            }
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory!"),
        }
    }
}
