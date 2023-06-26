use vulkano::{buffer::{BufferContents, Subbuffer, Buffer, BufferCreateInfo, BufferUsage}, memory::allocator::{AllocationCreateInfo, MemoryUsage}};
use vulkano_util::{context::VulkanoContext, window::VulkanoWindows};
use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

use crate::{attempt_update_gui_window, GuiWindowData};

pub enum BufferType {
    Vertex,
    Normal,
    Index
}

pub fn create_graphics_shader_data_buffer<T, I>(
    data: I,
    context: &VulkanoContext,
    shader_type: BufferType
) -> Subbuffer<[T]>
where
    T: BufferContents,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    let usage = match shader_type {
        BufferType::Vertex => {BufferUsage::VERTEX_BUFFER},
        BufferType::Normal => {BufferUsage::VERTEX_BUFFER},
        BufferType::Index => {BufferUsage::INDEX_BUFFER}
    };

    Buffer::from_iter(
        context.memory_allocator(),
        BufferCreateInfo {
            usage: usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        data
    ).unwrap()
}

pub fn generic_winit_event_handling(
    event: Event<'_, ()>,
    windows: &mut VulkanoWindows,
    gui: &mut Vec<GuiWindowData>,
    control_flow: &mut ControlFlow
) {
    match event {
        Event::WindowEvent { event, window_id } => {
            // Update Egui integration so the UI works!
            for window in gui.iter_mut() {
                attempt_update_gui_window(window, &event, window_id);
            }
            let renderer = windows.get_renderer_mut(window_id).unwrap();
            match event {
                WindowEvent::Resized(_) => {
                    renderer.resize();
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    renderer.resize();
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            }
        },
        Event::MainEventsCleared => {
            for (_, renderer) in windows.iter_mut() {
                renderer.window().request_redraw();
            }
        },
        _ => ()
    }
}