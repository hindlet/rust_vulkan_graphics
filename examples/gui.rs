use rust_vulkan_graphics::*;
// use std::time::Instant;

fn main() {

    let (event_loop, _vulkano_contex, mut vulkano_windows, window_ids, _, _) = get_general_graphics_data(vec![("".to_string(), 300.0, 500.0, false)]);
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
        match event {
            Event::WindowEvent { event, window_id } => {
                // Update Egui integration so the UI works!
                attempt_update_gui_window(&mut gui, &event, window_id);
                let renderer = vulkano_windows.get_renderer_mut(window_id).unwrap();
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
            Event::RedrawRequested(window_id) => {
                attempt_gui_redraw(&mut gui, &mut vulkano_windows, window_id);
            }
            Event::MainEventsCleared => {
                for (_, renderer) in vulkano_windows.iter_mut() {
                    renderer.window().request_redraw();
                }
                // if Instant::now().duration_since(last_print_time).as_secs_f32() > 2.5 {
                //     last_print_time = Instant::now();
                //     println!("{:?}", gui);
                // }
            }
            _ => (),
        }
    });

    
}