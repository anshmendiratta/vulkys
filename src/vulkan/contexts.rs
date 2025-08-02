use std::{cell::RefCell, sync::Arc};

use bytemuck::AnyBitPattern;
use glm::Mat4;
use rapier3d::prelude::{
    CCDSolver, DefaultBroadPhase, EventHandler, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsHooks, PhysicsPipeline,
};
use vulkano::{
    VulkanLibrary,
    command_buffer::allocator::StandardCommandBufferAllocator,
    device::{Device, Queue},
    image::Image,
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{FreeListAllocator, GenericMemoryAllocator},
    pipeline::{GraphicsPipeline, graphics::viewport::Viewport},
    render_pass::{Framebuffer, RenderPass},
    shader::ShaderModule,
    swapchain::Swapchain,
};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::WINDOW_LENGTH;

use super::{
    core::WINDOW_DIMENSION,
    primitives::{
        self, create_command_buffer_allocator, create_memory_allocator,
        create_swapchain_and_images, get_framebuffers, get_graphics_pipeline, get_render_pass,
        get_required_extensions,
    },
    type_aliases::RenderCommandBuffer,
};

#[derive(AnyBitPattern, Clone, Copy, Debug)]
pub struct PushConstants {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub model_matrix: Mat4,
}

impl PushConstants {
    pub fn new() -> Self {
        Self {
            view_matrix: Mat4::default(),
            projection_matrix: Mat4::default(),
            model_matrix: {
                let mut zeroes = Mat4::zeros();
                zeroes.m44 = 1.;
                zeroes
            },
        }
    }
}

/// Holds all the necessary data required for rendering.
pub struct RenderContext {
    // Render-useful fields.
    pub render_cb: Option<RefCell<RenderCommandBuffer>>,
    pub vs: Arc<ShaderModule>,
    pub fs: Arc<ShaderModule>,
    pub render_pass: Arc<RenderPass>,
    pub graphics_pipeline: Arc<GraphicsPipeline>,
    // Fields for render-useful types.
    pub swapchain: Arc<Swapchain>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    images: Vec<Arc<Image>>,
    // Front-facing, render-useful types.
    pub viewport: Viewport,
}

impl RenderContext {
    pub fn new(
        event_loop: &EventLoop<()>,
        window_ctx: &WindowContext,
        vk_ctx: &VulkanoContext,
    ) -> Self {
        let vs = super::shaders::vs::load(vk_ctx.get_device().clone()).unwrap();
        let fs = super::shaders::fs::load(vk_ctx.get_device().clone()).unwrap();
        let (swapchain, images) = create_swapchain_and_images(window_ctx, vk_ctx, event_loop);
        let render_pass = get_render_pass(vk_ctx.get_device().clone(), swapchain.clone());
        let framebuffers = get_framebuffers(&images, &render_pass);
        let viewport = Viewport {
            extent: [WINDOW_LENGTH; 2],
            ..Default::default()
        };
        let graphics_pipeline = get_graphics_pipeline(
            vk_ctx.device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            viewport.clone(),
        );

        Self {
            render_cb: None,
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

    pub fn images(&self) -> Vec<Arc<Image>> {
        self.images.clone()
    }
}

pub struct RapierContext {
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: Box<dyn PhysicsHooks>,
    pub event_handler: Box<dyn EventHandler>,
}

impl RapierContext {
    pub fn new() -> Self {
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        Self {
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            physics_hooks: Box::new(physics_hooks),
            event_handler: Box::new(event_handler),
        }
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
    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }
}

#[derive(Clone)]
pub struct VulkanoContext {
    pub device: Arc<Device>,
    pub queue_family_index: u32,
    pub queue: Arc<Queue>,
    // Allocators.
    pub memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

impl VulkanoContext {
    pub fn with_window_context(win_ctx: &WindowContext, event_loop: &EventLoop<()>) -> Self {
        let (device, queue_family_index, queue) =
            primitives::select_device_and_queue(win_ctx, event_loop);
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
    pub fn get_device(&self) -> Arc<Device> {
        self.device.clone()
    }
    pub fn get_queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    pub fn get_command_buffer_allocator(&self) -> Arc<StandardCommandBufferAllocator> {
        self.command_buffer_allocator.clone()
    }
}
