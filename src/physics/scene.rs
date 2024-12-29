use super::rigidbody::RigidBody;
use crate::renderer::core::handler::RuntimeBuffers;
use crate::renderer::core::handler::{App, AppInitializationInfo};
use crate::renderer::primitives::{
    get_device_and_queue_info, required_extensions, select_physical_device,
};
use crate::renderer::shaders::update_cs;
use crate::{
    renderer::{core::CustomVertex, primitives::create_memory_allocator, procedural::Polygon},
    FVec2,
};

use ecolor::Color32;
use std::hash::RandomState;
use std::{collections::HashMap, sync::Arc};
use tracing::info;
use vulkano::buffer::{Buffer, Subbuffer};
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Queue;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{FreeListAllocator, GenericMemoryAllocator};
use vulkano::sync::{self, GpuFuture};
use vulkano::VulkanLibrary;
use vulkano::{
    buffer::{BufferCreateInfo, BufferUsage},
    device::Device,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};
use winit::event_loop::EventLoop;

pub struct Scene {
    pub objects: Vec<RigidBody>,
    objects_hash: HashMap<u8, (RigidBody, Polygon)>,
    dt: f32,
    gravity: f32,
    pub background_color: Color32,
}

#[derive(Clone)]
pub struct SceneInfo {
    pub objects: Vec<RigidBody>,
    pub dt: f32,
    pub gravity: f32,
    pub background_color: Color32,
}

impl Scene {
    /// Initializes a new scene with the `RigidBody`s passed in.
    pub fn with_info(scene_info: SceneInfo) -> Self {
        // NOTE: Correcting for the vulkan coordinate system: resetting to (0,0) being lower left.
        scene_info.objects.clone().iter_mut().for_each(|obj| {
            let mut current_position = obj.get_position();
            current_position.y *= -1.;
            let updated_position = FVec2::new(current_position.x, current_position.y);
            obj.update_position(updated_position);
        });

        let polygons: Vec<Polygon> = scene_info
            .objects
            .iter()
            .map(|body| body.to_polygon())
            .collect();

        let mut objects_hash: HashMap<u8, (RigidBody, Polygon)> =
            HashMap::with_capacity_and_hasher(scene_info.objects.len(), RandomState::new());
        for (rigidbody, polygon) in std::iter::zip(&scene_info.objects, polygons) {
            objects_hash.insert(rigidbody.get_id(), (rigidbody.clone(), polygon));
        }

        Self {
            objects: scene_info.objects,
            dt: scene_info.dt,
            objects_hash,
            gravity: scene_info.gravity,
            background_color: scene_info.background_color,
        }
    }

    pub fn return_compute_shader_buffers(
        &self,
        memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    ) -> RuntimeBuffers {
        let objects_positions = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            self.objects
                .clone()
                .iter()
                .map(|obj| obj.get_position().as_array()),
        )
        .unwrap();
        let objects_velocities = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            self.objects
                .clone()
                .iter()
                .map(|obj| obj.get_velocity().as_array()),
        )
        .unwrap();
        let objects_radii = Buffer::from_iter(
            memory_allocator,
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            self.objects
                .clone()
                .iter()
                .map(|obj| [obj.get_radius(), 0.]),
        )
        .unwrap();

        RuntimeBuffers {
            positions: objects_positions,
            velocities: objects_velocities,
            radii: objects_radii,
        }
    }

    pub fn return_objects_as_vertex_buffer(
        &self,
        device: Arc<Device>,
    ) -> Subbuffer<[CustomVertex]> {
        let vertex_buffer_data = {
            let mut buffer_data: Vec<CustomVertex> =
                Vec::with_capacity(self.objects_hash.len() * 3);
            for (_, (_, polygon)) in &self.objects_hash {
                buffer_data = [buffer_data, polygon.clone().into_flattened()].concat();
            }
            buffer_data
        };
        Buffer::from_iter(
            create_memory_allocator(device.clone()),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertex_buffer_data.clone(),
        )
        .expect("scene: could not produce vertex buffer from objects")
    }

    pub fn run(self) -> anyhow::Result<()> {
        // Setup.
        let event_loop = EventLoop::new().unwrap();
        let library = VulkanLibrary::new().expect("no local vulkan lib");
        let required_extensions = required_extensions(&event_loop);
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions.1,
                ..Default::default()
            },
        )
        .unwrap();
        let push_constants = update_cs::ComputeConstants {
            gravity: self.gravity,
            dt: self.dt,
            num_objects: self.objects.len() as u32,
        };
        let physical_device = select_physical_device(instance.clone(), &event_loop, None);
        let device_queue_info =
            get_device_and_queue_info(physical_device, instance.clone(), &event_loop);
        let memory_allocator = create_memory_allocator(device_queue_info.device.clone());

        // Initializing state.
        let initialization_info = AppInitializationInfo {
            device_and_queue: device_queue_info,
            instance,
            event_loop: &event_loop,
            runtime_buffers: self.return_compute_shader_buffers(memory_allocator),
            scene: self,
            push_constants,
        };
        let mut window_ctx_handler = App::new(initialization_info)?;

        //  Running window.
        event_loop.run_app(&mut window_ctx_handler)?;
        Ok(())
    }

    pub fn update_with_buffers(
        &mut self,
        device: Arc<Device>,
        queue: Arc<Queue>,
        compute_command_buffer: Arc<PrimaryAutoCommandBuffer>,
        runtime_buffers: RuntimeBuffers,
    ) {
        let future = sync::now(device.clone())
            .then_execute(queue.clone(), compute_command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        let binding = runtime_buffers.positions.clone();
        let object_positions_reader = binding.read().unwrap();
        let binding = runtime_buffers.velocities.clone();
        let object_velocities_reader = binding.read().unwrap();
        for (idx, (updated_position, updated_velocity)) in std::iter::zip(
            object_positions_reader.iter(),
            object_velocities_reader.iter(),
        )
        .enumerate()
        {
            self.objects[idx].update_position(updated_position.into());
            self.objects[idx].update_velocity(updated_velocity.into());
        }
        // self.check_and_world_resolve_collisions();
        self.recreate_hash();

        info!("{:?}", {
            self.objects
                .clone()
                .iter()
                .map(|obj| {
                    format!(
                        "id = {:?} with velocity = {:?}",
                        obj.get_id(),
                        obj.get_velocity()
                    )
                })
                .collect::<Vec<String>>()
        });
    }

    pub fn recreate_hash(&mut self) {
        let polygons: Vec<Polygon> = self.objects.iter().map(|body| body.to_polygon()).collect();

        let mut objects_as_hash: HashMap<u8, (RigidBody, Polygon)> =
            HashMap::with_capacity_and_hasher(self.objects.len(), RandomState::new());
        for (rigidbody, polygon) in std::iter::zip(&self.objects, polygons) {
            objects_as_hash.insert(rigidbody.get_id(), (rigidbody.clone(), polygon));
        }

        self.objects_hash = objects_as_hash;
    }

    pub fn recreate_objects_from_hash(&mut self) {
        self.objects.clear();
        for (rigidbody, _) in self.objects_hash.values() {
            self.objects.push(rigidbody.clone());
        }
    }
}
