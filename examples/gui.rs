use rust_vulkan_graphics::{
    get_general_graphics_data,
    create_gui_window, draw_gui_window,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

fn main() {

    let (event_loop, _vulkano_contex, mut vulkano_windows, window_ids) = get_general_graphics_data(vec![("Gui".to_string(), 300.0, 500.0, false)]);
    let mut gui = create_gui_window(
        "Example Gui".to_string(),
        vec![("Do nothing".to_string(), false), ("Do something (lie)".to_string(), false)],
        Vec::new(),
        Vec::new(),

        &mut vulkano_windows, window_ids[0], &event_loop
    );

    event_loop.run(move |event, _, control_flow| {
        let renderer = vulkano_windows.get_renderer_mut(window_ids[0]).unwrap();
        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window().id() => {
                // Update Egui integration so the UI works!
                let _pass_events_to_game = !gui.gui.update(&event);
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
            }
            Event::RedrawRequested(window_id) if window_id == window_id => {
                // Set immediate UI in redraw here
                draw_gui_window(&mut gui);
                // Render UI
                // Acquire swapchain future
                let before_future = renderer.acquire().unwrap();
                // Render gui
                let after_future =
                    gui.gui.draw_on_image(before_future, renderer.swapchain_image_view());
                // Present swapchain
                renderer.present(after_future, true);
            }
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
            }
            _ => (),
        }
    });

    
}