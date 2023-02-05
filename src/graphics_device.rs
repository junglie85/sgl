use futures::executor::block_on;
use wgpu::{
    util::StagingBelt, Adapter, Backends, CommandEncoder, CommandEncoderDescriptor,
    CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError,
    SurfaceTexture, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;

use crate::{SglError, Window};

pub struct GraphicsDevice {
    pub(crate) _instance: Instance,
    pub(crate) surface: Surface,
    pub(crate) _adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) features: Features,
    pub(crate) limits: Limits,
    pub(crate) surface_config: SurfaceConfiguration,
    pub(crate) staging_belt: StagingBelt,
}

impl GraphicsDevice {
    const DEFAULT_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

    pub fn new(window: &Window) -> Result<Self, SglError> {
        let instance = Instance::new(Backends::all());

        let surface = unsafe { instance.create_surface(&window.native_window) };

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or(SglError::General("could not get adapter".to_string()))?;

        let features = Features::default();

        let limits = Limits {
            ..Default::default()
        };

        let (device, queue) = block_on(adapter.request_device(
            &DeviceDescriptor {
                label: Some("sgl::device"),
                features,
                limits: limits.clone(),
            },
            None,
        ))
        .map_err(|e| SglError::General(e.to_string()))?;

        let physical_size = window.native_window.inner_size();
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba8Unorm,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        let staging_belt = StagingBelt::new(1024);

        Ok(Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            features,
            limits,
            surface_config,
            staging_belt,
        })
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub(crate) fn get_frame(&self) -> Result<(SurfaceTexture, TextureView), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let surface_view = frame.texture.create_view(&TextureViewDescriptor::default());

        Ok((frame, surface_view))
    }

    pub(crate) fn create_command_encoder(&self) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("sgl::command_encoder"),
            })
    }

    pub(crate) fn present(&mut self, frame: SurfaceTexture, encoder: CommandEncoder) {
        self.staging_belt.finish();
        self.queue.submit([encoder.finish()]);
        frame.present();
        self.staging_belt.recall();
    }
}
