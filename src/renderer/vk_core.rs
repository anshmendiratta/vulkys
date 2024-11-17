#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::ops::Deref;
use std::sync::Arc;
use std::{thread, time};

use eframe::EventLoopBuilder;
use egui::Vec2;
use image::{ImageBuffer, Rgba};
use tracing::{error, event, info, span, Level};

use vulkano::buffer::{BufferContents, Subbuffer};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState;
use vulkano::pipeline::graphics::input_assembly::{InputAssemblyState, PrimitiveTopology};
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{
    self, AutoCommandBufferBuilder, ClearColorImageInfo, CopyBufferInfo, CopyImageToBufferInfo,
    PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo,
};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{self, PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
};
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
    ComputePipeline, DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
    PipelineShaderStageCreateInfo,
};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{self, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::{GpuFuture, PipelineStages};
use vulkano::{library, single_pass_renderpass, sync, Validated, VulkanError, VulkanLibrary};
use winit::dpi::{PhysicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{self, ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::physics::rigidbody::RigidBody;

use super::shaders;
use super::vk_prims::{
    self, create_command_buffer_allocator, create_memory_allocator, create_swapchain_and_images,
    get_framebuffers, get_required_extensions, select_device_and_queue,
};
use super::vk_proc_func::{generate_polygon_vertices, Polygon};

const WINDOW_LENGTH: f32 = 1000.;
const WINDOW_DIMENSION: Size = Size::Physical(winit::dpi::PhysicalSize {
    width: WINDOW_LENGTH as u32,
    height: WINDOW_LENGTH as u32,
});

pub struct WindowEventHandler {
    #[doc = "Vulkano context"]
    vulkancx: VulkanoContext,
    #[doc = "Window context"]
    windowcx: WindowContext,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    images: Vec<Arc<Image>>,
    render_pass: Arc<RenderPass>,
    graphics_pipeline: Arc<GraphicsPipeline>,
}

impl WindowEventHandler {
    pub fn new() -> Self {
        let windowcx = WindowContext::new();
        let vulkancx = VulkanoContext::with_window_context(&windowcx);
        let required_extensions = Surface::required_extensions(&windowcx.event_loop);
        let library = VulkanLibrary::new().expect("no local vulkan lib");
        let (swapchain, images) = create_swapchain_and_images(&windowcx, &vulkancx);
        let render_pass = get_render_pass(vulkancx.device.clone(), &swapchain);
        let framebuffers = get_framebuffers(&images, &render_pass);
        let vs = super::shaders::vertex_shader::load(vulkancx.device.clone()).unwrap();
        let fs = super::shaders::fragment_shader::load(vulkancx.device.clone()).unwrap();
        let graphics_pipeline = get_graphics_pipeline(
            vulkancx.device.clone(),
            vs,
            fs,
            render_pass.clone(),
            windowcx.rendercx.viewport.clone(),
        );
        Self {
            vulkancx,
            windowcx,
            swapchain,
            framebuffers,
            images,
            render_pass,
            graphics_pipeline,
        }
    }

    pub fn run(mut self) {
        // let objects: Vec<Polygon>  =
        // self.run_inner(objects);
    }
    pub fn run_inner(mut self) {
        let library = VulkanLibrary::new().expect("can't find vulkan library");
        let physical_device = vk_prims::select_physical_device(&self.windowcx);
        let surface =
            Surface::from_window(self.windowcx.instance.clone(), self.windowcx.window.clone())
                .expect("could not create window");
        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");
        let dimensions = self.windowcx.window.inner_size();
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;
        let queue = self.vulkancx.queue;
        let memory_allocator = create_memory_allocator(self.vulkancx.device.clone());

        // Polygon construction
        let central_vertices: Vec<CustomVertex> = vec![
            CustomVertex {
                position_in: [0.5; 2],
            },
            CustomVertex {
                position_in: [-0.5; 2],
            },
        ];
        // NOTE: Length of these two vectors should be the same
        // TODO: Rewrite later, refactor too
        // Type: Vec<[CustomVertex; 3]>
        let polygon_vector: Vec<Polygon> = [4 as u8, 5 as u8]
            .iter()
            .enumerate()
            .map(|(i, obj)| {
                // let vertex_count = obj.get_vertex_count();
                generate_polygon_vertices(*obj as u8, central_vertices[i].clone())
            })
            .collect();

        let vertex_buffer_data: Vec<CustomVertex> = {
            let mut buffer_data: Vec<CustomVertex> = Vec::with_capacity(polygon_vector.len() * 3);
            for polygon in polygon_vector {
                for triangle in polygon {
                    let [a, b, c] = triangle;
                    buffer_data.push(a);
                    buffer_data.push(b);
                    buffer_data.push(c);
                }
            }

            buffer_data
        };
        dbg!(&vertex_buffer_data);
        dbg!(&vertex_buffer_data.len());
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
            vertex_buffer_data,
        )
        .unwrap();

        let vs = super::shaders::vertex_shader::load(self.vulkancx.device.clone()).unwrap();
        let fs = super::shaders::fragment_shader::load(self.vulkancx.device.clone()).unwrap();

        let command_buffer_allocator =
            create_command_buffer_allocator(self.vulkancx.device.clone());
        let mut command_buffers = get_command_buffers(
            &command_buffer_allocator,
            &queue,
            &self.graphics_pipeline,
            &self.framebuffers,
            &vertex_buffer,
        );

        let frames_in_flight = self.images.len();
        let mut fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; frames_in_flight];
        let mut previous_fence_i = 0;

        let mut window_resized = false;
        let mut recreate_swapchain = false;

        self.windowcx
            .event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    window_resized = true;
                }
                Event::MainEventsCleared => {
                    if window_resized || recreate_swapchain {
                        recreate_swapchain = false;

                        let new_dimensions = self.windowcx.window.inner_size();
                        let (new_swapchain, new_images) = self
                            .swapchain
                            .recreate(SwapchainCreateInfo {
                                image_extent: new_dimensions.into(),
                                ..self.swapchain.create_info()
                            })
                            .expect("failed to recreate swapchain: {e}");
                        self.swapchain = new_swapchain;
                        self.framebuffers = get_framebuffers(&new_images, &self.render_pass);

                        if window_resized {
                            window_resized = false;

                            self.windowcx.rendercx.viewport.extent = new_dimensions.into();
                            self.graphics_pipeline = get_graphics_pipeline(
                                self.vulkancx.device.clone(),
                                vs.clone(),
                                fs.clone(),
                                self.render_pass.clone(),
                                self.windowcx.rendercx.viewport.clone(),
                            );
                            command_buffers = get_command_buffers(
                                &command_buffer_allocator,
                                &queue,
                                &self.graphics_pipeline,
                                &self.framebuffers,
                                &vertex_buffer,
                            );
                        }
                    }

                    let (image_i, suboptimal, acquire_future) =
                        match swapchain::acquire_next_image(self.swapchain.clone(), None)
                            .map_err(Validated::unwrap)
                        {
                            Ok(r) => r,
                            Err(VulkanError::OutOfDate) => {
                                recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("failed to acquire the next image: {e}"),
                        };

                    if suboptimal {
                        recreate_swapchain = true;
                    }

                    if let Some(image_fence) = &fences[image_i as usize] {
                        image_fence.wait(None).unwrap();
                    }

                    let previous_fence = match fences[previous_fence_i as usize].clone() {
                        None => {
                            let mut now = sync::now(self.vulkancx.device.clone());
                            now.cleanup_finished();

                            now.boxed()
                        }
                        Some(fence) => fence.boxed(),
                    };

                    let future = previous_fence
                        .join(acquire_future)
                        .then_execute(queue.clone(), command_buffers[image_i as usize].clone())
                        .unwrap()
                        .then_swapchain_present(
                            queue.clone(),
                            SwapchainPresentInfo::swapchain_image_index(
                                self.swapchain.clone(),
                                image_i,
                            ),
                        )
                        .then_signal_fence_and_flush();

                    fences[image_i as usize] = match future.map_err(Validated::unwrap) {
                        Ok(value) => Some(Arc::new(value)),
                        Err(VulkanError::OutOfDate) => {
                            recreate_swapchain = true;
                            None
                        }
                        Err(e) => {
                            error!("failed to flush future: {e}");
                            None
                        }
                    };

                    previous_fence_i = image_i;
                }
                _ => (),
            });
    }
    pub fn vulkancx(&self) -> VulkanoContext {
        self.vulkancx.clone()
    }
    pub fn windowcx(&self) -> &WindowContext {
        &self.windowcx
    }
}

pub struct RenderContext {
    scale_factor: f64,
    viewport: Viewport,
}

impl RenderContext {
    fn new(window_inner_size: PhysicalSize<u32>) -> Self {
        let scale_factor = 1.0;
        let viewport = Viewport {
            extent: window_inner_size.into(),
            ..Default::default()
        };
        RenderContext {
            scale_factor,
            viewport,
        }
    }
    fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
    fn viewport(&self) -> Viewport {
        self.viewport.clone()
    }
}

pub struct WindowContext {
    pub instance: Arc<Instance>,
    pub window: Arc<Window>,
    event_loop: EventLoop<()>,
    rendercx: RenderContext,
}

impl WindowContext {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Arc::new(
            WindowBuilder::new()
                .with_title("vulkys")
                .with_inner_size(WINDOW_DIMENSION)
                .with_resizable(false)
                .build(&event_loop)
                .unwrap(),
        );
        let (_, required_extensions) = get_required_extensions(&event_loop);
        let library = VulkanLibrary::new().expect("could not find local vulkan");
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create instance");
        let rendercx = RenderContext::new(window.inner_size());

        Self {
            instance,
            window,
            event_loop,
            rendercx,
        }
    }
    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }
    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}

#[derive(Clone)]
pub struct VulkanoContext {
    pub(crate) device: Arc<Device>,
    queue_family_index: u32,
    queue: Arc<Queue>,
}

impl VulkanoContext {
    pub fn with_window_context(win_ctx: &WindowContext) -> Self {
        let library = VulkanLibrary::new().expect("can't find vulkan library dll");
        let (device, queue_family_index, queue) = vk_prims::select_device_and_queue(win_ctx);
        let (_, required_extensions) = get_required_extensions(&win_ctx.event_loop());

        Self {
            device,
            queue_family_index,
            queue,
        }
    }
}

#[derive(BufferContents, Vertex, Debug, Clone)]
#[repr(C)]
pub struct CustomVertex {
    #[format(R32G32_SFLOAT)]
    pub position_in: [f32; 2],
}

fn get_command_buffers(
    command_buffer_allocator: &StandardCommandBufferAllocator,
    queue: &Arc<Queue>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Subbuffer<[CustomVertex]>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    framebuffers
        .iter()
        .map(|framebuffer| {
            let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                command_buffer_allocator,
                queue.queue_family_index(),
                command_buffer::CommandBufferUsage::MultipleSubmit,
            )
            .unwrap();

            command_buffer_builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.01, 0.01, 0.01, 1.0].into())],
                        ..command_buffer::RenderPassBeginInfo::framebuffer(framebuffer.clone())
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
                .draw(vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap()
                .end_render_pass(SubpassEndInfo::default())
                .unwrap();

            command_buffer_builder.build().unwrap()
        })
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
    let vertex_shader_state = CustomVertex::per_vertex()
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
            input_assembly_state: Some(InputAssemblyState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            }),
            viewport_state: Some(ViewportState {
                viewports: [viewport].into(),
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

pub fn get_compute_pipeline(ctx: VulkanoContext) {
    let (device, queue_family_index, queue) = (ctx.device, ctx.queue_family_index, ctx.queue);
    let memory_allocator = vk_prims::create_memory_allocator(device.clone());

    let shader = super::shaders::compute_shaders::load(device.clone())
        .expect("failed to create shader module");
    let cs = shader.entry_point("main").unwrap();

    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let data: Vec<Vec2> = Vec::from([Vec2::new(0.0, 0.0); 64]);
    let data_buffer = Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data,
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

    let command_buffer_allocator = vk_prims::create_command_buffer_allocator(device.clone());
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
        .dispatch([1, 1, 1])
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();
    let future = sync::now(device.clone())
        .then_execute(queue, command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();
}
