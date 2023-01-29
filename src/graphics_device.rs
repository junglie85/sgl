use futures::executor::block_on;
use wgpu::{
    Adapter, Backends, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance, Limits,
    PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration,
    TextureFormat, TextureUsages,
};
use winit::dpi::PhysicalSize;

use crate::SglError;

pub struct GraphicsDevice {
    pub(crate) _instance: Instance,
    pub(crate) surface: Surface,
    pub(crate) _adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) features: Features,
    pub(crate) limits: Limits,
    pub(crate) surface_config: SurfaceConfiguration,
}

impl GraphicsDevice {
    pub(crate) fn new(native_window: &winit::window::Window) -> Result<Self, SglError> {
        let instance = Instance::new(Backends::all());

        let surface = unsafe { instance.create_surface(&native_window) };

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

        let physical_size = native_window.inner_size();
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Rgba8Unorm,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        Ok(Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            features,
            limits,
            surface_config,
        })
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}
