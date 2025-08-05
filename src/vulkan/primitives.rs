use std::f32::consts::FRAC_PI_2;
use std::sync::Arc;

use glm::Mat4;
use glm::Vec3;
use glm::Vec4;
use vulkano::buffer::BufferContents;
use vulkano::buffer::IndexBuffer;
use vulkano::buffer::Subbuffer;
use vulkano::buffer::allocator::SubbufferAllocator;
use vulkano::command_buffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::command_buffer::RenderPassBeginInfo;
use vulkano::command_buffer::SubpassBeginInfo;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::descriptor_set::DescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::format::Format;
use vulkano::image::ImageCreateInfo;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageUsage};
use vulkano::instance::InstanceExtensions;
use vulkano::memory::allocator::AllocationCreateInfo;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::Pipeline;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::depth_stencil::DepthState;
use vulkano::pipeline::graphics::depth_stencil::DepthStencilState;
use vulkano::pipeline::graphics::input_assembly::{InputAssemblyState, PrimitiveTopology};
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::vertex_input::VertexDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
use winit::event_loop::EventLoop;

use super::camera::CAMERA;
use super::contexts::{VulkanoContext, WindowContext};
use super::core::vs;

#[derive(BufferContents, Vertex, Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomVertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: Vec3,
    #[format(R8G8B8A8_UNORM)]
    pub color: [u8; 4],
}

pub fn get_required_extensions(
    event_loop: &EventLoop<()>,
) -> (DeviceExtensions, InstanceExtensions) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let required_extensions = Surface::required_extensions(event_loop).unwrap();
    (device_extensions, required_extensions)
}

pub fn get_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
            depth_stencil: {
                format: Format::D16_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: Store,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth_stencil},
        }
    )
    .unwrap()
}

pub fn get_framebuffers(
    memory_allocator: &Arc<StandardMemoryAllocator>,
    images: &Vec<Arc<Image>>,
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    let depth_buffer = ImageView::new_default(
        Image::new(
            memory_allocator.clone(),
            ImageCreateInfo {
                image_type: vulkano::image::ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: images[0].extent(),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap(),
    )
    .unwrap();

    images
        .iter()
        .map(|image| -> Arc<Framebuffer> {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view, depth_buffer.clone()],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn create_swapchain_and_images(
    windowcx: &WindowContext,
    vulkancx: &VulkanoContext,
    event_loop: &EventLoop<()>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let surface = Surface::from_window(windowcx.instance.clone(), windowcx.window.clone())
        .expect("could not create window");
    let physical_device = select_physical_device(windowcx, event_loop);
    let device = vulkancx.get_device().clone();
    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");
    let dimensions = windowcx.window.inner_size();
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
            // present_mode: PresentMode::Fifo,
            composite_alpha,
            ..Default::default()
        },
    )
    .unwrap();

    (swapchain, images)
}

pub fn select_physical_device(
    win_ctx: &WindowContext,
    event_loop: &EventLoop<()>,
) -> Arc<PhysicalDevice> {
    let (window, instance) = (win_ctx.window(), win_ctx.instance.clone());
    let (device_extensions, _) = get_required_extensions(event_loop);
    let surface =
        Surface::from_window(instance.clone(), window.clone()).expect("could not create window");

    instance
        .enumerate_physical_devices()
        .expect("could not enumerate physical devices")
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
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

pub fn select_logical_device_and_queue(
    win_ctx: &WindowContext,
    event_loop: &EventLoop<()>,
) -> (Arc<Device>, u32, Arc<Queue>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let physical_device = select_physical_device(win_ctx, event_loop);
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

    (device, queue_family_index, queues.next().unwrap())
}

pub fn get_render_command_buffers(
    device: Arc<Device>,
    memory_allocator: &Arc<StandardMemoryAllocator>,
    command_buffer_allocator: &Arc<StandardCommandBufferAllocator>,
    queue: Arc<Queue>,
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: Vec<Arc<Framebuffer>>,
    vertex_buffer: &Subbuffer<[CustomVertex]>,
    index_buffer: IndexBuffer,
) -> anyhow::Result<Vec<Arc<PrimaryAutoCommandBuffer>>> {
    let uniform_buffer = {
        let view = unsafe { CAMERA.to_view_matrix() };
        let projection = glm::perspective_rh(1., FRAC_PI_2, 0.1, 100.);

        let model = Mat4::from_diagonal(&Vec4::identity());
        let uniforms = vs::Data {
            model: model.data.0,
            view: view.data.0,
            proj: projection.data.0,
        };

        let uniform_buffer_allocator =
            SubbufferAllocator::new(memory_allocator.clone(), Default::default());
        let buffer = uniform_buffer_allocator.allocate_sized().unwrap();
        *buffer.write().unwrap() = uniforms;

        buffer
    };

    // Descriptor set.
    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));
    let descriptor_set_layout = pipeline.layout().set_layouts()[0];
    let descriptor_set = DescriptorSet::new(
        descriptor_set_allocator,
        descriptor_set_layout,
        [WriteDescriptorSet::buffer(0, uniform_buffer)],
        [],
    )
    .unwrap();

    framebuffers
        .iter()
        .map(
            |framebuffer| -> anyhow::Result<Arc<PrimaryAutoCommandBuffer>> {
                let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                    command_buffer_allocator,
                    queue.queue_family_index(),
                    command_buffer::CommandBufferUsage::MultipleSubmit,
                )
                .unwrap();

                command_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0., 0., 0., 1.].into()), Some(1_f32.into())],
                            ..command_buffer::RenderPassBeginInfo::framebuffer(framebuffer.clone())
                        },
                        SubpassBeginInfo {
                            contents: command_buffer::SubpassContents::Inline,
                            ..Default::default()
                        },
                    )?
                    .bind_pipeline_graphics(pipeline.clone())?
                    .bind_vertex_buffers(0, vertex_buffer.clone())?
                    .bind_index_buffer(index_buffer.clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        vulkano::pipeline::PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        descriptor_set,
                    )
                    .unwrap();
                unsafe {
                    command_buffer_builder.draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                }
                .unwrap();
                command_buffer_builder.end_render_pass(Default::default())?;

                Ok(command_buffer_builder.build()?)
            },
        )
        .collect()
}

pub fn get_graphics_pipeline(
    device: Arc<Device>,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    viewport: Viewport,
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
                viewports: [viewport].into(),
                ..Default::default()
            }),
            depth_stencil_state: Some(DepthStencilState {
                depth: Some(DepthState::simple()),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState {
                attachments: [ColorBlendAttachmentState::default()].into(),
                ..Default::default()
            }),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )
    .unwrap()
}
