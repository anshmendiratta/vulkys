#![allow(unused_variables)]

use std::sync::Arc;

use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageUsage};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
use vulkano::VulkanLibrary;
use winit::event_loop::EventLoop;

use super::vk_core::{VulkanoContext, WindowContext};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
};
use vulkano::instance::InstanceExtensions;
use vulkano::memory::allocator::StandardMemoryAllocator;

pub fn get_required_extensions(
    event_loop: &EventLoop<()>,
) -> (DeviceExtensions, InstanceExtensions) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let required_extensions = Surface::required_extensions(&event_loop);

    (device_extensions, required_extensions)
}

pub fn get_framebuffers(
    images: &Vec<Arc<Image>>,
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| -> Arc<Framebuffer> {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn create_swapchain_and_images(
    windowcx: &WindowContext,
    vulkancx: &VulkanoContext,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let surface = Surface::from_window(windowcx.instance.clone(), windowcx.window.clone())
        .expect("could not create window");
    let physical_device = select_physical_device(windowcx);
    let device = vulkancx.device.clone();
    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");
    let dimensions = windowcx.window.inner_size();
    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = physical_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;
    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent: dimensions.into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )
    .unwrap();

    (swapchain, images)
}

pub fn select_physical_device(win_ctx: &WindowContext) -> Arc<PhysicalDevice> {
    let (window, event_loop, instance) = (
        win_ctx.window(),
        win_ctx.event_loop(),
        win_ctx.instance.clone(),
    );
    let (device_extensions, _) = get_required_extensions(event_loop);
    let library = VulkanLibrary::new().expect("no local vulkan lib");
    let surface =
        Surface::from_window(instance.clone(), window.clone()).expect("could not create window");

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
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
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
        .0
}

pub fn select_device_and_queue(win_ctx: &WindowContext) -> (Arc<Device>, u32, Arc<Queue>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let physical_device = select_physical_device(win_ctx);
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

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

    (device, queue_family_index, queues.next().unwrap())
}

pub fn create_memory_allocator(
    device: Arc<Device>,
) -> Arc<
    vulkano::memory::allocator::GenericMemoryAllocator<
        vulkano::memory::allocator::FreeListAllocator,
    >,
> {
    Arc::new(StandardMemoryAllocator::new_default(device))
}

pub fn create_command_buffer_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
    StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    )
}
