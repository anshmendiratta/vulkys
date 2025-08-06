use std::sync::Arc;

use rapier3d::prelude::{
    CCDSolver, DefaultBroadPhase, EventHandler, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJointSet, NarrowPhase, PhysicsHooks, PhysicsPipeline,
};
use vulkano::{
    pipeline::GraphicsPipeline,
    render_pass::{Framebuffer, RenderPass},
    shader::EntryPoint,
    swapchain::Swapchain,
};
use winit::window::Window;

use super::type_aliases::FenceFuture;

/// Holds all the necessary data required for rendering.
pub struct RenderContext {
    pub window: Arc<Window>,
    // pub render_command_buffers: Option<RefCell<RenderCommandBuffer>>,
    pub vs: EntryPoint,
    pub fs: EntryPoint,
    pub render_pass: Arc<RenderPass>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub swapchain: Arc<Swapchain>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    // Synchronization.
    pub fences: Vec<Option<Arc<FenceFuture>>>,
    pub frames_in_flight: usize,
    pub previous_fence_i: u32,
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

/// User-options for the application.
#[derive(Default)]
pub struct SimulationFlags {
    pub recreate_swapchain: bool,
    pub is_paused: bool,
}

/// Performance logging for debugging.
#[derive(Default)]
pub struct PerformanceStats {
    pub framerates: Vec<f32>,
}
impl PerformanceStats {
    pub fn avg(&self) -> f32 {
        self.framerates.iter().sum::<f32>() / (self.framerates.len() as f32)
    }
}
