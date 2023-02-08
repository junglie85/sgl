use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource,
    Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Sampler, SamplerDescriptor,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

use crate::{Bitmap, GraphicsDevice, SglError};

#[derive(Debug)]
pub struct Texture {
    size: Extent3d,
    sampler: Sampler,
    texture: wgpu::Texture,
    texture_view: TextureView,
    pub(crate) bind_group: BindGroup,
}

impl Texture {
    pub fn new(
        width: u32,
        height: u32,
        gpu: &GraphicsDevice,
        format: TextureFormat,
        layout: &BindGroupLayout,
        label: Option<&str>,
    ) -> Self {
        let (size, sampler, texture, texture_view, bind_group) =
            Self::create_gpu_resources(gpu, width, height, format, layout, label);

        Self {
            size,
            sampler,
            texture,
            texture_view,
            bind_group,
        }
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }

    fn create_gpu_resources(
        gpu: &GraphicsDevice,
        width: u32,
        height: u32,
        format: TextureFormat,
        layout: &BindGroupLayout,
        label: Option<&str>,
    ) -> (Extent3d, Sampler, wgpu::Texture, TextureView, BindGroup) {
        let sampler = gpu.device.create_sampler(&SamplerDescriptor {
            label,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest, // TODO: Make sampler configuration configurable.
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = gpu.device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC,
        });

        let texture_view = texture.create_view(&TextureViewDescriptor {
            label,
            ..Default::default()
        });

        let bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
            label,
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&texture_view),
                },
            ],
        });

        (size, sampler, texture, texture_view, bind_group)
    }

    pub fn upload_to_gpu(&self, gpu: &GraphicsDevice, bitmap: &Bitmap) -> Result<(), SglError> {
        if bitmap.width() != self.size.width || bitmap.height() != self.size.height {
            return Err(SglError::General(
                "bitmap has differing dimensions to texture".to_string(),
            ));
        }

        gpu.queue.write_texture(
            ImageCopyTexture {
                aspect: TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            bitmap,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.size.width),
                rows_per_image: std::num::NonZeroU32::new(self.size.height),
            },
            self.size,
        );

        Ok(())
    }
}
