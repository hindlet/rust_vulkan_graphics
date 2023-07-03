use std::time::Instant;

use rust_vulkan_graphics::*;
// use std::time::Instant;

fn main() {

    let (mut event_loop, _vulkano_contex, mut vulkano_windows, window_ids, _, _) = get_general_graphics_data(vec![("".to_string(), 300.0, 500.0, false)]);
    let mut gui = vec![create_gui_window(
        "Example Gui".to_string(),
        vec![("Do nothing".to_string(), false), ("Do something (lie)".to_string(), false)],
        vec![("Random Slider".to_string(), 0.5, 0.0..=1.0)],
        vec![("Random int Slider".to_string(), 5, 0..=15)],
        Vec::new(),
        Vec::new(),
        vec![("Unsigned int setting".to_string(), 5)],
        vec![("Name Field".to_string(), "Mr Testing".to_string())],

        &mut vulkano_windows, window_ids[0], &event_loop
    )];

    let mut last_frame_time = Instant::now();

    loop {
        if !generic_winit_event_handling(&mut event_loop, &mut vulkano_windows, &mut gui) {break;}

        let frame_time = last_frame_time.elapsed().as_secs_f32();
        if frame_time > 1.0 / 60.0 {
            last_frame_time = Instant::now();
            attempt_gui_redraw(&mut gui[0], &mut vulkano_windows, window_ids[0]);
        }
    }
}