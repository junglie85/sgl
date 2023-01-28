use std::borrow::Cow;

use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, IndexFormat,
    MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, VertexState,
};
use winit::dpi::PhysicalSize;

use crate::GraphicsDevice;

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
                    buffers: &[],
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
}

static SHADER: &str = r"
// Vertex

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32>  {
    if (i32(vertex_index) == 0) {
        return vec4<f32>(0.0, 0.5, 0.0, 1.0);
    } else if (i32(vertex_index) == 1) {
        return vec4<f32>(-0.5, -0.5, 0.0, 1.0);
    } else {
        return vec4<f32>(0.5, -0.5, 0.0, 1.0);
    }
}

// Fragment

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
";
