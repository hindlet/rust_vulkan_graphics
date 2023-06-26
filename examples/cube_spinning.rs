use std::time::Instant;
use rust_vulkan_graphics::*;
use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage, allocator::SubbufferAllocator, Subbuffer}, memory::allocator::{AllocationCreateInfo, MemoryUsage}, pipeline::graphics::vertex_input::Vertex};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow
};
use maths::{Matrix3, Matrix4};

mod vs {
    vulkano_shaders::shader!{
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
    vulkano_shaders::shader!{
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

    let (event_loop, vulkano_context, mut vulkano_windows, window_ids, commands_allocator, descriptor_set_allocator) = get_general_graphics_data(vec![("Cube".to_string(), 750.0, 500.0, false), ("".to_string(), 300.0, 500.0, false)]);
    let uniform_allocator = create_uniform_buffer_allocator(vulkano_context.memory_allocator());
    let mut gui = create_gui_window(
        "Cube Spinning Settings".to_string(),
        vec![("Enable Spinning".to_string(), true)],
        vec![("Spin Speed".to_string(), 0.5, -5.0..=5.0)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),

        &mut vulkano_windows, window_ids[1], &event_loop
    );


    let vertex_buffer = Buffer::from_iter(
        vulkano_context.memory_allocator(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        test_cube::COLOURED_VERTICES,
    ).unwrap();

    let index_buffer = Buffer::from_iter(
        vulkano_context.memory_allocator(),
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        test_cube::INDICES,
    ).unwrap();

    let camera = Camera::new(Some([-2.0, 0.0, 0.0]), None, None, None);

    let mut last_frame_time = Instant::now();
    let mut cube_rotation = 0.0;

    let cube_window_id = window_ids[0];

    let vs = vs::load(vulkano_context.device().clone()).unwrap();
    let fs = fs::load(vulkano_context.device().clone()).unwrap();

    let mut cube_render_pipeline = MultiSamplePipeline3D::new(
        &vulkano_context,
        &commands_allocator,
        &descriptor_set_allocator,
        &vs,
        &fs,
        ColouredVertex::per_vertex(),
    );

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

                if window_id == cube_window_id {
                    let renderer = vulkano_windows.get_renderer_mut(cube_window_id).unwrap();
                    if gui.checkboxes[0].1 {
                        cube_rotation += last_frame_time.elapsed().as_secs_f32() * gui.f32_sliders[0].1;
                    }
                    let uniforms = get_uniform_subbuffer(cube_rotation, renderer.swapchain_image_size(), &uniform_allocator, &camera);
                    let before_future = renderer.acquire().unwrap();
                    let after_future = cube_render_pipeline.draw_from_vertices(before_future, renderer.swapchain_image_view(), &vertex_buffer, &index_buffer, &uniforms);
                    renderer.present(after_future, true);

                    last_frame_time = Instant::now();
                }
            }
            Event::MainEventsCleared => {
                for (_, renderer) in vulkano_windows.iter_mut() {
                    renderer.window().request_redraw();
                }
            }
            _ => (),
        }
    });
    
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