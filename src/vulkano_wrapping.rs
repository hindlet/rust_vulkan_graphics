use vulkano::{buffer::{BufferContents, Subbuffer, Buffer, BufferCreateInfo, BufferUsage}, memory::allocator::{AllocationCreateInfo, MemoryUsage}};
use vulkano_util::{context::VulkanoContext, window::VulkanoWindows};
use winit::{event::{Event, WindowEvent, ElementState}, event_loop::{ControlFlow, EventLoop}, window::WindowId, platform::run_return::EventLoopExtRunReturn};

use crate::{attempt_update_gui_window, GuiWindowData, Camera};

pub enum BufferType {
    Vertex,
    Normal,
    Index,
    Storage
}

pub fn create_shader_data_buffer<T, I>(
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
        BufferType::Index => {BufferUsage::INDEX_BUFFER},
        BufferType::Storage => {BufferUsage::STORAGE_BUFFER}
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
    event_loop: &mut EventLoop<()>,
    windows: &mut VulkanoWindows,
    gui: &mut Vec<GuiWindowData>,
) -> bool {
    let mut running = true;

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match &event {
            Event::WindowEvent { event, window_id } => {
                // Update Egui integration so the UI works!
                for window in gui.iter_mut() {
                    attempt_update_gui_window(window, &event, window_id.clone());
                }
                let renderer = windows.get_renderer_mut(window_id.clone()).unwrap();
                match event {
                    WindowEvent::Resized(_) => {
                        renderer.resize();
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        renderer.resize();
                    }
                    WindowEvent::CloseRequested => {
                        running = false;
                    }
                    _ => (),
                }
            },
            Event::MainEventsCleared => *control_flow = ControlFlow::Exit,
            _ => ()
        }
    });

    running
}

pub fn generic_winit_event_handling_with_camera(
    event_loop: &mut EventLoop<()>,
    windows: &mut VulkanoWindows,
    gui: &mut Vec<GuiWindowData>,
    camera: (&mut Camera, &WindowId),
) -> bool {
    let mut running = true;

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match &event {
            Event::WindowEvent { event, window_id } => {
                // Update Egui integration so the UI works!
                for window in gui.iter_mut() {
                    attempt_update_gui_window(window, &event, window_id.clone());
                }
                let renderer = windows.get_renderer_mut(window_id.clone()).unwrap();
                match event {
                    WindowEvent::Resized(_) => {
                        renderer.resize();
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        renderer.resize();
                    }
                    WindowEvent::CloseRequested => {
                        running = false;
                    }
                    WindowEvent::KeyboardInput {
                        input: winit::event::KeyboardInput {
                            virtual_keycode: Some(keycode),
                            state,
                            ..
                        },
                        ..
                    } => {
                        if window_id == camera.1 {
                            camera.0.process_key(*keycode, *state == ElementState::Pressed);
                        }
                    }
                    _ => (),
                }
            },
            Event::MainEventsCleared => *control_flow = ControlFlow::Exit,
            _ => ()
        }
    });

    running
}