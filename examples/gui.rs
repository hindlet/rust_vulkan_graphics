use rust_vulkan_graphics::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
// use std::time::Instant;

fn main() {

    let (event_loop, _vulkano_contex, mut vulkano_windows, window_ids) = get_general_graphics_data(vec![("Gui".to_string(), 300.0, 500.0, false)]);
    let mut gui = create_gui_window(
        "Example Gui".to_string(),
        vec![("Do nothing".to_string(), false), ("Do something (lie)".to_string(), false)],
        vec![("Random Slider".to_string(), 0.5, 0.0..=1.0)],
        vec![("Random int Slider".to_string(), 5, 0..=15)],
        Vec::new(),
        Vec::new(),
        vec![("Unsigned int setting".to_string(), 5)],
        vec![("Name Field".to_string(), "Mr Testing".to_string())],

        &mut vulkano_windows, window_ids[0], &event_loop
    );

    // let mut last_print_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let renderer = vulkano_windows.get_renderer_mut(window_ids[0]).unwrap();
        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window().id() => {
                // Update Egui integration so the UI works!
                let _pass_events_to_game = !update_gui_window(&mut gui, &event);
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
                let after_future = draw_gui_on_image(&mut gui, before_future, renderer);
                // Present swapchain
                renderer.present(after_future, true);
            }
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
                // if Instant::now().duration_since(last_print_time).as_secs_f32() > 2.5 {
                //     last_print_time = Instant::now();
                //     println!("{:?}", gui);
                // }
            }
            _ => (),
        }
    });

    
}