use std::sync::Arc;

use vulkano::swapchain::Surface;
use vulkano::VulkanLibrary;

use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::StandardMemoryAllocator;

use super::lib::WindowContext;

pub fn select_physical_device(
    win_ctx: &WindowContext,
) -> (Arc<vulkano::device::physical::PhysicalDevice>, u32) {
    let (window, event_loop) = (win_ctx.window(), win_ctx.event_loop());
    let required_extensions = Surface::required_extensions(&event_loop);
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let library = VulkanLibrary::new().expect("can't find local vulkan dll");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .unwrap();

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
}

pub fn select_device_and_queues(
    win_ctx: WindowContext,
) -> (
    Arc<Device>,
    u32,
    impl ExactSizeIterator<Item = Arc<vulkano::device::Queue>>,
) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let physical_device = select_physical_device(&win_ctx);
    let queue_family_index = physical_device
        .0
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
        physical_device.0,
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
