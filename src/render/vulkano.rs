#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::Arc;

use image::{ImageBuffer, Rgba};
use tracing::info;

use vulkan_primitives::{
    create_command_buffer_allocator, create_device_and_queues, create_instance,
    create_memory_allocator,
};
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{
    self, AutoCommandBufferBuilder, ClearColorImageInfo, CopyBufferInfo, CopyImageToBufferInfo,
    RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo,
};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{self, PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::format::{self, Format};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{
    AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator,
};
use vulkano::pipeline::compute::{self, ComputePipelineCreateInfo};
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::{PipelineDescriptorSetLayoutCreateInfo, PipelineLayoutCreateInfo};
use vulkano::pipeline::{
    ComputePipeline, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
    PipelineShaderStageCreateInfo,
};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, Subpass};
use vulkano::sync::{GpuFuture, PipelineStages};
use vulkano::{single_pass_renderpass, sync, VulkanLibrary};

pub struct Context {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue_family_index: u32,
    queues: Box<dyn ExactSizeIterator<Item = Arc<vulkano::device::Queue>>>,
}

impl Default for Context {
    fn default() -> Self {
        let instance = create_instance();
        let (device, queue_family_index, queues) = create_device_and_queues(instance.clone());
        Self {
            instance,
            device,
            queue_family_index,
            queues: Box::new(queues),
        }
    }
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

pub fn do_graphics_pipeline(ctx: Context) {
    let (device, queue_family_index, mut queues) = (ctx.device, ctx.queue_family_index, ctx.queues);
    let queue = queues.next().unwrap();
    let memory_allocator = create_memory_allocator(device.clone());
    let vertex_1 = MyVertex {
        position: [-0.5, -0.5],
    };
    let vertex_2 = MyVertex {
        position: [0.0, 0.5],
    };
    let vertex_3 = MyVertex {
        position: [0.5, -0.25],
    };

    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vec![vertex_1, vertex_2, vertex_3],
    )
    .unwrap();

    let render_pass = single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: Format::R8G8B8A8_UNORM,
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
    .unwrap();

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_SRC | ImageUsage::STORAGE | ImageUsage::COLOR_ATTACHMENT,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();
    let view = ImageView::new_default(image.clone()).unwrap();
    let frame_buffer = Framebuffer::new(
        render_pass.clone(),
        FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
        },
    )
    .unwrap();

    let vertex_shader = vertex_shader::load(device.clone()).expect("failed to make vertex shader");
    let fragment_shader =
        fragment_shader::load(device.clone()).expect("failed to make fragment shader");

    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [1024.0, 1024.0],
        depth_range: 0.0..=1.0,
    };

    let pipeline = {
        let vs = vertex_shader.entry_point("main").unwrap();
        let fs = fragment_shader.entry_point("main").unwrap();

        let vertex_shader_state = MyVertex::per_vertex()
            .definition(&vs.info().input_interface)
            .unwrap();

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
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [viewport].into_iter().collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(vulkano::pipeline::graphics::color_blend::ColorBlendState::with_attachment_states(subpass.num_color_attachments(), ColorBlendAttachmentState::default())),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap()
    };

    let command_buffer_allocator = create_command_buffer_allocator(device.clone());
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        command_buffer::CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    command_buffer_builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
                ..command_buffer::RenderPassBeginInfo::framebuffer(frame_buffer.clone())
            },
            SubpassBeginInfo {
                contents: command_buffer::SubpassContents::Inline,
                ..Default::default()
            },
        )
        .unwrap()
        .bind_pipeline_graphics(pipeline.clone())
        .unwrap()
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .unwrap()
        .draw(3, 1, 0, 0)
        .unwrap()
        .end_render_pass(SubpassEndInfo::default())
        .unwrap();

    let image_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("couldn't make image buffer in graphics");

    command_buffer_builder
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image,
            image_buffer.clone(),
        ))
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();
    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();

    let image_buffer_content = image_buffer.read().unwrap();
    let image =
        ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &image_buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
}

pub fn draw_mandelbrot_fractal(ctx: Context) {
    let (device, queue_family_index, mut queues) = (ctx.device, ctx.queue_family_index, ctx.queues);
    let queue = queues.next().unwrap();
    let memory_allocator = vulkan_primitives::create_memory_allocator(device.clone());

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_SRC | ImageUsage::STORAGE,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();
    let view = ImageView::new_default(image.clone()).unwrap();

    let shader =
        mandelbrot_compute_shader::load(device.clone()).expect("failed to create shader module");
    let cs = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("failed to create compute pipeline");

    let descriptor_set_allocator =
        StandardDescriptorSetAllocator::new(device.clone(), Default::default());
    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();
    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .unwrap();
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::image_view(0, view.clone())],
        [],
    )
    .unwrap();

    let buffer = Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("couldn't create fractal data buffer");

    let command_buffer_allocator =
        vulkan_primitives::create_command_buffer_allocator(device.clone());
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        command_buffer::CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0,
            descriptor_set,
        )
        .unwrap()
        .dispatch([1024 / 8, 1024 / 8, 1])
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            buffer.clone(),
        ))
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();
    let future = sync::now(device.clone())
        .then_execute(queue, command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();

    let buffer_content = buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("mandelbrot.png").unwrap();
}

pub fn do_image_creation(ctx: Context) {
    let (device, mut queues) = (ctx.device, ctx.queues);
    let memory_allocator = vulkan_primitives::create_memory_allocator(device.clone());

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_SRC | ImageUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

    let command_buffer_allocator =
        vulkan_primitives::create_command_buffer_allocator(device.clone());
    // let (device, _, mut queues) = create_device_and_queues(instance.clone());
    let queue = queues.next().unwrap();

    let buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create image buffer");

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        command_buffer::CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    command_buffer_builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: vulkano::format::ClearColorValue::Float([0.0, 1.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .unwrap()
        .copy_image_to_buffer(command_buffer::CopyImageToBufferInfo::image_buffer(
            image.clone(),
            buffer.clone(),
        ))
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();

    let buffer_content = buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, buffer_content).unwrap();

    image.save("image.png").unwrap();
}

pub fn do_compute_pipeline(ctx: Context) {
    let (device, queue_family_index, mut queues) = (ctx.device, ctx.queue_family_index, ctx.queues);
    let queue = queues.next().unwrap();
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let data_iter = 0..65536u32;
    let data_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        vulkano::buffer::BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE
                | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data_iter,
    )
    .expect("Could not do thing for compute pipelines");

    let shader = compute_shaders::load(device.clone()).expect("failed to create shader module");
    let cs = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("failed to create compute pipeline");

    let descriptor_set_allocator =
        StandardDescriptorSetAllocator::new(device.clone(), Default::default());
    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();

    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .unwrap();
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::buffer(0, data_buffer.clone())],
        [],
    )
    .unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        command_buffer::CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let work_group_counts = [1024, 1, 1];

    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            vulkano::pipeline::PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            descriptor_set_layout_index as u32,
            descriptor_set,
        )
        .unwrap()
        .dispatch(work_group_counts)
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();
    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let content = &data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }
}

mod compute_shaders {
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(set = 0, binding = 0) buffer Data {
                uint data[];
            } buf;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                buf.data[idx] *= 12;
            }
        ",
    }
}

mod fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) out vec4 f_color;
            layout(location = 1) in vec2 pos;

            void main() {
                float t_along_x = (pos.x + gl_FragCoord.x)/1024.0;
                float t_along_y = (pos.y + gl_FragCoord.y)/1024.0;
                float r = t_along_x;
                float b = t_along_y;
                vec3 pixel_color = vec3(r, 0.0, b);
                f_color = vec4(pixel_color, 1.0);
            }
        ",
    }
}

mod vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;
            layout(location = 1) out vec2 pos;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                pos = gl_Position.xy;
            }
        ",
    }
}

mod mandelbrot_compute_shader {
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460
            layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

            layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;

            void main() {
                vec2 norm_coordinates = (gl_GlobalInvocationID.xy + vec2(0.5)) / vec2(imageSize(img));
                vec2 c = (norm_coordinates - vec2(0.5)) * 2.0 - vec2(1.0, 0.0);

                vec2 z = vec2(0.0, 0.0);
                float i;
                for (i = 0.0; i < 1.0; i += 0.005) {
                    z = vec2(
                        z.x * z.x - z.y * z.y + c.x,
                        z.y * z.x + z.x * z.y + c.y
                    );

                    if (length(z) > 4.0) {
                        break;
                    }
                }

                vec4 to_write = vec4(log(exp(sin(vec3(gl_GlobalInvocationID)))), i);
                imageStore(img, ivec2(gl_GlobalInvocationID.xy), to_write);
            }
        ",
    }
}

pub mod vulkan_primitives {
    use std::iter::Enumerate;
    use std::sync::Arc;

    use tracing::info;
    use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
    use vulkano::command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    };
    use vulkano::command_buffer::{self, AutoCommandBufferBuilder, CopyBufferInfo};
    use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
    use vulkano::descriptor_set::{self, PersistentDescriptorSet, WriteDescriptorSet};
    use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
    use vulkano::format::Format;
    use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
    use vulkano::instance::{Instance, InstanceCreateInfo};
    use vulkano::memory::allocator::{
        AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator,
    };
    use vulkano::pipeline::compute::ComputePipelineCreateInfo;
    use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
    use vulkano::pipeline::{
        ComputePipeline, Pipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    };
    use vulkano::sync::GpuFuture;
    use vulkano::{sync, VulkanLibrary};

    pub fn create_instance() -> Arc<Instance> {
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");

        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance")
    }

    pub fn create_device_and_queues(
        instance: Arc<Instance>,
    ) -> (
        Arc<Device>,
        u32,
        impl ExactSizeIterator<Item = Arc<vulkano::device::Queue>>,
    ) {
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        (device, queue_family_index, queues)
    }

    pub fn create_memory_allocator(
        device: Arc<Device>,
    ) -> Arc<
        vulkano::memory::allocator::GenericMemoryAllocator<
            vulkano::memory::allocator::FreeListAllocator,
        >,
    > {
        Arc::new(StandardMemoryAllocator::new_default(device))
    }

    pub fn create_command_buffer_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
        StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        )
    }
}
