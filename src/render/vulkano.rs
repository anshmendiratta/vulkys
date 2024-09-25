#![allow(unused_imports)]

use std::sync::Arc;

use tracing::info;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{self, AutoCommandBufferBuilder, CopyBufferInfo};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryAllocator, StandardMemoryAllocator};
use vulkano::sync::GpuFuture;
use vulkano::{sync, VulkanLibrary};

pub fn make_vulkano_instance() -> Arc<Instance> {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    instance
}

pub fn make_vulkano_device() {
    let instance = make_vulkano_instance();
    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");
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
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let data_iter = (0..5u32).into_iter().collect::<Vec<u32>>();
    let data_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        vulkano::buffer::BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE
                | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data_iter,
    )
    .expect("Could not do thing for compute pipelines");
    let data_iter_dst = [0; 5].into_iter().collect::<Vec<u32>>();
    let destination_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        vulkano::buffer::BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE
                | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data_iter_dst,
    )
    .expect("Could not do thing for compute pipelines");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );
    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .copy_buffer(CopyBufferInfo::buffers(
            data_buffer.clone(),
            destination_buffer.clone(),
        ))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let src_content = data_buffer.read().unwrap();
    let dest_content = destination_buffer.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);

    info!("WORKED");
}
