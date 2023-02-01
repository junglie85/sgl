use std::{borrow::Cow, mem::size_of, ops::Range};

use bytemuck::cast_slice;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferAddress,
    BufferBinding, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoder, DynamicOffset, Face, FragmentState, FrontFace,
    IndexFormat, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
    TextureView, VertexState,
};
use winit::dpi::PhysicalSize;

use crate::{geometry::Vertex, GraphicsDevice, Pixel, Scene, View};

pub struct Renderer {
    physical_size: PhysicalSize<u32>,
    pixel_size: PhysicalSize<u32>,
    vbo: Buffer,
    ibo: Buffer,
    view_ubo: Buffer,
    view_ubo_stride: usize,
    view_bind_group: BindGroup,
    pub(crate) pipeline: RenderPipeline,
}

impl Renderer {
    const MAX_INSTANCES: usize = 100_000;
    const MAX_VERTICES: usize = Self::MAX_INSTANCES * 4; // Assume rectangles.
    const MAX_INDICES: usize = Self::MAX_INSTANCES * 6; // Assume rectangles.
    const MAX_VIEWS: usize = 20;

    pub(crate) fn new(
        gpu: &GraphicsDevice,
        native_window: &winit::window::Window,
        pixel_size: PhysicalSize<u32>,
    ) -> Self {
        let physical_size = native_window.inner_size();

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

        let shader_module = gpu.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("sgl::shader_module"),
            source: ShaderSource::Wgsl(Cow::Borrowed(SHADER)).into(),
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("sgl::pipeline_layout"),
                bind_group_layouts: &[&view_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = gpu
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("sgl::pipeline"),
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

        Self {
            physical_size,
            pixel_size,
            vbo,
            ibo,
            view_ubo,
            view_ubo_stride,
            view_bind_group,
            pipeline,
        }
    }

    pub(crate) fn prepare(&self, scene: Scene) -> (Vec<RenderData>, RenderCommands) {
        let mut render_data = Vec::new();
        let mut render_commands = RenderCommands {
            load_op: scene
                .clear_color
                .map_or(LoadOp::Load, |color| LoadOp::Clear(color.into())),
            commands: Vec::new(),
        };

        let mut vbo_offset = 0;
        let mut ibo_offset = 0;
        let mut view_ubo_offset = 0;

        for draw_command in scene.draw_commands.into_iter() {
            match draw_command {
                DrawCommand::Line {
                    from,
                    to,
                    thickness,
                    color,
                } => {
                    // Create vertices and indices.
                    // TODO: Vector maths.
                    let x = to.0 - from.0;
                    let y = to.1 - from.1;
                    let perp_x = y;
                    let perp_y = -x;
                    let len = (perp_x * perp_x + perp_y * perp_y).sqrt();
                    let norm_x = if len != 0.0 { perp_x / len } else { 0.0 };
                    let norm_y = if len != 0.0 { perp_y / len } else { 0.0 };
                    let extent_x = norm_x * thickness;
                    let extent_y = norm_y * thickness;

                    let x0 = from.0 * self.pixel_size.width as f32;
                    let y0 = from.1 * self.pixel_size.height as f32;
                    let x1 = to.0 * self.pixel_size.width as f32;
                    let y1 = to.1 * self.pixel_size.height as f32;
                    let extent_x1 = extent_x * self.pixel_size.width as f32;
                    let extent_y1 = extent_y * self.pixel_size.height as f32;

                    let fill_color = color.to_array();

                    let vertices = vec![
                        Vertex {
                            coords: [x0, y0],
                            fill_color,
                        },
                        Vertex {
                            coords: [x0 + extent_x1, y0 + extent_y1],
                            fill_color,
                        },
                        Vertex {
                            coords: [x1, y1],
                            fill_color,
                        },
                        Vertex {
                            coords: [x1 + extent_x1, y1 + extent_y1],
                            fill_color,
                        },
                    ];

                    let indices = vec![0, 1, 2, 3];

                    // TODO: Next - Staging belt.
                    let vertices_size = size_of::<Vertex>() as u64 * vertices.len() as u64;
                    let indices_size = size_of::<u32>() as u64 * indices.len() as u64;
                    let index_count = indices.len() as u32;

                    render_data.push(RenderData::Line {
                        vbo_offset,
                        ibo_offset,
                        vertices,
                        indices,
                    });

                    render_commands.commands.push(RenderCommand::Line {
                        pipeline: &self.pipeline,
                        vbo_bounds: vbo_offset..vbo_offset + vertices_size,
                        ibo_bounds: ibo_offset..ibo_offset + indices_size,
                        index_count,
                    });

                    vbo_offset += vertices_size;
                    ibo_offset += indices_size;
                }

                DrawCommand::View(view) => {
                    render_data.push(RenderData::View {
                        offset: view_ubo_offset as BufferAddress,
                        transform: view.transform(),
                    });

                    render_commands.commands.push(RenderCommand::View {
                        view,
                        offset: view_ubo_offset,
                    });

                    view_ubo_offset += self.view_ubo_stride as DynamicOffset;
                }
            }
        }

        (render_data, render_commands)
    }

    pub(crate) fn render(
        &self,
        gpu: &GraphicsDevice,
        render_data: Vec<RenderData>,
        render_commands: RenderCommands,
        surface_view: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
        for data in render_data {
            match data {
                RenderData::Line {
                    vbo_offset,
                    ibo_offset,
                    vertices,
                    indices,
                } => {
                    gpu.queue
                        .write_buffer(&self.vbo, vbo_offset, cast_slice(&vertices));
                    gpu.queue
                        .write_buffer(&self.ibo, ibo_offset, cast_slice(&indices));
                }
                RenderData::View { offset, transform } => {
                    gpu.queue
                        .write_buffer(&self.view_ubo, offset, cast_slice(&[transform]));
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
                        vbo_bounds,
                        ibo_bounds,
                        index_count,
                    } => {
                        rpass.set_pipeline(pipeline);

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
}

pub(crate) enum RenderData {
    Line {
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

pub(crate) struct RenderCommands<'draw> {
    load_op: LoadOp<Color>,
    commands: Vec<RenderCommand<'draw>>,
}

enum RenderCommand<'draw> {
    Line {
        pipeline: &'draw RenderPipeline,
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
    @location(1) fill_color: vec4<f32>,
};

struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) fill_color: vec4<f32>,
};

@vertex
fn vs_main(in: VsIn) -> VsOut {
    let position = scene_transform * vec4<f32>(in.coords, 0.0, 1.0);

    return VsOut(position, in.fill_color);
}

// Fragment

struct FsIn {
    @location(0) fill_color: vec4<f32>,
};

struct FsOut {
    @location(0) color: vec4<f32>,
};

@fragment
fn fs_main(in: FsIn) -> FsOut {
    return FsOut(in.fill_color);
}
";

pub(crate) enum DrawCommand {
    Line {
        from: (f32, f32),
        to: (f32, f32),
        thickness: f32,
        color: Pixel,
    },
    View(View),
}
