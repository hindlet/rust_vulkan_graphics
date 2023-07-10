use std::{sync::Arc, time::Duration};
use vulkano::{
    descriptor_set::{allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet, DescriptorSetsCollection},
    command_buffer::{allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage},
    pipeline::{ComputePipeline, Pipeline, PipelineBindPoint},
    shader::ShaderModule,
    sync::{self, GpuFuture}, buffer::BufferContents,
};
use vulkano_util::context::{VulkanoContext, VulkanoConfig};



pub fn get_general_compute_data() -> (VulkanoContext, Arc<StandardCommandBufferAllocator>, Arc<StandardDescriptorSetAllocator>) {
    let context = VulkanoContext::new(VulkanoConfig::default());
    let command_allocator = Arc::new(StandardCommandBufferAllocator::new(
        context.device().clone(),
        Default::default()
    ));
    let descript_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        context.device().clone()
    ));

    (context, command_allocator, descript_allocator)
}

pub fn create_compute_pipeline(
    context: &VulkanoContext,
    shader: &Arc<ShaderModule>,
    entry_point: &str,
) -> Arc<ComputePipeline> {
    ComputePipeline::new(
        context.device().clone(),
        shader.entry_point(entry_point).unwrap(),
        &(),
        None,
        |_| {}
    ).unwrap()
}


pub fn get_descriptor_set(
    pipeline: &Arc<ComputePipeline>,
    descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
    set_index: usize,
    write_sets: impl IntoIterator<Item = WriteDescriptorSet>
) -> Arc<PersistentDescriptorSet> {
    let layout = pipeline.layout().set_layouts();


    PersistentDescriptorSet::new(
        descriptor_set_allocator,
        layout.get(set_index).unwrap().clone(),
        write_sets
    ).unwrap()
}

pub fn run_compute_operation<S>(
    context: &VulkanoContext,
    command_allocator: &Arc<StandardCommandBufferAllocator>,
    pipeline: &Arc<ComputePipeline>,
    num_to_process: [u32; 3],
    work_group_size: [u32; 3],
    descriptor_sets: S,
    timeout: Option<Duration>
)
where
    S: DescriptorSetsCollection,
{
    let mut builder = AutoCommandBufferBuilder::primary(
        command_allocator,
        context.compute_queue().queue_family_index(),
        CommandBufferUsage::OneTimeSubmit
    ).unwrap();

    builder
        .bind_pipeline_compute(pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            descriptor_sets
        )
        .dispatch([num_to_process[0] / work_group_size[0], num_to_process[1] / work_group_size[1], num_to_process[2] / work_group_size[2]])
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(context.device().clone())
        .then_execute(context.compute_queue().clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(timeout).unwrap();
}

pub fn run_compute_operation_push_constants<S, T>(
    context: &VulkanoContext,
    command_allocator: &Arc<StandardCommandBufferAllocator>,
    pipeline: &Arc<ComputePipeline>,
    num_to_process: [u32; 3],
    work_group_size: [u32; 3],
    descriptor_sets: S,
    push_constants: T,
    timeout: Option<Duration>
)
where
    S: DescriptorSetsCollection,
    T: BufferContents,
{
    let mut builder = AutoCommandBufferBuilder::primary(
        command_allocator,
        context.compute_queue().queue_family_index(),
        CommandBufferUsage::OneTimeSubmit
    ).unwrap();

    builder
        .bind_pipeline_compute(pipeline.clone())
        .push_constants(pipeline.layout().clone(), 0, push_constants)
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            descriptor_sets
        )
        .dispatch([num_to_process[0] / work_group_size[0], num_to_process[1] / work_group_size[1], num_to_process[2] / work_group_size[2]])
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(context.device().clone())
        .then_execute(context.compute_queue().clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(timeout).unwrap();
}