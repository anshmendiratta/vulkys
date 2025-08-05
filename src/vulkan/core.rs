#![allow(static_mut_refs)]

use nalgebra::vector;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};

use vulkano::swapchain::{self, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use vulkano::{Validated, VulkanError, sync};
use winit::dpi::Size;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;

use crate::WINDOW_LENGTH;
use crate::physics::scene::Scene;
use crate::vulkan::contexts::VulkanoContext;
use crate::vulkan::primitives::get_graphics_pipeline;

use super::camera;
use super::contexts::{
    PerformanceStats, RapierContext, RenderContext, SimulationFlags, WindowContext,
};
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
        event_loop.run(move |event, _| {
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
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: logical,
                                ..
                            },
                        ..
                    },
                ..
            } => match logical.to_text() {
                // Quit.
                Some("q") => {
                    dbg!(self.performance_stats.avg());
                    info!("10 fps samples: {:?}", {
                        self.performance_stats
                            .framerates
                            .iter()
                            .take(10)
                            .collect::<Vec<_>>()
                    });
                    std::process::exit(69420);
                }
                // Pause,
                Some("p") => {
                    self.simulation_flags.is_paused = true;
                    return;
                }
                // Camera controls.
                Some("ArrowRight") => {
                    unsafe { camera::CAMERA.increment_theta() };
                }
                Some("ArrowLeft") => {
                    unsafe { camera::CAMERA.decrement_theta() };
                }
                Some("ArrowUp") => {
                    unsafe { camera::CAMERA.decrement_phi() };
                }
                Some("ArrowDown") => {
                    unsafe { camera::CAMERA.increment_phi() };
                }
                Some("=") => {
                    unsafe { camera::CAMERA.decrement_r() };
                }
                Some("-") => {
                    unsafe { camera::CAMERA.increment_r() };
                }
                // Resume.
                Some("r") => self.simulation_flags.is_paused = false, // Resume.
                _ => info!("{:?} was pressed", logical.to_text()),
            },
            Event::Resumed => {
                if self.simulation_flags.is_paused {
                    return;
                }

                self.rapier_ctx.physics_pipeline.step(
                    &vector![0., scene.gravity, 0.],
                    &self.rapier_ctx.integration_parameters,
                    &mut self.rapier_ctx.island_manager,
                    &mut self.rapier_ctx.broad_phase,
                    &mut self.rapier_ctx.narrow_phase,
                    &mut scene.rigid_body_set,
                    &mut scene.collider_set,
                    &mut self.rapier_ctx.impulse_joint_set,
                    &mut self.rapier_ctx.multibody_joint_set,
                    &mut self.rapier_ctx.ccd_solver,
                    // Some(&mut self.rapier_ctx.query_pipeline),
                    &*self.rapier_ctx.physics_hooks,
                    &*self.rapier_ctx.event_handler,
                );
                // scene.update_polygon_set();

                if self.simulation_flags.recreate_swapchain {
                    self.recreate_swapchain_and_pipeline();
                }

                let vertex_buffer =
                    scene.return_vertex_buffer(self.vk_ctx.memory_allocator.clone());
                let index_buffer = scene.return_index_buffer(self.vk_ctx.memory_allocator.clone());
                let render_command_buffers = match &self.render_ctx.render_cb {
                    Some(render_cb) => &render_cb,
                    None => &RefCell::new(
                        get_render_command_buffers(
                            self.vk_ctx.device.clone(),
                            &self.vk_ctx.memory_allocator,
                            &self.vk_ctx.command_buffer_allocator,
                            self.vk_ctx.queue.clone(),
                            self.render_ctx.graphics_pipeline.clone(),
                            self.render_ctx.framebuffers.clone(),
                            &vertex_buffer,
                            index_buffer,
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
        self.render_ctx.framebuffers = get_framebuffers(
            &self.vk_ctx.memory_allocator,
            &new_images,
            &self.render_ctx.render_pass,
        );
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

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/vulkan/vert.glsl"
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/vulkan/frag.glsl"
    }
}
