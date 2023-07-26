use rust_vulkan_graphics::*;

fn main() {
    let (context, command_allocator, descript_allocator) = get_general_compute_data();
    let data_buffer = create_shader_data_buffer(0..65536u32, &context, BufferType::Storage);
    const MULTIPLIER: u32 = 4;

    let shader = cs::load(context.device().clone()).unwrap();

    let pipeline = create_compute_pipeline(&context, &shader, "main");

    let descriptor_set = get_descriptor_set(&pipeline, &descript_allocator, 0, [WriteDescriptorSet::buffer(0, data_buffer.clone())]);

    let push_constants = cs::PushConstantData {
        multiple: MULTIPLIER
    };

    run_compute_operation_push_constants(&context, &command_allocator, &pipeline, [65536, 1, 1], [64, 1, 1], descriptor_set, push_constants, None);

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * MULTIPLIER);
    }

    println!("Everything succeeded!");
}

mod cs {
    rust_vulkan_graphics::shader!{
        ty: "compute",
        src: r"
            #version 460

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

            layout(push_constant) uniform PushConstantData {
                uint multiple;
            } pc;

            layout(set = 0, binding = 0) buffer Data {
                uint data[];
            } buf;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                buf.data[idx] *= pc.multiple;
            }
        ",
    }
}