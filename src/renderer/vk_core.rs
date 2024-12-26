use crate::renderer::vk_primitives::get_graphics_pipeline;
use crate::WINDOW_LENGTH;
use eframe::WindowAttributes;
use handler::App;
use handler::RenderContext;
use std::sync::Arc;
use tracing::{error, info};
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::SwapchainCreateInfo;
use vulkano::swapchain::SwapchainPresentInfo;
use vulkano::sync::GpuFuture;
use winit::keyboard::{KeyCode, PhysicalKey};

use vulkano::{sync, Validated, VulkanError};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;

use crate::FVec2;

use super::shaders::update_cs;
use super::vk_primitives::create_swapchain_and_images;
use super::vk_primitives::get_compute_command_buffer;
use super::vk_primitives::get_render_pass;
use super::vk_primitives::{get_framebuffers, get_render_command_buffers};

pub mod handler {
    const WINDOW_DIMENSION: Size = Size::Physical(winit::dpi::PhysicalSize {
        width: WINDOW_LENGTH as u32,
        height: WINDOW_LENGTH as u32,
    });

    use std::sync::Arc;

    use vulkano::{
        buffer::Subbuffer,
        command_buffer::{
            allocator::StandardCommandBufferAllocator, CommandBufferExecFuture,
            PrimaryAutoCommandBuffer,
        },
        device::{Device, Queue},
        image::Image,
        instance::Instance,
        memory::allocator::{FreeListAllocator, GenericMemoryAllocator},
        pipeline::{graphics::viewport::Viewport, GraphicsPipeline},
        render_pass::{Framebuffer, RenderPass},
        shader::ShaderModule,
        swapchain::{PresentFuture, Swapchain, SwapchainAcquireFuture},
        sync::{
            future::{FenceSignalFuture, JoinFuture},
            GpuFuture,
        },
        VulkanLibrary,
    };
    use winit::{dpi::Size, event_loop::EventLoop, window::Window};

    use crate::{
        physics::scene::Scene,
        renderer::{
            shaders::update_cs::ComputeConstants,
            vk_primitives::{self, create_command_buffer_allocator, create_memory_allocator},
        },
        WINDOW_LENGTH,
    };

    type SwapchainJoinFuture = JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>;
    type FenceFuture =
        FenceSignalFuture<PresentFuture<CommandBufferExecFuture<SwapchainJoinFuture>>>;

    pub struct App {
        pub instance: Arc<Instance>,
        pub scene: Scene,
        pub rcx: Option<RenderContext>,
        pub runtime_buffers: RuntimeBuffers,
        pub push_constants: ComputeConstants,
        pub device: Arc<Device>,
        pub queue_family_index: u32,
        pub queue: Arc<Queue>,
        pub memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
        pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
        pub perf_stats: PerformanceStats,
        pub sim_flags: SimulationFlags,

        // Synchronization.
        pub fences: Vec<Option<Arc<FenceFuture>>>,
        pub frames_in_flight: usize,
        pub previous_fence_i: u32,
    }

    #[derive(Clone)]
    pub struct RenderContext {
        pub window: Arc<Window>,
        pub compute_command_buffer: Arc<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>>,
        pub cs: Arc<ShaderModule>,
        pub vs: Arc<ShaderModule>,
        pub fs: Arc<ShaderModule>,
        pub render_pass: Arc<RenderPass>,
        pub graphics_pipeline: Arc<GraphicsPipeline>,
        pub swapchain: Arc<Swapchain>,
        pub framebuffers: Vec<Arc<Framebuffer>>,
        pub images: Vec<Arc<Image>>,
        pub viewport: Viewport,
    }

    impl App {
        pub fn new(
            instance: Arc<Instance>,
            event_loop: &EventLoop<()>,
            runtime_buffers: RuntimeBuffers,
            scene: Scene,
            push_constants: ComputeConstants,
        ) -> anyhow::Result<Self> {
            let library = VulkanLibrary::new().expect("no local vulkan lib");
            let device_queue_info =
                vk_primitives::get_device_and_queue(instance.clone(), event_loop);
            let memory_allocator = create_memory_allocator(device_queue_info.device.clone());
            let command_buffer_allocator = Arc::new(create_command_buffer_allocator(
                device_queue_info.device.clone(),
            ));
            let perf_stats = PerformanceStats::new();
            let sim_flags = SimulationFlags {
                recreate_swapchain_flag: false,
                is_paused_flag: false,
            };
            let frames_in_flight = 0;
            let fences = vec![None; frames_in_flight];
            let previous_fence_i = 0;

            Ok(Self {
                device: device_queue_info.device,
                queue: device_queue_info.queue,
                queue_family_index: device_queue_info.queue_family_index,
                memory_allocator,
                command_buffer_allocator,
                rcx: None,
                frames_in_flight,
                fences,
                previous_fence_i,
                perf_stats,
                sim_flags,
                runtime_buffers,
                scene,
                instance,
                push_constants,
            })
        }
    }

    pub struct SimulationFlags {
        pub recreate_swapchain_flag: bool,
        pub is_paused_flag: bool,
    }

    pub struct PerformanceStats {
        framerates: Vec<f32>,
    }

    #[derive(Clone)]
    pub struct RuntimeBuffers {
        pub objects_positions: Subbuffer<[[f32; 2]]>,
        pub objects_velocities: Subbuffer<[[f32; 2]]>,
        pub objects_radii: Subbuffer<[[f32; 2]]>,
    }

    impl PerformanceStats {
        pub fn new() -> Self {
            Self { framerates: vec![] }
        }
        pub fn avg(&self) -> f32 {
            self.framerates.iter().sum::<f32>() / self.framerates.len() as f32
        }
        pub fn framerates(&self) -> Vec<f32> {
            self.framerates.clone()
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );
        let window_size = window.inner_size();
        let cs = update_cs::load(self.device.clone()).unwrap();
        let vs = crate::renderer::shaders::vs::load(self.device.clone()).unwrap();
        let fs = crate::renderer::shaders::fs::load(self.device.clone()).unwrap();
        let physical_device = self.device.physical_device();
        let (swapchain, images) = create_swapchain_and_images(
            self.instance.clone(),
            self.device.clone(),
            window.clone(),
            physical_device.clone(),
        );
        let render_pass = get_render_pass(self.device.clone(), &swapchain);
        let framebuffers = get_framebuffers(&images, &render_pass);
        let viewport = Viewport {
            extent: [WINDOW_LENGTH; 2],
            ..Default::default()
        };
        let graphics_pipeline = get_graphics_pipeline(
            self.device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            viewport.clone(),
        );
        let compute_command_buffer = get_compute_command_buffer(
            self.device.clone(),
            self.device.active_queue_family_indices()[0],
            vec![
                self.runtime_buffers.objects_positions.clone(),
                self.runtime_buffers.objects_velocities.clone(),
                // FIX: Remove need for the radii buffer to be [f32; 2].
                self.runtime_buffers.objects_radii.clone(),
            ],
            cs.clone(),
            Some(self.push_constants.clone()),
            [self.push_constants.num_objects, 1, 1],
            &self.command_buffer_allocator,
        )
        .unwrap()
        .build()
        .unwrap();

        let rcx = RenderContext {
            window,
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
        self.frames_in_flight = rcx.images.len();
        self.rcx = Some(rcx);
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
                    info!("10 fps samples: {:?}", self.perf_stats.framerates());
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
                if self.rcx.is_none() {
                    return;
                };

                self.scene.update_with_buffers(
                    self.device.clone(),
                    self.queue.clone(),
                    self.rcx.clone().unwrap().compute_command_buffer.clone(),
                    self.runtime_buffers.clone(),
                );

                let (new_swapchain, new_images) = self
                    .rcx
                    .clone()
                    .unwrap()
                    .swapchain
                    .recreate(SwapchainCreateInfo {
                        image_extent: self
                            .rcx
                            .as_ref()
                            .unwrap()
                            .window
                            .clone()
                            .inner_size()
                            .into(),
                        ..self.rcx.clone().unwrap().swapchain.create_info()
                    })
                    .expect("failed to recreate swapchain: {e}");
                let framebuffers =
                    get_framebuffers(&new_images, &self.rcx.clone().unwrap().render_pass);
                let viewport_extent: [f32; 2] =
                    self.rcx.clone().unwrap().window.clone().inner_size().into();
                let graphics_pipeline = get_graphics_pipeline(
                    self.device.clone(),
                    self.rcx.clone().unwrap().vs.clone(),
                    self.rcx.clone().unwrap().fs.clone(),
                    self.rcx.clone().unwrap().render_pass.clone(),
                    self.rcx.clone().unwrap().viewport.clone(),
                );
                self.rcx = Some(RenderContext {
                    swapchain: new_swapchain,
                    images: new_images,
                    framebuffers,
                    viewport: Viewport {
                        extent: viewport_extent,
                        ..Default::default()
                    },
                    graphics_pipeline,
                    ..self.rcx.clone().unwrap()
                });

                let vertex_buffer = self
                    .scene
                    .return_objects_as_vertex_buffer(self.device.clone());
                let command_buffers = get_render_command_buffers(
                    &self.command_buffer_allocator,
                    &self.queue,
                    &self.rcx.clone().unwrap().graphics_pipeline,
                    &self.rcx.clone().unwrap().framebuffers,
                    &vertex_buffer,
                )
                .unwrap();

                let (image_i, suboptimal, acquire_future) = match swapchain::acquire_next_image(
                    self.rcx.clone().unwrap().swapchain.clone(),
                    None,
                )
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
                        let mut now = sync::now(self.device.clone());
                        now.cleanup_finished();
                        now.boxed()
                    }
                    Some(fence) => fence.boxed(),
                };

                let future = previous_fence
                    .join(acquire_future)
                    .then_execute(
                        self.queue.clone(),
                        command_buffers[image_i as usize].clone(),
                    )
                    .unwrap()
                    .then_swapchain_present(
                        self.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            self.rcx.clone().unwrap().swapchain.clone(),
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

#[derive(BufferContents, Vertex, Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomVertex {
    #[format(R32G32_SFLOAT)]
    pub position_in: FVec2,
    #[format(R8G8B8A8_UNORM)]
    pub color: [u8; 4],
}
