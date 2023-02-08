#![allow(dead_code)]

pub use crate::bitmap::Bitmap;
pub use crate::error::SglError;
pub use crate::fs::*;
pub use crate::graphics_device::GraphicsDevice;
pub use crate::key::Key;
pub use crate::pixel::Pixel;
pub use crate::renderer::Renderer;
pub use crate::scene::Scene;
pub use crate::texture::Texture;
pub use crate::view::View;
pub use crate::window::Window;

mod bitmap;
mod error;
mod fs;
mod geometry;
mod graphics_device;
mod key;
mod pixel;
mod renderer;
mod scene;
pub(crate) mod shape;
mod texture;
mod view;
mod window;
