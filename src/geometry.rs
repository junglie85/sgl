use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
    pub coords: [f32; 2],
    pub fill_color: [f32; 4],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    pub(crate) fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                // Coords.
                VertexAttribute {
                    offset: 0,
                    format: VertexFormat::Float32x2,
                    shader_location: 0,
                },
                // Fill color.
                VertexAttribute {
                    offset: size_of::<[f32; 2]>() as BufferAddress,
                    format: VertexFormat::Float32x4,
                    shader_location: 1,
                },
            ],
        }
    }
}
