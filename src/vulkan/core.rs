use nalgebra::vector;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

use vulkano::swapchain::{self, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError};
use winit::dpi::Size;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;

use crate::physics::scene::Scene;
use crate::vulkan::contexts::VulkanoContext;
use crate::vulkan::primitives::get_graphics_pipeline;
use crate::{FVec2, WINDOW_LENGTH};

use super::contexts::{RapierContext, RenderContext, WindowContext};
use super::primitives::{get_framebuffers, get_render_command_buffers};
use super::type_aliases::FenceFuture;

pub const WINDOW_DIMENSION: Size = Size::Physical(winit::dpi::PhysicalSize {
    width: WINDOW_LENGTH as u32,
    height: WINDOW_LENGTH as u32,
});

/// Mega-struct for the application that manages state and the window.
pub struct WindowEventHandler {
    // Contexts.
    vk_ctx: VulkanoContext,
    window_ctx: WindowContext,
    render_ctx: RenderContext,
    rapier_ctx: RapierContext,
    // Synchronization.
    fences: Vec<Option<Arc<FenceFuture>>>,
    // frames_in_flight: usize,
    previous_fence_i: u32,
    // Miscellaneous.
    performance_stats: PerformanceStats,
    simulation_flags: SimulationFlags,
}

impl WindowEventHandler {
    pub fn new(
        event_loop: &EventLoop<()>,
        vk_ctx: VulkanoContext,
        window_ctx: WindowContext,
    ) -> Self {
        let render_ctx = RenderContext::new(event_loop, &window_ctx, &vk_ctx);
        let rapier_ctx = RapierContext::new();
        let perf_stats = PerformanceStats::default();
        let sim_flags = SimulationFlags {
            recreate_swapchain: false,
            is_paused: false,
        };
        let frames_in_flight = render_ctx.images().len();
        let fences = vec![None; frames_in_flight];
        let previous_fence_i = 0;

        Self {
            vk_ctx,
            window_ctx,
            render_ctx,
            rapier_ctx,
            // frames_in_flight,
            fences,
            previous_fence_i,
            performance_stats: perf_stats,
            simulation_flags: sim_flags,
        }
    }

    pub fn run_with_scene(mut self, mut scene: Scene, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, _| {
            let time_before_update = Instant::now();
            self.handle_window_event(&mut scene, &event);
            let fps = 1_f32 / time_before_update.elapsed().as_secs_f32();
            if fps < 10000. {
                self.performance_stats.framerates.push(fps);
            }
        });
    }

    pub fn handle_window_event(&mut self, scene: &mut Scene, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => std::process::exit(69),
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input.virtual_keycode {
                Some(winit::event::VirtualKeyCode::Q) => {
                    // Quit.
                    dbg!(self.performance_stats.avg());
                    info!("10 fps samples: {:?}", {
                        self.performance_stats
                            .framerates
                            .iter()
                            .take(10)
                            .collect::<Vec<_>>()
                    });
                    std::process::exit(0);
                }
                Some(winit::event::VirtualKeyCode::P) => {
                    // Pause.
                    self.simulation_flags.is_paused = true;
                    return;
                }
                Some(winit::event::VirtualKeyCode::R) => self.simulation_flags.is_paused = false, // Resume.
                _ => info!("{} was pressed", input.scancode),
            },
            Event::MainEventsCleared => {
                if self.simulation_flags.is_paused {
                    return;
                }

                self.rapier_ctx.physics_pipeline.step(
                    &vector![0.0, scene.gravity],
                    &self.rapier_ctx.integration_parameters,
                    &mut self.rapier_ctx.island_manager,
                    &mut self.rapier_ctx.broad_phase,
                    &mut self.rapier_ctx.narrow_phase,
                    &mut scene.rigid_body_set,
                    &mut scene.collider_set,
                    &mut self.rapier_ctx.impulse_joint_set,
                    &mut self.rapier_ctx.multibody_joint_set,
                    &mut self.rapier_ctx.ccd_solver,
                    Some(&mut self.rapier_ctx.query_pipeline),
                    &*self.rapier_ctx.physics_hooks,
                    &*self.rapier_ctx.event_handler,
                );
                scene.update_polygon_set();

                if self.simulation_flags.recreate_swapchain {
                    self.recreate_swapchain_and_pipeline();
                }

                let vertex_buffer =
                    scene.return_objects_as_vertex_buffer(self.vk_ctx.memory_allocator.clone());
                let render_command_buffers = match &self.render_ctx.render_cb {
                    Some(render_cb) => &render_cb,
                    None => &RefCell::new(
                        get_render_command_buffers(
                            &self.vk_ctx.command_buffer_allocator,
                            &self.vk_ctx.queue,
                            &self.render_ctx.graphics_pipeline,
                            &self.render_ctx.framebuffers,
                            &vertex_buffer,
                        )
                        .expect("Could not get render command buffers in frame loop."),
                    ),
                };

                let (image_idx, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(self.render_ctx.swapchain.clone(), None)
                        .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            // self.recreate_swapchain_flag = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire the next image: {e}"),
                    };

                self.simulation_flags.recreate_swapchain = if suboptimal { true } else { false };
                if let Some(image_fence) = &self.fences[image_idx as usize] {
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
                        render_command_buffers.borrow()[image_idx as usize].clone(),
                    )
                    .unwrap()
                    .then_swapchain_present(
                        self.vk_ctx.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            self.render_ctx.swapchain.clone(),
                            image_idx,
                        ),
                    )
                    .then_signal_fence_and_flush();

                self.fences[image_idx as usize] = match future.map_err(Validated::unwrap) {
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

                self.previous_fence_i = image_idx;
            }
            _ => (),
        }
    }

    fn recreate_swapchain_and_pipeline(&mut self) {
        let (new_swapchain, new_images) = self
            .render_ctx
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: self.window_ctx.window.inner_size().into(),
                ..self.render_ctx.swapchain.create_info()
            })
            .expect("failed to recreate swapchain: {e}");

        self.render_ctx.swapchain = new_swapchain;
        self.render_ctx.framebuffers = get_framebuffers(&new_images, &self.render_ctx.render_pass);
        self.render_ctx.viewport.extent = self.window_ctx.window.inner_size().into();
        self.render_ctx.graphics_pipeline = get_graphics_pipeline(
            self.vk_ctx.device.clone(),
            self.render_ctx.vs.clone(),
            self.render_ctx.fs.clone(),
            self.render_ctx.render_pass.clone(),
            self.render_ctx.viewport.clone(),
        );
    }
}

/// User-options for the application.
#[derive(Default)]
struct SimulationFlags {
    pub recreate_swapchain: bool,
    pub is_paused: bool,
}

/// Performance logging for debugging.
#[derive(Default)]
struct PerformanceStats {
    pub framerates: Vec<f32>,
}
impl PerformanceStats {
    fn avg(&self) -> f32 {
        self.framerates.iter().sum::<f32>() / (self.framerates.len() as f32)
    }
}

/// Holds all the necessary data required for rendering.
// struct RenderContext {
//     // Render-useful fields.
//     pub render_cb: Option<RefCell<RenderCommandBuffer>>,
//     pub vs: Arc<ShaderModule>,
//     pub fs: Arc<ShaderModule>,
//     pub render_pass: Arc<RenderPass>,
//     pub graphics_pipeline: Arc<GraphicsPipeline>,
//     // Fields for render-useful types.
//     pub swapchain: Arc<Swapchain>,
//     pub framebuffers: Vec<Arc<Framebuffer>>,
//     pub images: Vec<Arc<Image>>,
//     // Front-facing, render-useful types.
//     pub viewport: Viewport,
// }
// impl RenderContext {
//     fn new(
//         event_loop: &EventLoop<()>,
//         window_ctx: &WindowContext,
//         vk_ctx: &VulkanoContext,
//     ) -> Self {
//         let vs = super::shaders::vs::load(vk_ctx.get_device().clone()).unwrap();
//         let fs = super::shaders::fs::load(vk_ctx.get_device().clone()).unwrap();
//         let (swapchain, images) = create_swapchain_and_images(window_ctx, vk_ctx, event_loop);
//         let render_pass = get_render_pass(vk_ctx.get_device().clone(), swapchain.clone());
//         let framebuffers = get_framebuffers(&images, &render_pass);
//         let viewport = Viewport {
//             extent: [WINDOW_LENGTH; 2],
//             ..Default::default()
//         };
//         let graphics_pipeline = get_graphics_pipeline(
//             vk_ctx.device.clone(),
//             vs.clone(),
//             fs.clone(),
//             render_pass.clone(),
//             viewport.clone(),
//         );

//         Self {
//             render_cb: None,
//             vs,
//             fs,
//             render_pass,
//             graphics_pipeline,
//             viewport,
//             swapchain,
//             framebuffers,
//             images,
//         }
//     }
// }

// struct RapierContext {
//     pub integration_parameters: IntegrationParameters,
//     pub physics_pipeline: PhysicsPipeline,
//     pub island_manager: IslandManager,
//     pub broad_phase: BroadPhaseMultiSap,
//     pub narrow_phase: NarrowPhase,
//     pub impulse_joint_set: ImpulseJointSet,
//     pub multibody_joint_set: MultibodyJointSet,
//     pub ccd_solver: CCDSolver,
//     pub query_pipeline: QueryPipeline,
//     pub physics_hooks: Box<dyn PhysicsHooks>,
//     pub event_handler: Box<dyn EventHandler>,
// }

// impl RapierContext {
//     fn new() -> Self {
//         let integration_parameters = IntegrationParameters::default();
//         let physics_pipeline = PhysicsPipeline::new();
//         let island_manager = IslandManager::new();
//         let broad_phase = DefaultBroadPhase::new();
//         let narrow_phase = NarrowPhase::new();
//         let impulse_joint_set = ImpulseJointSet::new();
//         let multibody_joint_set = MultibodyJointSet::new();
//         let ccd_solver = CCDSolver::new();
//         let query_pipeline = QueryPipeline::new();
//         let physics_hooks = ();
//         let event_handler = ();

//         Self {
//             integration_parameters,
//             physics_pipeline {}
//             island_manager,
//             broad_phase,
//             narrow_phase,
//             impulse_joint_set,
//             multibody_joint_set,
//             ccd_solver,
//             query_pipeline,
//             physics_hooks: Box::new(physics_hooks),
//             event_handler: Box::new(event_handler),
//         }
//     }
// }

// pub struct WindowContext {
//     pub instance: Arc<Instance>,
//     pub window: Arc<Window>,
// }
// impl WindowContext {
//     pub fn new(event_loop: &EventLoop<()>) -> Self {
//         let window = Arc::new(
//             WindowBuilder::new()
//                 .with_title("vulkys")
//                 .with_inner_size(WINDOW_DIMENSION)
//                 .with_resizable(false)
//                 .build(&event_loop)
//                 .unwrap(),
//         );
//         let (_, required_extensions) = get_required_extensions(&event_loop);
//         let library = VulkanLibrary::new().expect("could not find local vulkan");
//         let instance = Instance::new(
//             library,
//             InstanceCreateInfo {
//                 enabled_extensions: required_extensions,
//                 ..Default::default()
//             },
//         )
//         .expect("failed to create instance");

//         Self { instance, window }
//     }

//     pub fn window(&self) -> Arc<Window> {
//         self.window.clone()
//     }
//     pub fn instance(&self) -> Arc<Instance> {
//         self.instance.clone()
//     }
// }

// #[derive(Clone)]
// pub struct VulkanoContext {
//     pub device: Arc<Device>,
//     pub queue_family_index: u32,
//     pub queue: Arc<Queue>,
//     // Allocators.
//     pub memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
//     pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
// }
// impl VulkanoContext {
//     pub fn with_window_context(win_ctx: &WindowContext, event_loop: &EventLoop<()>) -> Self {
//         let (device, queue_family_index, queue) =
//             primitives::select_device_and_queue(win_ctx, event_loop);
//         let memory_allocator = create_memory_allocator(device.clone());
//         let command_buffer_allocator = create_command_buffer_allocator(device.clone());

//         Self {
//             device,
//             queue_family_index,
//             queue,

//             memory_allocator,
//             command_buffer_allocator: Arc::new(command_buffer_allocator),
//         }
//     }
//     pub fn get_device(&self) -> Arc<Device> {
//         self.device.clone()
//     }
//     pub fn get_queue_family_index(&self) -> u32 {
//         self.queue_family_index
//     }
//     pub fn get_command_buffer_allocator(&self) -> Arc<StandardCommandBufferAllocator> {
//         self.command_buffer_allocator.clone()
//     }
// }

#[derive(BufferContents, Vertex, Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CustomVertex {
    #[format(R32G32_SFLOAT)]
    pub position_in: FVec2,
    #[format(R8G8B8A8_UNORM)]
    pub color: [u8; 4],
}
