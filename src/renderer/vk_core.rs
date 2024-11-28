#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::renderer::vk_core::command_buffer::allocator::StandardCommandBufferAllocator;
use crate::renderer::vk_primitives::get_graphics_pipeline;
use std::sync::Arc;
use tracing::error;
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

use egui::Vec2;
use serde::Deserialize;

use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::{self, AutoCommandBufferBuilder, CommandBufferExecFuture};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::{Device, Queue};
use vulkano::image::Image;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{
    AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryTypeFilter,
};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{
    ComputePipeline, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
    PipelineShaderStageCreateInfo,
};
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
    self, PresentFuture, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
    SwapchainPresentInfo,
};
use vulkano::sync::future::{FenceSignalFuture, JoinFuture};
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError, VulkanLibrary};
use winit::dpi::Size;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use crate::physics::scene::Scene;
use crate::{FVec2, WINDOW_LENGTH};

use super::vk_primitives::{
    self, create_command_buffer_allocator, create_memory_allocator, create_swapchain_and_images,
    get_command_buffers, get_framebuffers, get_render_pass, get_required_extensions,
};

const WINDOW_DIMENSION: Size = Size::Physical(winit::dpi::PhysicalSize {
    width: WINDOW_LENGTH as u32,
    height: WINDOW_LENGTH as u32,
});

type SwapchainJoinFuture = JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>;
type FenceFuture = FenceSignalFuture<PresentFuture<CommandBufferExecFuture<SwapchainJoinFuture>>>;
pub struct WindowEventHandler {
    vk_cx: VulkanoContext,
    window_cx: WindowContext,
    render_cx: RenderContext,

    fences: Vec<Option<Arc<FenceFuture>>>,
    frames_in_flight: usize,
    previous_fence_i: u32,

    recreate_swapchain_flag: bool,
}

struct RenderContext {
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    graphics_pipeline: Arc<GraphicsPipeline>,

    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    images: Vec<Arc<Image>>,

    viewport: Viewport,
}

impl RenderContext {
    fn new(
        device: Arc<Device>,
        window_cx: &WindowContext,
        vk_cx: &VulkanoContext,
        event_loop: &EventLoop<()>,
    ) -> Self {
        let vs = super::shaders::vertex_shader::load(device.clone()).unwrap();
        let fs = super::shaders::fragment_shader::load(device.clone()).unwrap();
        let (swapchain, images) = create_swapchain_and_images(window_cx, vk_cx, event_loop);
        let render_pass = get_render_pass(device.clone(), &swapchain);
        let framebuffers = get_framebuffers(&images, &render_pass);
        let viewport = Viewport {
            extent: [WINDOW_LENGTH; 2],
            ..Default::default()
        };
        let graphics_pipeline = get_graphics_pipeline(
            device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            viewport.clone(),
        );

        Self {
            vs,
            fs,
            render_pass,
            graphics_pipeline,
            viewport,
            swapchain,
            framebuffers,
            images,
        }
    }
    fn viewport(&self) -> Viewport {
        self.viewport.clone()
    }
}

impl WindowEventHandler {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window_cx = WindowContext::new(event_loop);
        let vk_cx = VulkanoContext::with_window_context(&window_cx, event_loop);
        let required_extensions = Surface::required_extensions(event_loop);
        let library = VulkanLibrary::new().expect("no local vulkan lib");
        let (swapchain, images) = create_swapchain_and_images(&window_cx, &vk_cx, event_loop);
        let render_pass = get_render_pass(vk_cx.device.clone(), &swapchain);
        let framebuffers = get_framebuffers(&images, &render_pass);
        let render_cx = RenderContext::new(vk_cx.device.clone(), &window_cx, &vk_cx, event_loop);

        let frames_in_flight = render_cx.images.len();
        let fences = vec![None; frames_in_flight];
        let previous_fence_i = 0;

        Self {
            vk_cx,
            window_cx,
            render_cx,
            frames_in_flight,
            fences,
            previous_fence_i,
            recreate_swapchain_flag: false,
        }
    }

    pub fn run_with_scene(mut self, mut scene: Scene, event_loop: EventLoop<()>) {
        let library = VulkanLibrary::new().expect("can't find vulkan library");
        let physical_device = vk_primitives::select_physical_device(&self.window_cx, &event_loop);
        let surface = Surface::from_window(
            self.window_cx.instance.clone(),
            self.window_cx.window.clone(),
        )
        .expect("could not create window");
        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");
        let dimensions = self.window_cx.window.inner_size();
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        event_loop.run(move |event, _, _| {
            self.handle_window_event(&mut scene, &event);
        })
    }

    pub fn handle_window_event(&mut self, scene: &mut Scene, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => std::process::exit(0),
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input.scancode {
                /* Code for Q */ 16 => std::process::exit(0),
                _ => (),
            },
            Event::MainEventsCleared => {
                // NOTE: Length of these two vectors should be the same
                // TODO: Rewrite later, refactor too
                let vertex_buffer =
                    scene.return_objects_as_vertex_buffer(self.vk_cx.device.clone());

                scene.update_objects();
                scene.recreate_hash();

                let (new_swapchain, new_images) = self
                    .render_cx
                    .swapchain
                    .recreate(SwapchainCreateInfo {
                        image_extent: self.window_cx.window.inner_size().into(),
                        ..self.render_cx.swapchain.create_info()
                    })
                    .expect("failed to recreate swapchain: {e}");
                self.render_cx.swapchain = new_swapchain;
                self.render_cx.framebuffers =
                    get_framebuffers(&new_images, &self.render_cx.render_pass);
                self.render_cx.viewport.extent = self.window_cx.window.inner_size().into();
                self.render_cx.graphics_pipeline = get_graphics_pipeline(
                    self.vk_cx.device.clone(),
                    self.render_cx.vs.clone(),
                    self.render_cx.fs.clone(),
                    self.render_cx.render_pass.clone(),
                    self.render_cx.viewport.clone(),
                );
                let command_buffers = get_command_buffers(
                    &self.vk_cx.command_buffer_allocator,
                    &self.vk_cx.queue,
                    &self.render_cx.graphics_pipeline,
                    &self.render_cx.framebuffers,
                    &vertex_buffer,
                )
                .unwrap();

                let (image_i, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(self.render_cx.swapchain.clone(), None)
                        .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            self.recreate_swapchain_flag = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire the next image: {e}"),
                    };

                let other_image_i = image_i ^ 1;

                self.recreate_swapchain_flag = if suboptimal { true } else { false };
                if let Some(image_fence) = &self.fences[image_i as usize] {
                    image_fence.wait(None).unwrap();
                }
                let previous_fence = match self.fences[self.previous_fence_i as usize].clone() {
                    None => {
                        let mut now = sync::now(self.vk_cx.device.clone());
                        now.cleanup_finished();
                        now.boxed()
                    }
                    Some(fence) => fence.boxed(),
                };

                let future = previous_fence
                    .join(acquire_future)
                    .then_execute(
                        self.vk_cx.queue.clone(),
                        command_buffers[image_i as usize].clone(),
                    )
                    .unwrap()
                    .then_swapchain_present(
                        self.vk_cx.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            self.render_cx.swapchain.clone(),
                            image_i,
                        ),
                    )
                    .then_signal_fence_and_flush();

                self.fences[image_i as usize] = match future.map_err(Validated::unwrap) {
                    Ok(value) => Some(Arc::new(value)),
                    Err(VulkanError::OutOfDate) => {
                        self.recreate_swapchain_flag = true;
                        None
                    }
                    Err(e) => {
                        error!("failed to flush future: {e}");
                        None
                    }
                };

                self.previous_fence_i = image_i;
            }
            _ => (),
        }
    }
    pub fn vulkancx(&self) -> VulkanoContext {
        self.vk_cx.clone()
    }
    pub fn windowcx(&self) -> &WindowContext {
        &self.window_cx
    }
}

pub struct WindowContext {
    pub instance: Arc<Instance>,
    pub window: Arc<Window>,
}

impl WindowContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
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

        Self { instance, window }
    }
    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }
}

#[derive(Clone)]
pub struct VulkanoContext {
    pub(crate) device: Arc<Device>,
    queue_family_index: u32,
    queue: Arc<Queue>,

    memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl VulkanoContext {
    pub fn with_window_context(win_ctx: &WindowContext, event_loop: &EventLoop<()>) -> Self {
        let library = VulkanLibrary::new().expect("can't find vulkan library dll");
        let (device, queue_family_index, queue) =
            vk_primitives::select_device_and_queue(win_ctx, event_loop);
        let memory_allocator = create_memory_allocator(device.clone());
        let command_buffer_allocator = create_command_buffer_allocator(device.clone());

        Self {
            device,
            queue_family_index,
            queue,

            memory_allocator,
            command_buffer_allocator: Arc::new(command_buffer_allocator),
        }
    }
}

#[derive(BufferContents, Vertex, Debug, Clone, Deserialize, PartialEq)]
#[repr(C)]
pub struct CustomVertex {
    #[format(R32G32_SFLOAT)]
    pub position_in: FVec2,
    #[format(R8G8B8A8_UNORM)]
    pub color: [u8; 4],
}

#[derive(BufferContents)]
#[repr(C)]
pub struct Vec3 {
    r: u8,
    g: u8,
    b: u8,
}

pub fn get_compute_pipeline(ctx: VulkanoContext) {
    let (device, queue_family_index, queue) = (ctx.device, ctx.queue_family_index, ctx.queue);
    let memory_allocator = vk_primitives::create_memory_allocator(device.clone());

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

    let command_buffer_allocator = vk_primitives::create_command_buffer_allocator(device.clone());
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
