use std::{borrow::Cow, mem::size_of, ops::Range};

use bytemuck::cast_slice;
use sgl_math::Vec2;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferAddress,
    BufferBinding, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoder, DynamicOffset, Face, FragmentState, FrontFace,
    IndexFormat, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, SurfaceError, TextureSampleType, TextureView, TextureViewDimension,
    VertexState,
};
use winit::dpi::PhysicalSize;

use crate::{
    geometry::Vertex,
    shape::{LineShape, RectangleShape},
    Bitmap, GraphicsDevice, Pixel, Scene, SglError, Texture, View, Window,
};

pub struct Renderer {
    physical_size: PhysicalSize<u32>,
    pixel_size: PhysicalSize<u32>,
    vbo: Buffer,
    ibo: Buffer,
    view_ubo: Buffer,
    view_ubo_stride: usize,
    view_bind_group: BindGroup,
    shape_bind_group_layout: BindGroupLayout,
    triangle_list_pipeline: RenderPipeline,
    triangle_strip_pipeline: RenderPipeline,
    default_texture: Texture,
}

impl Renderer {
    const MAX_INSTANCES: usize = 100_000;
    const MAX_VERTICES: usize = Self::MAX_INSTANCES * 4; // Assume rectangles.
    const MAX_INDICES: usize = Self::MAX_INSTANCES * 6; // Assume rectangles.
    const MAX_VIEWS: usize = 20;

    pub fn new(gpu: &GraphicsDevice, window: &Window) -> Result<Self, SglError> {
        let physical_size = window.native_window.inner_size();
        let pixel_size = window.pixel_size;

        let vbo = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("sgl::vbo"),
            size: (size_of::<Vertex>() * Self::MAX_VERTICES) as BufferAddress,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let ibo = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("sgl::ibo"),
            size: (size_of::<u32>() * Self::MAX_INDICES) as BufferAddress,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_ubo_stride = usize::max(
            size_of::<[f32; 16]>(),
            gpu.limits.min_uniform_buffer_offset_alignment as usize,
        );

        let view_ubo = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("sgl::ubo::view"),
            size: view_ubo_stride as u64 * Self::MAX_VIEWS as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_bind_group_layout =
            gpu.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("sgl::bind_group_layout::view"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: BufferSize::new(view_ubo_stride as u64),
                        },
                        count: None,
                    }],
                });

        let view_bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
            label: Some("sgl::bind_group::view"),
            layout: &view_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &view_ubo,
                    offset: 0,
                    size: BufferSize::new(view_ubo_stride as u64 * Self::MAX_VIEWS as u64),
                }),
            }],
        });

        let shape_bind_group_layout =
            gpu.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("sgl::bind_group_layout::shape"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: true },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                    ],
                });

        let shader_module = gpu.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("sgl::shader_module"),
            source: ShaderSource::Wgsl(Cow::Borrowed(SHADER)).into(),
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("sgl::pipeline_layout"),
                bind_group_layouts: &[&view_bind_group_layout, &shape_bind_group_layout],
                push_constant_ranges: &[],
            });

        let triangle_list_pipeline = gpu
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("sgl::pipeline::triangle_list"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: gpu.surface_config.format,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    polygon_mode: PolygonMode::Fill,
                    front_face: FrontFace::Ccw,
                    strip_index_format: None,
                    cull_mode: Some(Face::Back),
                    conservative: false,
                    unclipped_depth: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        let triangle_strip_pipeline =
            gpu.device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: Some("sgl::pipeline::triangle_strip"),
                    layout: Some(&pipeline_layout),
                    vertex: VertexState {
                        module: &shader_module,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                    },
                    fragment: Some(FragmentState {
                        module: &shader_module,
                        entry_point: "fs_main",
                        targets: &[Some(ColorTargetState {
                            format: gpu.surface_config.format,
                            blend: Some(BlendState::ALPHA_BLENDING),
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState {
                        topology: PrimitiveTopology::TriangleStrip,
                        polygon_mode: PolygonMode::Fill,
                        front_face: FrontFace::Ccw,
                        strip_index_format: Some(IndexFormat::Uint32),
                        cull_mode: Some(Face::Back),
                        conservative: false,
                        unclipped_depth: false,
                    },
                    depth_stencil: None,
                    multisample: MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        let default_texture = Texture::new(
            1,
            1,
            gpu,
            gpu.surface_config.format,
            &shape_bind_group_layout,
            Some("sgl::renderer::default_texture"),
        );

        default_texture.upload_to_gpu(gpu, &Bitmap::from_pixels(1, 1, [Pixel::WHITE])?)?;

        Ok(Self {
            physical_size,
            pixel_size,
            vbo,
            ibo,
            view_ubo,
            view_ubo_stride,
            view_bind_group,
            shape_bind_group_layout,
            triangle_list_pipeline,
            triangle_strip_pipeline,
            default_texture,
        })
    }

    pub fn begin_scene(&self, window: &Window) -> Scene {
        Scene::new(window.view())
    }

    pub fn end_scene(&self, scene: Scene, gpu: &mut GraphicsDevice) {
        let render_commands = self.prepare(scene);

        let (frame, surface_view) = match gpu.get_frame() {
            Ok((frame, surface_view)) => (frame, surface_view),
            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                let physical_size =
                    PhysicalSize::new(gpu.surface_config.width, gpu.surface_config.height);
                gpu.resize(physical_size);
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

        let mut encoder = gpu.create_command_encoder();
        self.render(gpu, render_commands, &surface_view, &mut encoder);
        gpu.present(frame, encoder);
    }

    pub(crate) fn prepare<'draw>(&'draw self, scene: Scene<'draw>) -> RenderCommands<'draw> {
        let mut render_commands = RenderCommands {
            load_op: scene
                .clear_color
                .map_or(LoadOp::Load, |color| LoadOp::Clear(color.into())),
            commands: Vec::new(),
            data: Vec::new(),
        };

        let mut vbo_offset = 0;
        let mut ibo_offset = 0;
        let mut view_ubo_offset = 0;

        for draw_command in scene.draw_commands.into_iter() {
            match draw_command {
                DrawCommand::Line {
                    from,
                    to,
                    color,
                    thickness,
                } => {
                    let line = LineShape::new(from, to);
                    let pixel_size = (self.pixel_size.width, self.pixel_size.height).into();
                    let (vertices, indices) = line.fill_geometry(thickness, color, pixel_size);

                    let vertices_size = size_of::<Vertex>() as u64 * vertices.len() as u64;
                    let indices_size = size_of::<u32>() as u64 * indices.len() as u64;

                    render_commands.commands.push(RenderCommand::Line {
                        pipeline: &self.triangle_strip_pipeline,
                        bind_group: &self.default_texture.bind_group,
                        vbo_bounds: vbo_offset..vbo_offset + vertices_size,
                        ibo_bounds: ibo_offset..ibo_offset + indices_size,
                        index_count: indices.len() as u32,
                    });

                    render_commands.data.push(RenderData::Line {
                        vbo_offset,
                        ibo_offset,
                        vertices,
                        indices,
                    });

                    vbo_offset += vertices_size;
                    ibo_offset += indices_size;
                }

                DrawCommand::Rect {
                    from,
                    to,
                    color,
                    thickness,
                } => {
                    if thickness <= 0.0 {
                        continue;
                    }

                    let rect = RectangleShape::new(from, to);
                    let pixel_size = (self.pixel_size.width, self.pixel_size.height).into();
                    let (vertices, indices) = rect.outline_geometry(thickness, color, pixel_size);

                    let vertices_size = size_of::<Vertex>() as u64 * vertices.len() as u64;
                    let indices_size = size_of::<u32>() as u64 * indices.len() as u64;

                    render_commands.commands.push(RenderCommand::Rect {
                        pipeline: &self.triangle_strip_pipeline,
                        bind_group: &self.default_texture.bind_group,
                        vbo_bounds: vbo_offset..vbo_offset + vertices_size,
                        ibo_bounds: ibo_offset..ibo_offset + indices_size,
                        index_count: indices.len() as u32,
                    });

                    render_commands.data.push(RenderData::Rect {
                        vbo_offset,
                        ibo_offset,
                        vertices,
                        indices,
                    });

                    vbo_offset += vertices_size;
                    ibo_offset += indices_size;
                }

                DrawCommand::RectFilled { from, to, color } => {
                    let rect = RectangleShape::new(from, to);
                    let pixel_size = (self.pixel_size.width, self.pixel_size.height).into();
                    let (vertices, indices) = rect.fill_geometry(color, pixel_size);

                    let vertices_size = size_of::<Vertex>() as u64 * vertices.len() as u64;
                    let indices_size = size_of::<u32>() as u64 * indices.len() as u64;

                    render_commands.commands.push(RenderCommand::RectFilled {
                        pipeline: &self.triangle_list_pipeline,
                        bind_group: &self.default_texture.bind_group,
                        vbo_bounds: vbo_offset..vbo_offset + vertices_size,
                        ibo_bounds: ibo_offset..ibo_offset + indices_size,
                        index_count: indices.len() as u32,
                    });

                    render_commands.data.push(RenderData::RectFilled {
                        vbo_offset,
                        ibo_offset,
                        vertices,
                        indices,
                    });

                    vbo_offset += vertices_size;
                    ibo_offset += indices_size;
                }

                DrawCommand::RectTextured {
                    from,
                    to,
                    texture,
                    sub_coords,
                } => {
                    let rect = RectangleShape::new(from, to);
                    let pixel_size = (self.pixel_size.width, self.pixel_size.height).into();
                    let (vertices, indices) =
                        rect.texture_geometry(texture, sub_coords, pixel_size);

                    let vertices_size = size_of::<Vertex>() as u64 * vertices.len() as u64;
                    let indices_size = size_of::<u32>() as u64 * indices.len() as u64;

                    render_commands.commands.push(RenderCommand::RectTextured {
                        pipeline: &self.triangle_list_pipeline,
                        bind_group: &texture.bind_group,
                        vbo_bounds: vbo_offset..vbo_offset + vertices_size,
                        ibo_bounds: ibo_offset..ibo_offset + indices_size,
                        index_count: indices.len() as u32,
                    });

                    render_commands.data.push(RenderData::RectTextured {
                        vbo_offset,
                        ibo_offset,
                        vertices,
                        indices,
                    });

                    vbo_offset += vertices_size;
                    ibo_offset += indices_size;
                }

                DrawCommand::View(view) => {
                    render_commands.commands.push(RenderCommand::View {
                        view,
                        offset: view_ubo_offset,
                    });

                    render_commands.data.push(RenderData::View {
                        offset: view_ubo_offset as BufferAddress,
                        transform: view.transform(),
                    });

                    view_ubo_offset += self.view_ubo_stride as DynamicOffset;
                }
            }
        }

        render_commands
    }

    pub(crate) fn render(
        &self,
        gpu: &mut GraphicsDevice,
        render_commands: RenderCommands,
        surface_view: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
        for render_data in render_commands.data {
            match render_data {
                RenderData::Line {
                    vbo_offset,
                    ibo_offset,
                    vertices,
                    indices,
                }
                | RenderData::Rect {
                    vbo_offset,
                    ibo_offset,
                    vertices,
                    indices,
                }
                | RenderData::RectFilled {
                    vbo_offset,
                    ibo_offset,
                    vertices,
                    indices,
                }
                | RenderData::RectTextured {
                    vbo_offset,
                    ibo_offset,
                    vertices,
                    indices,
                } => {
                    gpu.staging_belt
                        .write_buffer(
                            encoder,
                            &self.vbo,
                            vbo_offset,
                            BufferSize::new(size_of::<Vertex>() as u64 * vertices.len() as u64)
                                .expect("size must be non-zero"),
                            &gpu.device,
                        )
                        .copy_from_slice(cast_slice(&vertices));

                    gpu.staging_belt
                        .write_buffer(
                            encoder,
                            &self.ibo,
                            ibo_offset,
                            BufferSize::new(size_of::<u32>() as u64 * indices.len() as u64)
                                .expect("size must be non-zero"),
                            &gpu.device,
                        )
                        .copy_from_slice(cast_slice(&indices));
                }

                RenderData::View { offset, transform } => {
                    gpu.staging_belt
                        .write_buffer(
                            encoder,
                            &self.view_ubo,
                            offset,
                            BufferSize::new(size_of::<[f32; 16]>() as u64)
                                .expect("size must be non-zero"),
                            &gpu.device,
                        )
                        .copy_from_slice(cast_slice(&[transform]));
                }
            }
        }

        let color_attachment = RenderPassColorAttachment {
            view: surface_view,
            ops: Operations {
                load: render_commands.load_op,
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

            for render_command in render_commands.commands {
                match render_command {
                    RenderCommand::Line {
                        pipeline,
                        bind_group,
                        vbo_bounds,
                        ibo_bounds,
                        index_count,
                    }
                    | RenderCommand::Rect {
                        pipeline,
                        bind_group,
                        vbo_bounds,
                        ibo_bounds,
                        index_count,
                    }
                    | RenderCommand::RectFilled {
                        pipeline,
                        bind_group,
                        vbo_bounds,
                        ibo_bounds,
                        index_count,
                    }
                    | RenderCommand::RectTextured {
                        pipeline,
                        bind_group,
                        vbo_bounds,
                        ibo_bounds,
                        index_count,
                    } => {
                        rpass.set_pipeline(pipeline);

                        rpass.set_bind_group(1, bind_group, &[]);

                        rpass.set_vertex_buffer(0, self.vbo.slice(vbo_bounds));
                        rpass.set_index_buffer(self.ibo.slice(ibo_bounds), IndexFormat::Uint32);
                        rpass.draw_indexed(0..index_count, 0, 0..1)
                    }

                    RenderCommand::View { view, offset } => {
                        let left = view.width() * view.viewport_left();
                        let top = view.height() * view.viewport_top();
                        let right = view.width() * view.viewport_right();
                        let bottom = view.height() * view.viewport_bottom();
                        rpass.set_viewport(left, bottom, right, top, 0.0, 1.0);
                        rpass.set_scissor_rect(
                            left as u32,
                            bottom as u32,
                            right as u32,
                            top as u32,
                        );

                        rpass.set_bind_group(0, &self.view_bind_group, &[offset]);
                    }
                }
            }
        }
    }

    pub fn create_texture(
        &self,
        gpu: &GraphicsDevice,
        bitmap: &Bitmap,
        label: Option<&str>,
    ) -> Result<Texture, SglError> {
        let texture = Texture::new(
            bitmap.width(),
            bitmap.height(),
            gpu,
            gpu.surface_config.format,
            &self.shape_bind_group_layout,
            label,
        );

        texture.upload_to_gpu(gpu, bitmap)?;

        Ok(texture)
    }
}

pub(crate) struct RenderCommands<'draw> {
    load_op: LoadOp<Color>,
    commands: Vec<RenderCommand<'draw>>,
    data: Vec<RenderData>,
}

enum RenderData {
    Line {
        vbo_offset: BufferAddress,
        ibo_offset: BufferAddress,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    Rect {
        vbo_offset: BufferAddress,
        ibo_offset: BufferAddress,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    RectFilled {
        vbo_offset: BufferAddress,
        ibo_offset: BufferAddress,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    RectTextured {
        vbo_offset: BufferAddress,
        ibo_offset: BufferAddress,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
    View {
        offset: BufferAddress,
        transform: [f32; 16],
    },
}

enum RenderCommand<'draw> {
    Line {
        pipeline: &'draw RenderPipeline,
        bind_group: &'draw BindGroup,
        vbo_bounds: Range<BufferAddress>,
        ibo_bounds: Range<BufferAddress>,
        index_count: u32,
    },
    Rect {
        pipeline: &'draw RenderPipeline,
        bind_group: &'draw BindGroup,
        vbo_bounds: Range<BufferAddress>,
        ibo_bounds: Range<BufferAddress>,
        index_count: u32,
    },
    RectFilled {
        pipeline: &'draw RenderPipeline,
        bind_group: &'draw BindGroup,
        vbo_bounds: Range<BufferAddress>,
        ibo_bounds: Range<BufferAddress>,
        index_count: u32,
    },
    RectTextured {
        pipeline: &'draw RenderPipeline,
        bind_group: &'draw BindGroup,
        vbo_bounds: Range<BufferAddress>,
        ibo_bounds: Range<BufferAddress>,
        index_count: u32,
    },
    View {
        view: View,
        offset: DynamicOffset,
    },
}

static SHADER: &str = r"
// Vertex

@group(0) @binding(0)
var<uniform> scene_transform: mat4x4<f32>;

struct VsIn {
    @location(0) coords: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) fill_color: vec4<f32>,
};

struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) fill_color: vec4<f32>,
};

@vertex
fn vs_main(in: VsIn) -> VsOut {
    let position = scene_transform * vec4<f32>(in.coords, 0.0, 1.0);

    return VsOut(position, in.tex_coords, in.fill_color);
}

struct FsIn {
    @location(0) tex_coords: vec2<f32>,
    @location(1) fill_color: vec4<f32>,
};

struct FsOut {
    @location(0) color: vec4<f32>,
};

@group(1) @binding(0)
var texture_sampler: sampler;
@group(1) @binding(1)
var texture: texture_2d<f32>;

@fragment
fn fs_main(in: FsIn) -> FsOut {
    let color = textureSample(texture, texture_sampler, in.tex_coords) * in.fill_color;

    return FsOut(color);
}
";

#[derive(Debug)]
pub(crate) enum DrawCommand<'scene> {
    Line {
        from: Vec2,
        to: Vec2,
        color: Pixel,
        thickness: f32,
    },
    Rect {
        from: Vec2,
        to: Vec2,
        color: Pixel,
        thickness: f32,
    },
    RectFilled {
        from: Vec2,
        to: Vec2,
        color: Pixel,
    },
    RectTextured {
        from: Vec2,
        to: Vec2,
        texture: &'scene Texture,
        sub_coords: Option<(Vec2, Vec2)>,
    },
    View(View),
}
