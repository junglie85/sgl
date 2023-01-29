use std::{borrow::Cow, mem::size_of};

use bytemuck::cast_slice;
use wgpu::{
    BlendState, Buffer, BufferAddress, BufferDescriptor, BufferUsages, Color, ColorTargetState,
    ColorWrites, CommandEncoder, Face, FragmentState, FrontFace, IndexFormat, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureView, VertexState,
};
use winit::dpi::PhysicalSize;

use crate::{geometry::Vertex, GraphicsDevice, Pixel, Scene};

pub struct Renderer {
    physical_size: PhysicalSize<u32>,
    pub(crate) pipeline: RenderPipeline,
}

impl Renderer {
    pub(crate) fn new(gpu: &GraphicsDevice, native_window: &winit::window::Window) -> Self {
        let physical_size = native_window.inner_size();

        let shader_module = gpu.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("sgl::shader_module"),
            source: ShaderSource::Wgsl(Cow::Borrowed(SHADER)).into(),
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("sgl::pipeline_layout"),
                bind_group_layouts: &[],
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
            pipeline,
        }
    }

    pub(crate) fn prepare(&self, gpu: &GraphicsDevice, scene: &mut Scene) -> RenderCommands {
        // for draw_command in scene.draw_commands.drain(..) {
        // Create vertices and indices.
        let vertices = vec![
            Vertex {
                coords: [0.0, 0.5],
                fill_color: Pixel::rgb(0xff, 0x00, 0x00).to_array(),
            },
            Vertex {
                coords: [-0.5, -0.5],
                fill_color: Pixel::rgb(0xff, 0x00, 0x00).to_array(),
            },
            Vertex {
                coords: [0.5, -0.5],
                fill_color: Pixel::rgb(0xff, 0x00, 0x00).to_array(),
            },
        ];

        let indices = vec![0, 1, 2];

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
        // }

        // Return data for rendering.
        RenderCommands {
            load_op: scene
                .clear_color
                .map_or(LoadOp::Load, |color| LoadOp::Clear(color.into())),
            vbo,
            ibo,
            index_count: indices.len() as u32,
        }
    }

    pub(crate) fn render(
        &self,
        render_commands: RenderCommands,
        view: &TextureView,
        encoder: &mut CommandEncoder,
    ) {
        let color_attachment = RenderPassColorAttachment {
            view: view,
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

            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, render_commands.vbo.slice(..));
            rpass.set_index_buffer(render_commands.ibo.slice(..), IndexFormat::Uint32);
            rpass.draw_indexed(0..render_commands.index_count, 0, 0..1)
        }
    }
}

pub(crate) struct RenderCommands {
    load_op: LoadOp<Color>,
    vbo: Buffer,
    ibo: Buffer,
    index_count: u32,
}

static SHADER: &str = r"
// Vertex

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
    return VsOut(vec4<f32>(in.coords, 0.0, 1.0), in.fill_color);
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
        color: Pixel,
    },
}
