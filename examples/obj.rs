use std::time::Instant;
use rust_vulkan_graphics::*;
use maths::{Matrix3, Matrix4};

mod vs {
    rust_vulkan_graphics::shader!{
        ty: "vertex",
        src: r"
            #version 450

            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 normal;

            layout(location = 0) out vec4 v_colour;
            layout(location = 1) out vec3 v_normal;
            
            layout(set = 0, binding = 0) uniform Data {
                mat4 world;
                mat4 view;
                mat4 proj;
                vec3 colour;
            } uniforms;
            
            void main() {
                gl_Position = uniforms.proj * uniforms.view * uniforms.world * vec4(position, 1.0);
                v_colour = vec4(uniforms.colour, 1.0);
                v_normal = mat3(transpose(inverse(uniforms.world))) * normal;;
            }
        ",
    }
}

mod fs {
    rust_vulkan_graphics::shader!{
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec4 v_colour;
            layout(location = 1) in vec3 v_normal;

            layout(location = 0) out vec4 f_color;

            const vec3 LIGHTDIR = vec3(-0.302, -0.302, 0.905);

            void main() {
                float light = max(dot(v_normal, LIGHTDIR), 0.0) + 0.2;

                f_color = v_colour * light;
            }
        ",

    }
}


fn main() {

    let meshes = load_obj("assets/island.obj");

    let (mut event_loop, vulkano_context, mut vulkano_windows, window_ids, commands_allocator, descriptor_set_allocator) = get_general_graphics_data(vec![("Obj Example".to_string(), 750.0, 500.0, false)], gen_swapchain_func!(Format::B8G8R8A8_SRGB));
    let uniform_allocator = create_uniform_buffer_allocator(vulkano_context.memory_allocator());
    let mut gui = Vec::new();

    let total_mesh = combine_meshes(&meshes);

    let (vertex_buffer, normal_buffer, index_buffer) = total_mesh.get_buffers(&vulkano_context);


    let mut camera = Camera::new(Some([-20.0, 5.0, 0.0]), Some([1.0, -0.3, 0.0]), Some(10.0), None);

    let mut last_frame_time = Instant::now();
    let mut rotation = 0.0;

    let scene_window_id = window_ids[0];

    let vs = vs::load(vulkano_context.device().clone()).unwrap();
    let fs = fs::load(vulkano_context.device().clone()).unwrap();

    let mut render_pipeline = MultiSamplePipeline3D::new(
        &vulkano_context,
        &commands_allocator,
        &descriptor_set_allocator,
        &vs,
        &fs,
        &vertex_defs::position_normal(),
        Some(SampleCount::Sample4),
    );

    loop {
        if !generic_winit_event_handling_with_camera(&mut event_loop, &mut vulkano_windows, &mut gui, (&mut camera, &scene_window_id)) {break;}

        let frame_time = last_frame_time.elapsed().as_secs_f32();
        if frame_time > 1.0 / 60.0 {
            last_frame_time = Instant::now();

            rotation += frame_time * 0.5;


            draw(vulkano_windows.get_renderer_mut(scene_window_id).unwrap(), rotation, &vertex_buffer, &normal_buffer, &index_buffer, &mut render_pipeline, &camera, &uniform_allocator);


            camera.do_move(frame_time);
        }

    }

}

fn draw(
    renderer: &mut VulkanoWindowRenderer,
    cube_rotation: f32,
    vertex_buffer: &Subbuffer<[PositionVertex]>,
    normal_buffer: &Subbuffer<[Normal]>,
    index_buffer: &Subbuffer<[u32]>,
    pipeline: &mut MultiSamplePipeline3D, 
    camera: &Camera,
    uniform_allocator: &SubbufferAllocator,
) {
    let uniforms = get_uniform_subbuffer(cube_rotation, renderer.swapchain_image_size(), uniform_allocator, camera, [0.2; 3]);
    let before_future = renderer.acquire().unwrap();
    let after_future = pipeline.draw_from_vertices_and_normals(before_future, renderer.swapchain_image_view(), vertex_buffer, normal_buffer, index_buffer, &uniforms);
    renderer.present(after_future, true);
}


fn get_uniform_subbuffer (
    rotation: f32,
    swapchain_size: [u32; 2],
    allocator: &SubbufferAllocator,
    camera: &Camera,
    colour: [f32; 3],
) -> Subbuffer<vs::Data> {

    let rotation_mat = Matrix3::from_angle_y(rotation);

    let (view, proj) = get_generic_uniforms(swapchain_size, camera);
    

    let uniform_data = vs::Data {
        world: Matrix4::from(rotation_mat).into(),
        view: view.into(),
        proj: proj.into(),
        colour: colour.into(),
    };

    let subbuffer = allocator.allocate_sized().unwrap();
    *subbuffer.write().unwrap() = uniform_data;
    subbuffer

}