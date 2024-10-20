use std::sync::Arc;

use vulkano::image::{Image, ImageUsage};
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
use vulkano::VulkanLibrary;

use super::core::WindowContext;
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::StandardMemoryAllocator;

pub fn create_swapchain_and_images(
    instance: Instance,
    win_ctx: Arc<&WindowContext>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    // let instance = ctx.instance();
    // let window = ctx.window();
    let surface = Surface::from_window(Arc::new(instance), win_ctx.window.clone())
        .expect("could not create window");
    let physical_device = select_physical_device(win_ctx.clone());
    let device = select_device_and_queues(win_ctx.clone()).0;
    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");

    let dimensions = win_ctx.window.inner_size();
    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = physical_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;

    let (mut swapchain, images) = Swapchain::new(
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

pub fn select_physical_device(win_ctx: Arc<&WindowContext>) -> Arc<PhysicalDevice> {
    let (window, event_loop) = (win_ctx.window(), win_ctx.event_loop());
    let required_extensions = Surface::required_extensions(&event_loop);
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let library = VulkanLibrary::new().expect("can't find local vulkan dll");
    let instance = win_ctx.instance();
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

pub fn select_device_and_queues(
    win_ctx: Arc<&WindowContext>,
) -> (
    Arc<Device>,
    u32,
    impl ExactSizeIterator<Item = Arc<vulkano::device::Queue>>,
) {
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

    let (device, queues) = Device::new(
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

    (device, queue_family_index, queues)
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
