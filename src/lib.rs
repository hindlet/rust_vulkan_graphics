mod general_graphics;
mod gui;
mod camera_maths;
pub mod test_cube;
mod pipeline;
mod vulkano_wrapping;
mod meshes;
mod general_compute;

pub use camera_maths::Camera;
pub use gui::*;
pub use general_graphics::*;
pub use pipeline::*;
pub use vulkano_wrapping::*;
pub use vulkano::buffer::{allocator::*, Subbuffer};
pub use meshes::*;
pub use winit::{event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}};
pub use vulkano_util::{context::VulkanoContext, renderer::VulkanoWindowRenderer};
pub use vulkano::pipeline::ComputePipeline;
pub use general_compute::*;
pub use vulkano::descriptor_set::{allocator::StandardDescriptorSetAllocator, WriteDescriptorSet};
pub use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
pub use vulkano::image::SampleCount;
pub use vulkano_shaders::shader;

use vulkano;

pub mod vertex_types{
    pub use super::general_graphics::{ColouredVertex, PositionVertex, Normal};
}

pub mod all_vulkano{
    pub use super::vulkano::*;
}