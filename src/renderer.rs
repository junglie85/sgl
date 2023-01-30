use std::{borrow::Cow, mem::size_of};

use bytemuck::cast_slice;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferAddress,
    BufferBinding, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoder, Face, FragmentState, FrontFace, IndexFormat,
    LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureView,
    VertexState,
};
use winit::dpi::PhysicalSize;

use crate::{geometry::Vertex, GraphicsDevice, Pixel, Scene, View};

pub struct Renderer {
    physical_size: PhysicalSize<u32>,
    view_bind_group_layout: BindGroupLayout,
    pub(crate) pipeline: RenderPipeline,
}

impl Renderer {
    pub(crate) fn new(gpu: &GraphicsDevice, native_window: &winit::window::Window) -> Self {
        let physical_size = native_window.inner_size();

        let view_bind_group_layout =
            gpu.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("sgl::bind_group_layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(
                                size_of::<[f32; 16]>() as BufferAddress
                            ),
                        },
                        count: None,
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
            view_bind_group_layout,
            pipeline,
        }
    }

    pub(crate) fn prepare(&self, gpu: &GraphicsDevice, scene: Scene) -> RenderCommands {
        let mut render_commands = RenderCommands {
            load_op: scene
                .clear_color
                .map_or(LoadOp::Load, |color| LoadOp::Clear(color.into())),
            commands: Vec::new(),
        };

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

                    let fill_color = color.to_array();

                    let vertices = vec![
                        Vertex {
                            coords: [from.0, from.1],
                            fill_color,
                        },
                        Vertex {
                            coords: [from.0 + extent_x, from.1 + extent_y],
                            fill_color,
                        },
                        Vertex {
                            coords: [to.0, to.1],
                            fill_color,
                        },
                        Vertex {
                            coords: [to.0 + extent_x, to.1 + extent_y],
                            fill_color,
                        },
                    ];

                    let indices = vec![0, 1, 2, 3];

                    // Create VBO, IBO.
                    let vbo = gpu.device.create_buffer(&BufferDescriptor {
                        label: Some("sgl::vbo"),
                        size: (size_of::<Vertex>() * vertices.len()) as BufferAddress,
                        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });

                    let ibo = gpu.device.create_buffer(&BufferDescriptor {
                        label: Some("sgl::ibo"),
                        size: (size_of::<u32>() * indices.len()) as BufferAddress,
                        usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });

                    // Upload.
                    gpu.queue.write_buffer(&vbo, 0, cast_slice(&vertices));
                    gpu.queue.write_buffer(&ibo, 0, cast_slice(&indices));

                    render_commands.commands.push(RenderCommand::Line {
                        vbo,
                        ibo,
                        index_count: indices.len() as u32,
                    });
                }

                DrawCommand::View(view) => {
                    // UBO.
                    let ubo = gpu.device.create_buffer(&BufferDescriptor {
                        label: Some("sgl::ubo"),
                        size: size_of::<[f32; 16]>() as BufferAddress,
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });

                    // Create view bind group.

                    let bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
                        label: Some("sgl::bind_group"),
                        layout: &self.view_bind_group_layout,
                        entries: &[BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::Buffer(BufferBinding {
                                buffer: &ubo,
                                offset: 0,
                                size: BufferSize::new(size_of::<[f32; 16]>() as BufferAddress),
                            }),
                        }],
                    });

                    // Upload.
                    gpu.queue
                        .write_buffer(&ubo, 0, cast_slice(&[view.transform()]));

                    render_commands
                        .commands
                        .push(RenderCommand::View { view, bind_group });
                }
            }
        }

        // Return data for rendering.
        render_commands
    }

    pub(crate) fn render(
        &self,
        render_commands: RenderCommands,
        surface_view: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
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

            for render_command in &render_commands.commands {
                match render_command {
                    RenderCommand::Line {
                        vbo,
                        ibo,
                        index_count,
                    } => {
                        rpass.set_pipeline(&self.pipeline);

                        rpass.set_vertex_buffer(0, vbo.slice(..));
                        rpass.set_index_buffer(ibo.slice(..), IndexFormat::Uint32);
                        rpass.draw_indexed(0..*index_count, 0, 0..1)
                    }

                    RenderCommand::View { view, bind_group } => {
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

                        rpass.set_bind_group(0, bind_group, &[]);
                    }
                }
            }
        }
    }
}

pub(crate) struct RenderCommands {
    load_op: LoadOp<Color>,
    commands: Vec<RenderCommand>,
}

enum RenderCommand {
    Line {
        // pipeline: &RenderPipeline,
        vbo: Buffer,
        ibo: Buffer,
        index_count: u32,
    },
    View {
        view: View,
        bind_group: BindGroup,
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
