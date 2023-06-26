#![allow(unused_variables)]
use std::sync::Arc;
use winit::{
    event_loop::EventLoop, window::WindowId
};
use bytemuck::{Pod, Zeroable};
use vulkano_util::{context::*, window::*};
use super::Camera;
use maths::{Vector3, Matrix4, Matrix3};
use vulkano::{
    pipeline::graphics::vertex_input::Vertex,
    command_buffer::allocator::{StandardCommandBufferAllocator},
    descriptor_set::allocator::{StandardDescriptorSetAllocator},
    memory::allocator::StandardMemoryAllocator,
    buffer::{allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo}, BufferUsage},
    format::Format,
    image::ImageUsage,
};





// define Vertex and Normal Structs
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod, Vertex)]
pub struct ColouredVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    pub colour: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod, Vertex)]
pub struct PositionVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}


impl From<Vector3> for PositionVertex {
    fn from(value: Vector3) -> Self {
        PositionVertex {position: value.into()}
    }
}

impl From<[f32; 3]> for PositionVertex {
    fn from(value: [f32; 3]) -> Self {
        PositionVertex {position: value}
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod, Vertex)]
pub struct Normal {
    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],
}

impl From<Vector3> for Normal {
    fn from(value: Vector3) -> Self {
        Normal {normal: value.into()}
    }
}


pub fn get_general_graphics_data(
    window_data: Vec<(String, f32, f32, bool)>
) ->(EventLoop<()>, VulkanoContext, VulkanoWindows, Vec<WindowId>, Arc<StandardCommandBufferAllocator>, Arc<StandardDescriptorSetAllocator>) {
    let event_loop = EventLoop::new();

    let context = VulkanoContext::new(VulkanoConfig::default());
    let command_allocator = Arc::new(StandardCommandBufferAllocator::new(
        context.device().clone(),
        Default::default()
    ));
    let descript_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        context.device().clone()
    ));

    let mut windows = VulkanoWindows::default();
    let mut window_ids = Vec::new();
    for datum in window_data {
        window_ids.push(
            windows.create_window(
                &event_loop,
                &context,
                &WindowDescriptor {
                    width: datum.1,
                    height: datum.2,
                    title: datum.0,
                    resizable: datum.3,
                    ..Default::default()
                },
                |ci| {ci.image_format = Some(Format::B8G8R8A8_SRGB); ci.image_usage = ImageUsage::TRANSFER_DST | ci.image_usage},
            )
        )
    }

    (event_loop, context, windows, window_ids, command_allocator, descript_allocator)

}




pub fn create_uniform_buffer_allocator(
    memory_allocator: &Arc<StandardMemoryAllocator>
) -> SubbufferAllocator {
    SubbufferAllocator::new(
        memory_allocator.clone(),
        SubbufferAllocatorCreateInfo {
            buffer_usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        }
    )
}




pub fn get_generic_uniforms(
    swapchain_size: [u32; 2],
    camera: &Camera,
) -> (Matrix4, Matrix4){
    
    let aspect_ratio = swapchain_size[0] as f32 / swapchain_size[1] as f32;

    let proj = Matrix4::persective_matrix(
        std::f32::consts::FRAC_PI_2,
        aspect_ratio,
        0.01,
        100.0,
    );

    let scale = Matrix4::from(Matrix3::from_scale(1.0));

    (scale * camera.get_view_matrix(), proj)
}
