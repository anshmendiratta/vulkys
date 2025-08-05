use std::sync::Arc;

use vulkano::{
    command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer},
    swapchain::{PresentFuture, SwapchainAcquireFuture},
    sync::{
        GpuFuture,
        future::{FenceSignalFuture, JoinFuture},
    },
};

pub type SwapchainJoinFuture = JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>;
pub type FenceFuture =
    FenceSignalFuture<PresentFuture<CommandBufferExecFuture<SwapchainJoinFuture>>>;

pub type RenderCommandBuffer = Vec<Arc<PrimaryAutoCommandBuffer>>;
