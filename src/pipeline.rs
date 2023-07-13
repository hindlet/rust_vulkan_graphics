use std::sync::Arc;
use vulkano::{
    memory::allocator::StandardMemoryAllocator,
    device::Queue,
    render_pass::{RenderPass, Subpass, Framebuffer, FramebufferCreateInfo},
    pipeline::{Pipeline, GraphicsPipeline, graphics::{viewport::{Viewport, ViewportState}, vertex_input::VertexBufferDescription, input_assembly::InputAssemblyState, multisample::MultisampleState, depth_stencil::DepthStencilState}, PipelineBindPoint},
    image::{view::ImageView, AttachmentImage, ImageAccess, ImageViewAbstract, SampleCount},
    command_buffer::{allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents},
    sync::GpuFuture,
    buffer::Subbuffer,
    single_pass_renderpass,
    format::Format,
    shader::ShaderModule,
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator}
    };
use vulkano_util::{renderer::SwapchainImageView, context::VulkanoContext};
use super::Normal;




pub struct MultiSamplePipeline3D {
    allocator: Arc<StandardMemoryAllocator>,
    queue: Arc<Queue>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,

    intermediary: Arc<ImageView<AttachmentImage>>,
    depth: Arc<ImageView<AttachmentImage>>,
    sample_count: SampleCount,
}

impl MultiSamplePipeline3D {
    /// creates a new multisample pipeline for the given shaders
    /// 
    /// Important Params:
    /// - Vertex Def - defines how vertices are sent to the gpu, can be taken from the vertex defs module
    /// - Sample Count - How many samples the pipeline will take, defaults to 2
    pub fn new(
        context: &VulkanoContext,
        command_buffer_allocator: &Arc<StandardCommandBufferAllocator>,
        descriptor_set_allocator: &Arc<StandardDescriptorSetAllocator>,
        vertex_shader: &Arc<ShaderModule>,
        fragment_shader: &Arc<ShaderModule>,
        vertex_def: &[VertexBufferDescription],
        sample_count: Option<SampleCount>,
    ) -> Self {

        let samples = sample_count.unwrap_or(SampleCount::Sample2);

        let render_pass = Self::create_render_pass(context, samples);
        let pipeline = Self::create_pipeline(vertex_shader, fragment_shader, vertex_def, &render_pass, context, samples);

        let intermediary_image = ImageView::new_default(
            AttachmentImage::transient_multisampled(context.memory_allocator(), [1, 1], samples, Format::B8G8R8A8_SRGB).unwrap()
        ).unwrap();
        let depth = ImageView::new_default(
            AttachmentImage::transient_multisampled(context.memory_allocator(), [1, 1], samples, Format::D16_UNORM).unwrap()
        ).unwrap();
        
        Self {
            allocator: context.memory_allocator().clone(),
            queue: context.graphics_queue().clone(),
            render_pass,
            pipeline,
            command_buffer_allocator: command_buffer_allocator.clone(),
            descriptor_set_allocator: descriptor_set_allocator.clone(),
            intermediary: intermediary_image,
            depth: depth,
            sample_count: samples
        }
    }

    fn create_render_pass(
        context: &VulkanoContext,
        sample_num: SampleCount,
    ) -> Arc<RenderPass> {
        single_pass_renderpass!(
            context.device().clone(),
            attachments: {
                intermediary: {
                    load: Clear,
                    store: DontCare,
                    format: Format::B8G8R8A8_SRGB,
                    samples: sample_num,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16_UNORM,
                    samples: sample_num,
                },
                end: {
                    load: DontCare,
                    store: Store,
                    format: Format::B8G8R8A8_SRGB,
                    samples: 1,
                }
            },
            pass: {
                color: [intermediary],
                depth_stencil: {depth},
                resolve: [end],
            }
        )
        .unwrap()
    }

    fn create_pipeline(
        vertex_shader: &Arc<ShaderModule>,
        fragment_shader: &Arc<ShaderModule>,
        vertex_def: &[VertexBufferDescription],
        render_pass: &Arc<RenderPass>,
        context: &VulkanoContext,
        sample_count: SampleCount
    ) -> Arc<GraphicsPipeline> {

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
        GraphicsPipeline::start()
            .vertex_input_state(vertex_def)
            .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .render_pass(subpass.clone())
            .multisample_state(MultisampleState {
                rasterization_samples: sample_count,
                ..Default::default()
            })
            .depth_stencil_state(DepthStencilState::simple_depth_test())
            .build(context.device().clone())
            .unwrap()
    }

    pub fn draw_from_vertices<VertexType, UniformBufferType>(
        &mut self,
        before_future: Box<dyn GpuFuture>,
        image: SwapchainImageView,

        vertex_buffer: &Subbuffer<[VertexType]>,
        index_buffer: &Subbuffer<[u32]>,
        uniforms: &Subbuffer<UniformBufferType>
    ) -> Box<dyn GpuFuture> {

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let dimensions = image.image().dimensions().width_height();
        // Resize intermediary image
        if dimensions != self.intermediary.dimensions().width_height() {
            self.intermediary = ImageView::new_default(
                AttachmentImage::transient_multisampled(
                    &self.allocator,
                    dimensions,
                    self.sample_count,
                    image.image().format(),
                )
                .unwrap(),
            )
            .unwrap();
        }
        // Resize depth image
        if dimensions != self.depth.dimensions().width_height() {
            self.depth = ImageView::new_default(
                AttachmentImage::transient_multisampled(
                    &self.allocator,
                    dimensions,
                    self.sample_count,
                    Format::D16_UNORM,
                )
                .unwrap(),
            )
            .unwrap();
        }

        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();
        let set = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, uniforms.clone())],
        )
        .unwrap();

        let framebuffer = Framebuffer::new(self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![self.intermediary.clone(), self.depth.clone(), image],
            ..Default::default()
        })
        .unwrap();

        // Begin render pipeline commands
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![
                        Some([0.0, 0.0, 0.0, 1.0].into()),
                        Some(1f32.into()),
                        Some([0.0, 0.0, 0.0, 1.0].into()),
                    ],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassContents::Inline,
            )
            .unwrap();


        builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                set,
            )
            .set_viewport(0, vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                depth_range: 0.0..1.0,
            }])
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .bind_index_buffer(index_buffer.clone())
            .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap();


        builder.end_render_pass().unwrap();
        let command_buffer = builder.build().unwrap();
        let after_future = before_future.then_execute(self.queue.clone(), command_buffer).unwrap();

        after_future.boxed()

    }

    pub fn draw_from_vertices_and_normals<VertexType, UniformBufferType>(
        &mut self,
        before_future: Box<dyn GpuFuture>,
        image: SwapchainImageView,

        vertex_buffer: &Subbuffer<[VertexType]>,
        normal_buffer: &Subbuffer<[Normal]>,
        index_buffer: &Subbuffer<[u32]>,
        uniforms: &Subbuffer<UniformBufferType>
    ) -> Box<dyn GpuFuture>{

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let dimensions = image.image().dimensions().width_height();
        // Resize intermediary image
        if dimensions != self.intermediary.dimensions().width_height() {
            self.intermediary = ImageView::new_default(
                AttachmentImage::transient_multisampled(
                    &self.allocator,
                    dimensions,
                    SampleCount::Sample2,
                    image.image().format(),
                )
                .unwrap(),
            )
            .unwrap();
        }
        // Resize depth image
        if dimensions != self.depth.dimensions().width_height() {
            self.depth = ImageView::new_default(
                AttachmentImage::transient_multisampled(
                    &self.allocator,
                    dimensions,
                    SampleCount::Sample2,
                    Format::D16_UNORM,
                )
                .unwrap(),
            )
            .unwrap();
        }

        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();
        let set = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, uniforms.clone())],
        )
        .unwrap();

        let framebuffer = Framebuffer::new(self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![self.intermediary.clone(), self.depth.clone(), image],
            ..Default::default()
        })
        .unwrap();

        // Begin render pipeline commands
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![
                        Some([0.0, 0.0, 0.0, 1.0].into()),
                        Some(1f32.into()),
                        Some([0.0, 0.0, 0.0, 1.0].into()),
                    ],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassContents::Inline,
            )
            .unwrap();


        builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                set,
            )
            .set_viewport(0, vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                depth_range: 0.0..1.0,
            }])
            .bind_vertex_buffers(0, (vertex_buffer.clone(), normal_buffer.clone()))
            .bind_index_buffer(index_buffer.clone())
            .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap();


        builder.end_render_pass().unwrap();
        let command_buffer = builder.build().unwrap();
        let after_future = before_future.then_execute(self.queue.clone(), command_buffer).unwrap();

        after_future.boxed()
    }
}