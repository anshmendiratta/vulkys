use anyhow::Result;

use std::sync::Arc;

use vulkano::{
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        CommandBufferExecFuture, PrimaryAutoCommandBuffer,
    },
    swapchain::{PresentFuture, SwapchainAcquireFuture},
    sync::{
        future::{FenceSignalFuture, JoinFuture},
        GpuFuture,
    },
};

pub type SwapchainJoinFuture = JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>;
pub type FenceFuture =
    FenceSignalFuture<PresentFuture<CommandBufferExecFuture<SwapchainJoinFuture>>>;

pub type ComputeBufferBuilder = AutoCommandBufferBuilder<
    PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
    Arc<StandardCommandBufferAllocator>,
>;

pub type RenderCommandBuffer = Result<Vec<Arc<PrimaryAutoCommandBuffer>>>;
pub type ComputeCommandBuffer = Arc<PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>>;
