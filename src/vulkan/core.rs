#![allow(static_mut_refs)]

use glm::{Mat4, Vec4};
use nalgebra::vector;
use std::sync::Arc;
use std::time::Instant;
use tracing::{error, info};
use vulkano::buffer::BufferUsage;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
    SubpassContents,
};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::Format;
use vulkano::image::ImageUsage;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::Pipeline;
use winit::application::ApplicationHandler;
use winit::window::{Window, WindowId};

use vulkano::swapchain::{self, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use vulkano::{Validated, VulkanError, VulkanLibrary, sync};
use winit::dpi::Size;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};

use crate::WINDOW_LENGTH;
use crate::physics::scene::Scene;
use crate::vulkan::primitives::create_pipeline;

use super::camera::{self, CAMERA};
use super::contexts::{PerformanceStats, RapierContext, RenderContext, SimulationFlags};
use super::primitives::{create_framebuffers, get_required_extensions};

pub const WINDOW_DIMENSION: Size = Size::Physical(winit::dpi::PhysicalSize {
    width: WINDOW_LENGTH as u32,
    height: WINDOW_LENGTH as u32,
});

/// Mega-struct for the application that manages state and the window.
pub struct App {
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub queue_family_index: u32,
    pub queue: Arc<Queue>,
    // Allocators.
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    // Contexts.
    scene: Scene,
    render_cx: Option<RenderContext>,
    rapier_cx: RapierContext,
    // Miscellaneous.
    performance_stats: PerformanceStats,
    simulation_flags: SimulationFlags,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>, scene: Scene) -> Self {
        let rapier_ctx = RapierContext::new();
        let perf_stats = PerformanceStats::default();
        let sim_flags = SimulationFlags {
            recreate_swapchain: false,
            is_paused: false,
        };

        let (device_extensions, instance_extensions) = get_required_extensions(event_loop);

        let library = VulkanLibrary::new().expect("could not find local vulkan");
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: instance_extensions,
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        let (physical_device, queue_family_index) = {
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
                                && p.presentation_support(i as u32, &event_loop)
                                    .unwrap_or(false)
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
        };
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

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        Self {
            device,
            queue_family_index,
            queue,
            memory_allocator,
            command_buffer_allocator,
            render_cx: None,
            rapier_cx: rapier_ctx,
            // fences,
            // previous_fence_i,
            performance_stats: perf_stats,
            simulation_flags: sim_flags,
            instance,
            scene,
        }
    }

    // pub fn run_with_scene(mut self, mut scene: Scene, event_loop: EventLoop<()>) {
    //     event_loop.run(move |event, _| {
    //     });
    // }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let dimensions = window.inner_size();
        let surface = Surface::from_window(self.instance.clone(), window.clone())
            .expect("could not create window");

        let (swapchain, images) = {
            let surface_capabilities = self
                .device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .expect("failed to get surface capabilities");
            let composite_alpha = surface_capabilities
                .supported_composite_alpha
                .into_iter()
                .next()
                .unwrap();
            let (image_format, _) = self
                .device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0];

            Swapchain::new(
                self.device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count + 1,
                    image_format,
                    image_extent: dimensions.into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    // present_mode: PresentMode::Fifo,
                    composite_alpha,
                    ..Default::default()
                },
            )
            .unwrap()
        };

        let render_pass = vulkano::single_pass_renderpass!(
            self.device.clone(),
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
        .unwrap();

        let vs = vs::load(self.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap();
        let fs = fs::load(self.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap();

        let framebuffers = create_framebuffers(&self.memory_allocator, &images, &render_pass);

        let window_size = window.inner_size();
        let pipeline = create_pipeline(
            self.device.clone(),
            window_size,
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
        );

        let fences = vec![None; images.len()];
        let previous_fence_i = 0;

        self.render_cx = Some(RenderContext {
            window,
            vs,
            fs,
            render_pass,
            pipeline,
            swapchain,
            framebuffers,
            fences,
            frames_in_flight: images.len() as usize,
            previous_fence_i,
        })
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let rcx = self.render_cx.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                dbg!(self.performance_stats.avg());
                info!("10 fps samples: {:?}", {
                    self.performance_stats
                        .framerates
                        .iter()
                        .take(10)
                        .collect::<Vec<_>>()
                });

                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: logical,
                        ..
                    },
                ..
            } => match logical.to_text() {
                Some("p") => {
                    self.simulation_flags.is_paused = true;
                    return;
                }
                // Camera controls.
                Some("d") => {
                    unsafe { camera::CAMERA.increment_theta() };
                }
                Some("a") => {
                    unsafe { camera::CAMERA.decrement_theta() };
                }
                Some("w") => {
                    unsafe { camera::CAMERA.increment_phi() };
                }
                Some("s") => {
                    unsafe { camera::CAMERA.decrement_phi() };
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
            WindowEvent::RedrawRequested => {
                if self.simulation_flags.is_paused {
                    return;
                }

                // Performance logging start.
                let time_before_update = Instant::now();

                // Physics.
                self.rapier_cx.physics_pipeline.step(
                    &vector![0., self.scene.gravity, 0.],
                    &self.rapier_cx.integration_parameters,
                    &mut self.rapier_cx.island_manager,
                    &mut self.rapier_cx.broad_phase,
                    &mut self.rapier_cx.narrow_phase,
                    &mut self.scene.rigid_body_set,
                    &mut self.scene.collider_set,
                    &mut self.rapier_cx.impulse_joint_set,
                    &mut self.rapier_cx.multibody_joint_set,
                    &mut self.rapier_cx.ccd_solver,
                    // Some(&mut self.rapier_ctx.query_pipeline),
                    &*self.rapier_cx.physics_hooks,
                    &*self.rapier_cx.event_handler,
                );

                if self.simulation_flags.recreate_swapchain {
                    let (new_swapchain, new_images) = rcx
                        .swapchain
                        .recreate(SwapchainCreateInfo {
                            image_extent: rcx.window.inner_size().into(),
                            ..rcx.swapchain.create_info()
                        })
                        .expect("failed to recreate swapchain: {e}");

                    rcx.swapchain = new_swapchain;
                    rcx.framebuffers =
                        create_framebuffers(&self.memory_allocator, &new_images, &rcx.render_pass);
                    rcx.pipeline = create_pipeline(
                        self.device.clone(),
                        rcx.window.inner_size(),
                        rcx.vs.clone(),
                        rcx.fs.clone(),
                        rcx.render_pass.clone(),
                    );
                }

                let (image_idx, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(rcx.swapchain.clone(), None)
                        .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            // self.recreate_swapchain_flag = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire the next image: {e}"),
                    };

                // Set up for drawing.
                let vertex_buffer = self
                    .scene
                    .return_vertex_buffer(self.memory_allocator.clone());
                let normal_buffer = self
                    .scene
                    .return_normal_buffer(self.memory_allocator.clone());
                let index_buffer = self
                    .scene
                    .return_index_buffer(self.memory_allocator.clone());

                let uniform_buffer = {
                    let view = unsafe { CAMERA.to_view_matrix() };
                    let aspect_ratio = rcx.swapchain.image_extent()[0] as f32
                        / rcx.swapchain.image_extent()[1] as f32;
                    let projection =
                        glm::perspective_rh(aspect_ratio, glm::pi::<f32>() / 3., 0.1, 100.);
                    let model = Mat4::from_diagonal(&Vec4::new(1., 1., 1., 1.));

                    let uniforms = vs::Data {
                        model: model.data.0,
                        view: view.data.0,
                        proj: projection.data.0,
                    };

                    let uniform_buffer_allocator = SubbufferAllocator::new(
                        self.memory_allocator.clone(),
                        SubbufferAllocatorCreateInfo {
                            buffer_usage: BufferUsage::UNIFORM_BUFFER,
                            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                            ..Default::default()
                        },
                    );
                    let buffer = uniform_buffer_allocator.allocate_sized().unwrap();
                    *buffer.write().unwrap() = uniforms;

                    buffer
                };

                // Descriptor set.
                let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
                    self.device.clone(),
                    Default::default(),
                ));
                let descriptor_set_layout = rcx.pipeline.layout().set_layouts()[0].clone();
                let descriptor_set = DescriptorSet::new(
                    descriptor_set_allocator,
                    descriptor_set_layout,
                    [WriteDescriptorSet::buffer(0, uniform_buffer)],
                    [],
                )
                .unwrap();

                // Drawing.
                let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                    self.command_buffer_allocator.clone(),
                    self.queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .unwrap();

                command_buffer_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![
                                Some([0.2, 0.2, 0.2, 1.].into()),
                                Some(1_f32.into()),
                            ],
                            ..RenderPassBeginInfo::framebuffer(
                                rcx.framebuffers[image_idx as usize].clone(),
                            )
                        },
                        SubpassBeginInfo {
                            contents: SubpassContents::Inline,
                            ..Default::default()
                        },
                    )
                    .unwrap()
                    .bind_pipeline_graphics(rcx.pipeline.clone())
                    .unwrap()
                    .bind_vertex_buffers(0, (vertex_buffer.clone(), normal_buffer.clone()))
                    .unwrap()
                    .bind_index_buffer(index_buffer.clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        vulkano::pipeline::PipelineBindPoint::Graphics,
                        rcx.pipeline.layout().clone(),
                        0,
                        descriptor_set.clone(),
                    )
                    .unwrap();
                unsafe {
                    command_buffer_builder.draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                }
                .unwrap();
                command_buffer_builder
                    .end_render_pass(Default::default())
                    .unwrap();

                let command_buffer = command_buffer_builder.build().unwrap();

                // Synchronization.
                self.simulation_flags.recreate_swapchain = if suboptimal { true } else { false };
                if let Some(image_fence) = &rcx.fences[image_idx as usize] {
                    image_fence.wait(None).unwrap();
                }

                let previous_fence = match rcx.fences[rcx.previous_fence_i as usize].clone() {
                    None => {
                        let mut now = sync::now(self.device.clone());
                        now.cleanup_finished();
                        now.boxed()
                    }
                    Some(fence) => fence.boxed(),
                };
                let future = previous_fence
                    .join(acquire_future)
                    .then_execute(self.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        self.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(
                            rcx.swapchain.clone(),
                            image_idx,
                        ),
                    )
                    .then_signal_fence_and_flush();

                rcx.fences[image_idx as usize] = match future.map_err(Validated::unwrap) {
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

                rcx.previous_fence_i = image_idx;

                // Performance logging end.
                let fps = 1_f32 / time_before_update.elapsed().as_secs_f32();
                if fps < 10000. {
                    self.performance_stats.framerates.push(fps);
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let rcx = self.render_cx.as_mut().unwrap();
        rcx.window.request_redraw();
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
