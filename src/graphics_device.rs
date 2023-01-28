use futures::executor::block_on;
use wgpu::{
    Adapter, Backends, CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor,
    Features, Instance, Limits, LoadOp, Operations, PowerPreference, PresentMode, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Surface,
    SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;

use crate::{Renderer, Scene, SglError};

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

    pub fn display(&mut self, scene: Scene, renderer: &Renderer) {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                let physical_size =
                    PhysicalSize::new(self.surface_config.width, self.surface_config.height);
                self.resize(physical_size);
                return;
            }
            Err(SurfaceError::OutOfMemory) => {
                log::error!("surface out of memory");
                return;
            }
            Err(SurfaceError::Timeout) => {
                log::warn!("surface timeout");
                return;
            }
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("sgl::command_encoder"),
            });

        let color_attachment = RenderPassColorAttachment {
            view: &view,
            ops: Operations {
                load: scene
                    .clear_color
                    .map_or(LoadOp::Load, |pixel| LoadOp::Clear(pixel.into())),
                store: true,
            },
            resolve_target: None,
        };

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("sgl::render_pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&renderer.pipeline);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        frame.present();
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}
