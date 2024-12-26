// #![allow(unused_variables)]

use ecolor::Color32;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::command_buffer::RenderPassBeginInfo;
use vulkano::command_buffer::SubpassBeginInfo;
use vulkano::command_buffer::SubpassEndInfo;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::instance::Instance;
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::PipelineBindPoint;

use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageUsage};
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::input_assembly::{InputAssemblyState, PrimitiveTopology};
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::VertexDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::ComputePipeline;
use vulkano::pipeline::Pipeline;
use vulkano::pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
use vulkano::VulkanLibrary;
use winit::event_loop::EventLoop;
use winit::window::Window;

use super::shaders::update_cs;
use super::vk_core::CustomVertex;
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::instance::InstanceExtensions;
use vulkano::memory::allocator::{
    FreeListAllocator, GenericMemoryAllocator, StandardMemoryAllocator,
};

pub fn get_required_extensions(
    event_loop: &EventLoop<()>,
) -> (DeviceExtensions, InstanceExtensions) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let mut instance_extensions = Surface::required_extensions(event_loop).unwrap();
    instance_extensions.khr_surface = true;
    (device_extensions, instance_extensions)
}

pub fn get_compute_command_buffer<T: BufferContents>(
    device: Arc<Device>,
    queue_family_index: u32,
    data: Vec<Subbuffer<[T]>>,
    shader: Arc<ShaderModule>,
    push_constants: Option<update_cs::ComputeConstants>,
    work_group_counts: [u32; 3],
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
) -> anyhow::Result<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>> {
    let memory_allocator = create_memory_allocator(device.clone());
    let stage = PipelineShaderStageCreateInfo::new(shader.entry_point("main").unwrap());
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )?;
    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("failed to create compute pipeline");
    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));
    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();
    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .expect("compute shader: descriptor set layout index out of bounds");
    let descriptor_set = DescriptorSet::new(
        descriptor_set_allocator,
        descriptor_set_layout.clone(),
        {
            let mut write_descriptor_sets: Vec<WriteDescriptorSet> = vec![];
            for (idx, datum) in data.iter().enumerate() {
                write_descriptor_sets.push(WriteDescriptorSet::buffer(idx as u32, datum.clone()));
            }
            write_descriptor_sets
        },
        [],
    )?;
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator,
        queue_family_index,
        command_buffer::CommandBufferUsage::MultipleSubmit,
    )?;
    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())?
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0,
            descriptor_set,
        )?;
    if let Some(constants) = push_constants {
        command_buffer_builder.push_constants(pipeline_layout.clone(), 0, constants)?;
    };
    // command_buffer_builder.dispatch(work_group_counts)?;

    Ok(command_buffer_builder)
}

pub fn get_render_pass(device: Arc<Device>, swapchain: &Arc<Swapchain>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
        },
    },
        pass: {
            color: [color],
            depth_stencil: {},
        }
    )
    .unwrap()
}

pub fn get_framebuffers(
    images: &Vec<Arc<Image>>,
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| -> Arc<Framebuffer> {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn create_swapchain_and_images(
    instance: Arc<Instance>,
    device: Arc<Device>,
    window: Arc<Window>,
    physical_device: Arc<PhysicalDevice>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let surface =
        Surface::from_window(instance.clone(), window.clone()).expect("could not create window");
    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");
    let dimensions = window.inner_size();
    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = physical_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;
    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent: dimensions.into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )
    .unwrap();

    (swapchain, images)
}

pub fn select_physical_device(
    instance: Arc<Instance>,
    event_loop: &EventLoop<()>,
    window: Option<Arc<Window>>,
) -> Arc<PhysicalDevice> {
    let (device_extensions, _) = get_required_extensions(event_loop);
    let library = VulkanLibrary::new().expect("no local vulkan lib");

    instance
        .enumerate_physical_devices()
        .expect("could not enumerate physical devices")
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS) && {
                        if window.is_some() {
                            let surface = Surface::from_window(
                                instance.clone(),
                                window.clone().unwrap().clone(),
                            )
                            .expect("could not create window");
                            p.presentation_support(i as u32, event_loop).unwrap()
                        } else {
                            true
                        }
                    }
                })
                .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 0,
            PhysicalDeviceType::VirtualGpu => 0,
            PhysicalDeviceType::Cpu => 0,
            _ => 4,
        })
        .expect("no device available")
        .0
}

pub struct DeviceAndQueueInfo {
    pub device: Arc<Device>,
    pub queue_family_index: u32,
    pub queue: Arc<Queue>,
}

pub fn get_device_and_queue(
    instance: Arc<Instance>,
    event_loop: &EventLoop<()>,
    // window: Option<Arc<Window>>,
) -> DeviceAndQueueInfo {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let physical_device = select_physical_device(instance, event_loop, None);
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create device");
    let queue = queues.next().unwrap();

    DeviceAndQueueInfo {
        device,
        queue_family_index,
        queue,
    }
}

pub fn create_memory_allocator(
    device: Arc<Device>,
) -> Arc<GenericMemoryAllocator<FreeListAllocator>> {
    Arc::new(StandardMemoryAllocator::new_default(device))
}

pub fn create_command_buffer_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
    StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    )
}

pub fn get_render_command_buffers(
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    queue: &Arc<Queue>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Subbuffer<[CustomVertex]>,
    background_color: Color32,
) -> anyhow::Result<Vec<Arc<PrimaryAutoCommandBuffer>>> {
    let rgba_color: [f32; 4] = background_color.to_normalized_gamma_f32();
    framebuffers
        .iter()
        .map(
            |framebuffer| -> anyhow::Result<Arc<PrimaryAutoCommandBuffer>> {
                let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                    command_buffer_allocator.clone(),
                    queue.queue_family_index(),
                    command_buffer::CommandBufferUsage::MultipleSubmit,
                )
                .unwrap();

                command_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some(rgba_color.into())],
                            ..command_buffer::RenderPassBeginInfo::framebuffer(framebuffer.clone())
                        },
                        SubpassBeginInfo {
                            contents: command_buffer::SubpassContents::Inline,
                            ..Default::default()
                        },
                    )?
                    // .bind_descriptor_sets(, , , )
                    .bind_pipeline_graphics(pipeline.clone())?
                    .bind_vertex_buffers(0, vertex_buffer.clone())?;
                unsafe {
                    command_buffer_builder.draw(vertex_buffer.len() as u32, 1, 0, 0)?;
                }
                command_buffer_builder.end_render_pass(SubpassEndInfo::default())?;

                Ok(command_buffer_builder.build()?)
            },
        )
        .collect()
}

pub fn get_graphics_pipeline(
    device: &Arc<Device>,
    vertex_shader: &Arc<ShaderModule>,
    fragment_shader: &Arc<ShaderModule>,
    render_pass: &Arc<RenderPass>,
    viewport: &Viewport,
) -> Arc<GraphicsPipeline> {
    let vs = vertex_shader.entry_point("main").unwrap();
    let fs = fragment_shader.entry_point("main").unwrap();
    let vertex_shader_state = CustomVertex::per_vertex().definition(&vs).unwrap();
    let stages = [
        PipelineShaderStageCreateInfo::new(vs),
        PipelineShaderStageCreateInfo::new(fs),
    ];
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();
    let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

    GraphicsPipeline::new(
        device.clone(),
        None,
        vulkano::pipeline::graphics::GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(vertex_shader_state),
            input_assembly_state: Some(InputAssemblyState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            }),
            viewport_state: Some(ViewportState {
                viewports: [viewport.clone()].into(),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(
                vulkano::pipeline::graphics::color_blend::ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                ),
            ),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )
    .unwrap()
}
