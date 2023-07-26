use std::time::Instant;
use rust_vulkan_graphics::*;
use maths::{Matrix3, Matrix4};

mod vs {
    rust_vulkan_graphics::shader!{
        ty: "vertex",
        src: r"
            #version 450

            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 colour;

            layout(location = 0) out vec3 v_colour;
            
            layout(set = 0, binding = 0) uniform Data {
                mat4 world;
                mat4 view;
                mat4 proj;
            } uniforms;
            
            void main() {
                gl_Position = uniforms.proj * uniforms.view * uniforms.world * vec4(position, 1.0);
                v_colour = colour;
            }
        ",
    }
}

mod fs {
    rust_vulkan_graphics::shader!{
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec3 v_colour;

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(v_colour, 1.0);
            }
        ",

    }
}


fn main() {

    let (mut event_loop, vulkano_context, mut vulkano_windows, window_ids, commands_allocator, descriptor_set_allocator) = get_general_graphics_data(vec![("Cube".to_string(), 750.0, 500.0, false), ("".to_string(), 300.0, 500.0, false)]);
    let uniform_allocator = create_uniform_buffer_allocator(vulkano_context.memory_allocator());
    let mut gui = vec![create_gui_window(
        "Cube Spinning Settings".to_string(),
        vec![("Enable Spinning".to_string(), true)],
        vec![("Spin Speed".to_string(), 0.5, -5.0..=5.0)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),

        &mut vulkano_windows, window_ids[1], &event_loop
    )];

    let vertex_buffer = create_shader_data_buffer(test_cube::COLOURED_VERTICES, &vulkano_context, BufferType::Vertex);
    let index_buffer = create_shader_data_buffer(test_cube::INDICES, &vulkano_context, BufferType::Index);

    let mut camera = Camera::new(Some([-2.0, 0.0, 0.0]), None, Some(10.0), None);

    let mut last_frame_time = Instant::now();
    let mut cube_rotation = 0.0;

    let cube_window_id = window_ids[0];
    let gui_window_id = window_ids[1];

    let vs = vs::load(vulkano_context.device().clone()).unwrap();
    let fs = fs::load(vulkano_context.device().clone()).unwrap();

    let mut cube_render_pipeline = MultiSamplePipeline3D::new(
        &vulkano_context,
        &commands_allocator,
        &descriptor_set_allocator,
        &vs,
        &fs,
        &vertex_defs::coloured(),
        Some(SampleCount::Sample4),
    );

    loop {
        if !generic_winit_event_handling_with_camera(&mut event_loop, &mut vulkano_windows, &mut gui, (&mut camera, &cube_window_id)) {break;}

        let frame_time = last_frame_time.elapsed().as_secs_f32();
        if frame_time > 1.0 / 60.0 {
            last_frame_time = Instant::now();

            attempt_gui_redraw(&mut gui[0], &mut vulkano_windows, gui_window_id);

            if gui[0].checkboxes[0].1 {
                cube_rotation += frame_time * gui[0].f32_sliders[0].1;
            }

            draw_cube(vulkano_windows.get_renderer_mut(cube_window_id).unwrap(), cube_rotation, &vertex_buffer, &index_buffer, &mut cube_render_pipeline, &camera, &uniform_allocator);


            camera.do_move(frame_time);
        }

    }

}

fn draw_cube(
    renderer: &mut VulkanoWindowRenderer,
    cube_rotation: f32,
    vertex_buffer: &Subbuffer<[ColouredVertex]>,
    index_buffer: &Subbuffer<[u32]>,
    pipeline: &mut MultiSamplePipeline3D, 
    camera: &Camera,
    uniform_allocator: &SubbufferAllocator,
) {
    let uniforms = get_uniform_subbuffer(cube_rotation, renderer.swapchain_image_size(), uniform_allocator, camera);
    let before_future = renderer.acquire().unwrap();
    let after_future = pipeline.draw_from_vertices(before_future, renderer.swapchain_image_view(), vertex_buffer, index_buffer, &uniforms);
    renderer.present(after_future, true);
}


fn get_uniform_subbuffer (
    rotation: f32,
    swapchain_size: [u32; 2],
    allocator: &SubbufferAllocator,
    camera: &Camera
) -> Subbuffer<vs::Data> {

    let rotation_mat = Matrix3::from_angle_y(rotation);

    let (view, proj) = get_generic_uniforms(swapchain_size, camera);
    

    let uniform_data = vs::Data {
        world: Matrix4::from(rotation_mat).into(),
        view: view.into(),
        proj: proj.into(),
    };

    let subbuffer = allocator.allocate_sized().unwrap();
    *subbuffer.write().unwrap() = uniform_data;
    subbuffer

}