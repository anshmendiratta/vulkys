use crate::renderer::shaders::update_cs;
use crate::renderer::shaders::update_cs::ComputeConstants;
use crate::renderer::vk_core::command_buffer::allocator::StandardCommandBufferAllocator;
use crate::renderer::vk_primitives::get_graphics_pipeline;
use eframe::WindowAttributes;
use std::sync::Arc;
use tracing::{error, info};
use vulkano::buffer::{BufferContents, Subbuffer};
use vulkano::pipeline::graphics::vertex_input::Vertex;
use winit::keyboard::{KeyCode, PhysicalKey};

use vulkano::command_buffer::{self, CommandBufferExecFuture, PrimaryAutoCommandBuffer};
use vulkano::device::{Device, Queue};
use vulkano::image::Image;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{FreeListAllocator, GenericMemoryAllocator};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
    self, PresentFuture, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
    SwapchainPresentInfo,
};
use vulkano::sync::future::{FenceSignalFuture, JoinFuture};
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError, VulkanLibrary};
use winit::application::ApplicationHandler;
use winit::dpi::Size;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::physics::scene::Scene;
use crate::{FVec2, WINDOW_LENGTH};

use super::vk_primitives::{
    self, create_command_buffer_allocator, create_memory_allocator, create_swapchain_and_images,
    get_compute_command_buffer, get_framebuffers, get_render_command_buffers, get_render_pass,
    get_required_extensions,
};

const WINDOW_DIMENSION: Size = Size::Physical(winit::dpi::PhysicalSize {
    width: WINDOW_LENGTH as u32,
    height: WINDOW_LENGTH as u32,
});

#[derive(Clone)]
pub struct RuntimeBuffers {
    pub objects_positions: Subbuffer<[[f32; 2]]>,
    pub objects_velocities: Subbuffer<[[f32; 2]]>,
    pub objects_radii: Subbuffer<[[f32; 2]]>,
}

struct SimulationFlags {
    recreate_swapchain_flag: bool,
    is_paused_flag: bool,
}

struct PerformanceStats {
    framerates: Vec<f32>,
}

impl PerformanceStats {
    fn new() -> Self {
        Self { framerates: vec![] }
    }
    fn avg(&self) -> f32 {
        self.framerates.iter().sum::<f32>() / self.framerates.len() as f32
    }
}

struct RenderContext {
    cs: Arc<ShaderModule>,
    compute_command_buffer: Arc<PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    graphics_pipeline: Arc<GraphicsPipeline>,

    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    images: Vec<Arc<Image>>,

    viewport: Viewport,
}

type SwapchainJoinFuture = JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>;
type FenceFuture = FenceSignalFuture<PresentFuture<CommandBufferExecFuture<SwapchainJoinFuture>>>;
pub struct WindowEventHandler {
    scene: Scene,
    vk_ctx: VulkanoContext,
    wincx: WindowContext,
    rcx: RenderContext,

    runtime_buffers: RuntimeBuffers,

    fences: Vec<Option<Arc<FenceFuture>>>,
    frames_in_flight: usize,
    previous_fence_i: u32,

    perf_stats: PerformanceStats,
    sim_flags: SimulationFlags,
}

impl ApplicationHandler for WindowEventHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.wincx.window = Some(Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        ));
        let surface =
            Surface::from_window(self.wincx.instance(), self.wincx.window().unwrap()).unwrap();
        let window_size = self.wincx.window().unwrap().inner_size();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => std::process::exit(0),
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyQ) => {
                    dbg!("fps avg: {}", self.perf_stats.avg());
                    info!("10 fps samples: {:?}", self.perf_stats.framerates);
                    std::process::exit(0);
                }
                PhysicalKey::Code(KeyCode::KeyP) => {
                    self.sim_flags.is_paused_flag = true;
                    return;
                }
                PhysicalKey::Code(KeyCode::KeyR) => self.sim_flags.is_paused_flag = false,
                _ => info!("{:?} was pressed", event.logical_key),
            },
            WindowEvent::RedrawRequested => {
                if self.sim_flags.is_paused_flag {
                    return;
                }

                self.scene.update_with_buffers(
                    self.vk_ctx.device(),
                    self.vk_ctx.queue(),
                    self.rcx.compute_command_buffer.clone(),
                    self.runtime_buffers.clone(),
                );

                let (new_swapchain, new_images) = self
                    .rcx
                    .swapchain
                    .recreate(SwapchainCreateInfo {
                        image_extent: self.wincx.window.clone().unwrap().inner_size().into(),
                        ..self.rcx.swapchain.create_info()
                    })
                    .expect("failed to recreate swapchain: {e}");
                self.rcx.swapchain = new_swapchain;
                self.rcx.framebuffers = get_framebuffers(&new_images, &self.rcx.render_pass);
                self.rcx.viewport.extent = self.wincx.window.clone().unwrap().inner_size().into();
                self.rcx.graphics_pipeline = get_graphics_pipeline(
                    self.vk_ctx.device.clone(),
                    self.rcx.vs.clone(),
                    self.rcx.fs.clone(),
                    self.rcx.render_pass.clone(),
                    self.rcx.viewport.clone(),
                );

                let vertex_buffer = self
                    .scene
                    .return_objects_as_vertex_buffer(self.vk_ctx.device.clone());
                let command_buffers = get_render_command_buffers(
                    &self.vk_ctx.command_buffer_allocator,
                    &self.vk_ctx.queue,
                    &self.rcx.graphics_pipeline,
                    &self.rcx.framebuffers,
                    &vertex_buffer,
                )
                .unwrap();

                let (image_i, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(self.rcx.swapchain.clone(), None)
                        .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            // self.recreate_swapchain_flag = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire the next image: {e}"),
                    };

                self.sim_flags.recreate_swapchain_flag = if suboptimal { true } else { false };
                if let Some(image_fence) = &self.fences[image_i as usize] {
                    image_fence.wait(None).unwrap();
                }
                let previous_fence = match self.fences[self.previous_fence_i as usize].clone() {
                    None => {
                        let mut now = sync::now(self.vk_ctx.device.clone());
                        now.cleanup_finished();
                        now.boxed()
                    }
                    Some(fence) => fence.boxed(),
                };

                let future = previous_fence
                    .join(acquire_future)
                    .then_execute(
                        self.vk_ctx.queue.clone(),
                        command_buffers[image_i as usize].clone(),
                    )
                    .unwrap()
                    .then_swapchain_present(
                        self.vk_ctx.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            self.rcx.swapchain.clone(),
                            image_i,
                        ),
                    )
                    .then_signal_fence_and_flush();

                self.fences[image_i as usize] = match future.map_err(Validated::unwrap) {
                    Ok(value) => Some(Arc::new(value)),
                    Err(VulkanError::OutOfDate) => {
                        // FIX: crashes on MoltenVK
                        // self.recreate_swapchain_flag = true;
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
}

impl WindowEventHandler {
    pub fn new(
        event_loop: &EventLoop<()>,
        runtime_buffers: RuntimeBuffers,
        vk_ctx: VulkanoContext,
        wincx: WindowContext,
        scene: Scene,
        push_constants: ComputeConstants,
    ) -> anyhow::Result<Self> {
        let (_, required_extensions) = get_required_extensions(event_loop);
        let library = VulkanLibrary::new().expect("no local vulkan lib");
        let (swapchain, images) = create_swapchain_and_images(&wincx, &vk_ctx, event_loop);
        let render_pass = get_render_pass(vk_ctx.device().clone(), &swapchain);
        let framebuffers = get_framebuffers(&images, &render_pass);
        let surface =
            Surface::from_window(wincx.instance().clone(), wincx.window().clone().unwrap())
                .expect("could not create window");
        let physical_device = vk_primitives::select_physical_device(&wincx, &event_loop);
        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");
        let dimensions = wincx.window().clone().unwrap().inner_size();
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;
        let cs = update_cs::load(vk_ctx.device().clone())?;
        let vs = super::shaders::vs::load(vk_ctx.device().clone())?;
        let fs = super::shaders::fs::load(vk_ctx.device().clone())?;
        let (swapchain, images) = create_swapchain_and_images(&wincx, &vk_ctx, event_loop);
        let render_pass = get_render_pass(vk_ctx.device().clone(), &swapchain);
        let viewport = Viewport {
            extent: [WINDOW_LENGTH; 2],
            ..Default::default()
        };
        let graphics_pipeline = get_graphics_pipeline(
            vk_ctx.device().clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            viewport.clone(),
        );
        let compute_command_buffer = get_compute_command_buffer(
            vk_ctx.clone(),
            cs.clone(),
            vec![
                runtime_buffers.objects_positions.clone(),
                runtime_buffers.objects_velocities.clone(),
                // FIX: Remove need for the radii buffer to be [f32; 2].
                runtime_buffers.objects_radii.clone(),
            ],
            Some(push_constants),
            [push_constants.num_objects, 1, 1],
        )?
        .build()?;
        let render_ctx = RenderContext {
            cs,
            compute_command_buffer,
            vs,
            fs,
            render_pass,
            graphics_pipeline,
            swapchain,
            framebuffers,
            images,
            viewport,
        };
        let perf_stats = PerformanceStats::new();
        let sim_flags = SimulationFlags {
            recreate_swapchain_flag: false,
            is_paused_flag: false,
        };
        let frames_in_flight = render_ctx.images.len();
        let fences = vec![None; frames_in_flight];
        let previous_fence_i = 0;

        Ok(Self {
            vk_ctx,
            wincx,
            rcx: render_ctx,
            frames_in_flight,
            fences,
            previous_fence_i,
            perf_stats,
            sim_flags,
            runtime_buffers,
            scene,
        })
    }

    pub fn vulkancx(&self) -> VulkanoContext {
        self.vk_ctx.clone()
    }
    pub fn windowcx(&self) -> &WindowContext {
        &self.wincx
    }
}

pub struct WindowContext {
    instance: Arc<Instance>,
    window: Option<Arc<Window>>,
}

impl WindowContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let (_, required_extensions) = get_required_extensions(event_loop);
        let library = VulkanLibrary::new().expect("could not find local vulkan");
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        Self {
            instance,
            window: None,
        }
    }
    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }
    pub fn window(&self) -> Option<Arc<Window>> {
        if self.window.clone().is_some() {
            return self.window.clone();
        }
        None
    }
}

#[derive(Clone)]
pub struct VulkanoContext {
    device: Arc<Device>,
    queue_family_index: u32,
    queue: Arc<Queue>,

    memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl VulkanoContext {
    pub fn from_window_context(win_ctx: &WindowContext, event_loop: &EventLoop<()>) -> Self {
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
    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }
    pub fn queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    pub fn memory_allocator(&self) -> Arc<GenericMemoryAllocator<FreeListAllocator>> {
        self.memory_allocator.clone()
    }
    pub fn command_buffer_allocator(&self) -> Arc<StandardCommandBufferAllocator> {
        self.command_buffer_allocator.clone()
    }
}

#[derive(BufferContents, Vertex, Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomVertex {
    #[format(R32G32_SFLOAT)]
    pub position_in: FVec2,
    #[format(R8G8B8A8_UNORM)]
    pub color: [u8; 4],
}
